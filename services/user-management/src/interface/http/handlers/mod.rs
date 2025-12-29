//! # HTTP 处理器模块
//!
//! 本模块包含所有 HTTP 请求处理器。
//!
//! ## 所属层
//! Interface Layer > HTTP > Handlers
//!
//! ## 包含处理器
//! - `health`: 健康检查处理器
//! - `auth`: 认证相关处理器
//! - `user`: 用户相关处理器

/// 健康检查处理器
pub mod health;

/// 认证处理器 - 登录、注册
pub mod auth;

/// 用户处理器 - 用户资料管理
pub mod user;
