// src/kafka.rs

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 导入shared_models中的类型
use shared_models::{
    market::{MarketTick, Kline, OrderBook, Ticker24hr, Trade},
    strategy::{StrategySignal, Strategy, BacktestConfig, BacktestResult},
    risk::{RiskViolation, RiskMetric},
    user::User,
    trading::{Order, Position, Balance, Trade as TradingTrade},
};

/// Kafka消息包装�?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaMessage<T> {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
    pub event_type: String,
    pub version: String,
    pub data: T,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl<T> KafkaMessage<T> {
    pub fn new(event_type: &str, source: &str, data: T) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            source: source.to_string(),
            event_type: event_type.to_string(),
            version: "1.0".to_string(),
            data,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }
}

/// Kafka主题定义
pub struct KafkaTopics;

impl KafkaTopics {
    // 市场数据主题
    pub const MARKET_TICKS: &'static str = "market.ticks";
    pub const MARKET_KLINES: &'static str = "market.klines";
    pub const MARKET_ORDERBOOK: &'static str = "market.orderbook";
    pub const MARKET_TRADES: &'static str = "market.trades";
    pub const MARKET_TICKER24HR: &'static str = "market.ticker24hr";

    // 交易事件主题
    pub const TRADING_ORDERS: &'static str = "trading.orders";
    pub const TRADING_TRADES: &'static str = "trading.trades";
    pub const TRADING_POSITIONS: &'static str = "trading.positions";
    pub const TRADING_BALANCES: &'static str = "trading.balances";

    // 策略事件主题
    pub const STRATEGY_SIGNALS: &'static str = "strategy.signals";
    pub const STRATEGY_UPDATES: &'static str = "strategy.updates";
    pub const BACKTEST_EVENTS: &'static str = "backtest.events";

    // 风险管理主题
    pub const RISK_ALERTS: &'static str = "risk.alerts";
    pub const RISK_VIOLATIONS: &'static str = "risk.violations";
    pub const RISK_METRICS: &'static str = "risk.metrics";

    // 用户事件主题
    pub const USER_EVENTS: &'static str = "user.events";
    pub const USER_ACTIVITIES: &'static str = "user.activities";

    // 系统事件主题
    pub const SYSTEM_EVENTS: &'static str = "system.events";
    pub const SYSTEM_METRICS: &'static str = "system.metrics";
    pub const SYSTEM_LOGS: &'static str = "system.logs";

    // 通知主题
    pub const NOTIFICATIONS: &'static str = "notifications";
    pub const ALERTS: &'static str = "alerts";
}

/// 市场数据事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum MarketDataEvent {
    TickUpdate(MarketTick),
    KlineUpdate(Kline),
    OrderBookUpdate(OrderBook),
    TradeUpdate(Trade),
    Ticker24hrUpdate(Ticker24hr),
}

/// 交易事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum TradingEvent {
    OrderCreated(Order),
    OrderUpdated(Order),
    OrderCancelled(Order),
    OrderFilled(Order),
    TradeExecuted(TradingTrade),
    PositionOpened(Position),
    PositionUpdated(Position),
    PositionClosed(Position),
    BalanceUpdated(Balance),
}

/// 策略事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum StrategyEvent {
    SignalGenerated(StrategySignal),
    StrategyStarted(Strategy),
    StrategyStopped(Strategy),
    StrategyUpdated(Strategy),
    BacktestStarted(BacktestConfig),
    BacktestCompleted(BacktestResult),
    BacktestFailed { config_id: uuid::Uuid, error: String },
}

/// 风险管理事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RiskEvent {
    RiskViolation(RiskViolation),
    RiskAlert(shared_models::risk::RiskEvent),
    RiskMetricUpdate(RiskMetric),
    RiskLimitBreached { limit_id: uuid::Uuid, current_value: rust_decimal::Decimal },
}

/// 用户事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum UserEvent {
    UserRegistered(User),
    UserLoggedIn { user_id: uuid::Uuid, ip_address: String },
    UserLoggedOut { user_id: uuid::Uuid },
    UserUpdated(User),
    UserDeactivated { user_id: uuid::Uuid },
    ApiKeyCreated { user_id: uuid::Uuid, api_key_id: uuid::Uuid },
    ApiKeyRevoked { user_id: uuid::Uuid, api_key_id: uuid::Uuid },
}

/// 系统事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum SystemEvent {
    ServiceStarted { service_name: String, version: String },
    ServiceStopped { service_name: String },
    ServiceHealthChanged { service_name: String, status: String },
    DatabaseConnectionLost { database: String },
    DatabaseConnectionRestored { database: String },
    ExchangeConnectionLost { exchange: String },
    ExchangeConnectionRestored { exchange: String },
    MaintenanceStarted { reason: String },
    MaintenanceCompleted,
}

/// 通知事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub user_id: uuid::Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub channels: Vec<NotificationChannel>,
    pub priority: NotificationPriority,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 通知类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    TradingAlert,
    PriceAlert,
    RiskAlert,
    SystemAlert,
    MarketingMessage,
    AccountUpdate,
}

/// 通知渠道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    SMS,
    Push,
    InApp,
    Webhook,
}

/// 通知优先�?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Kafka生产者配�?
#[derive(Debug, Clone)]
pub struct KafkaProducerConfig {
    pub brokers: Vec<String>,
    pub client_id: String,
    pub acks: String,
    pub retries: u32,
    pub batch_size: u32,
    pub linger_ms: u32,
    pub buffer_memory: u64,
    pub compression_type: String,
    pub max_request_size: u32,
}

impl Default for KafkaProducerConfig {
    fn default() -> Self {
        Self {
            brokers: vec!["localhost:9092".to_string()],
            client_id: "trading-platform-producer".to_string(),
            acks: "all".to_string(),
            retries: 3,
            batch_size: 16384,
            linger_ms: 5,
            buffer_memory: 33554432, // 32MB
            compression_type: "snappy".to_string(),
            max_request_size: 1048576, // 1MB
        }
    }
}

/// Kafka消费者配�?
#[derive(Debug, Clone)]
pub struct KafkaConsumerConfig {
    pub brokers: Vec<String>,
    pub group_id: String,
    pub client_id: String,
    pub auto_offset_reset: String,
    pub enable_auto_commit: bool,
    pub auto_commit_interval_ms: u32,
    pub session_timeout_ms: u32,
    pub heartbeat_interval_ms: u32,
    pub max_poll_records: u32,
    pub fetch_min_bytes: u32,
    pub fetch_max_wait_ms: u32,
}

impl Default for KafkaConsumerConfig {
    fn default() -> Self {
        Self {
            brokers: vec!["localhost:9092".to_string()],
            group_id: "trading-platform-consumer".to_string(),
            client_id: "trading-platform-consumer".to_string(),
            auto_offset_reset: "earliest".to_string(),
            enable_auto_commit: true,
            auto_commit_interval_ms: 5000,
            session_timeout_ms: 30000,
            heartbeat_interval_ms: 3000,
            max_poll_records: 500,
            fetch_min_bytes: 1,
            fetch_max_wait_ms: 500,
        }
    }
}

/// Kafka消息生产者trait
#[async_trait::async_trait]
pub trait MessageProducer: Send + Sync {
    async fn send_message<T: Serialize + Send + Sync>(
        &self,
        topic: &str,
        key: Option<&str>,
        message: &KafkaMessage<T>,
    ) -> Result<()>;

    async fn send_batch<T: Serialize + Send + Sync>(
        &self,
        topic: &str,
        messages: Vec<(Option<String>, KafkaMessage<T>)>,
    ) -> Result<()>;
}

/// Kafka消息消费者trait
#[async_trait::async_trait]
pub trait MessageConsumer: Send + Sync {
    async fn consume_messages<F>(&mut self, handler: F) -> Result<()>
    where
        F: Fn(&str, &[u8]) -> Result<()> + Send + Sync;

    async fn subscribe(&mut self, topics: Vec<String>) -> Result<()>;
    async fn unsubscribe(&mut self) -> Result<()>;
}

/// 消息处理器trait
#[async_trait::async_trait]
pub trait MessageHandler<T>: Send + Sync {
    async fn handle(&self, message: &KafkaMessage<T>) -> Result<()>;
}

/// 市场数据消息处理�?
pub struct MarketDataHandler;

#[async_trait::async_trait]
impl MessageHandler<MarketDataEvent> for MarketDataHandler {
    async fn handle(&self, message: &KafkaMessage<MarketDataEvent>) -> Result<()> {
        match &message.data {
            MarketDataEvent::TickUpdate(tick) => {
                tracing::info!("Received tick update: {} @ {}", tick.symbol, tick.price);
                // 处理tick更新逻辑
            }
            MarketDataEvent::KlineUpdate(kline) => {
                tracing::info!("Received kline update: {} {} @ {}", kline.symbol, kline.interval, kline.close);
                // 处理K线更新逻辑
            }
            MarketDataEvent::OrderBookUpdate(orderbook) => {
                tracing::info!("Received orderbook update: {}", orderbook.symbol);
                // 处理订单簿更新逻辑
            }
            MarketDataEvent::TradeUpdate(trade) => {
                tracing::info!("Received trade update: {} {} @ {}", trade.symbol, trade.quantity, trade.price);
                // 处理交易更新逻辑
            }
            MarketDataEvent::Ticker24hrUpdate(ticker) => {
                tracing::info!("Received 24hr ticker update: {}", ticker.symbol);
                // 处理24小时统计更新逻辑
            }
        }
        Ok(())
    }
}

/// 交易事件处理�?
pub struct TradingEventHandler;

#[async_trait::async_trait]
impl MessageHandler<TradingEvent> for TradingEventHandler {
    async fn handle(&self, message: &KafkaMessage<TradingEvent>) -> Result<()> {
        match &message.data {
            TradingEvent::OrderCreated(order) => {
                tracing::info!("Order created: {} {} {} @ {:?}", order.symbol, order.side, order.quantity, order.price);
                // 处理订单创建逻辑
            }
            TradingEvent::OrderFilled(order) => {
                tracing::info!("Order filled: {} {} {}", order.symbol, order.side, order.filled_quantity);
                // 处理订单成交逻辑
            }
            TradingEvent::TradeExecuted(trade) => {
                tracing::info!("Trade executed: {} {} @ {}", trade.symbol, trade.quantity, trade.price);
                // 处理交易执行逻辑
            }
            TradingEvent::PositionUpdated(position) => {
                tracing::info!("Position updated: {} {}", position.symbol, position.size);
                // 处理仓位更新逻辑
            }
            TradingEvent::BalanceUpdated(balance) => {
                tracing::info!("Balance updated: {} {}", balance.asset, balance.total);
                // 处理余额更新逻辑
            }
            _ => {
                tracing::debug!("Received trading event: {:?}", message.event_type);
            }
        }
        Ok(())
    }
}

/// 消息路由�?
pub struct MessageRouter {
    handlers: HashMap<String, Box<dyn MessageHandlerDyn>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, event_type: &str, handler: Box<dyn MessageHandlerDyn>) {
        self.handlers.insert(event_type.to_string(), handler);
    }

    pub async fn route_message(&self, topic: &str, message: &[u8]) -> Result<()> {
        // 解析消息
        let raw_message: serde_json::Value = serde_json::from_slice(message)?;
        let event_type = raw_message
            .get("event_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        // 路由到对应的处理�?
        if let Some(handler) = self.handlers.get(event_type) {
            handler.handle_raw(topic, message).await?;
        } else {
            tracing::warn!("No handler found for event type: {}", event_type);
        }

        Ok(())
    }
}

impl Default for MessageRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// 动态消息处理器trait
#[async_trait::async_trait]
pub trait MessageHandlerDyn: Send + Sync {
    async fn handle_raw(&self, topic: &str, message: &[u8]) -> Result<()>;
}

/// 消息序列化工�?
pub struct MessageSerializer;

impl MessageSerializer {
    /// 序列化消�?
    pub fn serialize<T: Serialize>(message: &KafkaMessage<T>) -> Result<Vec<u8>> {
        let json = serde_json::to_vec(message)?;
        Ok(json)
    }

    /// 反序列化消息
    pub fn deserialize<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<KafkaMessage<T>> {
        let message = serde_json::from_slice(data)?;
        Ok(message)
    }

    /// 序列化为字符�?
    pub fn serialize_to_string<T: Serialize>(message: &KafkaMessage<T>) -> Result<String> {
        let json = serde_json::to_string(message)?;
        Ok(json)
    }

    /// 从字符串反序列化
    pub fn deserialize_from_string<T: for<'de> Deserialize<'de>>(data: &str) -> Result<KafkaMessage<T>> {
        let message = serde_json::from_str(data)?;
        Ok(message)
    }
}

/// 消息验证�?
pub struct MessageValidator;

impl MessageValidator {
    /// 验证消息格式
    pub fn validate_message<T>(message: &KafkaMessage<T>) -> Result<()> {
        if message.id.is_empty() {
            return Err(anyhow::anyhow!("Message ID cannot be empty"));
        }

        if message.event_type.is_empty() {
            return Err(anyhow::anyhow!("Event type cannot be empty"));
        }

        if message.source.is_empty() {
            return Err(anyhow::anyhow!("Source cannot be empty"));
        }

        if message.version.is_empty() {
            return Err(anyhow::anyhow!("Version cannot be empty"));
        }

        Ok(())
    }

    /// 验证主题名称
    pub fn validate_topic(topic: &str) -> Result<()> {
        if topic.is_empty() {
            return Err(anyhow::anyhow!("Topic name cannot be empty"));
        }

        if topic.len() > 249 {
            return Err(anyhow::anyhow!("Topic name too long (max 249 characters)"));
        }

        // 检查有效字�?
        if !topic.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-') {
            return Err(anyhow::anyhow!("Topic name contains invalid characters"));
        }

        Ok(())
    }
}



