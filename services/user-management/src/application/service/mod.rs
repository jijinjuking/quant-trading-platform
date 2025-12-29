//! # 应用服务模块
//!
//! 本模块包含所有应用层服务，负责用例编排。
//!
//! ## 所属层
//! Application Layer > Service
//!
//! ## 包含服务
//! - `auth_service`: 认证服务（登录、注册）
//! - `user_service`: 用户服务（用户管理）

/// 认证服务 - 处理登录、注册等认证用例
pub mod auth_service;

/// 用户服务 - 处理用户管理用例
pub mod user_service;
