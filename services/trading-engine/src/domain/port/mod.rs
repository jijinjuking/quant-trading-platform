//! # 端口模块 (Domain Ports)
//! 
//! 这是 Hexagonal（六边形）架构的核心部分。
//! 
//! ## 什么是端口？
//! 端口是 Domain 层定义的抽象接口（trait），
//! 定义了 Domain 需要的外部能力，但不关心具体实现。
//! 
//! ## 端口规则（强制）
//! - 只允许定义 trait
//! - 入参/出参只能是 Domain 对象或基础类型
//! - ❌ 禁止出现 HTTP/DB/SDK/Redis/Kafka 类型
//! 
//! ## 子模块
//! - `exchange_port`: 交易所接口端口
//! - `order_repository_port`: 订单仓储端口

/// 交易所端口 - 定义与交易所交互的抽象接口
pub mod exchange_port;

/// 订单仓储端口 - 定义订单持久化的抽象接口
pub mod order_repository_port;
