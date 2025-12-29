//! # Domain Ports - 端口定义
//!
//! ## 模块职责
//! 定义领域层的端口（Port），即抽象接口（trait）
//!
//! ## 六边形架构说明
//! 端口是六边形架构的核心概念：
//! - **入站端口**: 定义领域层对外提供的能力（由Application层调用）
//! - **出站端口**: 定义领域层需要的外部能力（由Infrastructure层实现）
//!
//! ## 设计原则
//! - ✅ 只定义 trait，不包含实现
//! - ✅ 入参/出参只能是领域对象或基础类型
//! - ❌ 禁止出现 HTTP/DB/Redis/Kafka 等外部类型
//! - ❌ 禁止依赖任何外部框架
//!
//! ## 端口列表
//! - `analytics_repository_port`: 分析数据仓储端口

/// 分析数据仓储端口
pub mod analytics_repository_port;
