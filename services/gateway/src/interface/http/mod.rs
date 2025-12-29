//! # HTTP 接口模块
//!
//! 本模块提供 HTTP 协议的接口实现。
//!
//! ## 职责
//! - 定义 HTTP 路由
//! - 实现请求处理器（handlers）
//! - DTO 转换
//!
//! ## 子模块
//! - `handlers`: 请求处理器
//! - `routes`: 路由配置

/// 请求处理器模块
pub mod handlers;

/// 路由配置模块
pub mod routes;
