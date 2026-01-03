//! # Kafka 生产者适配器 (Kafka Producer Adapter)
//!
//! 实现 MessagePort trait。

use std::time::Duration;

use anyhow::Context;
use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use shared::event::market_event::MarketEvent;

use crate::domain::port::MessagePort;

/// Kafka 生产者适配器
pub struct KafkaProducer {
    #[allow(dead_code)]
    brokers: String,
    #[allow(dead_code)]
    topic: String,
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: String, topic: String) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .context("failed to create kafka producer")?;

        Ok(Self {
            brokers,
            topic,
            producer,
        })
    }
}

#[async_trait]
impl MessagePort for KafkaProducer {
    async fn publish(&self, event: MarketEvent) -> anyhow::Result<()> {
        let payload = serde_json::to_string(&event)
            .context("serialize market event")?;
        let key = event.symbol.clone();

        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&key);

        let delivery = self.producer.send(record, Duration::from_secs(5)).await;
        match delivery {
            Ok(_) => Ok(()),
            Err((err, _)) => Err(err.into()),
        }
    }
}
