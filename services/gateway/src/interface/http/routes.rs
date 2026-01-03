//! # HTTP 路由配置模块
//!
//! 本模块负责配置 API Gateway 的 HTTP 路由。
//!
//! ## 路由列表
//! | 方法 | 路径 | 处理器 | 说明 |
//! |------|------|--------|------|
//! | GET | /health | health_check | 健康检查 |
//! | GET | /api/v1/services | check_services | 服务状态 |
//! | GET | /api/v1/strategies | list_strategies | 策略列表 |
//! | POST | /api/v1/strategies | create_strategy | 创建策略 |
//! | GET | /api/v1/orders | list_orders | 订单列表 |
//! | POST | /api/v1/orders | create_order | 创建订单 |
//! | GET | /api/v1/positions | list_positions | 持仓列表 |

use axum::{routing::{get, post}, Router};
use tower_http::cors::{CorsLayer, Any};
use crate::state::AppState;
use super::handlers;

/// 创建 HTTP 路由器
///
/// 配置所有 HTTP 端点并绑定应用状态。
pub fn create_router(state: AppState) -> Router {
    // CORS 配置 - 允许前端跨域访问
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // 服务状态端点
        .route("/api/v1/services", get(handlers::services::check_services))
        // 策略管理端点（代理到 strategy-engine）
        .route("/api/v1/strategies", get(handlers::strategies::list_strategies))
        .route("/api/v1/strategies", post(handlers::strategies::create_strategy))
        // 订单管理端点（代理到 trading-engine）
        .route("/api/v1/orders", get(handlers::orders::list_orders))
        .route("/api/v1/orders", post(handlers::orders::create_order))
        // 持仓管理端点（代理到 trading-engine）
        .route("/api/v1/positions", get(handlers::positions::list_positions))
        // CORS 中间件
        .layer(cors)
        // 注入应用状态
        .with_state(state)
}
