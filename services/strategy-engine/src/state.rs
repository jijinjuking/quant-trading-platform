//! # 策略引擎状态与配置 (Strategy Engine State)
//!
//! 应用状态与环境配置读取。

use anyhow::{Context, Result};
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::domain::logic::grid::GridConfig;
use crate::domain::logic::mean::MeanReversionConfig;
use crate::domain::model::strategy_config::StrategyType;
use crate::domain::port::StrategyStatePort;
use crate::infrastructure::cache::RedisStrategyStateAdapter;

/// 应用状态容器 (Application State)
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    /// 策略状态存储端口（Redis）
    pub strategy_state: Arc<dyn StrategyStatePort>,
}

/// 应用配置 (Application Configuration)
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub kafka_brokers: String,
    pub kafka_market_topic: String,
    pub kafka_signal_topic: String,
    pub kafka_consumer_group: String,
    pub strategy_type: StrategyType,
    pub grid_config: GridConfig,
    pub mean_reversion_config: MeanReversionConfig,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let grid_config = GridConfig {
            upper_price: read_decimal_env("STRATEGY_GRID_UPPER", Decimal::new(100_000, 0)),
            lower_price: read_decimal_env("STRATEGY_GRID_LOWER", Decimal::new(90_000, 0)),
            grid_count: read_u32_env("STRATEGY_GRID_COUNT", 10),
            quantity_per_grid: read_decimal_env("STRATEGY_GRID_QUANTITY", Decimal::new(1, 3)),
        };

        let mean_reversion_config = MeanReversionConfig {
            window_size: read_usize_env("STRATEGY_MEAN_WINDOW", 20),
            threshold_percent: read_decimal_env("STRATEGY_MEAN_THRESHOLD", Decimal::new(2, 2)),
            quantity: read_decimal_env("STRATEGY_MEAN_QUANTITY", Decimal::new(1, 3)),
        };

        let config = AppConfig {
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            kafka_market_topic: std::env::var("KAFKA_MARKET_TOPIC")
                .unwrap_or_else(|_| "market-events".to_string()),
            kafka_signal_topic: std::env::var("KAFKA_SIGNAL_TOPIC")
                .unwrap_or_else(|_| "trading.signals".to_string()),
            kafka_consumer_group: std::env::var("KAFKA_CONSUMER_GROUP")
                .unwrap_or_else(|_| "strategy-engine".to_string()),
            strategy_type: read_strategy_type(),
            grid_config,
            mean_reversion_config,
        };

        // 创建 Redis 策略状态存储
        let strategy_state: Arc<dyn StrategyStatePort> = Arc::new(
            RedisStrategyStateAdapter::from_env()
                .context("创建 Redis 策略状态存储失败")?,
        );

        Ok(Self {
            config: Arc::new(config),
            strategy_state,
        })
    }
}

fn read_decimal_env(key: &str, default: Decimal) -> Decimal {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<Decimal>().ok())
        .unwrap_or(default)
}

fn read_u32_env(key: &str, default: u32) -> u32 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(default)
}

fn read_usize_env(key: &str, default: usize) -> usize {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
}

fn read_strategy_type() -> StrategyType {
    match std::env::var("STRATEGY_TYPE").ok() {
        Some(value)
            if value.eq_ignore_ascii_case("mean")
                || value.eq_ignore_ascii_case("mean_reversion")
                || value.eq_ignore_ascii_case("meanreversion") =>
        {
            StrategyType::MeanReversion
        }
        Some(value) if value.eq_ignore_ascii_case("grid") => StrategyType::Grid,
        _ => StrategyType::Grid,
    }
}
