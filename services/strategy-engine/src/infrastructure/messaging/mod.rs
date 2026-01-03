//! # 消息队列基础设施 (Messaging Infrastructure)

/// Kafka 生产者 - 发布交易信号
pub mod kafka_producer;

/// Kafka 消费者 - 消费行情事件
pub mod kafka_consumer;

/// Mock 消费者 - 测试用
pub mod mock_consumer;

pub use kafka_producer::KafkaProducer;
pub use kafka_consumer::KafkaConsumer;
pub use mock_consumer::MockConsumer;
