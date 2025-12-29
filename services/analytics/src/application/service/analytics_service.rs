//! # Analytics Service - 数据分析应用服务
//!
//! ## 模块职责
//! 应用层服务，负责编排数据分析相关的用例：
//! - 用户绩效查询
//! - 策略绩效查询
//! - 统计报表生成
//!
//! ## 架构说明
//! - 只依赖 `domain::port` 中定义的 trait
//! - 通过泛型参数注入具体实现（依赖倒置）
//! - 不包含业务规则，只做流程编排
//!
//! ## 依赖方向
//! ```text
//! AnalyticsService → AnalyticsRepositoryPort (trait)
//!                           ↑
//!                    ClickHouseClient (实现)
//! ```

use uuid::Uuid;
use crate::domain::model::performance::PerformanceMetrics;
use crate::domain::port::analytics_repository_port::AnalyticsRepositoryPort;

/// 数据分析应用服务
///
/// 泛型参数 `R` 必须实现 `AnalyticsRepositoryPort` trait，
/// 这样可以在运行时注入不同的实现（如 ClickHouse、Mock等）
#[allow(dead_code)] // 骨架阶段，结构体暂未使用
pub struct AnalyticsService<R: AnalyticsRepositoryPort> {
    /// 分析数据仓储（通过trait抽象）
    repository: R,
}

#[allow(dead_code)] // 骨架阶段，方法暂未使用
impl<R: AnalyticsRepositoryPort> AnalyticsService<R> {
    /// 创建新的分析服务实例
    ///
    /// ## 参数
    /// - `repository`: 实现了 `AnalyticsRepositoryPort` 的仓储实例
    ///
    /// ## 示例
    /// ```ignore
    /// let client = ClickHouseClient::new(url);
    /// let service = AnalyticsService::new(client);
    /// ```
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    /// 获取用户绩效指标
    ///
    /// ## 参数
    /// - `user_id`: 用户唯一标识
    ///
    /// ## 返回
    /// - `Some(PerformanceMetrics)`: 找到用户的绩效数据
    /// - `None`: 用户不存在或无绩效数据
    ///
    /// ## 用例流程
    /// 1. 调用仓储查询用户绩效
    /// 2. 返回领域模型
    pub fn get_user_performance(&self, user_id: Uuid) -> Option<PerformanceMetrics> {
        self.repository.get_performance(user_id)
    }
    
    /// 获取策略绩效指标
    ///
    /// ## 参数
    /// - `strategy_id`: 策略唯一标识
    ///
    /// ## 返回
    /// - `Some(PerformanceMetrics)`: 找到策略的绩效数据
    /// - `None`: 策略不存在或无绩效数据
    ///
    /// ## 用例流程
    /// 1. 调用仓储查询策略绩效
    /// 2. 返回领域模型
    pub fn get_strategy_performance(&self, strategy_id: Uuid) -> Option<PerformanceMetrics> {
        self.repository.get_strategy_performance(strategy_id)
    }
}
