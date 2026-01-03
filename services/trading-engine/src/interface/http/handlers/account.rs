//! # 账户处理器
//!
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 处理账户相关的 HTTP 请求

use std::sync::Arc;

use axum::{extract::State, Json};

use crate::domain::port::exchange_query_port::ExchangeQueryPort;
use crate::interface::http::dto::{ApiResponse, BalanceListResponse, BalanceResponse};

/// 应用状态（包含交易所查询端口）
#[derive(Clone)]
pub struct AccountHandlerState {
    pub exchange_query: Arc<dyn ExchangeQueryPort>,
}

/// GET /api/v1/account/balances
///
/// 查询现货账户余额
pub async fn get_balances(
    State(state): State<AccountHandlerState>,
) -> Json<ApiResponse<BalanceListResponse>> {
    match state.exchange_query.get_spot_balances().await {
        Ok(balances) => {
            let responses: Vec<BalanceResponse> = balances
                .into_iter()
                .map(|b| BalanceResponse {
                    asset: b.asset,
                    free: b.free,
                    locked: b.locked,
                    total: b.free + b.locked,
                })
                .collect();

            let total = responses.len();
            Json(ApiResponse::ok(BalanceListResponse {
                balances: responses,
                total,
            }))
        }
        Err(e) => {
            tracing::error!(error = %e, "查询账户余额失败");
            Json(ApiResponse::err(format!("查询账户余额失败: {}", e)))
        }
    }
}
