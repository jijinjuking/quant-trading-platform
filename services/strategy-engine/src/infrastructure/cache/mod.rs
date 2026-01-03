//! # 缓存模块 (Cache Infrastructure)
//!
//! 提供缓存相关的基础设施实现。
//!
//! ## 实现
//! - `redis_strategy_state`: Redis 策略状态存储

/// Redis 策略状态存储
pub mod redis_strategy_state;

pub use redis_strategy_state::RedisStrategyStateAdapter;
