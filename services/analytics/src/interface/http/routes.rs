//! # Route Configuration - 路由配置
//!
//! ## 模块职责
//! 配置 HTTP 路由映射，将 URL 路径映射到对应的处理器
//!
//! ## API 端点
//! | 方法 | 路径 | 描述 |
//! |------|------|------|
//! | GET | `/health` | 健康检查 |
//! | GET | `/api/v1/analytics/performance` | 获取绩效指标 |
//! | GET | `/api/v1/analytics/report` | 获取统计报表 |

use axum::{routing::get, Router};
use crate::state::AppState;
use super::handlers;

/// 创建 HTTP 路由
///
/// ## 参数
/// - `state`: 应用状态，将被注入到所有处理器中
///
/// ## 返回
/// 配置好的 Axum Router 实例
///
/// ## 路由说明
/// - `/health`: 健康检查端点，用于服务探活
/// - `/api/v1/analytics/performance`: 绩效分析端点
/// - `/api/v1/analytics/report`: 统计报表端点
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // 绩效分析端点
        .route("/api/v1/analytics/performance", get(handlers::analytics::get_performance))
        // 统计报表端点
        .route("/api/v1/analytics/report", get(handlers::analytics::get_report))
        // 注入应用状态
        .with_state(state)
}
