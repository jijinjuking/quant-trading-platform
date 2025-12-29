//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::repository::strategy_repository::StrategyRepository;
use crate::infrastructure::messaging::kafka_producer::KafkaProducer;
use crate::application::service::strategy_service::StrategyService;

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
