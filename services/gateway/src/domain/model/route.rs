//! # 路由配置模型
//!
//! 本模块定义 API Gateway 的路由配置领域模型。
//!
//! ## 用途
//! - 描述请求路由规则
//! - 配置目标服务映射
//! - 定义认证要求
//!
//! ## 领域概念
//! 路由配置是 API Gateway 的核心领域概念，
//! 决定了请求如何被转发到后端服务。

use serde::{Deserialize, Serialize};

/// 路由配置
///
/// 定义单个路由规则，包含路径匹配、目标服务和认证要求。
///
/// # 字段
/// - `path`: 请求路径模式（支持通配符）
/// - `target_service`: 目标后端服务地址
/// - `auth_required`: 是否需要认证
///
/// # 示例
/// ```ignore
/// let route = RouteConfig {
///     path: "/api/users/*".to_string(),
///     target_service: "http://user-service:8084".to_string(),
///     auth_required: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RouteConfig {
    /// 请求路径模式（如 "/api/users/*"）
    pub path: String,
    /// 目标服务地址（如 "http://user-service:8084"）
    pub target_service: String,
    /// 是否需要认证
    pub auth_required: bool,
}
