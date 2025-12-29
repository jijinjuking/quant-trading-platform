//! # 风险检查服务 (Risk Check Service)
//!
//! 本模块属于应用层，负责风险检查用例的编排。
//!
//! ## 职责
//! - 接收风险检查请求
//! - 调用领域服务进行风险评估
//! - 返回风险决策结果
//!
//! ## 依赖规则
//! - 只依赖 `domain::port` 中定义的 trait（依赖倒置）
//! - 不直接依赖数据库、缓存等基础设施

use uuid::Uuid;
#[allow(unused_imports)]
use crate::domain::model::risk_profile::RiskProfile;
use crate::domain::port::risk_repository_port::RiskRepositoryPort;
use crate::domain::service::risk_evaluator::{RiskEvaluator, RiskDecision};

/// 风险检查服务
///
/// 应用层服务，负责编排风险检查流程。
/// 通过泛型参数 `R` 实现依赖倒置，只依赖 `RiskRepositoryPort` trait。
#[allow(dead_code)]
pub struct RiskCheckService<R: RiskRepositoryPort> {
    /// 风控仓储（通过端口抽象）
    repository: R,
    /// 风险评估器（领域服务）
    evaluator: RiskEvaluator,
}

#[allow(dead_code)]
impl<R: RiskRepositoryPort> RiskCheckService<R> {
    /// 创建风险检查服务实例
    ///
    /// # 参数
    /// - `repository`: 实现了 `RiskRepositoryPort` trait 的仓储实例
    ///
    /// # 返回值
    /// 返回新的 `RiskCheckService` 实例
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            evaluator: RiskEvaluator::new(),
        }
    }
    
    /// 检查订单风险
    ///
    /// 根据用户的风控配置评估订单是否符合风险要求。
    ///
    /// # 参数
    /// - `user_id`: 用户唯一标识
    ///
    /// # 返回值
    /// - `RiskDecision::Approved`: 风险检查通过
    /// - `RiskDecision::Rejected`: 风险检查拒绝
    /// - `RiskDecision::RequiresReview`: 需要人工审核
    pub fn check_order_risk(&self, user_id: Uuid) -> RiskDecision {
        // 获取用户风控配置
        if let Some(profile) = self.repository.get_profile(user_id) {
            // 使用领域服务进行风险评估
            return self.evaluator.evaluate(&profile).unwrap_or(RiskDecision::RequiresReview);
        }
        // 未找到风控配置，需要人工审核
        RiskDecision::RequiresReview
    }
}
