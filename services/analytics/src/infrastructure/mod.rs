//! # Infrastructure Layer - 基础设施层
//!
//! ## 层级职责
//! 基础设施层负责实现领域层定义的端口（trait），包括：
//! - 数据持久化（数据库访问）
//! - 外部服务集成（API调用）
//! - 消息队列（Kafka生产/消费）
//! - 缓存（Redis）
//!
//! ## 架构约束
//! - ✅ 必须实现 `domain::port` 中的 trait
//! - ✅ 负责 DTO ↔ Domain 对象的转换
//! - ❌ 不允许被 application/interface 层直接调用
//! - ❌ 不允许包含业务逻辑
//!
//! ## 子模块
//! - `clickhouse`: ClickHouse 时序数据库适配器
//!
//! ## 依赖方向
//! ```text
//! Infrastructure → Domain::Port (实现trait)
//!       ↓
//! 外部系统 (ClickHouse, Kafka, Redis...)
//! ```

/// ClickHouse 时序数据库适配器
pub mod clickhouse;
