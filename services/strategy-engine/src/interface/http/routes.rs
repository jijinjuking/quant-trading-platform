//! # 路由配置 (Route Configuration)
//! 
//! 定义策略引擎服务的所有 HTTP 路由。
//! 
//! ## API 端点
//! - `GET /health`: 健康检查
//! - `GET /api/v1/strategies`: 获取策略列表
//! - `POST /api/v1/strategies`: 创建新策略
//! - `POST /api/v1/backtest`: 运行回测

// ============================================================================
// 外部依赖导入
// ============================================================================

use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use super::handlers;

// ============================================================================
// 路由创建函数
// ============================================================================

/// 创建 HTTP 路由器
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查
        .route("/health", get(handlers::health::health_check))
        // 策略管理
        .route("/api/v1/strategies", get(handlers::strategies::list_strategies))
        .route("/api/v1/strategies", post(handlers::strategies::create_strategy))
        // 回测
        .route("/api/v1/backtest", post(handlers::backtest::run_backtest))
        .with_state(state)
}
