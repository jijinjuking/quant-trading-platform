//! # 最大未完成订单规则 (Max Open Orders Rule)
//!
//! 路径: services/trading-engine/src/domain/risk/rule/max_open_orders_rule.rs
//!
//! ## 职责
//! 检查当前未完成订单数量是否超过限制。
//!
//! ## 检查逻辑
//! 1. 获取当前交易对的未完成订单数量
//! 2. 如果已达到限制，拒绝新订单

use crate::domain::risk::result::{RiskCheckResult, RiskRejectReason};
use crate::domain::risk::rule::risk_rule::{RiskContext, RiskRule};

/// 最大未完成订单规则配置
#[derive(Debug, Clone)]
pub struct MaxOpenOrdersRuleConfig {
    /// 单个交易对最大未完成订单数
    pub max_orders_per_symbol: usize,
    /// 全局最大未完成订单数（可选）
    pub max_orders_global: Option<usize>,
}

impl Default for MaxOpenOrdersRuleConfig {
    fn default() -> Self {
        Self {
            max_orders_per_symbol: 10,
            max_orders_global: Some(50),
        }
    }
}

/// 最大未完成订单规则
///
/// 检查当前未完成订单数量是否超过限制。
pub struct MaxOpenOrdersRule {
    config: MaxOpenOrdersRuleConfig,
}

impl MaxOpenOrdersRule {
    /// 创建规则实例
    pub fn new(config: MaxOpenOrdersRuleConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    pub fn with_default() -> Self {
        Self::new(MaxOpenOrdersRuleConfig::default())
    }

    /// 从最大订单数创建（仅限制单交易对）
    pub fn with_max_per_symbol(max_orders: usize) -> Self {
        Self::new(MaxOpenOrdersRuleConfig {
            max_orders_per_symbol: max_orders,
            max_orders_global: None,
        })
    }

    /// 从最大订单数创建（同时限制单交易对和全局）
    pub fn with_limits(max_per_symbol: usize, max_global: usize) -> Self {
        Self::new(MaxOpenOrdersRuleConfig {
            max_orders_per_symbol: max_per_symbol,
            max_orders_global: Some(max_global),
        })
    }
}

impl RiskRule for MaxOpenOrdersRule {
    fn name(&self) -> &'static str {
        "max_open_orders"
    }

    fn description(&self) -> &'static str {
        "检查当前未完成订单数量是否超过限制"
    }

    fn check(&self, ctx: &RiskContext) -> RiskCheckResult {
        let intent = ctx.intent;
        let state = ctx.state;

        // 检查单交易对限制
        let symbol_order_count = state.get_open_order_count(&intent.symbol);
        if symbol_order_count >= self.config.max_orders_per_symbol {
            return RiskCheckResult::rejected(RiskRejectReason::OpenOrderLimitExceeded {
                current: symbol_order_count,
                max_allowed: self.config.max_orders_per_symbol,
            });
        }

        // 检查全局限制（如果配置了）
        if let Some(max_global) = self.config.max_orders_global {
            let total_order_count = state.open_orders.len();
            if total_order_count >= max_global {
                return RiskCheckResult::rejected(RiskRejectReason::OpenOrderLimitExceeded {
                    current: total_order_count,
                    max_allowed: max_global,
                });
            }
        }

        RiskCheckResult::passed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::order_intent::{OrderIntent, OrderSide};
    use crate::domain::port::risk_state_port::{RiskOpenOrder, RiskStateSnapshot};
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    fn create_intent(symbol: &str) -> OrderIntent {
        OrderIntent {
            id: Uuid::new_v4(),
            strategy_id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            side: OrderSide::Buy,
            quantity: Decimal::from(1),
            price: Some(Decimal::from(50000)),
            confidence: 0.9,
            created_at: Utc::now(),
        }
    }

    fn create_open_order(symbol: &str) -> RiskOpenOrder {
        RiskOpenOrder {
            order_id: Uuid::new_v4().to_string(),
            symbol: symbol.to_string(),
            side: "BUY".to_string(),
            quantity: Decimal::from(1),
            price: Decimal::from(50000),
            created_at: Utc::now().timestamp_millis(),
        }
    }

    fn create_state_with_orders(symbol: &str, count: usize) -> RiskStateSnapshot {
        let orders: Vec<RiskOpenOrder> = (0..count)
            .map(|_| create_open_order(symbol))
            .collect();
        
        RiskStateSnapshot {
            balances: vec![],
            positions: vec![],
            open_orders: orders,
        }
    }

    #[test]
    fn test_pass_when_under_limit() {
        let rule = MaxOpenOrdersRule::with_max_per_symbol(10);
        let intent = create_intent("BTCUSDT");
        let state = create_state_with_orders("BTCUSDT", 5);
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_passed());
    }

    #[test]
    fn test_reject_when_at_limit() {
        let rule = MaxOpenOrdersRule::with_max_per_symbol(5);
        let intent = create_intent("BTCUSDT");
        let state = create_state_with_orders("BTCUSDT", 5);
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_rejected());
        
        if let Some(RiskRejectReason::OpenOrderLimitExceeded { current, max_allowed }) = result.reject_reason() {
            assert_eq!(*current, 5);
            assert_eq!(*max_allowed, 5);
        } else {
            panic!("Expected OpenOrderLimitExceeded");
        }
    }

    #[test]
    fn test_pass_different_symbol() {
        let rule = MaxOpenOrdersRule::with_max_per_symbol(5);
        let intent = create_intent("ETHUSDT");
        let state = create_state_with_orders("BTCUSDT", 10); // 不同交易对
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_passed());
    }

    #[test]
    fn test_reject_global_limit() {
        let rule = MaxOpenOrdersRule::with_limits(10, 5);
        let intent = create_intent("ETHUSDT");
        
        // 创建 5 个 BTCUSDT 订单
        let state = create_state_with_orders("BTCUSDT", 5);
        let ctx = RiskContext::new(&intent, &state);

        // 虽然 ETHUSDT 没有订单，但全局已达到限制
        let result = rule.check(&ctx);
        assert!(result.is_rejected());
    }

    #[test]
    fn test_pass_no_orders() {
        let rule = MaxOpenOrdersRule::with_default();
        let intent = create_intent("BTCUSDT");
        let state = RiskStateSnapshot::default();
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_passed());
    }
}
