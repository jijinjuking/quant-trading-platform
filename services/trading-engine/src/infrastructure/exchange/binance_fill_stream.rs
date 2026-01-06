//! # 币安成交事件流 (Binance Fill Stream)
//!
//! 路径: services/trading-engine/src/infrastructure/exchange/binance_fill_stream.rs
//!
//! ## 职责
//! 连接 Binance User Data Stream，监听 ORDER_TRADE_UPDATE 事件，
//! 将真实成交转换为 ExecutionFill，通过 channel 发送给 ExecutionService。
//!
//! ## 架构位置
//! - 所属层级: Infrastructure Layer
//! - 不实现任何 Port trait（纯基础设施组件）
//!
//! ## 禁止
//! - ❌ 不做业务判断
//! - ❌ 不更新 RiskState
//! - ❌ 不做聚合
//! - ❌ 不做策略逻辑

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use futures_util::{SinkExt, StreamExt};
use hmac::{Hmac, Mac};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::Deserialize;
use sha2::Sha256;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{
    connect_async, MaybeTlsStream, WebSocketStream,
    tungstenite::{protocol::Message, Error as WsError},
};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::domain::model::execution_fill::{ExecutionFill, FillSide, FillType};

type HmacSha256 = Hmac<Sha256>;

/// 币安 User Data Stream 执行报告消息
#[derive(Debug, Deserialize)]
struct BinanceExecutionReport {
    /// 事件类型 (executionReport)
    #[serde(rename = "e")]
    event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    _event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    symbol: String,
    /// 客户端订单 ID
    #[serde(rename = "c")]
    client_order_id: String,
    /// 订单方向 (BUY/SELL)
    #[serde(rename = "S")]
    side: String,
    /// 订单类型 (LIMIT/MARKET)
    #[serde(rename = "o")]
    _order_type: String,
    /// 订单状态
    #[serde(rename = "X")]
    order_status: String,
    /// 订单 ID
    #[serde(rename = "i")]
    order_id: i64,
    /// 成交 ID（用于幂等判断）
    #[serde(rename = "t")]
    trade_id: i64,
    /// 最新成交数量
    #[serde(rename = "l")]
    last_executed_qty: String,
    /// 累计成交数量
    #[serde(rename = "z")]
    cumulative_qty: String,
    /// 最新成交价格
    #[serde(rename = "L")]
    last_executed_price: String,
    /// 手续费
    #[serde(rename = "n")]
    commission: String,
    /// 手续费资产
    #[serde(rename = "N")]
    commission_asset: Option<String>,
    /// 成交时间
    #[serde(rename = "T")]
    trade_time: i64,
    /// 原始订单数量
    #[serde(rename = "q")]
    original_qty: String,
}

/// 成交流配置
#[derive(Debug, Clone)]
pub struct FillStreamConfig {
    /// API Key
    pub api_key: String,
    /// Secret Key
    pub secret_key: String,
    /// REST API Base URL
    pub rest_base_url: String,
    /// WebSocket Base URL
    pub ws_base_url: String,
    /// 是否启用
    pub enabled: bool,
}

impl FillStreamConfig {
    /// 从环境变量创建配置
    pub fn from_env() -> Self {
        Self {
            api_key: std::env::var("BINANCE_API_KEY").unwrap_or_default(),
            secret_key: std::env::var("BINANCE_SECRET_KEY").unwrap_or_default(),
            rest_base_url: std::env::var("BINANCE_BASE_URL")
                .unwrap_or_else(|_| "https://testnet.binance.vision".to_string()),
            ws_base_url: std::env::var("BINANCE_WS_BASE_URL")
                .unwrap_or_else(|_| "wss://testnet.binance.vision/ws".to_string()),
            enabled: std::env::var("FILL_STREAM_ENABLED")
                .map(|v| v.to_lowercase() == "true" || v == "1")
                .unwrap_or(false),
        }
    }

    /// 检查配置是否有效
    pub fn is_valid(&self) -> bool {
        self.enabled && !self.api_key.is_empty() && !self.secret_key.is_empty()
    }
}

/// 币安成交事件流
///
/// 连接 Binance User Data Stream，监听成交事件。
/// 将成交事件转换为 ExecutionFill 并通过 channel 发送。
pub struct BinanceFillStream {
    config: FillStreamConfig,
    /// 成交事件发送通道
    fill_tx: mpsc::Sender<ExecutionFill>,
    /// Listen Key（用于 User Data Stream）
    listen_key: Arc<RwLock<Option<String>>>,
    /// HTTP 客户端
    client: Client,
    /// 重连回调（v1.1 安全修补: 用于重连后同步状态）
    on_reconnect: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl BinanceFillStream {
    /// 创建成交事件流
    ///
    /// # 参数
    /// - `config`: 配置
    /// - `fill_tx`: 成交事件发送通道
    pub fn new(config: FillStreamConfig, fill_tx: mpsc::Sender<ExecutionFill>) -> Self {
        Self {
            config,
            fill_tx,
            listen_key: Arc::new(RwLock::new(None)),
            client: Client::new(),
            on_reconnect: None,
        }
    }

    /// 从环境变量创建
    pub fn from_env(fill_tx: mpsc::Sender<ExecutionFill>) -> Self {
        Self::new(FillStreamConfig::from_env(), fill_tx)
    }

    /// 设置重连回调 (v1.1 安全修补)
    ///
    /// 当 WebSocket 重连成功后，会调用此回调。
    /// 用于触发 RiskStateCoordinator.notify_reconnect() 同步状态。
    ///
    /// # 注意
    /// 回调应调用 coordinator.notify_reconnect()，
    /// 禁止直接调用 RiskStateInitializer。
    pub fn with_reconnect_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_reconnect = Some(Arc::new(callback));
        self
    }

    /// 启动成交事件流（阻塞，内部循环重连）
    pub async fn run(&self) -> Result<()> {
        if !self.config.is_valid() {
            warn!("BinanceFillStream 配置无效或未启用，跳过启动");
            return Ok(());
        }

        info!("BinanceFillStream 启动中...");

        loop {
            match self.connect_and_listen().await {
                Ok(_) => {
                    info!("User Data Stream 连接正常关闭，5秒后重连...");
                }
                Err(e) => {
                    error!("User Data Stream 错误: {}, 5秒后重连...", e);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    /// 连接并监听成交事件
    async fn connect_and_listen(&self) -> Result<()> {
        // Step 1: 获取 Listen Key
        let listen_key = self.create_listen_key().await?;
        {
            let mut key_guard = self.listen_key.write().await;
            *key_guard = Some(listen_key.clone());
        }

        // Step 2: 构建 WebSocket URL
        let ws_url = format!("{}/{}", self.config.ws_base_url, listen_key);
        info!("连接 User Data Stream: {}", ws_url);

        // Step 3: 连接 WebSocket
        let (ws_stream, _) = connect_async(&ws_url)
            .await
            .context("连接 User Data Stream 失败")?;

        info!("User Data Stream 连接成功");

        // Step 3.5 (v1.1 安全修补): 重连后同步风控状态
        if let Some(ref callback) = self.on_reconnect {
            info!("触发重连回调，同步风控状态...");
            callback();
        }

        // Step 4: 启动 Listen Key 保活任务
        let keep_alive_handle = self.spawn_keep_alive_task();

        // Step 5: 接收消息
        self.receive_messages(ws_stream).await?;

        // 清理
        keep_alive_handle.abort();
        Ok(())
    }

    /// 创建 Listen Key
    async fn create_listen_key(&self) -> Result<String> {
        let url = format!("{}/api/v3/userDataStream", self.config.rest_base_url);

        let response = self
            .client
            .post(&url)
            .header("X-MBX-APIKEY", &self.config.api_key)
            .send()
            .await
            .context("创建 Listen Key 请求失败")?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "创建 Listen Key 失败: status={} body={}",
                status,
                body
            ));
        }

        let json: serde_json::Value =
            serde_json::from_str(&body).context("解析 Listen Key 响应失败")?;

        let listen_key = json["listenKey"]
            .as_str()
            .context("响应中缺少 listenKey")?
            .to_string();

        info!("获取 Listen Key 成功");
        Ok(listen_key)
    }

    /// 保活 Listen Key（每 30 分钟 PUT 一次）
    async fn keep_alive_listen_key(&self) -> Result<()> {
        let listen_key = {
            let guard = self.listen_key.read().await;
            guard.clone()
        };

        let Some(key) = listen_key else {
            return Ok(());
        };

        let url = format!(
            "{}/api/v3/userDataStream?listenKey={}",
            self.config.rest_base_url, key
        );

        let response = self
            .client
            .put(&url)
            .header("X-MBX-APIKEY", &self.config.api_key)
            .send()
            .await
            .context("保活 Listen Key 请求失败")?;

        if response.status().is_success() {
            debug!("Listen Key 保活成功");
        } else {
            warn!("Listen Key 保活失败: {}", response.status());
        }

        Ok(())
    }

    /// 启动 Listen Key 保活任务
    fn spawn_keep_alive_task(&self) -> tokio::task::JoinHandle<()> {
        let config = self.config.clone();
        let listen_key = Arc::clone(&self.listen_key);
        let client = self.client.clone();

        tokio::spawn(async move {
            loop {
                // 每 30 分钟保活一次
                tokio::time::sleep(tokio::time::Duration::from_secs(30 * 60)).await;

                let key = {
                    let guard = listen_key.read().await;
                    guard.clone()
                };

                if let Some(key) = key {
                    let url = format!(
                        "{}/api/v3/userDataStream?listenKey={}",
                        config.rest_base_url, key
                    );

                    match client
                        .put(&url)
                        .header("X-MBX-APIKEY", &config.api_key)
                        .send()
                        .await
                    {
                        Ok(resp) if resp.status().is_success() => {
                            debug!("Listen Key 保活成功");
                        }
                        Ok(resp) => {
                            warn!("Listen Key 保活失败: {}", resp.status());
                        }
                        Err(e) => {
                            error!("Listen Key 保活请求错误: {}", e);
                        }
                    }
                }
            }
        })
    }

    /// 接收并处理消息
    async fn receive_messages(
        &self,
        ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<()> {
        let (mut write, mut read) = ws_stream.split();

        // 发送 ping 保持连接
        let ping_handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                if write.send(Message::Ping(vec![])).await.is_err() {
                    break;
                }
            }
        });

        // 接收消息
        while let Some(msg_result) = read.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    self.handle_message(&text).await;
                }
                Ok(Message::Ping(_)) => {
                    debug!("收到 Ping");
                }
                Ok(Message::Pong(_)) => {
                    debug!("收到 Pong");
                }
                Ok(Message::Close(_)) => {
                    info!("收到关闭消息");
                    break;
                }
                Ok(_) => {}
                Err(WsError::ConnectionClosed) => {
                    info!("连接已关闭");
                    break;
                }
                Err(e) => {
                    error!("WebSocket 错误: {}", e);
                    break;
                }
            }
        }

        ping_handle.abort();
        Ok(())
    }

    /// 处理单条消息
    async fn handle_message(&self, text: &str) {
        // 尝试解析为执行报告
        let value: serde_json::Value = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(e) => {
                debug!("解析消息失败: {}", e);
                return;
            }
        };

        // 检查事件类型
        let event_type = value.get("e").and_then(|v| v.as_str()).unwrap_or("");

        match event_type {
            "executionReport" => {
                self.handle_execution_report(&value).await;
            }
            "outboundAccountPosition" => {
                debug!("收到账户更新事件（忽略）");
            }
            "balanceUpdate" => {
                debug!("收到余额更新事件（忽略）");
            }
            _ => {
                debug!("收到未知事件类型: {}", event_type);
            }
        }
    }

    /// 处理执行报告（成交事件）
    async fn handle_execution_report(&self, value: &serde_json::Value) {
        let report: BinanceExecutionReport = match serde_json::from_value(value.clone()) {
            Ok(r) => r,
            Err(e) => {
                error!("解析执行报告失败: {}", e);
                return;
            }
        };

        // 只处理成交相关状态
        // TRADE = 有成交发生
        // FILLED = 完全成交
        // PARTIALLY_FILLED = 部分成交
        let order_status = report.order_status.as_str();
        if order_status != "TRADE" && order_status != "FILLED" && order_status != "PARTIALLY_FILLED"
        {
            debug!(
                "忽略非成交状态: order_id={}, status={}",
                report.order_id, order_status
            );
            return;
        }

        // 转换为 ExecutionFill
        let fill = match self.convert_to_fill(&report) {
            Some(f) => f,
            None => {
                error!("转换 ExecutionFill 失败: {:?}", report);
                return;
            }
        };

        info!(
            "收到成交事件: order_id={}, symbol={}, side={:?}, qty={}, price={}",
            fill.order_id, fill.symbol, fill.side, fill.filled_quantity, fill.fill_price
        );

        // 发送到 channel
        if let Err(e) = self.fill_tx.send(fill).await {
            error!("发送成交事件失败: {}", e);
        }
    }

    /// 将币安执行报告转换为 ExecutionFill
    fn convert_to_fill(&self, report: &BinanceExecutionReport) -> Option<ExecutionFill> {
        // 解析方向
        let side = FillSide::from_str(&report.side)?;

        // 解析数量和价格
        let filled_qty: Decimal = report.last_executed_qty.parse().ok()?;
        let fill_price: Decimal = report.last_executed_price.parse().ok()?;
        let cumulative_qty: Decimal = report.cumulative_qty.parse().ok()?;
        let original_qty: Decimal = report.original_qty.parse().ok()?;
        let commission: Decimal = report.commission.parse().unwrap_or(Decimal::ZERO);

        // 判断成交类型
        let fill_type = if cumulative_qty >= original_qty {
            FillType::Full
        } else {
            FillType::Partial
        };

        // 解析成交时间
        let fill_time = Utc
            .timestamp_millis_opt(report.trade_time)
            .single()
            .unwrap_or_else(Utc::now);

        Some(ExecutionFill {
            id: Uuid::new_v4(),
            order_id: report.order_id.to_string(),
            trade_id: report.trade_id.to_string(),
            client_order_id: Some(report.client_order_id.clone()),
            symbol: report.symbol.clone(),
            side,
            fill_type,
            filled_quantity: filled_qty,
            fill_price,
            cumulative_quantity: cumulative_qty,
            original_quantity: original_qty,
            commission,
            commission_asset: report
                .commission_asset
                .clone()
                .unwrap_or_else(|| "USDT".to_string()),
            fill_time,
            created_at: Utc::now(),
        })
    }
}

/// 创建成交事件流和接收通道
///
/// # 返回
/// - `(BinanceFillStream, mpsc::Receiver<ExecutionFill>)`
pub fn create_fill_stream() -> (BinanceFillStream, mpsc::Receiver<ExecutionFill>) {
    let (tx, rx) = mpsc::channel::<ExecutionFill>(1000);
    let stream = BinanceFillStream::from_env(tx);
    (stream, rx)
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap_or_default()
    }

    #[test]
    fn test_config_from_env_disabled() {
        // 默认情况下应该是禁用的
        let config = FillStreamConfig::from_env();
        // 如果没有设置环境变量，应该是无效的
        assert!(!config.is_valid() || config.api_key.is_empty());
    }

    #[test]
    fn test_convert_execution_report_buy() {
        let (tx, _rx) = mpsc::channel(10);
        let stream = BinanceFillStream::new(
            FillStreamConfig {
                api_key: "test".to_string(),
                secret_key: "test".to_string(),
                rest_base_url: "https://test".to_string(),
                ws_base_url: "wss://test".to_string(),
                enabled: true,
            },
            tx,
        );

        let report = BinanceExecutionReport {
            event_type: "executionReport".to_string(),
            _event_time: 1704067200000,
            symbol: "BTCUSDT".to_string(),
            client_order_id: "client123".to_string(),
            side: "BUY".to_string(),
            _order_type: "LIMIT".to_string(),
            order_status: "FILLED".to_string(),
            order_id: 12345,
            trade_id: 99001,
            last_executed_qty: "0.1".to_string(),
            cumulative_qty: "0.1".to_string(),
            last_executed_price: "50000".to_string(),
            commission: "5".to_string(),
            commission_asset: Some("USDT".to_string()),
            trade_time: 1704067200000,
            original_qty: "0.1".to_string(),
        };

        let fill = stream.convert_to_fill(&report).unwrap();

        assert_eq!(fill.order_id, "12345");
        assert_eq!(fill.symbol, "BTCUSDT");
        assert_eq!(fill.side, FillSide::Buy);
        assert_eq!(fill.fill_type, FillType::Full);
        assert_eq!(fill.filled_quantity, dec("0.1"));
        assert_eq!(fill.fill_price, dec("50000"));
        assert_eq!(fill.commission, dec("5"));
    }

    #[test]
    fn test_convert_execution_report_partial() {
        let (tx, _rx) = mpsc::channel(10);
        let stream = BinanceFillStream::new(
            FillStreamConfig {
                api_key: "test".to_string(),
                secret_key: "test".to_string(),
                rest_base_url: "https://test".to_string(),
                ws_base_url: "wss://test".to_string(),
                enabled: true,
            },
            tx,
        );

        let report = BinanceExecutionReport {
            event_type: "executionReport".to_string(),
            _event_time: 1704067200000,
            symbol: "ETHUSDT".to_string(),
            client_order_id: "client456".to_string(),
            side: "SELL".to_string(),
            _order_type: "LIMIT".to_string(),
            order_status: "PARTIALLY_FILLED".to_string(),
            order_id: 67890,
            trade_id: 99002,
            last_executed_qty: "1.0".to_string(),
            cumulative_qty: "1.0".to_string(),
            last_executed_price: "3000".to_string(),
            commission: "3".to_string(),
            commission_asset: Some("USDT".to_string()),
            trade_time: 1704067200000,
            original_qty: "2.0".to_string(),
        };

        let fill = stream.convert_to_fill(&report).unwrap();

        assert_eq!(fill.order_id, "67890");
        assert_eq!(fill.symbol, "ETHUSDT");
        assert_eq!(fill.side, FillSide::Sell);
        assert_eq!(fill.fill_type, FillType::Partial);
        assert_eq!(fill.filled_quantity, dec("1.0"));
        assert_eq!(fill.original_quantity, dec("2.0"));
        assert_eq!(fill.cumulative_quantity, dec("1.0"));
    }
}
