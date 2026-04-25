//! Risk handlers.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;

use crate::interface::http::handlers::proxy::forward_post;
use crate::state::AppState;

pub async fn check_risk(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_post(
        &state,
        &state.config.services.risk_management,
        "/api/v1/risk/check",
        &headers,
        payload,
        "risk:write",
    )
    .await
}
