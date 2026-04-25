//! Strategy handlers.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;

use crate::interface::http::handlers::proxy::{forward_get, forward_post};
use crate::state::AppState;

pub async fn list_strategies(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_get(
        &state,
        &state.config.services.strategy_engine,
        "/api/v1/strategies",
        &headers,
        "strategies:read",
    )
    .await
}

pub async fn create_strategy(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_post(
        &state,
        &state.config.services.strategy_engine,
        "/api/v1/strategies",
        &headers,
        payload,
        "strategies:write",
    )
    .await
}
