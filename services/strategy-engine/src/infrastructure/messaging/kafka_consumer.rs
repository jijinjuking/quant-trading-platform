//! # Kafka 消费者适配器 (Kafka Consumer Adapter)
//!
//! 实现 MarketEventPort trait。

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;
use crate::domain::port::market_event_port::MarketEventPort;

/// Kafka 消费者适配器
pub struct KafkaConsumer {
    #[allow(dead_code)]
    brokers: String,
    #[allow(dead_code)]
    topic: String,
    #[allow(dead_code)]
    group_id: String,
    // TODO: Kafka consumer client
}

impl KafkaConsumer {
    pub fn new(brokers: String, topic: String, group_id: String) -> Self {
        Self {
            brokers,
            topic,
            group_id,
        }
    }
}

#[async_trait]
impl MarketEventPort for KafkaConsumer {
    async fn next_event(&self) -> anyhow::Result<MarketEvent> {
        // TODO: 从 Kafka 消费消息并反序列化
        todo!("实现 Kafka 消息消费")
    }
}
