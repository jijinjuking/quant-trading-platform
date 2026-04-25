//! User HTTP handlers.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::application::service::auth_service::{AuthService, UserView};
use crate::state::AppState;

pub async fn get_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserView>, (StatusCode, String)> {
    let token = extract_bearer_token(&headers)?;
    let service = AuthService::new(
        state.user_repository.clone(),
        state.config.jwt_secret.clone(),
    );

    let user = service
        .user_from_token(&token)
        .map_err(|err| (StatusCode::UNAUTHORIZED, err.to_string()))?;

    Ok(Json(user))
}

fn extract_bearer_token(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    let value = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "missing authorization header".to_string()))?;

    let mut parts = value.split_whitespace();
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
