//! # 策略引擎依赖注入 (Strategy Engine Bootstrap)
//!
//! 负责创建并组装适配器与服务。

use std::sync::Arc;

use anyhow::Result;

use crate::application::service::market_event_consumer_service::MarketEventConsumerService;
use crate::application::service::risk_service::RiskService;
use crate::application::service::strategy_service::StrategyService;
use crate::domain::logic::grid::GridConfig;
use crate::domain::logic::mean::MeanReversionConfig;
use crate::domain::model::strategy_config::StrategyType;
use crate::infrastructure::messaging::{KafkaConsumer, KafkaProducer, MockConsumer};
use crate::infrastructure::repository::strategy_repository::StrategyRepository;
use crate::infrastructure::risk::noop_risk::NoopRisk;
use crate::infrastructure::strategy::noop_strategy::NoopStrategy;
use crate::infrastructure::strategy::signal_strategy::SignalStrategy;

pub fn create_strategy_service(
    kafka_brokers: String,
    kafka_topic: String,
) -> Result<StrategyService<StrategyRepository, KafkaProducer>> {
    let repository = StrategyRepository::new();
    let messenger = KafkaProducer::new(kafka_brokers, kafka_topic)?;
    Ok(StrategyService::new(repository, messenger))
}

pub fn create_market_event_consumer(
    brokers: String,
    market_topic: String,
    signal_topic: String,
    group_id: String,
    strategy_type: StrategyType,
    grid_config: GridConfig,
    mean_reversion_config: MeanReversionConfig,
) -> Result<MarketEventConsumerService<Arc<KafkaConsumer>, Arc<SignalStrategy<KafkaProducer>>>> {
    let consumer = Arc::new(KafkaConsumer::new(brokers.clone(), market_topic, group_id)?);
    let publisher = KafkaProducer::new(brokers, signal_topic)?;
    let strategy = Arc::new(SignalStrategy::new(
        publisher,
        strategy_type,
        grid_config,
        mean_reversion_config,
    ));
    Ok(MarketEventConsumerService::new(consumer, strategy))
}

#[allow(dead_code)]
pub fn create_mock_market_event_consumer(
) -> MarketEventConsumerService<Arc<MockConsumer>, Arc<NoopStrategy>> {
    let consumer = Arc::new(MockConsumer::new());
    let strategy = Arc::new(NoopStrategy::new());
    MarketEventConsumerService::new(consumer, strategy)
}

pub fn create_risk_service() -> RiskService<Arc<NoopRisk>> {
    let risk = Arc::new(NoopRisk::new());
    RiskService::new(risk)
}
