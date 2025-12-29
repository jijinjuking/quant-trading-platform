//! # HTTP Handlers - HTTP请求处理器
//!
//! ## 模块职责
//! 包含所有 HTTP 请求的处理器函数，负责：
//! - 解析请求参数
//! - 调用应用层服务
//! - 构造响应结果
//!
//! ## 子模块
//! - `health`: 健康检查处理器
//! - `analytics`: 数据分析处理器

/// 健康检查处理器
pub mod health;

/// 数据分析处理器
pub mod analytics;
