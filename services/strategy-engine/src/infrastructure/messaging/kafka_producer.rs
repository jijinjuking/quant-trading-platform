//! Signal 的 Kafka 生产者适配器。

use std::time::Duration;

use anyhow::Context;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use tracing::error;

use crate::domain::model::signal::{Signal, SignalType};
use crate::domain::port::message_port::SignalMessagePort;
use shared::event::signal_event::{SignalEvent, SignalType as EventSignalType};

pub struct KafkaProducer {
    #[allow(dead_code)]
    brokers: String,
    #[allow(dead_code)]
    topic: String,
    producer: BaseProducer,
}

impl KafkaProducer {
    pub fn new(brokers: String, topic: String) -> anyhow::Result<Self> {
        let producer: BaseProducer = ClientConfig::new()
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

impl SignalMessagePort for KafkaProducer {
    fn publish_signal(&self, signal: &Signal) -> bool {
        let event = SignalEvent {
            id: signal.id,
            strategy_id: signal.strategy_id,
            symbol: signal.symbol.clone(),
            signal_type: match &signal.signal_type {
                SignalType::Buy => EventSignalType::Buy,
                SignalType::Sell => EventSignalType::Sell,
                SignalType::Hold => EventSignalType::Hold,
            },
            price: signal.price,
            quantity: signal.quantity,
            confidence: signal.confidence,
            created_at: signal.created_at,
        };

        let payload = match serde_json::to_string(&event) {
            Ok(value) => value,
            Err(err) => {
                error!(error = %err, "serialize signal failed");
                return false;
            }
        };

        let record = BaseRecord::to(&self.topic)
            .payload(&payload)
            .key(&signal.symbol);

        match self.producer.send(record) {
            Ok(_) => {
                let _ = self.producer.flush(Duration::from_secs(5));
                true
            }
            Err((err, _)) => {
                error!(error = %err, "kafka send failed");
                false
            }
        }
    }
}
