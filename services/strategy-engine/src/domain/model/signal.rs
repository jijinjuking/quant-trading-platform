//! # 信号模型 (Signal Model)
//! 
//! 定义交易信号的领域实体。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 交易信号实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// 信号唯一标识符
    pub id: Uuid,
    /// 关联的策略ID
    pub strategy_id: Uuid,
    /// 交易对符号
    pub symbol: String,
    /// 信号类型
    pub signal_type: SignalType,
    /// 建议价格
    pub price: Decimal,
    /// 建议数量
    pub quantity: Decimal,
    /// 信号置信度 (0.0 - 1.0)
    pub confidence: f64,
    /// 生成时间
    pub created_at: DateTime<Utc>,
}

/// 信号类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    /// 买入信号
    Buy,
    /// 卖出信号
    Sell,
    /// 持有信号
    Hold,
}
