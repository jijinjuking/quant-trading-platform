//! # 策略执行器端口 (Strategy Executor Port)
//!
//! 定义策略执行的抽象接口。
//!
//! ## DDD 定位
//! - Port（端口）：定义契约
//! - 由 Infrastructure 层实现

use anyhow::Result;

use crate::domain::model::strategy_runtime::{ExecutionRequest, ExecutionResult};

/// 策略执行器端口
///
/// 由具体策略实现。
pub trait StrategyExecutorPort: Send + Sync {
    /// 执行策略计算
    fn execute(&self, request: &ExecutionRequest) -> Result<ExecutionResult>;

    /// 重置策略状态
    fn reset(&self) -> Result<()>;

    /// 获取状态快照
    fn state_snapshot(&self) -> Result<serde_json::Value>;
}
