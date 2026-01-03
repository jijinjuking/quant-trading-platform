//! # 成交回报领域事件 (Execution Fill Domain Events)
//!
//! 路径: services/trading-engine/src/domain/model/execution_fill.rs
//!
//! ## 职责
//! 定义成交回报相关的领域事件，用于驱动 RiskState 修正。
//!
//! ## 设计原则
//! - 纯数据结构，不包含任何外部依赖
//! - 明确区分：订单接受 vs 订单成交
//! - 支持：部分成交、全部成交、撤单

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 成交回报事件
///
/// 当订单在交易所成交时产生此事件。
/// 用于驱动 RiskState 修正：update_position / update_balance / remove_open_order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFill {
    /// 事件 ID
    pub id: Uuid,
    /// 订单 ID（交易所返回的）
    pub order_id: String,
    /// 客户端订单 ID（可选）
    pub client_order_id: Option<String>,
    /// 交易对
    pub symbol: String,
    /// 方向: "BUY" / "SELL"
    pub side: FillSide,
    /// 成交类型
    pub fill_type: FillType,
    /// 本次成交数量
    pub filled_quantity: Decimal,
    /// 本次成交价格
    pub fill_price: Decimal,
    /// 累计成交数量
    pub cumulative_quantity: Decimal,
    /// 订单原始数量
    pub original_quantity: Decimal,
    /// 手续费
    pub commission: Decimal,
    /// 手续费资产
    pub commission_asset: String,
    /// 成交时间
    pub fill_time: DateTime<Utc>,
    /// 事件创建时间
    pub created_at: DateTime<Utc>,
}

/// 成交方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillSide {
    /// 买入
    Buy,
    /// 卖出
    Sell,
}

impl FillSide {
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "BUY" => Some(Self::Buy),
            "SELL" => Some(Self::Sell),
            _ => None,
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }
}

/// 成交类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillType {
    /// 部分成交 - 订单还有剩余数量
    Partial,
    /// 全部成交 - 订单完全成交
    Full,
}

impl ExecutionFill {
    /// 创建部分成交事件
    pub fn partial(
        order_id: String,
        symbol: String,
        side: FillSide,
        filled_quantity: Decimal,
        fill_price: Decimal,
        cumulative_quantity: Decimal,
        original_quantity: Decimal,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            client_order_id: None,
            symbol,
            side,
            fill_type: FillType::Partial,
            filled_quantity,
            fill_price,
            cumulative_quantity,
            original_quantity,
            commission: Decimal::ZERO,
            commission_asset: "USDT".to_string(),
            fill_time: Utc::now(),
            created_at: Utc::now(),
        }
    }

    /// 创建全部成交事件
    pub fn full(
        order_id: String,
        symbol: String,
        side: FillSide,
        filled_quantity: Decimal,
        fill_price: Decimal,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            client_order_id: None,
            symbol,
            side,
            fill_type: FillType::Full,
            filled_quantity,
            fill_price,
            cumulative_quantity: filled_quantity,
            original_quantity: filled_quantity,
            commission: Decimal::ZERO,
            commission_asset: "USDT".to_string(),
            fill_time: Utc::now(),
            created_at: Utc::now(),
        }
    }

    /// 是否为全部成交
    pub fn is_full(&self) -> bool {
        self.fill_type == FillType::Full
    }

    /// 是否为部分成交
    pub fn is_partial(&self) -> bool {
        self.fill_type == FillType::Partial
    }

    /// 获取剩余数量
    pub fn remaining_quantity(&self) -> Decimal {
        self.original_quantity - self.cumulative_quantity
    }

    /// 计算持仓变化量（正数加仓，负数减仓）
    pub fn position_delta(&self) -> Decimal {
        match self.side {
            FillSide::Buy => self.filled_quantity,
            FillSide::Sell => -self.filled_quantity,
        }
    }

    /// 计算成交金额
    pub fn notional(&self) -> Decimal {
        self.filled_quantity * self.fill_price
    }
}

/// 订单取消事件
///
/// 当订单被取消时产生此事件。
/// 用于驱动 RiskState 修正：remove_open_order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCanceled {
    /// 事件 ID
    pub id: Uuid,
    /// 订单 ID
    pub order_id: String,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: FillSide,
    /// 原始数量
    pub original_quantity: Decimal,
    /// 已成交数量
    pub filled_quantity: Decimal,
    /// 取消原因
    pub reason: CancelReason,
    /// 取消时间
    pub canceled_at: DateTime<Utc>,
}

/// 取消原因
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancelReason {
    /// 用户主动取消
    UserRequested,
    /// 系统取消（如风控触发）
    SystemCanceled,
    /// 超时取消
    Expired,
    /// 交易所拒绝
    ExchangeRejected,
}

impl OrderCanceled {
    /// 创建订单取消事件
    pub fn new(
        order_id: String,
        symbol: String,
        side: FillSide,
        original_quantity: Decimal,
        filled_quantity: Decimal,
        reason: CancelReason,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            symbol,
            side,
            original_quantity,
            filled_quantity,
            reason,
            canceled_at: Utc::now(),
        }
    }

    /// 是否有未成交部分
    pub fn has_unfilled(&self) -> bool {
        self.filled_quantity < self.original_quantity
    }

    /// 获取未成交数量
    pub fn unfilled_quantity(&self) -> Decimal {
        self.original_quantity - self.filled_quantity
    }
}

/// 订单接受事件
///
/// 当订单被交易所接受时产生此事件。
/// 注意：订单接受 ≠ 订单成交
/// 用于驱动 RiskState 修正：add_open_order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAccepted {
    /// 事件 ID
    pub id: Uuid,
    /// 订单 ID（交易所返回的）
    pub order_id: String,
    /// 客户端订单 ID
    pub client_order_id: Option<String>,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: FillSide,
    /// 订单类型: "LIMIT" / "MARKET"
    pub order_type: String,
    /// 委托数量
    pub quantity: Decimal,
    /// 委托价格（限价单）
    pub price: Option<Decimal>,
    /// 接受时间
    pub accepted_at: DateTime<Utc>,
}

impl OrderAccepted {
    /// 创建订单接受事件
    pub fn new(
        order_id: String,
        symbol: String,
        side: FillSide,
        order_type: &str,
        quantity: Decimal,
        price: Option<Decimal>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            client_order_id: None,
            symbol,
            side,
            order_type: order_type.to_uppercase(),
            quantity,
            price,
            accepted_at: Utc::now(),
        }
    }

    /// 是否为限价单
    pub fn is_limit(&self) -> bool {
        self.order_type == "LIMIT"
    }

    /// 是否为市价单
    pub fn is_market(&self) -> bool {
        self.order_type == "MARKET"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap_or_default()
    }

    #[test]
    fn test_fill_side_conversion() {
        assert_eq!(FillSide::from_str("BUY"), Some(FillSide::Buy));
        assert_eq!(FillSide::from_str("buy"), Some(FillSide::Buy));
        assert_eq!(FillSide::from_str("SELL"), Some(FillSide::Sell));
        assert_eq!(FillSide::from_str("invalid"), None);

        assert_eq!(FillSide::Buy.as_str(), "BUY");
        assert_eq!(FillSide::Sell.as_str(), "SELL");
    }

    #[test]
    fn test_execution_fill_partial() {
        let fill = ExecutionFill::partial(
            "order123".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            dec("0.1"),      // 本次成交
            dec("50000"),    // 成交价
            dec("0.1"),      // 累计成交
            dec("1.0"),      // 原始数量
        );

        assert!(fill.is_partial());
        assert!(!fill.is_full());
        assert_eq!(fill.remaining_quantity(), dec("0.9"));
        assert_eq!(fill.position_delta(), dec("0.1"));
        assert_eq!(fill.notional(), dec("5000"));
    }

    #[test]
    fn test_execution_fill_full() {
        let fill = ExecutionFill::full(
            "order456".to_string(),
            "ETHUSDT".to_string(),
            FillSide::Sell,
            dec("2.0"),
            dec("3000"),
        );

        assert!(fill.is_full());
        assert!(!fill.is_partial());
        assert_eq!(fill.remaining_quantity(), dec("0"));
        assert_eq!(fill.position_delta(), dec("-2.0"));
        assert_eq!(fill.notional(), dec("6000"));
    }

    #[test]
    fn test_order_canceled() {
        let cancel = OrderCanceled::new(
            "order789".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            dec("1.0"),      // 原始数量
            dec("0.3"),      // 已成交
            CancelReason::UserRequested,
        );

        assert!(cancel.has_unfilled());
        assert_eq!(cancel.unfilled_quantity(), dec("0.7"));
    }

    #[test]
    fn test_order_accepted() {
        let accepted = OrderAccepted::new(
            "order001".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            "LIMIT",
            dec("0.5"),
            Some(dec("48000")),
        );

        assert!(accepted.is_limit());
        assert!(!accepted.is_market());
    }
}
