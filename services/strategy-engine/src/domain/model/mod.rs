//! # Domain Models
//!
//! 策略引擎的领域模型。

/// 策略运行时（执行壳）
pub mod strategy_runtime;

/// 策略句柄（生命周期管理）
pub mod strategy_handle;

/// 策略元数据
pub mod strategy_metadata;

/// 生命周期状态
pub mod lifecycle_state;

/// 故障记录
pub mod failure_record;

/// 市场类型
pub mod market_type;

/// 信号模型
pub mod signal;

/// 策略配置
pub mod strategy_config;

/// 策略实体（兼容旧代码）
pub mod strategy;

// 重新导出常用类型
pub use strategy_runtime::{
    ExecutionRequest, ExecutionResult, TradeIntent,
    RuntimeHandle, RuntimeExitReason, StrategyExecutor,
    spawn_runtime,
};
pub use strategy_handle::StrategyHandle;
pub use strategy_metadata::{StrategyMetadata, StrategyKind, MarketType};
pub use lifecycle_state::LifecycleState;
