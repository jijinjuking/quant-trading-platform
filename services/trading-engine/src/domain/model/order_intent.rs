//! # 交易意图 (Order Intent)
//!
//! 策略产生的交易意图，不是执行指令。
//! 需要经过风控检查后才能转换为执行指令。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 交易意图
///
/// 策略计算产生的交易意图，表示"想要"执行的交易。
/// 必须经过风控检查后才能转换为 ExecutionCommand。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIntent {
    /// 意图 ID
    pub id: Uuid,
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: OrderSide,
    /// 数量
    pub quantity: Decimal,
    /// 期望价格（可选，市价单为 None）
    pub price: Option<Decimal>,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f64,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// 买入
    Buy,
    /// 卖出
    Sell,
}

impl OrderIntent {
    /// 创建新的交易意图
    pub fn new(
        strategy_id: Uuid,
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Option<Decimal>,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            strategy_id,
            symbol,
            side,
            quantity,
            price,
            confidence,
            created_at: Utc::now(),
        }
    }
}
