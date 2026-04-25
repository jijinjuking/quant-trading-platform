//! # 币安合约交易执行器 (Binance Futures Execution)
//!
//! 负责调用币安合约 API 进行开仓、平仓、杠杆管理等操作。
//!
//! ## 功能
//! - 合约开仓/平仓
//! - 杠杆设置
//! - 保证金模式切换（全仓/逐仓）
//! - 持仓模式切换（单向/双向）
//! - 止损止盈设置

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rust_decimal::Decimal;
use sha2::Sha256;
use tracing::{info, warn};

use crate::domain::port::execution_port::{
    ExecutionCommand, ExecutionPort, ExecutionResult, OrderSide, OrderType,
};
use crate::infrastructure::execution::{RateLimiter, RetryPolicy};

type HmacSha256 = Hmac<Sha256>;

/// 持仓方向（合约专用）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    /// 多头（做多）
    Long,
    /// 空头（做空）
    Short,
    /// 双向持仓模式（默认）
    Both,
}

impl PositionSide {
    /// 转换为币安 API 字符串
    pub fn to_binance_str(&self) -> &'static str {
        match self {
            PositionSide::Long => "LONG",
            PositionSide::Short => "SHORT",
            PositionSide::Both => "BOTH",
        }
    }
}

/// 保证金模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginType {
    /// 全仓
    Cross,
    /// 逐仓
    Isolated,
}

impl MarginType {
    /// 转换为币安 API 字符串
    pub fn to_binance_str(&self) -> &'static str {
        match self {
            MarginType::Cross => "CROSSED",
            MarginType::Isolated => "ISOLATED",
        }
    }
}

/// 合约执行指令
#[derive(Debug, Clone)]
pub struct FuturesCommand {
    /// 基础执行指令
    pub base: ExecutionCommand,
    /// 持仓方向（仅双向持仓模式需要）
    pub position_side: Option<PositionSide>,
    /// 是否只减仓（平仓时使用）
    pub reduce_only: bool,
}

impl FuturesCommand {
    /// 创建开仓指令（市价单）
    pub fn open_market(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        position_side: Option<PositionSide>,
    ) -> Self {
        Self {
            base: ExecutionCommand::market(symbol, side, quantity),
            position_side,
            reduce_only: false,
        }
    }

    /// 创建开仓指令（限价单）
    pub fn open_limit(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Decimal,
        position_side: Option<PositionSide>,
    ) -> Self {
        Self {
            base: ExecutionCommand::limit(symbol, side, quantity, price),
            position_side,
            reduce_only: false,
        }
    }

    /// 创建平仓指令（市价单）
    pub fn close_market(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        position_side: Option<PositionSide>,
    ) -> Self {
        Self {
            base: ExecutionCommand::market(symbol, side, quantity),
            position_side,
            reduce_only: true,
        }
    }

    /// 创建平仓指令（限价单）
    pub fn close_limit(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Decimal,
        position_side: Option<PositionSide>,
    ) -> Self {
        Self {
            base: ExecutionCommand::limit(symbol, side, quantity, price),
            position_side,
            reduce_only: true,
        }
    }
}

/// 币安合约执行器
pub struct BinanceFuturesExecution {
    api_key: String,
    secret_key: String,
    base_url: String,
    client: Client,
    rate_limiter: RateLimiter,
    retry_policy: RetryPolicy,
}

impl BinanceFuturesExecution {
    /// 创建合约执行器实例
    pub fn new(api_key: String, secret_key: String, base_url: String) -> Self {
        Self {
            api_key,
            secret_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
            rate_limiter: RateLimiter::default(),
            retry_policy: RetryPolicy::default(),
        }
    }

    /// 创建合约执行器实例（带自定义配置）
    pub fn with_config(
        api_key: String,
        secret_key: String,
        base_url: String,
        rate_limiter: RateLimiter,
        retry_policy: RetryPolicy,
    ) -> Self {
        Self {
            api_key,
            secret_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
            rate_limiter,
            retry_policy,
        }
    }

    /// HMAC-SHA256 签名
    fn sign(&self, query: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .context("invalid binance secret key")?;
        mac.update(query.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// 获取当前时间戳（毫秒）
    fn now_millis() -> Result<u128> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("system time before unix epoch")?;
        Ok(duration.as_millis())
    }

    /// 执行合约订单
    pub async fn execute_futures(&self, command: &FuturesCommand) -> Result<ExecutionResult> {
        self.retry_policy
            .execute_with_retry(
                || self.execute_futures_internal(command),
                "binance_futures_order",
            )
            .await
    }

    /// 内部合约下单方法
    async fn execute_futures_internal(&self, command: &FuturesCommand) -> Result<ExecutionResult> {
        // 先获取限流令牌
        self.rate_limiter.acquire(1).await;

        info!(
            symbol = %command.base.symbol,
            side = ?command.base.side,
            order_type = ?command.base.order_type,
            quantity = %command.base.quantity,
            position_side = ?command.position_side,
            reduce_only = command.reduce_only,
            "Executing futures order"
        );

        let symbol = command.base.symbol.trim().to_uppercase();
        let side = command.base.side.to_binance_str();
        let order_type = command.base.order_type.to_binance_str();
        let quantity = command.base.quantity.to_string();

        // 构建查询参数
        let mut params = vec![
            format!("symbol={}", symbol),
            format!("side={}", side),
            format!("type={}", order_type),
        ];

        // 添加持仓方向（双向持仓模式需要）
        if let Some(position_side) = command.position_side {
            params.push(format!("positionSide={}", position_side.to_binance_str()));
        }

        // 添加数量参数
        params.push(format!("quantity={}", quantity));

        // 添加只减仓标志
        if command.reduce_only {
            params.push("reduceOnly=true".to_string());
        }

        // 根据订单类型添加额外参数
        match command.base.order_type {
            OrderType::Market => {
                // 市价单不需要额外参数
            }
            OrderType::Limit => {
                let price = command.base.price.context("Limit order requires price")?;
                params.push(format!("price={}", price));

                let tif = command.base.time_in_force.unwrap_or(crate::domain::port::execution_port::TimeInForce::GTC);
                params.push(format!("timeInForce={}", tif.to_binance_str()));
            }
            OrderType::StopLossLimit | OrderType::TakeProfitLimit => {
                let price = command.base.price.context("Stop limit order requires price")?;
                let stop_price = command.base.stop_price.context("Stop limit order requires stop price")?;

                params.push(format!("price={}", price));
                params.push(format!("stopPrice={}", stop_price));

                let tif = command.base.time_in_force.unwrap_or(crate::domain::port::execution_port::TimeInForce::GTC);
                params.push(format!("timeInForce={}", tif.to_binance_str()));
            }
            OrderType::StopLossMarket | OrderType::TakeProfitMarket => {
                let stop_price = command.base.stop_price.context("Stop market order requires stop price")?;
                params.push(format!("stopPrice={}", stop_price));
            }
        }

        // 添加客户端订单 ID（如果有）
        if let Some(ref client_order_id) = command.base.client_order_id {
            params.push(format!("newClientOrderId={}", client_order_id));
        }

        // 添加时间戳
        let timestamp = Self::now_millis()?;
        params.push(format!("timestamp={}", timestamp));

        // 构建查询字符串
        let query = params.join("&");
        let signature = self.sign(&query)?;
        let url = format!("{}/fapi/v1/order?{}&signature={}", self.base_url, query, signature);

        // 发送请求
        let response = self
            .client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("binance futures order request failed")?;

        let status = response.status();
        let body = match response.text().await {
            Ok(text) => text,
            Err(err) => format!("<failed to read response body: {}>", err),
        };

        if !status.is_success() {
            warn!(
                symbol = %symbol,
                side = %side,
                order_type = %order_type,
                status = %status,
                body = %body,
                "Futures order failed"
            );
            return Err(anyhow!(
                "binance futures order failed: status={} body={}",
                status,
                body
            ));
        }

        // 解析响应
        let json: serde_json::Value = serde_json::from_str(&body)
            .context("failed to parse binance response")?;

        let order_id = json["orderId"]
            .as_i64()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let client_order_id = json["clientOrderId"]
            .as_str()
            .map(|s| s.to_string());

        let order_status = json["status"]
            .as_str()
            .unwrap_or("UNKNOWN")
            .to_string();

        let executed_qty = json["executedQty"]
            .as_str()
            .and_then(|s| s.parse::<Decimal>().ok())
            .unwrap_or(Decimal::ZERO);

        let avg_price = json["avgPrice"]
            .as_str()
            .and_then(|s| s.parse::<Decimal>().ok());

        info!(
            symbol = %symbol,
            side = %side,
            order_type = %order_type,
            order_id = %order_id,
            status = %order_status,
            executed_qty = %executed_qty,
            "Futures order accepted"
        );

        Ok(ExecutionResult {
            order_id,
            client_order_id,
            symbol,
            status: order_status,
            executed_qty,
            avg_price,
        })
    }

    /// 设置杠杆倍数
    ///
    /// # 参数
    /// - `symbol`: 交易对
    /// - `leverage`: 杠杆倍数（1-125）
    pub async fn set_leverage(&self, symbol: &str, leverage: u32) -> Result<()> {
        self.rate_limiter.acquire(1).await;

        let symbol = symbol.trim().to_uppercase();
        let timestamp = Self::now_millis()?;
        let query = format!("symbol={}&leverage={}&timestamp={}", symbol, leverage, timestamp);
        let signature = self.sign(&query)?;
        let url = format!("{}/fapi/v1/leverage?{}&signature={}", self.base_url, query, signature);

        let response = self
            .client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("set leverage request failed")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("set leverage failed: status={} body={}", status, body));
        }

        info!(symbol = %symbol, leverage = leverage, "Leverage set successfully");
        Ok(())
    }

    /// 切换保证金模式
    ///
    /// # 参数
    /// - `symbol`: 交易对
    /// - `margin_type`: 保证金模式
    pub async fn set_margin_type(&self, symbol: &str, margin_type: MarginType) -> Result<()> {
        self.rate_limiter.acquire(1).await;

        let symbol = symbol.trim().to_uppercase();
        let margin_type_str = margin_type.to_binance_str();
        let timestamp = Self::now_millis()?;
        let query = format!("symbol={}&marginType={}&timestamp={}", symbol, margin_type_str, timestamp);
        let signature = self.sign(&query)?;
        let url = format!("{}/fapi/v1/marginType?{}&signature={}", self.base_url, query, signature);

        let response = self
            .client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("set margin type request failed")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("set margin type failed: status={} body={}", status, body));
        }

        info!(symbol = %symbol, margin_type = ?margin_type, "Margin type set successfully");
        Ok(())
    }

    /// 切换持仓模式（单向/双向）
    ///
    /// # 参数
    /// - `dual_side`: true=双向持仓，false=单向持仓
    pub async fn set_position_mode(&self, dual_side: bool) -> Result<()> {
        self.rate_limiter.acquire(1).await;

        let timestamp = Self::now_millis()?;
        let query = format!("dualSidePosition={}&timestamp={}", dual_side, timestamp);
        let signature = self.sign(&query)?;
        let url = format!("{}/fapi/v1/positionSide/dual?{}&signature={}", self.base_url, query, signature);

        let response = self
            .client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("set position mode request failed")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("set position mode failed: status={} body={}", status, body));
        }

        info!(dual_side = dual_side, "Position mode set successfully");
        Ok(())
    }
}

// 为合约执行器实现 ExecutionPort（使用基础指令）
#[async_trait]
impl ExecutionPort for BinanceFuturesExecution {
    async fn execute(&self, command: &ExecutionCommand) -> Result<ExecutionResult> {
        // 将基础指令转换为合约指令（默认单向持仓）
        let futures_command = FuturesCommand {
            base: command.clone(),
            position_side: None,
            reduce_only: false,
        };
        self.execute_futures(&futures_command).await
    }
}
