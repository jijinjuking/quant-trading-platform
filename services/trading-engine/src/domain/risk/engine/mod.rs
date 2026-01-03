//! # 风控引擎模块 (Risk Engine Module)
//!
//! 路径: services/trading-engine/src/domain/risk/engine/mod.rs
//!
//! ## 职责
//! 导出风控引擎相关类型

pub mod risk_engine;

pub use risk_engine::{create_default_risk_engine, RiskEngine};
