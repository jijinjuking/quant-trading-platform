//! # AI 服务库
//!
//! 本模块导出 AI 服务的所有公共组件，供外部集成或测试使用。
//!
//! ## 模块结构
//! - `state`: 应用状态管理
//! - `interface`: 接口层（HTTP 处理器和路由）
//! - `application`: 应用层（用例编排服务）
//! - `domain`: 领域层（核心模型和端口定义）
//! - `infrastructure`: 基础设施层（DeepSeek 客户端实现）

/// 应用状态模块
pub mod state;
/// 接口层模块
pub mod interface;
/// 应用层模块
pub mod application;
/// 领域层模块
pub mod domain;
/// 基础设施层模块
pub mod infrastructure;
