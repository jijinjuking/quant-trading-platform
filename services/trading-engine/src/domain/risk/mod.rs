//! # 风控规则层 (Risk Rule Layer)
//!
//! 路径: services/trading-engine/src/domain/risk/mod.rs
//!
//! ## 职责
//! 提供可扩展的风控规则体系，基于只读 RiskStateSnapshot 进行决策。
//!
//! ## 模块结构
//! - `rule/`: 风控规则定义和具体实现
//! - `engine/`: 规则编排器
//! - `result/`: 结构化检查结果
//!
//! ## 设计原则
//! - 所有规则只读 RiskState，不修改
//! - 每条规则职责单一
//! - 结果必须是结构化的，不允许返回 bool
//! - 不调用外部 API，不引入数据库

pub mod engine;
pub mod result;
pub mod rule;

// 重新导出常用类型
pub use engine::{create_default_risk_engine, RiskEngine};
pub use result::{RiskCheckResult, RiskRejectReason};
pub use rule::{
    MaxOpenOrdersRule, MaxOpenOrdersRuleConfig,
    MaxPositionRule, MaxPositionRuleConfig,
    RiskContext, RiskRule,
};
