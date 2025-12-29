//! # 基础设施层 (Infrastructure Layer)
//!
//! 本模块是用户管理服务的基础设施层，负责实现领域层定义的端口。
//!
//! ## 所属层
//! Infrastructure Layer - DDD 架构的基础设施层
//!
//! ## 职责
//! - 实现 `domain::port` 中定义的 trait
//! - 数据库访问（PostgreSQL）
//! - 缓存访问（Redis）
//! - 消息队列（Kafka）
//! - 外部服务调用
//!
//! ## 依赖规则
//! - ✅ 实现 domain 层的端口（trait）
//! - ✅ 可以使用外部框架（sqlx、redis 等）
//! - ❌ 不被 application 层直接依赖（通过 trait 解耦）

/// 仓储实现 - 数据持久化
pub mod repository;
