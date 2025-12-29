//! # HTTP 接口模块
//!
//! 提供 AI 服务的 HTTP API 实现。
//!
//! ## 子模块
//! - `handlers`: 请求处理器（健康检查、AI 分析等）
//! - `routes`: 路由配置

/// HTTP 请求处理器
pub mod handlers;
/// 路由配置
pub mod routes;
