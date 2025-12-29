//! Risk Events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAlertEvent {
    pub user_id: Uuid,
    pub alert_type: RiskAlertType,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskAlertType {
    DrawdownWarning,
    LeverageExceeded,
    PositionLimitReached,
    DailyLossLimit,
}
