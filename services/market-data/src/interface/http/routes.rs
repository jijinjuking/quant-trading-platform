//! # 路由配置 (Route Configuration)
//! 
//! 定义行情数据服务的所有 HTTP 路由。
//! 
//! ## API 端点
//! - `GET /health`: 健康检查
//! - `GET /api/v1/market/pairs`: 获取交易对列表
//! - `GET /api/v1/market/ticker/:symbol`: 获取指定交易对的 Ticker
//! - `GET /api/v1/market/klines/:symbol`: 获取指定交易对的 K 线数据

// ============================================================================
// 外部依赖导入
// ============================================================================

use axum::{routing::get, Router};  // Axum 路由
use crate::state::AppState;         // 应用状态
use super::handlers;                // 请求处理器

// ============================================================================
// 路由创建函数
// ============================================================================

/// 创建 HTTP 路由器
/// 
/// 配置所有 API 端点和对应的处理器。
/// 
/// # 参数
/// - `state`: 应用状态，将被注入到所有处理器
/// 
/// # 返回
/// - 配置好的 Axum Router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点 - 用于负载均衡器探测
        .route("/health", get(handlers::health::health_check))
        // 交易对列表 - 获取所有支持的交易对
        .route("/api/v1/market/pairs", get(handlers::market::list_pairs))
        // Ticker 数据 - 获取指定交易对的最新价格
        .route("/api/v1/market/ticker/:symbol", get(handlers::market::get_ticker))
        // K 线数据 - 获取指定交易对的历史 K 线
        .route("/api/v1/market/klines/:symbol", get(handlers::market::get_klines))
        // 注入应用状态
        .with_state(state)
}
