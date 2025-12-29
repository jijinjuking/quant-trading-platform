//! # 接口层 (Interface Layer)
//!
//! 本模块是用户管理服务的接口层，负责处理外部请求。
//!
//! ## 所属层
//! Interface Layer - DDD 架构的接口层
//!
//! ## 职责
//! - 接收 HTTP/gRPC 请求
//! - 请求参数验证
//! - DTO 转换（Request DTO → Application DTO）
//! - 响应格式化
//!
//! ## 依赖规则
//! - ✅ 依赖 application 层的服务
//! - ❌ 不包含业务逻辑
//! - ❌ 不直接访问 domain 层的仓储

/// HTTP 接口 - RESTful API
pub mod http;
