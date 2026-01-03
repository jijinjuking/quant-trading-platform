//! # 风控结果模块 (Risk Result Module)
//!
//! 路径: services/trading-engine/src/domain/risk/result/mod.rs
//!
//! ## 职责
//! 导出风控检查结果相关类型

pub mod risk_check_result;

pub use risk_check_result::{RiskCheckResult, RiskRejectReason};
