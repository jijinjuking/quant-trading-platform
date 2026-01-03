//! # 策略适配器模块 (Strategy Adapters)
//!
//! 实现 StrategyPort 的适配器。

pub mod noop_strategy;
pub mod remote_strategy;

pub use noop_strategy::NoopStrategy;
pub use remote_strategy::RemoteStrategy;
