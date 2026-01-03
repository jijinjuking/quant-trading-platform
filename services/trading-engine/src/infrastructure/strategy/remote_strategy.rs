//! # 远程策略适配器 (Remote Strategy Adapter)
//!
//! 通过 HTTP 调用 strategy-engine 服务获取交易意图。

use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::event::market_event::MarketEvent;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::domain::model::order_intent::{OrderIntent, OrderSide};
use crate::domain::port::strategy_port::StrategyPort;

/// 远程策略适配器
pub struct RemoteStrategy {
    base_url: String,
    client: Client,
}

impl RemoteStrategy {
    /// 创建远程策略适配器
    ///
    /// # 参数
    /// - `base_url`: strategy-engine 服务地址
    pub fn new(base_url: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }
}

/// 策略评估请求
#[derive(Debug, Serialize)]
struct EvaluateRequest<'a> {
    event: &'a MarketEvent,
}

/// 策略评估响应
#[derive(Debug, Deserialize)]
struct EvaluateResponse {
    has_intent: bool,
    intent: Option<IntentDto>,
}

/// 交易意图 DTO
#[derive(Debug, Deserialize)]
struct IntentDto {
    strategy_id: Uuid,
    symbol: String,
    side: String,
    quantity: Decimal,
    price: Option<Decimal>,
    confidence: f64,
}

#[async_trait]
impl StrategyPort for RemoteStrategy {
    async fn evaluate(&self, event: &MarketEvent) -> anyhow::Result<Option<OrderIntent>> {
        let url = format!("{}/api/v1/strategy/evaluate", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&EvaluateRequest { event })
            .send()
            .await
            .context("failed to call strategy-engine")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(
                status = %status,
                body = %body,
                "strategy-engine returned error"
            );
            return Ok(None);
        }

        let result: EvaluateResponse = response
            .json()
            .await
            .context("failed to parse strategy response")?;

        if !result.has_intent {
            debug!(symbol = %event.symbol, "No intent from strategy-engine");
            return Ok(None);
        }

        let dto = match result.intent {
            Some(dto) => dto,
            None => return Ok(None),
        };

        let side = match dto.side.to_lowercase().as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => {
                warn!(side = %dto.side, "Unknown order side from strategy");
                return Ok(None);
            }
        };

        Ok(Some(OrderIntent::new(
            dto.strategy_id,
            dto.symbol,
            side,
            dto.quantity,
            dto.price,
            dto.confidence,
        )))
    }
}
