//! # 订单模型 (Order Model)
//!
//! 定义订单的领域模型。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 待处理
    Pending,
    /// 部分成交
    PartiallyFilled,
    /// 完全成交
    Filled,
    /// 已取消
    Cancelled,
    /// 已拒绝
    Rejected,
    /// 已过期
    Expired,
}

impl OrderStatus {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "PENDING",
            OrderStatus::PartiallyFilled => "PARTIALLY_FILLED",
            OrderStatus::Filled => "FILLED",
            OrderStatus::Cancelled => "CANCELLED",
            OrderStatus::Rejected => "REJECTED",
            OrderStatus::Expired => "EXPIRED",
        }
    }

    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "PENDING" => Some(OrderStatus::Pending),
            "PARTIALLY_FILLED" => Some(OrderStatus::PartiallyFilled),
            "FILLED" => Some(OrderStatus::Filled),
            "CANCELLED" => Some(OrderStatus::Cancelled),
            "REJECTED" => Some(OrderStatus::Rejected),
            "EXPIRED" => Some(OrderStatus::Expired),
            _ => None,
        }
    }
}

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

impl OrderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
        }
    }
}

/// 订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 订单 ID
    pub id: Uuid,
    /// 用户 ID
    pub user_id: Uuid,
    /// 策略 ID（可选）
    pub strategy_id: Option<Uuid>,
    /// 交易对
    pub symbol: String,
    /// 订单类型
    pub order_type: OrderType,
    /// 订单方向
    pub side: OrderSide,
    /// 订单数量
    pub quantity: Decimal,
    /// 订单价格（限价单）
    pub price: Option<Decimal>,
    /// 订单状态
    pub status: OrderStatus,
    /// 已成交数量
    pub filled_quantity: Decimal,
    /// 平均成交价格
    pub average_price: Option<Decimal>,
    /// 交易所订单 ID
    pub exchange_order_id: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

impl Order {
    /// 创建新订单
    pub fn new(
        user_id: Uuid,
        strategy_id: Option<Uuid>,
        symbol: String,
        order_type: OrderType,
        side: OrderSide,
        quantity: Decimal,
        price: Option<Decimal>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            strategy_id,
            symbol,
            order_type,
            side,
            quantity,
            price,
            status: OrderStatus::Pending,
            filled_quantity: Decimal::ZERO,
            average_price: None,
            exchange_order_id: None,
            created_at: now,
            updated_at: now,
        }
    }
}
