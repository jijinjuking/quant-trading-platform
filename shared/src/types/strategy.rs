//! Strategy Types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyId(pub Uuid);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StrategyType {
    Grid,
    MeanReversion,
    Momentum,
    Arbitrage,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyInfo {
    pub id: StrategyId,
    pub name: String,
    pub strategy_type: StrategyType,
    pub is_active: bool,
}
