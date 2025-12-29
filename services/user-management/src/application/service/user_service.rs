//! # 用户应用服务
//!
//! 本文件定义用户管理的应用服务，负责用户相关用例的编排。
//!
//! ## 所属层
//! Application Layer > Service
//!
//! ## 职责
//! - 创建用户用例
//! - 查询用户用例
//! - 更新用户用例
//!
//! ## 依赖规则
//! - ✅ 只依赖 `domain::port` 中的 trait
//! - ❌ 不直接依赖数据库、缓存等基础设施

use uuid::Uuid;
use crate::domain::model::user::User;
use crate::domain::port::user_repository_port::UserRepositoryPort;

/// 用户应用服务
///
/// 负责用户管理相关用例的编排，通过泛型参数接收仓储实现。
/// 遵循依赖倒置原则，只依赖 `UserRepositoryPort` trait。
///
/// # 类型参数
/// - `R`: 实现了 `UserRepositoryPort` trait 的仓储类型
///
/// # 示例
/// ```ignore
/// let repo = PostgresUserRepository::new();
/// let service = UserService::new(repo);
/// let user = service.get_user(user_id);
/// ```
#[allow(dead_code)]
pub struct UserService<R: UserRepositoryPort> {
    /// 用户仓储（通过端口抽象）
    repository: R,
}

#[allow(dead_code)]
impl<R: UserRepositoryPort> UserService<R> {
    /// 创建用户服务实例
    ///
    /// # 参数
    /// - `repository`: 实现了 `UserRepositoryPort` 的仓储实例
    ///
    /// # 返回值
    /// 新的 `UserService` 实例
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    /// 创建用户用例
    ///
    /// 将用户实体保存到仓储中。
    ///
    /// # 参数
    /// - `user`: 要创建的用户实体引用
    ///
    /// # 返回值
    /// - `true`: 创建成功
    /// - `false`: 创建失败
    pub fn create_user(&self, user: &User) -> bool {
        self.repository.save(user)
    }
    
    /// 根据 ID 获取用户用例
    ///
    /// # 参数
    /// - `id`: 用户唯一标识（UUID）
    ///
    /// # 返回值
    /// - `Some(User)`: 找到用户
    /// - `None`: 用户不存在
    pub fn get_user(&self, id: Uuid) -> Option<User> {
        self.repository.find_by_id(id)
    }
    
    /// 根据邮箱获取用户用例
    ///
    /// # 参数
    /// - `email`: 用户邮箱地址
    ///
    /// # 返回值
    /// - `Some(User)`: 找到用户
    /// - `None`: 用户不存在
    pub fn get_user_by_email(&self, email: &str) -> Option<User> {
        self.repository.find_by_email(email)
    }
}
