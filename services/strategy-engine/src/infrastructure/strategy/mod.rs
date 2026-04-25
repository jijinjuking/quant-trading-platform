//! Strategy infrastructure adapters.

/// Noop strategy adapter.
pub mod noop_strategy;

/// Signal strategy adapter.
pub mod signal_strategy;

/// Strategy executor adapter.
pub mod strategy_adapter;

pub use noop_strategy::NoopStrategy;
pub use signal_strategy::SignalStrategy;
pub use strategy_adapter::StrategyExecutorAdapter;
