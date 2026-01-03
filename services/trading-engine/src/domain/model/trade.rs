//! # 成交记录模型 (Trade Model)
//!
//! 定义成交记录的领域模型。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::order::OrderSide;

/// 成交记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// 成交 ID
    pub id: Uuid,
    /// 订单 ID
    pub order_id: Uuid,
    /// 用户 ID
    pub user_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 成交方向
    pub side: OrderSide,
    /// 成交数量
    pub quantity: Decimal,
    /// 成交价格
    pub price: Decimal,
    /// 手续费
    pub fee: Decimal,
    /// 手续费币种
    pub fee_currency: String,
    /// 交易所成交 ID
    pub exchange_trade_id: Option<String>,
    /// 是否为 maker
    pub is_maker: bool,
    /// 成交时间
    pub trade_time: DateTime<Utc>,
}

impl Trade {
    /// 创建新成交记录
    pub fn new(
        order_id: Uuid,
        user_id: Uuid,
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Decimal,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            user_id,
            symbol,
            side,
            quantity,
            price,
            fee: Decimal::ZERO,
            fee_currency: "USDT".to_string(),
            exchange_trade_id: None,
            is_maker: false,
            trade_time: Utc::now(),
        }
    }
}
