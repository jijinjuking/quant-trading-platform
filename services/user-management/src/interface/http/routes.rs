//! # 路由配置
//!
//! 本文件定义用户管理服务的 HTTP 路由配置。
//!
//! ## 所属层
//! Interface Layer > HTTP
//!
//! ## API 端点
//! - `GET /health`: 健康检查
//! - `POST /api/v1/auth/login`: 用户登录
//! - `POST /api/v1/auth/register`: 用户注册
//! - `GET /api/v1/user/profile`: 获取用户资料

use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use super::handlers;

/// 创建路由器
///
/// 配置所有 HTTP 路由并注入应用状态。
///
/// # 参数
/// - `state`: 应用状态，包含配置和共享资源
///
/// # 返回值
/// 配置好的 Axum 路由器
///
/// # 路由列表
/// | 方法 | 路径 | 处理器 | 说明 |
/// |------|------|--------|------|
/// | GET | /health | health_check | 健康检查 |
/// | POST | /api/v1/auth/login | login | 用户登录 |
/// | POST | /api/v1/auth/register | register | 用户注册 |
/// | GET | /api/v1/user/profile | get_profile | 获取用户资料 |
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // 认证相关端点
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        // 用户相关端点
        .route("/api/v1/user/profile", get(handlers::user::get_profile))
        // 注入应用状态
        .with_state(state)
}
