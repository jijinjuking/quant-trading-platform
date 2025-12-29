//! # 行情数据服务库 (Market Data Library)
//! 
//! 对外暴露行情数据服务的所有模块。
//! 
//! ## 模块说明
//! - `state`: 应用状态管理
//! - `interface`: 接口层（HTTP API）
//! - `application`: 应用层（用例编排）
//! - `domain`: 领域层（核心模型和端口）
//! - `infrastructure`: 基础设施层（适配器实现）

/// 应用状态 - 配置和共享资源管理
pub mod state;

/// 接口层 - HTTP/WebSocket API
pub mod interface;

/// 应用层 - 业务用例编排
pub mod application;

/// 领域层 - 核心业务模型
pub mod domain;

/// 基础设施层 - 外部系统适配器
pub mod infrastructure;
