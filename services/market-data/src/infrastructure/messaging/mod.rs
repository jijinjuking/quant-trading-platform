//! # 消息适配器模块
//!
//! 提供消息队列的发布实现。

pub mod kafka_producer;

pub use kafka_producer::KafkaProducer;
