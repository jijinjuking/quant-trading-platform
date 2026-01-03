//! # 持仓处理器
//!
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 处理持仓相关的 HTTP 请求

use std::sync::Arc;

use axum::{extract::State, Json};

use crate::domain::port::exchange_query_port::ExchangeQueryPort;
use crate::interface::http::dto::{ApiResponse, PositionListResponse, PositionResponse};

/// 应用状态（包含交易所查询端口）
#[derive(Clone)]
pub struct PositionHandlerState {
    pub exchange_query: Arc<dyn ExchangeQueryPort>,
}

/// GET /api/v1/positions
///
/// 查询合约持仓列表
pub async fn list_positions(
    State(state): State<PositionHandlerState>,
) -> Json<ApiResponse<PositionListResponse>> {
    match state.exchange_query.get_futures_positions().await {
        Ok(positions) => {
            let responses: Vec<PositionResponse> = positions
                .into_iter()
                .map(|p| PositionResponse {
                    symbol: p.symbol,
                    side: p.side,
                    quantity: p.quantity,
                    entry_price: p.entry_price,
                    mark_price: p.mark_price,
                    unrealized_pnl: p.unrealized_pnl,
                    leverage: p.leverage,
                    margin_type: p.margin_type,
                })
                .collect();

            let total = responses.len();
            Json(ApiResponse::ok(PositionListResponse {
                positions: responses,
                total,
            }))
        }
        Err(e) => {
            tracing::error!(error = %e, "查询持仓失败");
            Json(ApiResponse::err(format!("查询持仓失败: {}", e)))
        }
    }
}
