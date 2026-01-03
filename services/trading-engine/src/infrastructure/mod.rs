//! # 基础设施层 (Infrastructure Layer)
//! 
//! 这是 DDD 架构的最外层，负责与外部系统交互。
//! 
//! ## 子模块说明
//! - `exchange`: 交易所连接器（实现 ExchangePort）
//! - `repository`: 数据仓储（实现 OrderRepositoryPort）
//! - `messaging`: 消息队列（Kafka 消费者/生产者）
//! 
//! ## Hexagonal 架构角色
//! Infrastructure 层是「适配器」(Adapter)，负责：
//! - 实现 Domain 层定义的 Port trait
//! - 处理 SDK/DTO/外部结构 ↔ Domain 的转换
//! 
//! ## 依赖规则
//! - Infrastructure 只能实现 domain::port 中的 trait
//! - Application/Interface 禁止直接调用 Infrastructure

/// 交易所连接器模块 - 实现 ExchangePort
pub mod exchange;

/// 数据仓储模块 - 实现 OrderRepositoryPort
pub mod repository;

/// 消息队列模块 - Kafka 消费者/生产者
pub mod messaging;

/// 执行模块 - 实现 ExecutionPort (v1 占位骨架)
pub mod execution;

/// 风控模块 - 下单前风控与限额校验
pub mod risk;

/// 策略模块 - 实现 StrategyPort
pub mod strategy;

/// 审计模块 - 实现 TradeAuditPort
pub mod audit;
