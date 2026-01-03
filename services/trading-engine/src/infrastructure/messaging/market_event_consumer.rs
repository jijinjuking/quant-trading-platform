//! # 行情事件 Kafka 消费者 (Market Event Kafka Consumer)
//!
//! 从 Kafka 消费 MarketEvent。

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::Message;
use shared::event::market_event::MarketEvent;

use crate::domain::port::market_event_port::MarketEventPort;

/// 行情事件 Kafka 消费者
pub struct MarketEventKafkaConsumer {
    consumer: StreamConsumer,
}

impl MarketEventKafkaConsumer {
    /// 创建行情事件 Kafka 消费者
    pub fn new(brokers: String, topic: String, group_id: String) -> anyhow::Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("group.id", &group_id)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .create()
            .context("failed to create kafka consumer")?;

        consumer
            .subscribe(&[&topic])
            .context("failed to subscribe kafka topic")?;

        Ok(Self { consumer })
    }
}

#[async_trait]
impl MarketEventPort for MarketEventKafkaConsumer {
    async fn next_event(&self) -> anyhow::Result<MarketEvent> {
        let message = self
            .consumer
            .recv()
            .await
            .context("failed to receive kafka message")?;

        let payload = message
            .payload()
            .ok_or_else(|| anyhow!("kafka message payload is empty"))?;

        let event: MarketEvent =
            serde_json::from_slice(payload).context("deserialize market event")?;

        self.consumer
            .commit_message(&message, CommitMode::Async)
            .context("commit kafka message")?;

        Ok(event)
    }
}
