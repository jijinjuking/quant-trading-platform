//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use std::sync::Arc;
use crate::infrastructure::repository::strategy_repository::StrategyRepository;
use crate::infrastructure::messaging::kafka_producer::KafkaProducer;
use crate::infrastructure::messaging::kafka_consumer::KafkaConsumer;
use crate::infrastructure::messaging::mock_consumer::MockConsumer;
use crate::infrastructure::strategy::noop_strategy::NoopStrategy;
use crate::application::service::strategy_service::StrategyService;
use crate::application::service::market_event_consumer_service::MarketEventConsumerService;

/// 创建策略服务实例
///
/// # 参数
/// - `kafka_brokers`: Kafka broker 地址
#[allow(dead_code)]
pub fn create_strategy_service(
    kafka_brokers: String,
) -> StrategyService<StrategyRepository, KafkaProducer> {
    let repository = StrategyRepository::new();
    let messenger = KafkaProducer::new(kafka_brokers);
    StrategyService::new(repository, messenger)
}

/// 创建行情事件消费服务（Kafka + NoopStrategy）
///
/// # 参数
/// - `brokers`: Kafka broker 地址
/// - `topic`: 消费的 topic
/// - `group_id`: 消费者组 ID
#[allow(dead_code)]
pub fn create_market_event_consumer(
    brokers: String,
    topic: String,
    group_id: String,
) -> MarketEventConsumerService<Arc<KafkaConsumer>, Arc<NoopStrategy>> {
    let consumer = Arc::new(KafkaConsumer::new(brokers, topic, group_id));
    let strategy = Arc::new(NoopStrategy::new());
    MarketEventConsumerService::new(consumer, strategy)
}

/// 创建行情事件消费服务（Mock + NoopStrategy，用于测试）
#[allow(dead_code)]
pub fn create_mock_market_event_consumer() -> MarketEventConsumerService<Arc<MockConsumer>, Arc<NoopStrategy>> {
    let consumer = Arc::new(MockConsumer::new());
    let strategy = Arc::new(NoopStrategy::new());
    MarketEventConsumerService::new(consumer, strategy)
}
