//! Generic proxy forwarding helpers.

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    Json,
};
use serde_json::Value;

use crate::state::AppState;

const DEFAULT_RATE_LIMIT_PER_MINUTE: u32 = 120;

pub async fn forward_get(
    state: &AppState,
    service_base: &str,
    backend_path: &str,
    headers: &HeaderMap,
    permission: &str,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_json_request(
        state,
        Method::GET,
        service_base,
        backend_path,
        headers,
        None,
        permission,
    )
    .await
}

pub async fn forward_post(
    state: &AppState,
    service_base: &str,
    backend_path: &str,
    headers: &HeaderMap,
    payload: Value,
    permission: &str,
) -> Result<Json<Value>, (StatusCode, String)> {
    forward_json_request(
        state,
        Method::POST,
        service_base,
        backend_path,
        headers,
        Some(payload),
        permission,
    )
    .await
}


pub async fn forward_public_post(
    state: &AppState,
    service_base: &str,
    backend_path: &str,
    payload: Value,
) -> Result<Json<Value>, (StatusCode, String)> {
    let url = format!(
        "{}/{}",
        service_base.trim_end_matches('/'),
        backend_path.trim_start_matches('/')
    );

    let response = state
        .http_client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_GATEWAY,
                format!("request to backend failed: {}", err),
            )
        })?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        let value = serde_json::from_str::<Value>(&body).unwrap_or_else(|_| {
            serde_json::json!({
                "raw": body,
            })
        });
        Ok(Json(value))
    } else {
        Err((
            StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            body,
        ))
    }
}

pub async fn forward_json_request(
    state: &AppState,
    method: Method,
    service_base: &str,
    backend_path: &str,
    headers: &HeaderMap,
    payload: Option<Value>,
    permission: &str,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_bearer_token(headers)?;

    if !state.auth.validate_token(&token) {
        return Err((StatusCode::UNAUTHORIZED, "invalid token".to_string()));
    }

    let user_id = state
        .auth
        .get_user_id(&token)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "token missing subject".to_string()))?;

    if !state.auth.check_permission(&user_id, permission) {
        return Err((StatusCode::FORBIDDEN, "permission denied".to_string()));
    }

    let rate_key = format!("rate:{}:{}", user_id, permission);
    if !state
        .cache
        .check_rate_limit(&rate_key, DEFAULT_RATE_LIMIT_PER_MINUTE)
    {
        return Err((StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded".to_string()));
    }

    let url = format!(
        "{}/{}",
        service_base.trim_end_matches('/'),
        backend_path.trim_start_matches('/')
    );

    let req_method = reqwest::Method::from_bytes(method.as_str().as_bytes()).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            format!("unsupported http method: {}", method),
        )
    })?;

    let mut request = state
        .http_client
        .request(req_method, &url)
        .header("authorization", format!("Bearer {}", token))
        .header("x-user-id", &user_id);

    if let Some(request_id) = headers.get("x-request-id").and_then(|v| v.to_str().ok()) {
        request = request.header("x-request-id", request_id);
    }

    if let Some(body) = payload {
        request = request.json(&body);
    }

    let response = request.send().await.map_err(|err| {
        (
            StatusCode::BAD_GATEWAY,
            format!("request to backend failed: {}", err),
        )
    })?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        let value = serde_json::from_str::<Value>(&body).unwrap_or_else(|_| {
            serde_json::json!({
                "raw": body,
            })
        });
        Ok(Json(value))
    } else {
        Err((
            StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            body,
        ))
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    let raw = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "missing authorization header".to_string()))?;

    let mut parts = raw.split_whitespace();
    let scheme = parts.next().unwrap_or_default();
    let token = parts.next().unwrap_or_default();

    if !scheme.eq_ignore_ascii_case("bearer") || token.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "authorization must be Bearer <token>".to_string(),
        ));
    }

    Ok(token.to_string())
}

/// Generic proxy endpoint:
/// /api/v1/proxy/{service}/{*path}
pub async fn proxy_request(
    State(state): State<AppState>,
    Path((service, path)): Path<(String, String)>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<Value>, (StatusCode, String)> {
    let service_base = match service.as_str() {
        "strategy-engine" => state.config.services.strategy_engine.as_str(),
        "trading-engine" => state.config.services.trading_engine.as_str(),
        "market-data" => state.config.services.market_data.as_str(),
        "user-management" => state.config.services.user_management.as_str(),
        "risk-management" => state.config.services.risk_management.as_str(),
        _ => return Err((StatusCode::BAD_REQUEST, "unknown backend service".to_string())),
    };

    let payload = if body.is_empty() {
        None
    } else {
        Some(serde_json::from_slice::<Value>(&body).map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("proxy request body must be valid JSON: {}", err),
            )
        })?)
    };

    let permission = if method == Method::GET {
        "proxy:read"
    } else {
        "proxy:write"
    };

    forward_json_request(
        &state,
        method,
        service_base,
        &path,
        &headers,
        payload,
        permission,
    )
    .await
}
