//! # 风控规则 Trait (Risk Rule Trait)
//!
//! 路径: services/trading-engine/src/domain/risk/rule/risk_rule.rs
//!
//! ## 职责
//! 定义风控规则的统一接口，所有具体规则必须实现此 trait。
//!
//! ## 设计原则
//! - 输入：订单上下文 + 只读 RiskStateSnapshot
//! - 输出：RiskCheckResult
//! - 规则不得修改 RiskState
//! - 每条规则职责单一（一个规则只检查一个约束）

use crate::domain::model::order_intent::OrderIntent;
use crate::domain::port::risk_state_port::RiskStateSnapshot;
use crate::domain::risk::result::RiskCheckResult;

/// 风控规则上下文
///
/// 包含规则检查所需的所有只读信息
#[derive(Debug, Clone)]
pub struct RiskContext<'a> {
    /// 交易意图
    pub intent: &'a OrderIntent,
    /// 风控状态快照（只读）
    pub state: &'a RiskStateSnapshot,
}

impl<'a> RiskContext<'a> {
    /// 创建风控上下文
    pub fn new(intent: &'a OrderIntent, state: &'a RiskStateSnapshot) -> Self {
        Self { intent, state }
    }
}

/// 风控规则 Trait
///
/// 所有风控规则必须实现此 trait。
///
/// ## 实现要求
/// - `check` 方法必须是纯函数，不得有副作用
/// - 不得修改 RiskState
/// - 不得调用外部 API
/// - 每条规则只检查一个约束
pub trait RiskRule: Send + Sync {
    /// 规则名称（用于日志和监控）
    fn name(&self) -> &'static str;

    /// 规则描述
    fn description(&self) -> &'static str;

    /// 执行风控检查
    ///
    /// # 参数
    /// - `ctx`: 风控上下文（只读）
    ///
    /// # 返回
    /// - `RiskCheckResult::Passed`: 检查通过
    /// - `RiskCheckResult::Rejected(reason)`: 检查拒绝，附带原因
    fn check(&self, ctx: &RiskContext) -> RiskCheckResult;

    /// 规则是否启用
    ///
    /// 默认启用，子类可覆盖实现动态开关
    fn is_enabled(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::risk::result::RiskRejectReason;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    /// 测试用的简单规则
    struct AlwaysPassRule;

    impl RiskRule for AlwaysPassRule {
        fn name(&self) -> &'static str {
            "always_pass"
        }

        fn description(&self) -> &'static str {
            "Always passes"
        }

        fn check(&self, _ctx: &RiskContext) -> RiskCheckResult {
            RiskCheckResult::passed()
        }
    }

    struct AlwaysRejectRule;

    impl RiskRule for AlwaysRejectRule {
        fn name(&self) -> &'static str {
            "always_reject"
        }

        fn description(&self) -> &'static str {
            "Always rejects"
        }

        fn check(&self, _ctx: &RiskContext) -> RiskCheckResult {
            RiskCheckResult::rejected(RiskRejectReason::Custom {
                rule_name: "always_reject".to_string(),
                message: "Test rejection".to_string(),
            })
        }
    }

    #[test]
    fn test_always_pass_rule() {
        let rule = AlwaysPassRule;
        let intent = OrderIntent {
            id: Uuid::new_v4(),
            strategy_id: Uuid::new_v4(),
            symbol: "BTCUSDT".to_string(),
            side: crate::domain::model::order_intent::OrderSide::Buy,
            quantity: Decimal::from(1),
            price: Some(Decimal::from(50000)),
            confidence: 0.9,
            created_at: Utc::now(),
        };
        let state = RiskStateSnapshot::default();
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_passed());
    }

    #[test]
    fn test_always_reject_rule() {
        let rule = AlwaysRejectRule;
        let intent = OrderIntent {
            id: Uuid::new_v4(),
            strategy_id: Uuid::new_v4(),
            symbol: "BTCUSDT".to_string(),
            side: crate::domain::model::order_intent::OrderSide::Buy,
            quantity: Decimal::from(1),
            price: Some(Decimal::from(50000)),
            confidence: 0.9,
            created_at: Utc::now(),
        };
        let state = RiskStateSnapshot::default();
        let ctx = RiskContext::new(&intent, &state);

        let result = rule.check(&ctx);
        assert!(result.is_rejected());
    }
}
