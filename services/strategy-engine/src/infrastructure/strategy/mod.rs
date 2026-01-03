//! Strategy infrastructure adapters.

/// Noop strategy adapter.
pub mod noop_strategy;

/// Signal strategy adapter.
pub mod signal_strategy;

pub use noop_strategy::NoopStrategy;
pub use signal_strategy::SignalStrategy;
