//! # HTTP 处理器模块 (HTTP Handlers)
//!
//! 本模块包含所有 HTTP 请求处理器。
//!
//! ## 包含的处理器
//! - [`health`]: 健康检查处理器
//! - [`risk`]: 风险检查处理器

/// 健康检查处理器
pub mod health;

/// 风险检查处理器
pub mod risk;
