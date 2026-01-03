//! # 风控引擎 (Risk Engine)
//!
//! 路径: services/trading-engine/src/domain/risk/engine/risk_engine.rs
//!
//! ## 职责
//! 编排多个风控规则，顺序执行检查。
//!
//! ## 行为要求
//! - 顺序执行多个 RiskRule
//! - 一旦出现拒绝，立即返回拒绝原因
//! - 不包含任何业务状态，仅负责编排
//! - 不修改 RiskState

use crate::domain::model::order_intent::OrderIntent;
use crate::domain::port::risk_state_port::RiskStateSnapshot;
use crate::domain::risk::result::RiskCheckResult;
use crate::domain::risk::rule::{RiskContext, RiskRule};

/// 风控引擎
///
/// 负责编排多个风控规则，顺序执行检查。
/// 不包含任何业务状态，仅负责编排。
pub struct RiskEngine {
    /// 规则列表（按顺序执行）
    rules: Vec<Box<dyn RiskRule>>,
}

impl RiskEngine {
    /// 创建空的风控引擎
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// 添加规则
    pub fn add_rule<R: RiskRule + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// 添加规则（可变引用版本）
    pub fn push_rule<R: RiskRule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    /// 获取规则数量
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// 获取所有规则名称
    pub fn rule_names(&self) -> Vec<&'static str> {
        self.rules.iter().map(|r| r.name()).collect()
    }

    /// 执行风控检查
    ///
    /// 顺序执行所有规则，一旦出现拒绝立即返回。
    ///
    /// # 参数
    /// - `intent`: 交易意图
    /// - `state`: 风控状态快照（只读）
    ///
    /// # 返回
    /// - `RiskCheckResult::Passed`: 所有规则通过
    /// - `RiskCheckResult::Rejected(reason)`: 某规则拒绝，附带原因
    pub fn check(&self, intent: &OrderIntent, state: &RiskStateSnapshot) -> RiskCheckResult {
        let ctx = RiskContext::new(intent, state);

        for rule in &self.rules {
            // 跳过禁用的规则
            if !rule.is_enabled() {
                tracing::debug!(rule = rule.name(), "规则已禁用，跳过");
                continue;
            }

            let result = rule.check(&ctx);

            if result.is_rejected() {
                tracing::info!(
                    rule = rule.name(),
                    symbol = %intent.symbol,
                    reason = ?result.reject_reason(),
                    "风控规则拒绝"
                );
                return result;
            }

            tracing::debug!(rule = rule.name(), "规则通过");
        }

        tracing::debug!(
            symbol = %intent.symbol,
            rules_checked = self.rules.len(),
            "所有风控规则通过"
        );

        RiskCheckResult::passed()
    }

    /// 执行风控检查并转换为 Result
    ///
    /// 便捷方法，将 RiskCheckResult 转换为 anyhow::Result
    pub fn check_as_result(
        &self,
        intent: &OrderIntent,
        state: &RiskStateSnapshot,
    ) -> anyhow::Result<()> {
        self.check(intent, state).into_result()
    }
}

impl Default for RiskEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 创建默认配置的风控引擎
///
/// 包含常用的风控规则：
/// - 最大持仓规则
/// - 最大未完成订单规则
pub fn create_default_risk_engine() -> RiskEngine {
    use crate::domain::risk::rule::{MaxOpenOrdersRule, MaxPositionRule};

    RiskEngine::new()
        .add_rule(MaxPositionRule::with_default())
        .add_rule(MaxOpenOrdersRule::with_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::order_intent::OrderSide;
    use crate::domain::port::risk_state_port::{RiskOpenOrder, RiskPosition};
    use crate::domain::risk::result::RiskRejectReason;
    use crate::domain::risk::rule::{MaxOpenOrdersRule, MaxPositionRule};
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    fn create_intent(symbol: &str, side: OrderSide, quantity: Decimal) -> OrderIntent {
        OrderIntent {
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

    #[test]
    fn test_empty_engine_passes() {
        let engine = RiskEngine::new();
        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(1));
        let state = RiskStateSnapshot::default();

        let result = engine.check(&intent, &state);
        assert!(result.is_passed());
    }

    #[test]
    fn test_all_rules_pass() {
        let engine = RiskEngine::new()
            .add_rule(MaxPositionRule::with_max(Decimal::from(100)))
            .add_rule(MaxOpenOrdersRule::with_max_per_symbol(10));

        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(10));
        let state = RiskStateSnapshot::default();

        let result = engine.check(&intent, &state);
        assert!(result.is_passed());
    }

    #[test]
    fn test_first_rule_rejects() {
        let engine = RiskEngine::new()
            .add_rule(MaxPositionRule::with_max(Decimal::from(5))) // 会拒绝
            .add_rule(MaxOpenOrdersRule::with_max_per_symbol(10));

        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(10));
        let state = RiskStateSnapshot::default();

        let result = engine.check(&intent, &state);
        assert!(result.is_rejected());
        
        // 应该是 PositionLimitExceeded
        assert!(matches!(
            result.reject_reason(),
            Some(RiskRejectReason::PositionLimitExceeded { .. })
        ));
    }

    #[test]
    fn test_second_rule_rejects() {
        let engine = RiskEngine::new()
            .add_rule(MaxPositionRule::with_max(Decimal::from(100)))
            .add_rule(MaxOpenOrdersRule::with_max_per_symbol(2)); // 会拒绝

        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(10));
        
        // 创建 2 个未完成订单
        let state = RiskStateSnapshot {
            balances: vec![],
            positions: vec![],
            open_orders: vec![
                RiskOpenOrder {
                    order_id: "1".to_string(),
                    symbol: "BTCUSDT".to_string(),
                    side: "BUY".to_string(),
                    quantity: Decimal::from(1),
                    price: Decimal::from(50000),
                    created_at: 0,
                },
                RiskOpenOrder {
                    order_id: "2".to_string(),
                    symbol: "BTCUSDT".to_string(),
                    side: "BUY".to_string(),
                    quantity: Decimal::from(1),
                    price: Decimal::from(50000),
                    created_at: 0,
                },
            ],
        };

        let result = engine.check(&intent, &state);
        assert!(result.is_rejected());
        
        // 应该是 OpenOrderLimitExceeded
        assert!(matches!(
            result.reject_reason(),
            Some(RiskRejectReason::OpenOrderLimitExceeded { .. })
        ));
    }

    #[test]
    fn test_check_as_result() {
        let engine = RiskEngine::new()
            .add_rule(MaxPositionRule::with_max(Decimal::from(100)));

        let intent = create_intent("BTCUSDT", OrderSide::Buy, Decimal::from(10));
        let state = RiskStateSnapshot::default();

        let result = engine.check_as_result(&intent, &state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_engine() {
        let engine = create_default_risk_engine();
        assert_eq!(engine.rule_count(), 2);
        
        let names = engine.rule_names();
        assert!(names.contains(&"max_position"));
        assert!(names.contains(&"max_open_orders"));
    }

    #[test]
    fn test_rule_count() {
        let mut engine = RiskEngine::new();
        assert_eq!(engine.rule_count(), 0);

        engine.push_rule(MaxPositionRule::with_default());
        assert_eq!(engine.rule_count(), 1);

        engine.push_rule(MaxOpenOrdersRule::with_default());
        assert_eq!(engine.rule_count(), 2);
    }
}
