//! # 应用服务 (Application Services)

/// 策略服务 - 策略管理用例
pub mod strategy_service;

/// 回测服务 - 策略回测用例
pub mod backtest_service;

/// 行情事件消费服务 - 接收 MarketEvent
pub mod market_event_consumer_service;

pub use market_event_consumer_service::MarketEventConsumerService;
