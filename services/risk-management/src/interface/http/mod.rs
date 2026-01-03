//! # HTTP 接口模块 (HTTP Interface)
//!
//! 本模块提供 HTTP API 接口。
//!
//! ## 包含的子模块
//! - [`handlers`]: HTTP 请求处理器
//! - [`routes`]: 路由配置
//! - [`dto`]: 数据传输对象

/// HTTP 请求处理器
pub mod handlers;

/// 路由配置
pub mod routes;

/// 数据传输对象
pub mod dto;
