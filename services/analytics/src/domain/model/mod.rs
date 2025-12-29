//! # Domain Models - 领域模型
//!
//! ## 模块职责
//! 定义数据分析领域的核心模型：
//! - 实体（Entity）：有唯一标识的对象
//! - 值对象（Value Object）：无标识，通过属性定义
//! - 聚合（Aggregate）：一组相关对象的集合
//!
//! ## 模型列表
//! - `performance`: 绩效指标模型

/// 绩效指标模型
pub mod performance;
