//! # 策略调度器 (Strategy Scheduler)
//!
//! 负责：
//! 1. 从Kafka消费行情数据
//! 2. 路由到对应的策略实例
//! 3. 执行策略计算
//! 4. 聚合信号
//! 5. 发布信号到Kafka

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{ClientConfig, Message};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::domain::model::strategy_runtime::{ExecutionRequest, ExecutionResult};
use crate::domain::service::strategy_registry::StrategyRegistry;

/// 调度器配置
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Kafka broker地址
    pub kafka_brokers: String,
    /// 行情主题
    pub market_topic: String,
    /// 信号主题
    pub signal_topic: String,
    /// 消费者组ID
    pub consumer_group: String,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            kafka_brokers: "localhost:9092".to_string(),
            market_topic: "market-events".to_string(),
            signal_topic: "strategy-signals".to_string(),
            consumer_group: "strategy-scheduler".to_string(),
        }
    }
}

/// 策略调度器
pub struct StrategyScheduler {
    /// 策略注册表
    registry: Arc<StrategyRegistry>,
    /// Kafka消费者
    consumer: StreamConsumer,
    /// Kafka生产者
    producer: FutureProducer,
    /// 配置
    config: SchedulerConfig,
}

impl StrategyScheduler {
    /// 创建调度器
    pub fn new(registry: Arc<StrategyRegistry>, config: SchedulerConfig) -> Result<Self> {
        // 创建Kafka消费者
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &config.consumer_group)
            .set("bootstrap.servers", &config.kafka_brokers)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .create()
            .context("Failed to create Kafka consumer")?;

        // 订阅行情主题
        consumer
            .subscribe(&[&config.market_topic])
            .context("Failed to subscribe to market topic")?;

        // 创建Kafka生产者
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.kafka_brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .context("Failed to create Kafka producer")?;

        info!(
            "StrategyScheduler created: market_topic={}, signal_topic={}",
            config.market_topic, config.signal_topic
        );

        Ok(Self {
            registry,
            consumer,
            producer,
            config,
        })
    }

    /// 运行调度器
    pub async fn run(&self) -> Result<()> {
        info!("StrategyScheduler starting...");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    if let Err(e) = self.handle_message(&message).await {
                        error!(error = %e, "Failed to handle message");
                    }
                }
                Err(e) => {
                    error!(error = %e, "Kafka consumer error");
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// 处理单条消息
    async fn handle_message(&self, message: &rdkafka::message::BorrowedMessage<'_>) -> Result<()> {
        // 解析消息
        let payload = message
            .payload()
            .context("Message has no payload")?;

        let market_event: shared::event::market_event::MarketEvent =
            serde_json::from_slice(payload)
                .context("Failed to parse market event")?;

        debug!(
            symbol = %market_event.symbol,
            event_type = ?market_event.event_type,
            "Received market event"
        );

        // 转换为执行请求
        let request = self.market_event_to_request(&market_event)?;

        // 获取所有运行中的策略
        let running_strategies = self.registry.running_instances();

        if running_strategies.is_empty() {
            debug!("No running strategies, skipping");
            return Ok(());
        }

        // 执行所有策略
        let mut results = Vec::new();
        for handle in running_strategies {
            let instance_id = handle.instance_id();

            // 检查策略是否订阅了这个交易对
            if handle.metadata().symbol != market_event.symbol {
                continue;
            }

            match self.registry.execute(instance_id, &request) {
                Ok(result) => {
                    if result.has_intent {
                        debug!(
                            instance_id = %instance_id,
                            has_intent = result.has_intent,
                            "Strategy executed"
                        );
                        results.push(result);
                    }
                }
                Err(e) => {
                    warn!(
                        instance_id = %instance_id,
                        error = %e,
                        "Strategy execution failed"
                    );
                }
            }
        }

        // 发布信号
        for result in results {
            if let Err(e) = self.publish_signal(&result).await {
                error!(error = %e, "Failed to publish signal");
            }
        }

        Ok(())
    }

    /// 将行情事件转换为执行请求
    fn market_event_to_request(
        &self,
        event: &shared::event::market_event::MarketEvent,
    ) -> Result<ExecutionRequest> {
        use shared::event::market_event::MarketEventData;

        let (price, quantity, is_buyer_maker) = match &event.data {
            MarketEventData::Trade(trade) => {
                (trade.price, trade.quantity, trade.is_buyer_maker)
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported market event type"));
            }
        };

        Ok(ExecutionRequest {
            request_id: Uuid::new_v4(),
            symbol: event.symbol.clone(),
            price,
            quantity,
            timestamp: event.timestamp,
            is_buyer_maker,
        })
    }

    /// 发布信号到Kafka
    async fn publish_signal(&self, result: &ExecutionResult) -> Result<()> {
        if let Some(ref intent) = result.intent {
            let signal_json = serde_json::to_string(intent)
                .context("Failed to serialize signal")?;

            let record = FutureRecord::to(&self.config.signal_topic)
                .payload(&signal_json)
                .key(&intent.symbol);

            self.producer
                .send(record, Duration::from_secs(0))
                .await
                .map_err(|(e, _)| anyhow::anyhow!("Failed to send signal: {}", e))?;

            info!(
                signal_id = %intent.id,
                strategy_id = %intent.strategy_id,
                symbol = %intent.symbol,
                side = ?intent.side,
                "Signal published"
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_config_default() {
        let config = SchedulerConfig::default();
        assert_eq!(config.kafka_brokers, "localhost:9092");
        assert_eq!(config.market_topic, "market-events");
        assert_eq!(config.signal_topic, "strategy-signals");
    }
}
