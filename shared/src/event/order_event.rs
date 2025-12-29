//! Order Events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::order::{OrderId, OrderStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub order_id: OrderId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusChangedEvent {
    pub order_id: OrderId,
    pub old_status: OrderStatus,
    pub new_status: OrderStatus,
    pub timestamp: DateTime<Utc>,
}
