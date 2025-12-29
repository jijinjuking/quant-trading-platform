//! # 用户仓储端口
//!
//! 本文件定义用户仓储的抽象接口（端口）。
//!
//! ## 所属层
//! Domain Layer > Port
//!
//! ## 设计说明
//! - 这是一个端口定义，只包含 trait
//! - 具体实现在 infrastructure 层
//! - 遵循依赖倒置原则（DIP）

use uuid::Uuid;
use crate::domain::model::user::User;

/// 用户仓储端口 - Domain 层定义的抽象接口
///
/// 定义用户持久化操作的抽象接口，由 infrastructure 层实现。
/// 遵循六边形架构的端口-适配器模式。
///
/// # 实现要求
/// - 实现者必须是 `Send + Sync`（支持多线程）
/// - 所有方法的入参和出参只能是领域对象
///
/// # 示例
/// ```ignore
/// // infrastructure 层的实现
/// impl UserRepositoryPort for PostgresUserRepository {
///     fn save(&self, user: &User) -> bool {
///         // 数据库操作
///     }
/// }
/// ```
#[allow(dead_code)]
pub trait UserRepositoryPort: Send + Sync {
    /// 保存用户
    ///
    /// 将用户实体持久化到存储中。如果用户已存在则更新，否则创建。
    ///
    /// # 参数
    /// - `user`: 要保存的用户实体引用
    ///
    /// # 返回值
    /// - `true`: 保存成功
    /// - `false`: 保存失败
    fn save(&self, user: &User) -> bool;
    
    /// 根据 ID 查询用户
    ///
    /// # 参数
    /// - `id`: 用户唯一标识（UUID）
    ///
    /// # 返回值
    /// - `Some(User)`: 找到用户
    /// - `None`: 用户不存在
    fn find_by_id(&self, id: Uuid) -> Option<User>;
    
    /// 根据邮箱查询用户
    ///
    /// # 参数
    /// - `email`: 用户邮箱地址
    ///
    /// # 返回值
    /// - `Some(User)`: 找到用户
    /// - `None`: 用户不存在
    fn find_by_email(&self, email: &str) -> Option<User>;
    
    /// 根据用户名查询用户
    ///
    /// # 参数
    /// - `username`: 用户名
    ///
    /// # 返回值
    /// - `Some(User)`: 找到用户
    /// - `None`: 用户不存在
    fn find_by_username(&self, username: &str) -> Option<User>;
}
