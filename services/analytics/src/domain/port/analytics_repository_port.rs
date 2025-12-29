//! # Analytics Repository Port - 分析数据仓储端口
//!
//! ## 模块职责
//! 定义分析数据仓储的抽象接口（出站端口），
//! 由基础设施层（如 ClickHouseClient）实现
//!
//! ## 六边形架构说明
//! ```text
//! Application Layer
//!        ↓ 调用
//! AnalyticsRepositoryPort (trait) ← Domain Layer 定义
//!        ↑ 实现
//! ClickHouseClient ← Infrastructure Layer 实现
//! ```
//!
//! ## 设计原则
//! - 只使用领域对象（PerformanceMetrics）和基础类型（Uuid）
//! - 不暴露任何存储细节（SQL、ClickHouse语法等）

use uuid::Uuid;
use crate::domain::model::performance::PerformanceMetrics;

/// 分析数据仓储端口
///
/// Domain层定义的抽象接口，用于获取分析数据。
/// 具体实现由 Infrastructure 层提供（如 ClickHouseClient）
///
/// ## 实现要求
/// - 必须实现 `Send + Sync`，支持多线程环境
/// - 返回领域对象，不暴露存储细节
///
/// ## 示例实现
/// ```ignore
/// impl AnalyticsRepositoryPort for ClickHouseClient {
///     fn get_performance(&self, user_id: Uuid) -> Option<PerformanceMetrics> {
///         // 查询 ClickHouse，转换为领域对象
///     }
/// }
/// ```
#[allow(dead_code)] // 骨架阶段，trait暂未使用
pub trait AnalyticsRepositoryPort: Send + Sync {
    /// 获取用户绩效指标
    ///
    /// ## 参数
    /// - `user_id`: 用户唯一标识
    ///
    /// ## 返回
    /// - `Some(PerformanceMetrics)`: 找到用户的绩效数据
    /// - `None`: 用户不存在或无绩效数据
    fn get_performance(&self, user_id: Uuid) -> Option<PerformanceMetrics>;
    
    /// 获取策略绩效指标
    ///
    /// ## 参数
    /// - `strategy_id`: 策略唯一标识
    ///
    /// ## 返回
    /// - `Some(PerformanceMetrics)`: 找到策略的绩效数据
    /// - `None`: 策略不存在或无绩效数据
    fn get_strategy_performance(&self, strategy_id: Uuid) -> Option<PerformanceMetrics>;
}
