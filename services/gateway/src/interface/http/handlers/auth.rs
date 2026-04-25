//! Auth handlers for gateway.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;

use crate::interface::http::handlers::proxy::{forward_get, forward_public_post};
use crate::state::AppState;

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_public_post(
        &state,
        &state.config.services.user_management,
        "/api/v1/auth/login",
        payload,
    )
    .await
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_public_post(
        &state,
        &state.config.services.user_management,
        "/api/v1/auth/register",
        payload,
    )
    .await
}

pub async fn profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_get(
        &state,
        &state.config.services.user_management,
        "/api/v1/user/profile",
        &headers,
        "users:read",
    )
    .await
}
