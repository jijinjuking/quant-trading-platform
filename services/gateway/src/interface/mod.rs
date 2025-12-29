//! # 接口层模块
//!
//! 本模块是 API Gateway 服务的接口层入口。
//!
//! ## 架构位置
//! ```text
//! interface → application → domain ← infrastructure
//! ^^^^^^^^^
//! 当前层
//! ```
//!
//! ## 职责
//! - 接收外部请求（HTTP/gRPC/WebSocket）
//! - 请求参数验证和转换
//! - 调用应用层服务
//! - 响应格式化和返回
//!
//! ## 子模块
//! - `http`: HTTP 协议接口

/// HTTP 接口模块
pub mod http;
