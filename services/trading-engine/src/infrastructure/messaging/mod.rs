//! # 消息基础设施模块
//!
//! 提供 Kafka 消费者实现。

/// 行情事件 Kafka 消费者
pub mod market_event_consumer;

pub use market_event_consumer::MarketEventKafkaConsumer;
