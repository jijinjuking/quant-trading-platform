//! Order handlers.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;

use crate::interface::http::handlers::proxy::{forward_get, forward_post};
use crate::state::AppState;

pub async fn list_orders(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_get(
        &state,
        &state.config.services.trading_engine,
        "/api/v1/orders",
        &headers,
        "orders:read",
    )
    .await
}

pub async fn create_order(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_post(
        &state,
        &state.config.services.trading_engine,
        "/api/v1/orders",
        &headers,
        payload,
        "orders:write",
    )
    .await
}
