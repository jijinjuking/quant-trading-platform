//! # 路由配置模块
//!
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 定义URL路径到Handler的映射关系

use std::sync::Arc;

use axum::{routing::{delete, get}, Router};

use crate::domain::port::exchange_query_port::ExchangeQueryPort;
use super::handlers;

/// # 创建路由器
///
/// ## 参数:
/// - exchange_query: 交易所查询端口
///
/// ## 返回:
/// - Router: 配置好的Axum路由器
///
/// ## 路由映射:
/// - GET  /health                          -> 健康检查
/// - GET  /api/v1/orders                   -> 查询未完成订单
/// - GET  /api/v1/orders/:symbol/:order_id -> 查询单个订单
/// - DELETE /api/v1/orders/:symbol/:order_id -> 撤销订单
/// - DELETE /api/v1/orders/:symbol         -> 撤销某交易对所有订单
/// - GET  /api/v1/positions                -> 查询持仓
/// - GET  /api/v1/account/balances         -> 查询账户余额
pub fn create_router(exchange_query: Arc<dyn ExchangeQueryPort>) -> Router {
    // 订单相关状态
    let order_state = handlers::orders::OrderHandlerState {
        exchange_query: exchange_query.clone(),
    };

    // 持仓相关状态
    let position_state = handlers::positions::PositionHandlerState {
        exchange_query: exchange_query.clone(),
    };

    // 账户相关状态
    let account_state = handlers::account::AccountHandlerState {
        exchange_query: exchange_query.clone(),
    };

    // 订单路由
    let order_routes = Router::new()
        .route("/", get(handlers::orders::list_orders))
        .route("/:symbol/:order_id", get(handlers::orders::get_order))
        .route("/:symbol/:order_id", delete(handlers::orders::cancel_order))
        .route("/:symbol", delete(handlers::orders::cancel_all_orders))
        .with_state(order_state);

    // 持仓路由
    let position_routes = Router::new()
        .route("/", get(handlers::positions::list_positions))
        .with_state(position_state);

    // 账户路由
    let account_routes = Router::new()
        .route("/balances", get(handlers::account::get_balances))
        .with_state(account_state);

    // 组合所有路由
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .nest("/api/v1/orders", order_routes)
        .nest("/api/v1/positions", position_routes)
        .nest("/api/v1/account", account_routes)
}
