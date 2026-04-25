//! Application state for user-management.

#![allow(dead_code)]

use anyhow::{Context, Result};
use std::sync::Arc;

use crate::domain::port::user_repository_port::UserRepositoryPort;
use crate::infrastructure::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_repository: Arc<dyn UserRepositoryPort>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let config = AppConfig {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/trading".to_string()),
            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET is required")?,
        };

        let user_repository: Arc<dyn UserRepositoryPort> = Arc::new(UserRepository::new());

        Ok(Self {
            config: Arc::new(config),
            user_repository,
        })
    }
}
