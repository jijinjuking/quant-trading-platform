//! # 路由配置模块
//!
//! 本文件定义通知服务的 HTTP 路由配置。
//!
//! ## 路由列表
//! - `GET /health`: 健康检查接口
//! - `POST /api/v1/notifications`: 发送通知接口
//!
//! ## 架构位置
//! 属于接口层（Interface Layer），负责将 HTTP 请求路由到对应的处理器。

use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use super::handlers;

/// 创建 HTTP 路由器
///
/// 配置所有 HTTP 端点并绑定应用状态。
///
/// # 参数
/// - `state`: 应用状态，包含配置和共享资源
///
/// # 返回值
/// 配置完成的 Axum 路由器
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // 发送通知端点
        .route("/api/v1/notifications", post(handlers::notification::send_notification))
        // 绑定应用状态
        .with_state(state)
}
