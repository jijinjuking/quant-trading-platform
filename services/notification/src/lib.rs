//! # 通知服务库
//!
//! 本文件是通知服务的库入口，导出所有公共模块。
//!
//! ## 模块结构（DDD四层架构）
//! - `state`: 应用状态管理
//! - `interface`: 接口层（HTTP API）
//! - `application`: 应用层（用例编排）
//! - `domain`: 领域层（核心模型和端口）
//! - `infrastructure`: 基础设施层（适配器实现）

/// 应用状态模块
pub mod state;

/// 接口层 - HTTP/gRPC 接口定义
pub mod interface;

/// 应用层 - 用例编排和业务流程
pub mod application;

/// 领域层 - 核心业务模型和端口定义
pub mod domain;

/// 基础设施层 - 外部服务适配器实现
pub mod infrastructure;
