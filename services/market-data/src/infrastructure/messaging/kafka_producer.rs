//! # Kafka 生产者适配器 (Kafka Producer Adapter)
//!
//! 实现 MessagePort trait。

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;
use crate::domain::port::MessagePort;

/// Kafka 生产者适配器
pub struct KafkaProducer {
    #[allow(dead_code)]
    brokers: String,
    #[allow(dead_code)]
    topic: String,
    // TODO: Kafka producer client
}

impl KafkaProducer {
    pub fn new(brokers: String, topic: String) -> Self {
        Self { brokers, topic }
    }
}

#[async_trait]
impl MessagePort for KafkaProducer {
    async fn publish(&self, _event: MarketEvent) -> anyhow::Result<()> {
        // TODO: 序列化并发送到 Kafka
        Ok(())
    }
}
