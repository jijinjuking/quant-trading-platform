//! # HTTP 接口 (HTTP Interface)
//! 
//! 提供 HTTP REST API 接口。
//! 
//! ## 子模块
//! - `handlers`: 请求处理器
//! - `routes`: 路由配置

/// 请求处理器 - 处理具体的 HTTP 请求
pub mod handlers;

/// 路由配置 - 定义 URL 到处理器的映射
pub mod routes;
