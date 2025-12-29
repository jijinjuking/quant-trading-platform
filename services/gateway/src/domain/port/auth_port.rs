//! # 认证端口
//!
//! 本模块定义认证相关的端口（Port）接口。
//!
//! ## 六边形架构
//! 认证端口是领域层定义的抽象接口，由基础设施层实现。
//!
//! ## 职责
//! - 定义 Token 验证接口
//! - 定义用户身份获取接口
//! - 定义权限检查接口
//!
//! ## 实现者
//! - `infrastructure::auth::JwtAuthAdapter`（JWT 实现）
//! - 其他认证适配器

/// 认证端口 - 领域层定义的抽象接口
///
/// 定义认证相关的操作，由基础设施层提供具体实现。
/// 遵循依赖倒置原则，领域层不依赖具体实现。
///
/// # 约束
/// - `Send + Sync`: 支持多线程环境
///
/// # 实现要求
/// 实现者需要处理：
/// - Token 解析和验证
/// - 用户身份提取
/// - 权限规则检查
#[allow(dead_code)]
pub trait AuthPort: Send + Sync {
    /// 验证 Token 有效性
    ///
    /// # 参数
    /// - `token`: JWT Token 字符串
    ///
    /// # 返回值
    /// - `true`: Token 有效
    /// - `false`: Token 无效或已过期
    fn validate_token(&self, token: &str) -> bool;
    
    /// 从 Token 中获取用户 ID
    ///
    /// # 参数
    /// - `token`: JWT Token 字符串
    ///
    /// # 返回值
    /// - `Some(user_id)`: 成功提取用户 ID
    /// - `None`: Token 无效或不包含用户信息
    fn get_user_id(&self, token: &str) -> Option<String>;
    
    /// 检查用户对资源的访问权限
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    /// - `resource`: 资源标识（如 "orders:read"）
    ///
    /// # 返回值
    /// - `true`: 有权限
    /// - `false`: 无权限
    fn check_permission(&self, user_id: &str, resource: &str) -> bool;
}
