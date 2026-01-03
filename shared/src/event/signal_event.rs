//! 跨服务的交易信号事件。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEvent {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub symbol: String,
    pub signal_type: SignalType,
    pub price: Decimal,
    pub quantity: Decimal,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    Buy,
    Sell,
    Hold,
}
