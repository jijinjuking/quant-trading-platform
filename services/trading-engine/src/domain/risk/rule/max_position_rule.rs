//! # 最大持仓规则 (Max Position Rule)
//!
//! 路径: services/trading-engine/src/domain/risk/rule/max_position_rule.rs
//!
//! ## 职责
//! 检查订单执行后是否会超过最大持仓限制。
//!
//! ## 检查逻辑
//! 1. 获取当前持仓数量
//! 2. 计算订单执行后的预期持仓
//! 3. 如果预期持仓超过限制，拒绝订单

use rust_decimal::Decimal;

use crate::domain::model::order_intent::OrderSide;
use crate::domain::risk::result::{RiskCheckResult, RiskRejectReason};
use crate::domain::risk::rule::risk_rule::{RiskContext, RiskRule};

/// 最大持仓规则配置
#[derive(Debug, Clone)]
pub struct MaxPositionRuleConfig {
    /// 最大持仓数量（绝对值）
    pub max_position: Decimal,
}

impl Default for MaxPositionRuleConfig {
    fn default() -> Self {
        Self {
            max_position: Decimal::from(100),
        }
    }
}

/// 最大持仓规则
///
/// 检查订单执行后是否会超过最大持仓限制。
pub struct MaxPositionRule {
    config: MaxPositionRuleConfig,
}

impl MaxPositionRule {
    /// 创建规则实例
    pub fn new(config: MaxPositionRuleConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    pub fn with_default() -> Self {
        Self::new(MaxPositionRuleConfig::default())
    }

    /// 从最大持仓值创建
    pub fn with_max(max_position: Decimal) -> Self {
        Self::new(MaxPositionRuleConfig { max_position })
    }
}

impl RiskRule for MaxPositionRule {
    fn name(&self) -> &'static str {
        "max_position"
    }

    fn description(&self) -> &'static str {
        "检查订单执行后是否会超过最大持仓限制"
    }

    fn check(&self, ctx: &RiskContext) -> RiskCheckResult {
        let intent = ctx.intent;
        let state = ctx.state;

        // 获取当前持仓
        let current_position = state.get_position_qty(&intent.symbol);

        // 计算订单执行后的预期持仓
        let position_delta = match intent.side {
            OrderSide::Buy => intent.quantity,
            OrderSide::Sell => -intent.quantity,
        };
        let expected_position = current_position + position_delta;

        // 检查是否超过限制（使用绝对值）
        if expected_position.abs() > self.config.max_position {
            return RiskCheckResult::rejected(RiskRejectReason::PositionLimitExceeded {
                current: current_position,
                requested: intent.quantity,
                max_allowed: self.config.max_position,
            });
        }

        RiskCheckResult::passed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::port::risk_state_port::{RiskPosition, RiskStateSnapshot};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_intent(symbol: &str, side: OrderSide, quantity: Decimal) -> crate::domain::model::order_intent::OrderIntent {
        crate::domain::model::order_intent::OrderIntent {
            id: Uuid::new_v4(),
            strategy_id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            side,
            quantity,
            price: Some(Decimal::from(50000)),
            confidence: 0.9,
            created_at: Utc::now(),
        }
    }

    fn create_state_with_position(symbol: &str, quantity: Decimal) -> RiskStateSnapshot {
        RiskStateSnapshot {
            balances: vec![],
            positions: vec![RiskPosition {
                symbol: symbol.to_string(),
                quantity,
                entry_price: Decimal::from(50000),
                unrealized_pnl: Decimal::ZERO,
            }],
            open_orders: vec![],
        }
    }

    #[test]
    fn test_pass_when_under_limit() {
        let rule = MaxPositionRule::with_max(Decimal::from(100));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(10));
        let state = create_state_with_position("BTCUSDT", Decimal::from(50));
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_passed());
    }

    #[test]
    fn test_reject_when_over_limit() {
        let rule = MaxPositionRule::with_max(Decimal::from(100));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(60));
        let state = create_state_with_position("BTCUSDT", Decimal::from(50));
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_rejected());
        
        if let Some(RiskRejectReason::PositionLimitExceeded { current, requested, max_allowed }) = result.reject_reason() {
            assert_eq!(*current, Decimal::from(50));
            assert_eq!(*requested, Decimal::from(60));
            assert_eq!(*max_allowed, Decimal::from(100));
        } else {
            panic!("Expected PositionLimitExceeded");
        }
    }

    #[test]
    fn test_pass_sell_reduces_position() {
        let rule = MaxPositionRule::with_max(Decimal::from(100));
        let intent = create_intent("BTCUSDT", OrderSide::Sell, Decimal::from(30));
        let state = create_state_with_position("BTCUSDT", Decimal::from(90));
        let ctx = RiskContext::new(&intent, &state);

        // 90 - 30 = 60, 在限制内
        let result = rule.check(&ctx);
        assert!(result.is_passed());
    }

    #[test]
    fn test_reject_short_over_limit() {
        let rule = MaxPositionRule::with_max(Decimal::from(100));
        let intent = create_intent("BTCUSDT", OrderSide::Sell, Decimal::from(150));
        let state = create_state_with_position("BTCUSDT", Decimal::ZERO);
        let ctx = RiskContext::new(&intent, &state);

        // 0 - 150 = -150, 绝对值超过限制
        let result = rule.check(&ctx);
        assert!(result.is_rejected());
    }

    #[test]
    fn test_pass_no_existing_position() {
        let rule = MaxPositionRule::with_max(Decimal::from(100));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(50));
        let state = RiskStateSnapshot::default();
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_passed());
    }
}
