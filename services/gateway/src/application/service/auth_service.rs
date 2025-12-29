//! # 认证服务 - 应用层
//!
//! 本模块提供认证相关的应用层服务。
//!
//! ## 架构位置
//! 应用层服务，只依赖 `domain::port` 中定义的 trait。
//!
//! ## 职责
//! - Token 验证流程编排
//! - 限流检查流程编排
//! - 缓存与认证端口的协调
//!
//! ## 依赖规则
//! - ✅ 依赖 `domain::port::AuthPort`
//! - ✅ 依赖 `domain::port::CachePort`
//! - ❌ 不依赖具体实现
//! - ❌ 不包含业务逻辑

use crate::domain::port::auth_port::AuthPort;
use crate::domain::port::cache_port::CachePort;

/// 认证服务
///
/// 编排认证和缓存端口，完成认证相关的用例。
/// 使用泛型参数实现依赖倒置，不依赖具体实现。
///
/// # 类型参数
/// - `A`: 实现 `AuthPort` trait 的认证适配器
/// - `C`: 实现 `CachePort` trait 的缓存适配器
///
/// # 字段
/// - `auth`: 认证端口实例
/// - `cache`: 缓存端口实例
#[allow(dead_code)]
pub struct AuthService<A: AuthPort, C: CachePort> {
    /// 认证端口
    auth: A,
    /// 缓存端口
    cache: C,
}

#[allow(dead_code)]
impl<A: AuthPort, C: CachePort> AuthService<A, C> {
    /// 创建新的认证服务实例
    ///
    /// # 参数
    /// - `auth`: 认证端口实现
    /// - `cache`: 缓存端口实现
    ///
    /// # 返回值
    /// 认证服务实例
    pub fn new(auth: A, cache: C) -> Self {
        Self { auth, cache }
    }
    
    /// 验证 Token
    ///
    /// 执行 Token 验证流程：
    /// 1. 先查询缓存，命中则直接返回
    /// 2. 缓存未命中，调用认证端口验证
    ///
    /// # 参数
    /// - `token`: 待验证的 Token 字符串
    ///
    /// # 返回值
    /// - `true`: Token 有效
    /// - `false`: Token 无效
    pub fn validate_token(&self, token: &str) -> bool {
        // 先查缓存，提高性能
        if self.cache.get(&format!("token:{}", token)).is_some() {
            return true;
        }
        // 缓存未命中，调用认证端口验证
        self.auth.validate_token(token)
    }
    
    /// 检查限流
    ///
    /// 检查指定用户是否超过请求频率限制。
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    ///
    /// # 返回值
    /// - `true`: 未超过限制，允许请求
    /// - `false`: 超过限制，拒绝请求
    pub fn check_rate_limit(&self, user_id: &str) -> bool {
        self.cache.check_rate_limit(&format!("rate:{}", user_id), 100)
    }
}
