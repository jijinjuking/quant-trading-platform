//! # 用户管理服务 - 库入口
//!
//! 本文件导出用户管理服务的所有公共模块。
//!
//! ## 模块结构
//! - `state`: 应用状态管理
//! - `interface`: 接口层（HTTP API 处理）
//! - `application`: 应用层（用例编排服务）
//! - `domain`: 领域层（核心业务模型和端口定义）
//! - `infrastructure`: 基础设施层（端口的具体实现）

/// 应用状态模块
pub mod state;

/// 接口层 - HTTP/gRPC 等外部接口
pub mod interface;

/// 应用层 - 用例编排和业务流程协调
pub mod application;

/// 领域层 - 核心业务逻辑和模型
pub mod domain;

/// 基础设施层 - 外部依赖的适配器实现
pub mod infrastructure;
