//! 交易引擎应用状态与配置。

use anyhow::{Context, Result};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
}

/// 应用配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Kafka broker 地址
    pub kafka_brokers: String,
    /// 行情事件 topic
    pub kafka_market_topic: String,
    /// 消费者组 ID
    pub kafka_consumer_group: String,
    /// 执行模式: "binance" 或 "noop"
    pub execution_mode: String,
    /// 策略模式: "remote" 或 "noop"
    pub strategy_mode: String,
    /// Strategy Engine 服务地址
    pub strategy_engine_url: Option<String>,
    /// 风控模式: "remote" 或 "local"
    pub risk_mode: String,
    /// Risk Management 服务地址（remote 模式需要）
    pub risk_management_url: Option<String>,
    /// 币安 API 密钥
    pub binance_api_key: Option<String>,
    /// 币安 API 密钥签名
    pub binance_secret_key: Option<String>,
    /// 币安 API 地址
    pub binance_base_url: String,
    /// 风控: 最小数量
    pub risk_min_qty: Option<Decimal>,
    /// 风控: 最大数量
    pub risk_max_qty: Option<Decimal>,
    /// 风控: 最大名义价值
    pub risk_max_notional: Option<Decimal>,
    /// 风控: 允许的交易对
    pub risk_allow_symbols: Option<Vec<String>>,
    /// 是否启用订单存储
    pub storage_enabled: bool,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let config = AppConfig {
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            kafka_market_topic: std::env::var("KAFKA_MARKET_TOPIC")
                .unwrap_or_else(|_| "market-events".to_string()),
            kafka_consumer_group: std::env::var("KAFKA_CONSUMER_GROUP")
                .unwrap_or_else(|_| "trading-engine".to_string()),
            execution_mode: std::env::var("TRADING_EXECUTION_MODE")
                .unwrap_or_else(|_| "noop".to_string()),
            strategy_mode: std::env::var("TRADING_STRATEGY_MODE")
                .unwrap_or_else(|_| "noop".to_string()),
            strategy_engine_url: std::env::var("STRATEGY_ENGINE_URL").ok(),
            risk_mode: std::env::var("TRADING_RISK_MODE")
                .unwrap_or_else(|_| "local".to_string()),
            risk_management_url: std::env::var("RISK_MANAGEMENT_URL").ok(),
            binance_api_key: std::env::var("BINANCE_API_KEY").ok(),
            binance_secret_key: std::env::var("BINANCE_SECRET_KEY").ok(),
            binance_base_url: std::env::var("BINANCE_BASE_URL")
                .unwrap_or_else(|_| "https://testnet.binance.vision".to_string()),
            risk_min_qty: parse_decimal_env("TRADING_RISK_MIN_QTY")?,
            risk_max_qty: parse_decimal_env("TRADING_RISK_MAX_QTY")?,
            risk_max_notional: parse_decimal_env("TRADING_RISK_MAX_NOTIONAL")?,
            risk_allow_symbols: parse_symbols_env("TRADING_RISK_ALLOW_SYMBOLS"),
            storage_enabled: std::env::var("TRADING_STORAGE_ENABLED")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
        };

        Ok(Self {
            config: Arc::new(config),
        })
    }
}

fn parse_decimal_env(key: &str) -> Result<Option<Decimal>> {
    let raw = std::env::var(key).ok();
    let value = match raw {
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(
                    Decimal::from_str(trimmed)
                        .with_context(|| format!("invalid decimal for {}", key))?,
                )
            }
        }
        None => None,
    };

    Ok(value)
}

fn parse_symbols_env(key: &str) -> Option<Vec<String>> {
    std::env::var(key)
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(|item| item.trim().to_string())
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|items| !items.is_empty())
}
