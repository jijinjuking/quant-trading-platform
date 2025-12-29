//! # 消息队列基础设施 (Messaging Infrastructure)
//! 
//! 提供与消息队列（Kafka）交互的实现。
//! 
//! ## 子模块
//! - `kafka_consumer`: Kafka 消息消费者
//! 
//! ## 职责
//! - 消费交易信号事件
//! - 发布订单状态变更事件

/// Kafka 消费者 - 消费交易信号
pub mod kafka_consumer;
