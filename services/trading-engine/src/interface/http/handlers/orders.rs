//! # 订单处理器
//!
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 处理订单相关的 HTTP 请求

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::domain::port::exchange_query_port::ExchangeQueryPort;
use crate::interface::http::dto::{
    ApiResponse, CancelOrderResponse, OrderListResponse, OrderQueryParams, OrderResponse,
};

/// 应用状态（包含交易所查询端口）
#[derive(Clone)]
pub struct OrderHandlerState {
    pub exchange_query: Arc<dyn ExchangeQueryPort>,
}

/// GET /api/v1/orders
///
/// 查询未完成订单列表
pub async fn list_orders(
    State(state): State<OrderHandlerState>,
    Query(params): Query<OrderQueryParams>,
) -> Json<ApiResponse<OrderListResponse>> {
    let symbol = params.symbol.as_deref();

    match state.exchange_query.get_open_orders(symbol).await {
        Ok(orders) => {
            let responses: Vec<OrderResponse> = orders
                .into_iter()
                .map(|o| OrderResponse {
                    order_id: o.order_id,
                    client_order_id: o.client_order_id,
                    symbol: o.symbol,
                    side: o.side,
                    order_type: o.order_type,
                    status: format!("{:?}", o.status),
                    price: o.price,
                    quantity: o.quantity,
                    executed_qty: o.executed_qty,
                    avg_price: o.avg_price,
                    created_at: o.created_at,
                    updated_at: o.updated_at,
                })
                .collect();

            let total = responses.len();
            Json(ApiResponse::ok(OrderListResponse {
                orders: responses,
                total,
            }))
        }
        Err(e) => {
            tracing::error!(error = %e, "查询订单列表失败");
            Json(ApiResponse::err(format!("查询订单失败: {}", e)))
        }
    }
}

/// GET /api/v1/orders/:symbol/:order_id
///
/// 查询单个订单状态
pub async fn get_order(
    State(state): State<OrderHandlerState>,
    Path((symbol, order_id)): Path<(String, String)>,
) -> Json<ApiResponse<OrderResponse>> {
    match state.exchange_query.get_order(&symbol, &order_id).await {
        Ok(Some(order)) => Json(ApiResponse::ok(OrderResponse {
            order_id: order.order_id,
            client_order_id: order.client_order_id,
            symbol: order.symbol,
            side: order.side,
            order_type: order.order_type,
            status: format!("{:?}", order.status),
            price: order.price,
            quantity: order.quantity,
            executed_qty: order.executed_qty,
            avg_price: order.avg_price,
            created_at: order.created_at,
            updated_at: order.updated_at,
        })),
        Ok(None) => Json(ApiResponse::err("订单不存在")),
        Err(e) => {
            tracing::error!(error = %e, symbol = %symbol, order_id = %order_id, "查询订单失败");
            Json(ApiResponse::err(format!("查询订单失败: {}", e)))
        }
    }
}

/// DELETE /api/v1/orders/:symbol/:order_id
///
/// 撤销单个订单
pub async fn cancel_order(
    State(state): State<OrderHandlerState>,
    Path((symbol, order_id)): Path<(String, String)>,
) -> Json<ApiResponse<CancelOrderResponse>> {
    match state.exchange_query.cancel_order(&symbol, &order_id).await {
        Ok(result) => Json(ApiResponse::ok(CancelOrderResponse {
            order_id: result.order_id,
            symbol: result.symbol,
            success: result.success,
            error: result.error,
        })),
        Err(e) => {
            tracing::error!(error = %e, symbol = %symbol, order_id = %order_id, "撤单失败");
            Json(ApiResponse::err(format!("撤单失败: {}", e)))
        }
    }
}

/// DELETE /api/v1/orders/:symbol
///
/// 撤销某交易对所有订单
pub async fn cancel_all_orders(
    State(state): State<OrderHandlerState>,
    Path(symbol): Path<String>,
) -> Json<ApiResponse<Vec<CancelOrderResponse>>> {
    match state.exchange_query.cancel_all_orders(&symbol).await {
        Ok(results) => {
            let responses: Vec<CancelOrderResponse> = results
                .into_iter()
                .map(|r| CancelOrderResponse {
                    order_id: r.order_id,
                    symbol: r.symbol,
                    success: r.success,
                    error: r.error,
                })
                .collect();
            Json(ApiResponse::ok(responses))
        }
        Err(e) => {
            tracing::error!(error = %e, symbol = %symbol, "批量撤单失败");
            Json(ApiResponse::err(format!("批量撤单失败: {}", e)))
        }
    }
}
