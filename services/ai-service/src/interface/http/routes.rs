//! # 路由配置模块
//!
//! 定义 AI 服务的所有 HTTP 路由。
//!
//! ## API 端点
//! - `GET /health`: 健康检查
//! - `POST /api/v1/ai/analyze`: 市场分析
//! - `POST /api/v1/ai/strategy`: 策略生成
//! - `POST /api/v1/ai/chat`: AI 对话

use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use super::handlers;

/// 创建 HTTP 路由器
///
/// 配置所有 API 端点并绑定应用状态。
///
/// # Arguments
/// * `state` - 应用全局状态
///
/// # Returns
/// 配置完成的 Axum 路由器
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // AI 市场分析端点
        .route("/api/v1/ai/analyze", post(handlers::ai::analyze_market))
        // AI 策略生成端点
        .route("/api/v1/ai/strategy", post(handlers::ai::generate_strategy))
        // AI 对话端点
        .route("/api/v1/ai/chat", post(handlers::ai::chat))
        // 绑定应用状态
        .with_state(state)
}
