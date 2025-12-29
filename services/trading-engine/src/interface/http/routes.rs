//! # 路由配置模块
//! 
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 定义URL路径到Handler的映射关系

// ============================================================
// 外部依赖导入
// ============================================================
use axum::{routing::{get, post}, Router};  // Axum路由
use crate::state::AppState;                 // 应用状态
use super::handlers;                        // Handler模块

/// # 创建路由器
/// 
/// ## 参数:
/// - state: 应用状态，会被注入到所有handler中
/// 
/// ## 返回:
/// - Router: 配置好的Axum路由器
/// 
/// ## 路由映射:
/// - GET  /health           -> 健康检查
/// - POST /api/v1/orders    -> 创建订单
/// - GET  /api/v1/orders    -> 查询订单列表
/// - GET  /api/v1/positions -> 查询持仓列表
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // 订单相关端点
        .route("/api/v1/orders", post(handlers::orders::create_order))
        .route("/api/v1/orders", get(handlers::orders::list_orders))
        // 持仓相关端点
        .route("/api/v1/positions", get(handlers::positions::list_positions))
        // 注入应用状态
        .with_state(state)
}
