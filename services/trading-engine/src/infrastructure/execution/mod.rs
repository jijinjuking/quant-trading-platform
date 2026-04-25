//! # 执行基础设施 (Execution Infrastructure) - v2
//!
//! 提供 ExecutionPort 的具体实现。
//!
//! ## v2 新增功能
//! - API 限流控制
//! - 请求重试机制
//! - 多种订单类型支持
//! - 合约交易支持

/// Noop 执行 - 占位实现
pub mod noop_execution;
/// 币安真实下单执行实现
pub mod binance_execution;
/// 币安合约交易执行实现
pub mod binance_futures_execution;
/// 订单执行适配器 - 实现 OrderExecutionPort
pub mod order_executor;
/// API 限流器
pub mod rate_limiter;
/// 请求重试策略
pub mod retry_policy;

pub use binance_execution::BinanceExecution;
pub use binance_futures_execution::{
    BinanceFuturesExecution, FuturesCommand, MarginType, PositionSide,
};
pub use noop_execution::NoopExecution;
pub use order_executor::OrderExecutor;
pub use rate_limiter::{RateLimiter, RateLimiterConfig};
pub use retry_policy::{RetryPolicy, RetryConfig};
