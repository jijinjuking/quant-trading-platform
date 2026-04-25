//! Application state for risk-management.

use anyhow::Result;
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::domain::port::risk_repository_port::RiskRepositoryPort;
use crate::infrastructure::repository::risk_repository::RiskRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub risk_repository: Arc<dyn RiskRepositoryPort>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub allowed_symbols: Vec<String>,
    pub min_quantity: Decimal,
    pub max_quantity: Decimal,
    pub max_notional: Decimal,
    pub max_position: Decimal,
    pub max_leverage: Decimal,
    pub max_drawdown: Decimal,
    pub daily_loss_limit: Decimal,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let allowed_symbols = std::env::var("RISK_ALLOW_SYMBOLS")
            .ok()
            .map(|s| s.split(",").map(|x| x.trim().to_uppercase()).collect())
            .unwrap_or_default();

        let config = AppConfig {
            allowed_symbols,
            min_quantity: read_decimal_env("RISK_MIN_QTY", Decimal::new(1, 4)),
            max_quantity: read_decimal_env("RISK_MAX_QTY", Decimal::new(10, 0)),
            max_notional: read_decimal_env("RISK_MAX_NOTIONAL", Decimal::new(100000, 0)),
            max_position: read_decimal_env("RISK_MAX_POSITION", Decimal::new(100, 0)),
            max_leverage: read_decimal_env("RISK_MAX_LEVERAGE", Decimal::new(10, 0)),
            max_drawdown: read_decimal_env("RISK_MAX_DRAWDOWN", Decimal::new(20, 2)),
            daily_loss_limit: read_decimal_env("RISK_DAILY_LOSS_LIMIT", Decimal::new(5000, 0)),
        };

        let risk_repository: Arc<dyn RiskRepositoryPort> = Arc::new(RiskRepository::with_defaults(
            config.max_leverage,
            config.max_drawdown,
            config.max_position,
            config.daily_loss_limit,
        ));

        Ok(Self {
            config: Arc::new(config),
            risk_repository,
        })
    }
}

fn read_decimal_env(key: &str, default: Decimal) -> Decimal {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
