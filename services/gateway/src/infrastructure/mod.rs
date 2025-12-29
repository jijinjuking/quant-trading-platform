//! # 基础设施层模块
//!
//! 本模块是 API Gateway 服务的基础设施层入口。
//!
//! ## 架构位置
//! ```text
//! interface → application → domain ← infrastructure
//!                                    ^^^^^^^^^^^^^^
//!                                    当前层
//! ```
//!
//! ## 职责
//! - 实现领域层定义的端口（Port）
//! - 提供外部依赖的适配器（Adapter）
//! - 处理技术细节（数据库、缓存、消息队列等）
//!
//! ## 规则
//! - 必须实现 `domain::port` 中的 trait
//! - 负责 DTO ↔ Domain 对象转换
//! - 封装所有外部依赖
//!
//! ## 子模块
//! - `cache`: 缓存适配器（Redis）

/// 缓存适配器模块
pub mod cache;
