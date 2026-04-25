//! # 策略句柄 (Strategy Handle)
//!
//! 策略实例的外部管理句柄，用于生命周期管理和状态查询。
//!
//! ## 职责边界（冻结）
//! - ✅ 持有策略执行器
//! - ✅ 管理生命周期状态
//! - ✅ 记录故障历史
//! - ❌ 不持有策略逻辑
//! - ❌ 不做调度决策

use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use uuid::Uuid;

use super::failure_record::{FailureHistory, FailureRecord, FailureType};
use super::lifecycle_state::{LifecycleState, LifecycleTransition, LifecycleTransitionError};
use super::strategy_metadata::StrategyMetadata;
use super::strategy_runtime::{ExecutionRequest, ExecutionResult};
use crate::domain::port::strategy_executor_port::StrategyExecutorPort;

/// 策略句柄
///
/// Registry 持有此句柄，通过它管理策略实例的生命周期。
/// 使用内部可变性（RwLock）支持并发访问。
pub struct StrategyHandle {
    /// 策略元数据
    metadata: StrategyMetadata,
    /// 内部可变状态
    inner: RwLock<StrategyHandleInner>,
    /// 策略执行器
    executor: Arc<dyn StrategyExecutorPort>,
}

/// 内部可变状态
struct StrategyHandleInner {
    /// 当前生命周期状态
    state: LifecycleState,
    /// 故障历史
    failure_history: FailureHistory,
    /// 生命周期转换历史
    transitions: Vec<LifecycleTransition>,
    /// 最后执行时间
    last_execution_at: Option<DateTime<Utc>>,
    /// 执行计数
    execution_count: u64,
    /// 成功计数
    success_count: u64,
}

impl StrategyHandle {
    /// 创建策略句柄（Created 状态）
    pub fn new(metadata: StrategyMetadata, executor: Arc<dyn StrategyExecutorPort>) -> Self {
        Self {
            metadata,
            executor,
            inner: RwLock::new(StrategyHandleInner {
                state: LifecycleState::Created,
                failure_history: FailureHistory::new(),
                transitions: Vec::new(),
                last_execution_at: None,
                execution_count: 0,
                success_count: 0,
            }),
        }
    }

    // ========================================================================
    // 基本信息
    // ========================================================================

    /// 获取实例 ID
    pub fn instance_id(&self) -> Uuid {
        self.metadata.instance_id
    }

    /// 获取元数据
    pub fn metadata(&self) -> &StrategyMetadata {
        &self.metadata
    }

    /// 获取当前生命周期状态
    pub fn lifecycle_state(&self) -> LifecycleState {
        self.inner.read().state
    }

    /// 是否可以执行
    pub fn can_execute(&self) -> bool {
        self.inner.read().state.can_execute()
    }

    /// 是否可以注销
    pub fn can_unregister(&self) -> bool {
        let state = self.inner.read().state;
        matches!(state, LifecycleState::Created | LifecycleState::Stopped)
    }

    /// 获取执行统计
    pub fn stats(&self) -> HandleStats {
        let inner = self.inner.read();
        HandleStats {
            execution_count: inner.execution_count,
            success_count: inner.success_count,
            failure_count: inner.failure_history.total_failures(),
            consecutive_failures: inner.failure_history.consecutive_failures(),
            last_execution_at: inner.last_execution_at,
        }
    }

    // ========================================================================
    // 执行（同步，使用内部可变性）
    // ========================================================================

    /// 执行策略计算
    pub fn execute(&self, request: &ExecutionRequest) -> Result<ExecutionResult> {
        {
            let inner = self.inner.read();
            if !inner.state.can_execute() {
                return Err(anyhow!(
                    "策略 {} 当前状态 {} 不可执行",
                    self.instance_id(),
                    inner.state
                ));
            }
        }

        // 更新执行计数
        {
            let mut inner = self.inner.write();
            inner.execution_count += 1;
            inner.last_execution_at = Some(Utc::now());
        }

        // 执行策略
        let result = self.executor.execute(request);

        // 更新统计
        {
            let mut inner = self.inner.write();
            match &result {
                Ok(r) if r.error.is_none() => {
                    inner.success_count += 1;
                    inner.failure_history.record_success();
                }
                Ok(r) => {
                    inner.failure_history.record(FailureRecord::new(
                        FailureType::LogicError,
                        r.error.as_deref().unwrap_or("未知错误"),
                    ));
                    self.check_fault_threshold(&mut inner);
                }
                Err(e) => {
                    inner.failure_history.record(FailureRecord::new(
                        FailureType::Unknown,
                        &e.to_string(),
                    ));
                    self.check_fault_threshold(&mut inner);
                }
            }
        }

        result
    }

    /// 检查是否超过故障阈值
    fn check_fault_threshold(&self, inner: &mut StrategyHandleInner) {
        if inner.failure_history.exceeds_threshold(3) && inner.state == LifecycleState::Running {
            let from = inner.state;
            inner.state = LifecycleState::Faulted;
            inner.transitions.push(LifecycleTransition::new(from, LifecycleState::Faulted, "连续故障超过阈值"));
        }
    }

    // ========================================================================
    // 生命周期管理（同步，使用内部可变性）
    // ========================================================================

    /// 启动策略
    pub fn start(&self) -> Result<(), LifecycleTransitionError> {
        let mut inner = self.inner.write();
        if !inner.state.can_start() {
            return Err(LifecycleTransitionError {
                current: inner.state,
                target: LifecycleState::Running,
                reason: "当前状态不允许启动".to_string(),
            });
        }

        let from = inner.state;
        inner.state = LifecycleState::Running;
        let to = inner.state;
        inner.transitions.push(LifecycleTransition::new(from, to, "启动"));
        Ok(())
    }

    /// 暂停策略
    pub fn pause(&self) -> Result<(), LifecycleTransitionError> {
        let mut inner = self.inner.write();
        if !inner.state.can_pause() {
            return Err(LifecycleTransitionError {
                current: inner.state,
                target: LifecycleState::Paused,
                reason: "当前状态不允许暂停".to_string(),
            });
        }

        let from = inner.state;
        inner.state = LifecycleState::Paused;
        let to = inner.state;
        inner.transitions.push(LifecycleTransition::new(from, to, "暂停"));
        Ok(())
    }

    /// 恢复策略
    pub fn resume(&self) -> Result<(), LifecycleTransitionError> {
        let mut inner = self.inner.write();
        if !inner.state.can_resume() {
            return Err(LifecycleTransitionError {
                current: inner.state,
                target: LifecycleState::Running,
                reason: "当前状态不允许恢复".to_string(),
            });
        }

        let from = inner.state;
        inner.state = LifecycleState::Running;
        let to = inner.state;
        inner.transitions.push(LifecycleTransition::new(from, to, "恢复"));
        Ok(())
    }

    /// 停止策略
    pub fn stop(&self) -> Result<(), LifecycleTransitionError> {
        let mut inner = self.inner.write();
        if !inner.state.can_stop() {
            return Err(LifecycleTransitionError {
                current: inner.state,
                target: LifecycleState::Stopped,
                reason: "当前状态不允许停止".to_string(),
            });
        }

        let from = inner.state;
        inner.state = LifecycleState::Stopped;
        let to = inner.state;
        inner.transitions.push(LifecycleTransition::new(from, to, "停止"));
        Ok(())
    }

    /// 重启策略
    pub fn restart(&self) -> Result<(), LifecycleTransitionError> {
        let mut inner = self.inner.write();
        
        // 先停止
        if inner.state.can_stop() {
            let from = inner.state;
            inner.state = LifecycleState::Stopped;
            let to = inner.state;
            inner.transitions.push(LifecycleTransition::new(from, to, "重启-停止"));
        }

        // 再启动
        if inner.state.can_start() {
            let from = inner.state;
            inner.state = LifecycleState::Running;
            let to = inner.state;
            inner.transitions.push(LifecycleTransition::new(from, to, "重启-启动"));
            Ok(())
        } else {
            Err(LifecycleTransitionError {
                current: inner.state,
                target: LifecycleState::Running,
                reason: "重启失败：无法从当前状态启动".to_string(),
            })
        }
    }

    /// 标记为故障状态
    pub fn mark_faulted(&self, reason: impl Into<String>) {
        let mut inner = self.inner.write();
        let from = inner.state;
        inner.state = LifecycleState::Faulted;
        let to = inner.state;
        inner.transitions.push(LifecycleTransition::new(from, to, reason));
    }

    // ========================================================================
    // 故障管理
    // ========================================================================

    /// 获取故障历史
    pub fn failure_history(&self) -> FailureHistory {
        self.inner.read().failure_history.clone()
    }

    /// 获取生命周期转换历史
    pub fn transitions(&self) -> Vec<LifecycleTransition> {
        self.inner.read().transitions.clone()
    }
}

/// 句柄统计信息
#[derive(Debug, Clone)]
pub struct HandleStats {
    /// 执行次数
    pub execution_count: u64,
    /// 成功次数
    pub success_count: u64,
    /// 故障次数
    pub failure_count: u64,
    /// 连续故障次数
    pub consecutive_failures: u32,
    /// 最后执行时间
    pub last_execution_at: Option<DateTime<Utc>>,
}

impl HandleStats {
    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.execution_count == 0 {
            return 0.0;
        }
        self.success_count as f64 / self.execution_count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::strategy_metadata::{MarketType, StrategyKind};
    use crate::domain::model::strategy_runtime::ExecutionResult;

    /// 测试用的空执行器
    struct NoopExecutor {
        instance_id: Uuid,
    }

    impl StrategyExecutorPort for NoopExecutor {
        fn execute(&self, request: &ExecutionRequest) -> Result<ExecutionResult> {
            Ok(ExecutionResult {
                request_id: request.request_id,
                has_intent: false,
                intent: None,
                execution_time_us: 0,
                error: None,
            })
        }

        fn reset(&self) -> Result<()> {
            Ok(())
        }

        fn state_snapshot(&self) -> Result<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
    }

    fn create_test_metadata() -> StrategyMetadata {
        StrategyMetadata::new(
            StrategyKind::Grid,
            MarketType::Spot,
            "BTCUSDT",
            Uuid::new_v4(),
            "测试策略",
        )
    }

    fn create_test_handle() -> StrategyHandle {
        let metadata = create_test_metadata();
        let executor = Arc::new(NoopExecutor {
            instance_id: metadata.instance_id,
        });
        StrategyHandle::new(metadata, executor)
    }

    #[test]
    fn test_handle_creation() {
        let handle = create_test_handle();
        assert_eq!(handle.lifecycle_state(), LifecycleState::Created);
        assert!(!handle.can_execute());
        assert!(handle.can_unregister());
    }

    #[test]
    fn test_lifecycle_transitions() {
        let handle = create_test_handle();

        // Created 状态不能暂停
        assert!(handle.pause().is_err());

        // Created 状态不能恢复
        assert!(handle.resume().is_err());

        // Created 状态可以启动
        assert!(handle.start().is_ok());
        assert_eq!(handle.lifecycle_state(), LifecycleState::Running);
        assert!(handle.can_execute());
        assert!(!handle.can_unregister());

        // Running 状态可以暂停
        assert!(handle.pause().is_ok());
        assert_eq!(handle.lifecycle_state(), LifecycleState::Paused);

        // Paused 状态可以恢复
        assert!(handle.resume().is_ok());
        assert_eq!(handle.lifecycle_state(), LifecycleState::Running);

        // Running 状态可以停止
        assert!(handle.stop().is_ok());
        assert_eq!(handle.lifecycle_state(), LifecycleState::Stopped);
        assert!(handle.can_unregister());
    }

    #[test]
    fn test_stats() {
        let handle = create_test_handle();
        let stats = handle.stats();
        assert_eq!(stats.execution_count, 0);
        assert_eq!(stats.success_count, 0);
        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn test_restart() {
        let handle = create_test_handle();
        handle.start().unwrap();
        assert_eq!(handle.lifecycle_state(), LifecycleState::Running);

        handle.restart().unwrap();
        assert_eq!(handle.lifecycle_state(), LifecycleState::Running);
    }
}
