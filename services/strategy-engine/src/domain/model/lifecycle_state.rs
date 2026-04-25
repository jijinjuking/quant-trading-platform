//! # 策略生命周期状态 (Lifecycle State)
//!
//! 定义策略实例的生命周期状态机。
//!
//! ## 状态转换图
//! ```text
//! Created → Running ⇄ Paused → Stopped
//!               ↓
//!            Faulted → Stopped
//! ```
//!
//! ## 工程约束
//! - 禁止 bool 生命周期（activate/deactivate）
//! - 状态转换必须显式
//! - 非法转换返回错误

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 策略生命周期状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LifecycleState {
    /// 已创建，尚未启动
    Created,
    /// 运行中，可接受执行请求
    Running,
    /// 已暂停，拒绝执行请求但保留状态
    Paused,
    /// 已停止，资源已释放
    Stopped,
    /// 故障状态，需要人工干预或自动恢复
    Faulted,
}

impl LifecycleState {
    /// 是否可以接受执行请求
    pub fn can_execute(&self) -> bool {
        matches!(self, LifecycleState::Running)
    }

    /// 是否可以启动
    pub fn can_start(&self) -> bool {
        matches!(self, LifecycleState::Created | LifecycleState::Stopped)
    }

    /// 是否可以暂停
    pub fn can_pause(&self) -> bool {
        matches!(self, LifecycleState::Running)
    }

    /// 是否可以恢复
    pub fn can_resume(&self) -> bool {
        matches!(self, LifecycleState::Paused)
    }

    /// 是否可以停止
    pub fn can_stop(&self) -> bool {
        matches!(
            self,
            LifecycleState::Running | LifecycleState::Paused | LifecycleState::Faulted
        )
    }

    /// 是否处于终态
    pub fn is_terminal(&self) -> bool {
        matches!(self, LifecycleState::Stopped)
    }

    /// 是否处于故障状态
    pub fn is_faulted(&self) -> bool {
        matches!(self, LifecycleState::Faulted)
    }
}

impl std::fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifecycleState::Created => write!(f, "created"),
            LifecycleState::Running => write!(f, "running"),
            LifecycleState::Paused => write!(f, "paused"),
            LifecycleState::Stopped => write!(f, "stopped"),
            LifecycleState::Faulted => write!(f, "faulted"),
        }
    }
}

impl Default for LifecycleState {
    fn default() -> Self {
        LifecycleState::Created
    }
}

/// 生命周期转换事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleTransition {
    /// 转换前状态
    pub from: LifecycleState,
    /// 转换后状态
    pub to: LifecycleState,
    /// 转换时间
    pub timestamp: DateTime<Utc>,
    /// 转换原因
    pub reason: String,
}

impl LifecycleTransition {
    /// 创建转换记录
    pub fn new(from: LifecycleState, to: LifecycleState, reason: impl Into<String>) -> Self {
        Self {
            from,
            to,
            timestamp: Utc::now(),
            reason: reason.into(),
        }
    }
}

/// 生命周期转换错误
#[derive(Debug, Clone)]
pub struct LifecycleTransitionError {
    /// 当前状态
    pub current: LifecycleState,
    /// 目标状态
    pub target: LifecycleState,
    /// 错误原因
    pub reason: String,
}

impl std::fmt::Display for LifecycleTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "无法从 {} 转换到 {}: {}",
            self.current, self.target, self.reason
        )
    }
}

impl std::error::Error for LifecycleTransitionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_execute() {
        assert!(!LifecycleState::Created.can_execute());
        assert!(LifecycleState::Running.can_execute());
        assert!(!LifecycleState::Paused.can_execute());
        assert!(!LifecycleState::Stopped.can_execute());
        assert!(!LifecycleState::Faulted.can_execute());
    }

    #[test]
    fn test_can_start() {
        assert!(LifecycleState::Created.can_start());
        assert!(!LifecycleState::Running.can_start());
        assert!(!LifecycleState::Paused.can_start());
        assert!(LifecycleState::Stopped.can_start());
        assert!(!LifecycleState::Faulted.can_start());
    }

    #[test]
    fn test_state_display() {
        assert_eq!(LifecycleState::Running.to_string(), "running");
        assert_eq!(LifecycleState::Faulted.to_string(), "faulted");
    }
}
