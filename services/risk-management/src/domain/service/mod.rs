//! # 领域服务模块 (Domain Services)
//!
//! 本模块包含跨模型的领域服务。

/// 风险评估器 - 综合风险评估服务
pub mod risk_evaluator;

/// 风控检查器 - 核心风控规则
pub mod risk_checker;

pub use risk_checker::*;
