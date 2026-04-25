//! # 领域服务 (Domain Services)
//!
//! 跨模型的领域规则。

/// 策略注册表
pub mod strategy_registry;

pub use strategy_registry::{RegistryStats, StrategyQuery, StrategyRegistry};
