//! Auth HTTP handlers.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::application::service::auth_service::{AuthService, UserView};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserView,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user: UserView,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let service = AuthService::new(
        state.user_repository.clone(),
        state.config.jwt_secret.clone(),
    );

    match service.login(&req.email, &req.password).await {
        Ok((token, user)) => Ok(Json(LoginResponse { token, user })),
        Err(err) => Err((StatusCode::UNAUTHORIZED, err.to_string())),
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {
    let service = AuthService::new(
        state.user_repository.clone(),
        state.config.jwt_secret.clone(),
    );

    match service.register(&req.username, &req.email, &req.password).await {
        Ok(user) => Ok(Json(RegisterResponse { user })),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}
