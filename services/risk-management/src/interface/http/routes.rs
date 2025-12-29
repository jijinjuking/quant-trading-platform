//! # 路由配置 (Route Configuration)
//!
//! 本模块配置风险管理服务的 HTTP 路由。
//!
//! ## API 端点
//! - `GET /health`: 健康检查
//! - `POST /api/v1/risk/check`: 风险检查

use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use super::handlers;

/// 创建 HTTP 路由
///
/// 配置所有 HTTP 端点并绑定应用状态。
///
/// # 参数
/// - `state`: 应用状态，包含配置和数据库连接等
///
/// # 返回值
/// 返回配置好的 `Router` 实例
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // 风险检查 API
        .route("/api/v1/risk/check", post(handlers::risk::check_risk))
        // 绑定应用状态
        .with_state(state)
}
