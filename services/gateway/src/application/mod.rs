//! # 应用层模块
//!
//! 本模块是 API Gateway 服务的应用层入口。
//!
//! ## 架构位置
//! ```text
//! interface → application → domain ← infrastructure
//!             ^^^^^^^^^^^
//!             当前层
//! ```
//!
//! ## 职责
//! - 用例编排（Orchestration）
//! - 调用领域服务完成业务流程
//! - 事务管理
//! - 跨聚合协调
//!
//! ## 规则
//! - 只依赖 `domain::port` 中的 trait
//! - 不包含业务逻辑
//! - 不直接访问基础设施
//!
//! ## 子模块
//! - `service`: 应用服务

/// 应用服务模块
pub mod service;
