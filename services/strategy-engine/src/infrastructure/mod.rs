//! # 基础设施层 (Infrastructure Layer)

/// 消息队列 - Kafka
pub mod messaging;

/// 数据仓储 - 数据库
pub mod repository;

/// 策略实现
pub mod strategy;

/// 风控实现
pub mod risk;

/// 缓存 - Redis
pub mod cache;
