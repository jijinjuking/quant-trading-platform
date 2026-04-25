//! # 币安真实下单执行器 (Binance Execution Adapter) - v2
//!
//! 负责调用币安 REST API 进行下单。
//! 仅做协议适配与外部交互，不包含任何业务规则。
//!
//! ## v2 新增功能
//! - 支持多种订单类型（市价、限价、止损、止盈）
//! - 返回详细的执行结果
//! - 更完善的错误处理

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rust_decimal::Decimal;
use sha2::Sha256;
use tracing::{info, warn};

use crate::domain::port::execution_port::{
    ExecutionCommand, ExecutionPort, ExecutionResult, OrderType,
};
use crate::infrastructure::execution::{RateLimiter, RetryPolicy};

type HmacSha256 = Hmac<Sha256>;

/// 币安真实下单执行器
pub struct BinanceExecution {
    api_key: String,
    secret_key: String,
    base_url: String,
    client: Client,
    /// API 限流器
    rate_limiter: RateLimiter,
    /// 重试策略
    retry_policy: RetryPolicy,
}

impl BinanceExecution {
    /// 创建执行器实例
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

    /// 创建执行器实例（带自定义限流和重试配置）
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

    fn now_millis() -> Result<u128> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("system time before unix epoch")?;
        Ok(duration.as_millis())
    }
}

#[async_trait]
impl ExecutionPort for BinanceExecution {
    async fn execute(&self, command: &ExecutionCommand) -> Result<ExecutionResult> {
        // 使用重试策略执行下单
        self.retry_policy
            .execute_with_retry(
                || self.execute_order_internal(command),
                "binance_order",
            )
            .await
    }
}

impl BinanceExecution {
    /// 内部下单方法（带限流）
    async fn execute_order_internal(&self, command: &ExecutionCommand) -> Result<ExecutionResult> {
        // 先获取限流令牌
        self.rate_limiter.acquire(1).await;

        info!(
            symbol = %command.symbol,
            side = ?command.side,
            order_type = ?command.order_type,
            quantity = %command.quantity,
            "Executing order"
        );

        let symbol = command.symbol.trim().to_uppercase();
        let side = command.side.to_binance_str();
        let order_type = command.order_type.to_binance_str();
        let quantity = command.quantity.to_string();

        // 构建查询参数
        let mut params = vec![
            format!("symbol={}", symbol),
            format!("side={}", side),
            format!("type={}", order_type),
        ];

        // 添加数量参数
        params.push(format!("quantity={}", quantity));

        // 根据订单类型添加额外参数
        match command.order_type {
            OrderType::Market => {
                // 市价单不需要额外参数
            }
            OrderType::Limit => {
                // 限价单需要价格和时间有效性
                let price = command.price.context("Limit order requires price")?;
                params.push(format!("price={}", price));

                let tif = command.time_in_force.unwrap_or(crate::domain::port::execution_port::TimeInForce::GTC);
                params.push(format!("timeInForce={}", tif.to_binance_str()));
            }
            OrderType::StopLossLimit | OrderType::TakeProfitLimit => {
                // 止损/止盈限价单需要价格、止损价和时间有效性
                let price = command.price.context("Stop limit order requires price")?;
                let stop_price = command.stop_price.context("Stop limit order requires stop price")?;

                params.push(format!("price={}", price));
                params.push(format!("stopPrice={}", stop_price));

                let tif = command.time_in_force.unwrap_or(crate::domain::port::execution_port::TimeInForce::GTC);
                params.push(format!("timeInForce={}", tif.to_binance_str()));
            }
            OrderType::StopLossMarket | OrderType::TakeProfitMarket => {
                // 止损/止盈市价单只需要止损价
                let stop_price = command.stop_price.context("Stop market order requires stop price")?;
                params.push(format!("stopPrice={}", stop_price));
            }
        }

        // 添加客户端订单 ID（如果有）
        if let Some(ref client_order_id) = command.client_order_id {
            params.push(format!("newClientOrderId={}", client_order_id));
        }

        // 添加时间戳
        let timestamp = Self::now_millis()?;
        params.push(format!("timestamp={}", timestamp));

        // 构建查询字符串
        let query = params.join("&");
        let signature = self.sign(&query)?;
        let url = format!("{}/api/v3/order?{}&signature={}", self.base_url, query, signature);

        // 发送请求
        let response = self
            .client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("binance order request failed")?;

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
                "Order failed"
            );
            return Err(anyhow!(
                "binance order failed: status={} body={}",
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
            .and_then(|s| s.parse::<Decimal>().ok())
            .or_else(|| {
                // 如果没有 avgPrice，尝试使用 price
                json["price"]
                    .as_str()
                    .and_then(|s| s.parse::<Decimal>().ok())
            });

        info!(
            symbol = %symbol,
            side = %side,
            order_type = %order_type,
            order_id = %order_id,
            status = %order_status,
            executed_qty = %executed_qty,
            "Order accepted"
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
}
