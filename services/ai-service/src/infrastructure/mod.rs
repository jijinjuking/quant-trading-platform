//! # 基础设施层模块
//!
//! 基础设施层负责实现领域层定义的端口（trait），
//! 提供与外部系统的集成能力。
//!
//! ## 职责
//! - 实现 `domain::port` 中定义的 trait
//! - 处理外部 SDK/API 调用
//! - 执行 DTO ↔ Domain 对象转换
//!
//! ## 子模块
//! - `deepseek`: DeepSeek API 客户端适配器

/// DeepSeek 集成模块
pub mod deepseek;
