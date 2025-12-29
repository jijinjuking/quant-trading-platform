//! # 风控仓储端口 (Risk Repository Port)
//!
//! 本模块定义风控仓储的端口（trait），属于领域层。
//!
//! ## 六边形架构说明
//! 这是一个"驱动端口"（Driven Port），定义了领域层需要的持久化能力。
//! 具体实现由 `infrastructure::repository::RiskRepository` 提供。
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 入参/出参只能是领域对象（如 `RiskProfile`）或基础类型

use uuid::Uuid;
use crate::domain::model::risk_profile::RiskProfile;

/// 风控仓储端口
///
/// 定义风控配置的持久化操作接口。
/// 领域层通过此 trait 与持久化层解耦。
///
/// ## 实现要求
/// - 实现者必须是 `Send + Sync`（支持多线程）
/// - 具体实现位于 `infrastructure::repository`
#[allow(dead_code)]
pub trait RiskRepositoryPort: Send + Sync {
    /// 获取用户风控配置
    ///
    /// # 参数
    /// - `user_id`: 用户唯一标识
    ///
    /// # 返回值
    /// - `Some(RiskProfile)`: 找到用户的风控配置
    /// - `None`: 未找到配置
    fn get_profile(&self, user_id: Uuid) -> Option<RiskProfile>;
    
    /// 保存风控配置
    ///
    /// # 参数
    /// - `profile`: 要保存的风控配置
    ///
    /// # 返回值
    /// - `true`: 保存成功
    /// - `false`: 保存失败
    fn save_profile(&self, profile: &RiskProfile) -> bool;
}
