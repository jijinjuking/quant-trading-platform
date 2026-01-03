//! # 币安 WebSocket 适配器 (Binance WebSocket Adapter)
//!
//! 实现 MarketExchangePort trait，连接币安 WebSocket 获取实时行情。
//!
//! ## 功能
//! - 连接币安 WebSocket（支持代理）
//! - 订阅 Trade 数据流
//! - 解析币安消息格式
//! - 转换为标准 MarketEvent
//! - 支持断线重连

use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio_socks::tcp::Socks5Stream;
use tokio_tungstenite::{
    client_async_tls, connect_async, MaybeTlsStream, WebSocketStream,
    tungstenite::{protocol::Message, Error as WsError},
};
use tracing::{debug, error, info, warn};
use url::Url;

use crate::domain::port::MarketExchangePort;
use shared::event::market_event::{MarketEvent, MarketEventData, MarketEventType, TradeData};

/// 币安 Trade 消息格式
#[derive(Debug, Deserialize)]
struct BinanceTradeMsg {
    /// 事件类型
    #[serde(rename = "e")]
    #[allow(dead_code)]
    event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    #[allow(dead_code)]
    event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    symbol: String,
    /// 成交 ID
    #[serde(rename = "t")]
    trade_id: i64,
    /// 成交价格
    #[serde(rename = "p")]
    price: String,
    /// 成交数量
    #[serde(rename = "q")]
    quantity: String,
    /// 买方订单 ID
    #[serde(rename = "b")]
    _buyer_order_id: i64,
    /// 卖方订单 ID
    #[serde(rename = "a")]
    _seller_order_id: i64,
    /// 成交时间
    #[serde(rename = "T")]
    _trade_time: i64,
    /// 买方是否为 maker
    #[serde(rename = "m")]
    is_buyer_maker: bool,
}

/// 币安 AggTrade 消息格式
#[derive(Debug, Deserialize)]
struct BinanceAggTradeMsg {
    /// 事件类型
    #[serde(rename = "e")]
    #[allow(dead_code)]
    event_type: String,
    /// 事件时间
    #[serde(rename = "E")]
    _event_time: i64,
    /// 交易对
    #[serde(rename = "s")]
    symbol: String,
    /// 聚合成交 ID
    #[serde(rename = "a")]
    agg_trade_id: i64,
    /// 成交价格
    #[serde(rename = "p")]
    price: String,
    /// 成交数量
    #[serde(rename = "q")]
    quantity: String,
    /// 买方是否为 maker
    #[serde(rename = "m")]
    is_buyer_maker: bool,
}

/// 币安 WebSocket 客户端状态
struct WsState {
    /// 是否已连接
    connected: bool,
    /// 已订阅的交易对
    subscribed_symbols: Vec<String>,
}

/// 币安 WebSocket 适配器
pub struct BinanceWebSocket {
    /// WebSocket URL
    ws_url: String,
    /// 代理地址（可选）
    proxy: Option<String>,
    /// 事件接收通道
    event_rx: Arc<RwLock<Option<mpsc::Receiver<MarketEvent>>>>,
    /// 事件发送通道
    event_tx: Arc<RwLock<Option<mpsc::Sender<MarketEvent>>>>,
    /// 状态
    state: Arc<RwLock<WsState>>,
}

impl BinanceWebSocket {
    /// 创建新的币安 WebSocket 客户端
    ///
    /// # 参数
    /// - `ws_url`: WebSocket URL，如 `wss://stream.binance.com:9443/ws`
    /// - `proxy`: 代理地址（可选），如 `http://127.0.0.1:4780`
    pub fn new(ws_url: String, proxy: Option<String>) -> Self {
        let (tx, rx) = mpsc::channel::<MarketEvent>(10000);
        
        Self {
            ws_url,
            proxy,
            event_rx: Arc::new(RwLock::new(Some(rx))),
            event_tx: Arc::new(RwLock::new(Some(tx))),
            state: Arc::new(RwLock::new(WsState {
                connected: false,
                subscribed_symbols: Vec::new(),
            })),
        }
    }

    /// 从环境变量创建
    pub fn from_env() -> Self {
        let ws_url = std::env::var("BINANCE_WS_URL")
            .unwrap_or_else(|_| "wss://stream.binance.com:9443/ws".to_string());
        let proxy = std::env::var("MARKET_DATA_PROXY").ok();
        
        Self::new(ws_url, proxy)
    }

    /// 构建订阅 URL
    fn build_subscribe_url(&self, symbols: &[String]) -> String {
        if symbols.is_empty() {
            return self.ws_url.clone();
        }

        // 构建 combined stream URL
        // 格式: wss://stream.binance.com:9443/stream?streams=btcusdt@trade/ethusdt@trade
        let streams: Vec<String> = symbols
            .iter()
            .map(|s| format!("{}@trade", s.to_lowercase()))
            .collect();
        
        let base = self.ws_url.trim_end_matches("/ws");
        format!("{}/stream?streams={}", base, streams.join("/"))
    }

    /// 解析币安消息
    fn parse_message(&self, text: &str) -> Option<MarketEvent> {
        // 尝试解析 combined stream 格式
        if let Ok(combined) = serde_json::from_str::<serde_json::Value>(text) {
            if let Some(data) = combined.get("data") {
                return self.parse_trade_data(data);
            }
        }

        // 尝试直接解析 trade 消息
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
            return self.parse_trade_data(&value);
        }

        None
    }

    /// 解析 trade 数据
    fn parse_trade_data(&self, value: &serde_json::Value) -> Option<MarketEvent> {
        let event_type = value.get("e")?.as_str()?;

        match event_type {
            "trade" => self.parse_trade_msg(value),
            "aggTrade" => self.parse_agg_trade_msg(value),
            _ => {
                debug!("忽略未知事件类型: {}", event_type);
                None
            }
        }
    }

    /// 解析 Trade 消息
    fn parse_trade_msg(&self, value: &serde_json::Value) -> Option<MarketEvent> {
        let msg: BinanceTradeMsg = serde_json::from_value(value.clone()).ok()?;
        
        let price = msg.price.parse::<Decimal>().ok()?;
        let quantity = msg.quantity.parse::<Decimal>().ok()?;

        Some(MarketEvent {
            event_type: MarketEventType::Trade,
            exchange: "binance".to_string(),
            symbol: msg.symbol.to_uppercase(),
            timestamp: Utc::now(),
            data: MarketEventData::Trade(TradeData {
                trade_id: msg.trade_id.to_string(),
                price,
                quantity,
                is_buyer_maker: msg.is_buyer_maker,
            }),
        })
    }

    /// 解析 AggTrade 消息
    fn parse_agg_trade_msg(&self, value: &serde_json::Value) -> Option<MarketEvent> {
        let msg: BinanceAggTradeMsg = serde_json::from_value(value.clone()).ok()?;
        
        let price = msg.price.parse::<Decimal>().ok()?;
        let quantity = msg.quantity.parse::<Decimal>().ok()?;

        Some(MarketEvent {
            event_type: MarketEventType::Trade,
            exchange: "binance".to_string(),
            symbol: msg.symbol.to_uppercase(),
            timestamp: Utc::now(),
            data: MarketEventData::Trade(TradeData {
                trade_id: msg.agg_trade_id.to_string(),
                price,
                quantity,
                is_buyer_maker: msg.is_buyer_maker,
            }),
        })
    }

    /// 创建 WebSocket 连接（支持代理）
    async fn create_ws_connection(
        &self,
        url: &str,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let parsed_url = Url::parse(url).context("解析 WebSocket URL 失败")?;
        
        match &self.proxy {
            Some(proxy_url) if !proxy_url.is_empty() => {
                info!("使用代理连接: {}", proxy_url);
                self.connect_via_proxy(url, &parsed_url, proxy_url).await
            }
            _ => {
                info!("直接连接（无代理）");
                let (ws_stream, _) = connect_async(url)
                    .await
                    .context("连接 WebSocket 失败")?;
                Ok(ws_stream)
            }
        }
    }

    /// 通过代理连接 WebSocket
    async fn connect_via_proxy(
        &self,
        url: &str,
        parsed_url: &Url,
        proxy_url: &str,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        // 解析代理地址
        let proxy_parsed = Url::parse(proxy_url).context("解析代理 URL 失败")?;
        let proxy_host = proxy_parsed.host_str().context("代理缺少 host")?;
        let proxy_port = proxy_parsed.port().unwrap_or(1080);
        let proxy_addr = format!("{}:{}", proxy_host, proxy_port);

        // 目标地址
        let target_host = parsed_url.host_str().context("WebSocket URL 缺少 host")?;
        let target_port = parsed_url.port().unwrap_or(if parsed_url.scheme() == "wss" { 443 } else { 80 });

        info!(
            "代理连接: {} -> {}:{}",
            proxy_addr, target_host, target_port
        );

        // 根据代理类型选择连接方式
        let scheme = proxy_parsed.scheme().to_lowercase();
        
        if scheme == "socks5" || scheme == "socks" {
            // SOCKS5 代理
            let stream = Socks5Stream::connect(
                proxy_addr.as_str(),
                (target_host, target_port),
            )
            .await
            .context("SOCKS5 代理连接失败")?;

            let tcp_stream = stream.into_inner();
            let (ws_stream, _) = client_async_tls(url, tcp_stream)
                .await
                .context("WebSocket 握手失败")?;
            Ok(ws_stream)
        } else {
            // HTTP 代理 - 使用 CONNECT 方法
            let tcp_stream = TcpStream::connect(&proxy_addr)
                .await
                .context("连接 HTTP 代理失败")?;

            // 发送 CONNECT 请求
            let connect_req = format!(
                "CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\n\r\n",
                target_host, target_port, target_host, target_port
            );

            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let mut tcp_stream = tcp_stream;
            tcp_stream
                .write_all(connect_req.as_bytes())
                .await
                .context("发送 CONNECT 请求失败")?;

            // 读取响应
            let mut buf = [0u8; 1024];
            let n = tcp_stream
                .read(&mut buf)
                .await
                .context("读取代理响应失败")?;
            let response = String::from_utf8_lossy(&buf[..n]);

            if !response.contains("200") {
                return Err(anyhow::anyhow!("HTTP 代理连接失败: {}", response.trim()));
            }

            info!("HTTP 代理隧道建立成功");

            // 升级到 WebSocket
            let (ws_stream, _) = client_async_tls(url, tcp_stream)
                .await
                .context("WebSocket 握手失败")?;
            Ok(ws_stream)
        }
    }

    /// 启动 WebSocket 连接循环（内部使用）
    async fn run_ws_loop(&self, symbols: Vec<String>) -> Result<()> {
        let url = self.build_subscribe_url(&symbols);
        info!("连接币安 WebSocket: {}", url);

        // 获取发送通道
        let tx = {
            let guard = self.event_tx.read().await;
            guard.clone()
        };

        let tx = match tx {
            Some(tx) => tx,
            None => {
                return Err(anyhow::anyhow!("事件通道已关闭"));
            }
        };

        loop {
            match self.connect_and_receive(&url, &tx).await {
                Ok(_) => {
                    info!("WebSocket 连接正常关闭，5秒后重连...");
                }
                Err(e) => {
                    error!("WebSocket 错误: {}, 5秒后重连...", e);
                }
            }

            // 更新状态
            {
                let mut state = self.state.write().await;
                state.connected = false;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    /// 连接并接收消息
    async fn connect_and_receive(
        &self,
        url: &str,
        tx: &mpsc::Sender<MarketEvent>,
    ) -> Result<()> {
        // 根据是否有代理选择不同的连接方式
        let ws_stream = self.create_ws_connection(url).await?;

        info!("WebSocket 连接成功");

        // 更新状态
        {
            let mut state = self.state.write().await;
            state.connected = true;
        }

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
                    if let Some(event) = self.parse_message(&text) {
                        debug!(
                            "收到行情: {} @ {}",
                            event.symbol,
                            match &event.data {
                                MarketEventData::Trade(t) => t.price.to_string(),
                                _ => "N/A".to_string(),
                            }
                        );
                        
                        if tx.send(event).await.is_err() {
                            warn!("事件通道已关闭");
                            break;
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    debug!("收到 Ping");
                    // Pong 会自动发送
                    let _ = data;
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
}

#[async_trait]
impl MarketExchangePort for BinanceWebSocket {
    /// 连接交易所
    async fn connect(&self) -> Result<()> {
        info!("BinanceWebSocket connect() 调用");
        // 实际连接在 subscribe 时进行
        Ok(())
    }

    /// 订阅现货行情
    async fn subscribe_spot(&self, symbols: Vec<String>) -> Result<()> {
        info!("订阅现货行情: {:?}", symbols);
        
        // 保存订阅的交易对
        {
            let mut state = self.state.write().await;
            state.subscribed_symbols = symbols.clone();
        }

        // 启动 WebSocket 循环（在后台任务中）
        let self_clone = BinanceWebSocket {
            ws_url: self.ws_url.clone(),
            proxy: self.proxy.clone(),
            event_rx: Arc::clone(&self.event_rx),
            event_tx: Arc::clone(&self.event_tx),
            state: Arc::clone(&self.state),
        };

        tokio::spawn(async move {
            if let Err(e) = self_clone.run_ws_loop(symbols).await {
                error!("WebSocket 循环异常退出: {}", e);
            }
        });

        // 等待连接建立
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        Ok(())
    }

    /// 订阅合约行情
    async fn subscribe_futures(&self, symbols: Vec<String>) -> Result<()> {
        // 合约使用不同的 URL，暂时用现货实现
        info!("订阅合约行情: {:?}", symbols);
        self.subscribe_spot(symbols).await
    }

    /// 获取下一个行情事件
    async fn next_event(&self) -> Result<MarketEvent> {
        let mut rx_guard = self.event_rx.write().await;
        
        if let Some(ref mut rx) = *rx_guard {
            match rx.recv().await {
                Some(event) => Ok(event),
                None => Err(anyhow::anyhow!("事件通道已关闭")),
            }
        } else {
            Err(anyhow::anyhow!("事件接收器未初始化"))
        }
    }
}
