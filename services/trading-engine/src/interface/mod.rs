//! # 接口层模块
//! 
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 
//! - 接收外部请求（HTTP/gRPC/WebSocket）
//! - 请求参数校验
//! - DTO转换
//! - 调用Application层
//! 
//! ## 依赖规则:
//! - 可以依赖: application层
//! - 不可依赖: domain层直接、infrastructure层

/// HTTP接口模块
pub mod http;
