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
    /// 订阅的交易对（小写）
    pub symbols: Vec<String>,
    /// 代理地址（可选）
    pub proxy_url: Option<String>,
    /// ClickHouse URL
    pub clickhouse_url: String,
    /// ClickHouse 数据库名
    pub clickhouse_database: String,
    /// ClickHouse 表名
    pub clickhouse_table: String,
    /// 是否启用存储
    pub storage_enabled: bool,
}

impl MarketDataConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let symbols_raw = std::env::var("MARKET_DATA_SYMBOLS")
            .unwrap_or_else(|_| "btcusdt".to_string());
        let mut symbols: Vec<String> = symbols_raw
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        if symbols.is_empty() {
            symbols.push("btcusdt".to_string());
        }

        Self {
            ws_url: std::env::var("BINANCE_WS_URL")
                .unwrap_or_else(|_| "wss://stream.binance.com:9443/ws".to_string()),
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            kafka_topic: std::env::var("KAFKA_MARKET_TOPIC")
                .unwrap_or_else(|_| "market-events".to_string()),
            symbols,
            proxy_url: std::env::var("MARKET_DATA_PROXY")
                .ok()
                .or_else(|| std::env::var("HTTPS_PROXY").ok())
                .or_else(|| std::env::var("HTTP_PROXY").ok()),
            clickhouse_url: std::env::var("CLICKHOUSE_URL")
                .unwrap_or_else(|_| "http://localhost:8123".to_string()),
            clickhouse_database: std::env::var("CLICKHOUSE_DATABASE")
                .unwrap_or_else(|_| "market_data".to_string()),
            clickhouse_table: std::env::var("CLICKHOUSE_TABLE")
                .unwrap_or_else(|_| "trades".to_string()),
            storage_enabled: std::env::var("MARKET_DATA_STORAGE_ENABLED")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),
        }
    }
}
