//! # HTTP Interface - HTTP接口模块
//!
//! ## 模块职责
//! 提供 HTTP RESTful API 接口，包括：
//! - 绩效分析接口
//! - 统计报表接口
//! - 健康检查接口
//!
//! ## 子模块
//! - `handlers`: 请求处理器
//! - `routes`: 路由配置

/// HTTP 请求处理器
pub mod handlers;

/// HTTP 路由配置
pub mod routes;
