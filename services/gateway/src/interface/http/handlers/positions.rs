//! Position handlers.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;

use crate::interface::http::handlers::proxy::forward_get;
use crate::state::AppState;

pub async fn list_positions(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_get(
        &state,
        &state.config.services.trading_engine,
        "/api/v1/positions",
        &headers,
        "positions:read",
    )
    .await
}
