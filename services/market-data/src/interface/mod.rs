//! # 接口层 (Interface Layer)
//! 
//! 处理外部请求的入口层。
//! 
//! ## 子模块
//! - `http`: HTTP REST API 接口
//! 
//! ## 职责
//! - 接收外部请求
//! - 参数验证和转换
//! - 调用 Application 层处理业务
//! - 返回响应

/// HTTP 接口模块 - REST API
pub mod http;
