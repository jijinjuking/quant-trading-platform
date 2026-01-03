//! # 风控规则模块 (Risk Rule Module)
//!
//! 路径: services/trading-engine/src/domain/risk/rule/mod.rs
//!
//! ## 职责
//! 导出所有风控规则相关类型和具体规则实现。
//!
//! ## 规则列表
//! - `RiskRule`: 风控规则 trait
//! - `MaxPositionRule`: 最大持仓规则
//! - `MaxOpenOrdersRule`: 最大未完成订单规则

pub mod risk_rule;
pub mod max_position_rule;
pub mod max_open_orders_rule;

pub use risk_rule::{RiskContext, RiskRule};
pub use max_position_rule::{MaxPositionRule, MaxPositionRuleConfig};
pub use max_open_orders_rule::{MaxOpenOrdersRule, MaxOpenOrdersRuleConfig};
