//! # 策略基础设施 (Strategy Infrastructure)
//!
//! 提供 StrategyPort 的具体实现。

/// Noop 策略 - 占位实现
pub mod noop_strategy;

pub use noop_strategy::NoopStrategy;
