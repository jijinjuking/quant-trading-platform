//! # HTTP接口模块
//!
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: HTTP协议相关的入口点

/// HTTP处理器 - 具体的请求处理函数
pub mod handlers;

/// 路由配置 - URL到Handler的映射
pub mod routes;

/// DTO - 请求/响应数据传输对象
pub mod dto;
