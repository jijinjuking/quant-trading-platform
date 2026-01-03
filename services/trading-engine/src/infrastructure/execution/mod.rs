//! # 执行基础设施 (Execution Infrastructure) - v1 占位骨架
//!
//! 提供 ExecutionPort 的具体实现。

/// Noop 执行 - 占位实现
pub mod noop_execution;
/// 币安真实下单执行实现
pub mod binance_execution;
/// 订单执行适配器 - 实现 OrderExecutionPort
pub mod order_executor;

pub use binance_execution::BinanceExecution;
pub use noop_execution::NoopExecution;
pub use order_executor::OrderExecutor;
