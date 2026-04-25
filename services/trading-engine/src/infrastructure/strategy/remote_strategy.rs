//! Remote strategy adapter

use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::event::market_event::{MarketEvent, MarketEventData};
use tracing::{debug, warn};
use uuid::Uuid;

use crate::domain::model::order_intent::{OrderIntent, OrderSide};
use crate::domain::port::strategy_port::StrategyPort;

pub struct RemoteStrategy {
    base_url: String,
    client: Client,
}

impl RemoteStrategy {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct EvaluateRequest {
    strategy_id: Uuid,
    symbol: String,
    price: Decimal,
    quantity: Decimal,
    timestamp: i64,
    is_buyer_maker: bool,
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EvaluateResponse {
    has_intent: bool,
    intent: Option<IntentDto>,
}

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
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return Ok(None),
        };

        let req = EvaluateRequest {
            strategy_id: Uuid::nil(),
            symbol: event.symbol.clone(),
            price: trade.price,
            quantity: trade.quantity,
            timestamp: event.timestamp.timestamp_millis(),
            is_buyer_maker: trade.is_buyer_maker,
        };

        let url = format!("{}/api/v1/strategy/evaluate", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("failed to call strategy-engine")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "strategy-engine returned non-success");
            return Ok(None);
        }

        let body: ApiResponse<EvaluateResponse> = response
            .json()
            .await
            .context("failed to parse strategy response")?;

        if !body.success {
            warn!(error = ?body.error, "strategy-engine business response failed");
            return Ok(None);
        }

        let result = match body.data {
            Some(v) => v,
            None => return Ok(None),
        };

        if !result.has_intent {
            debug!(symbol = %event.symbol, "no intent from strategy-engine");
            return Ok(None);
        }

        let dto = match result.intent {
            Some(v) => v,
            None => return Ok(None),
        };

        let side = match dto.side.to_ascii_lowercase().as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => return Ok(None),
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
