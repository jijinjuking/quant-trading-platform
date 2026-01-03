//! Kafka consumer adapter for MarketEvent.

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::Message;
use shared::event::market_event::MarketEvent;

use crate::domain::port::market_event_port::MarketEventPort;

pub struct KafkaConsumer {
    #[allow(dead_code)]
    brokers: String,
    #[allow(dead_code)]
    topic: String,
    #[allow(dead_code)]
    group_id: String,
    consumer: StreamConsumer,
}

impl KafkaConsumer {
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

        Ok(Self {
            brokers,
            topic,
            group_id,
            consumer,
        })
    }
}

#[async_trait]
impl MarketEventPort for KafkaConsumer {
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
