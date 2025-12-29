//! # 风险管理服务库 (Risk Management Library)
//!
//! 本模块导出风险管理服务的所有公共模块，供外部调用或测试使用。
//!
//! ## 模块结构
//! - [`state`]: 应用状态管理
//! - [`interface`]: 接口层（HTTP API）
//! - [`application`]: 应用层（用例编排）
//! - [`domain`]: 领域层（核心业务逻辑）
//! - [`infrastructure`]: 基础设施层（适配器实现）

/// 应用状态模块
pub mod state;

/// 接口层 - HTTP/gRPC 等外部接口
pub mod interface;

/// 应用层 - 用例编排和业务流程协调
pub mod application;

/// 领域层 - 核心业务逻辑和领域模型
pub mod domain;

/// 基础设施层 - 数据库、缓存、消息队列等适配器
pub mod infrastructure;
