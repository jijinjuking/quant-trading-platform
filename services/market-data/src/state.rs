//! # 应用配置 (Application Config)
//!
//! market-data 服务的配置管理。
//! 注意：market-data 不需要 AppState，只需要配置。

/// 行情服务配置
#[derive(Debug, Clone)]
pub struct MarketDataConfig {
    /// WebSocket 连接地址
    pub ws_url: String,
    /// Kafka broker 地址
    pub kafka_brokers: String,
    /// Kafka topic 名称
    pub kafka_topic: String,
}

impl MarketDataConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        Self {
            ws_url: std::env::var("BINANCE_WS_URL")
                .unwrap_or_else(|_| "wss://stream.binance.com:9443/ws".to_string()),
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            kafka_topic: std::env::var("KAFKA_MARKET_TOPIC")
                .unwrap_or_else(|_| "market-events".to_string()),
        }
    }
}
