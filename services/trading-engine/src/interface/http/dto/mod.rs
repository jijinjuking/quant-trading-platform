//! # HTTP DTO 模块
//!
//! 定义 HTTP 请求/响应的数据传输对象。
//!
//! ## 架构位置
//! - 所属层级: Interface Layer
//! - 职责: HTTP 请求解析、响应序列化

/// 订单相关 DTO
pub mod order;

/// 持仓相关 DTO
pub mod position;

/// 账户相关 DTO
pub mod account;

/// 通用响应 DTO
pub mod common;

pub use order::*;
pub use position::*;
pub use account::*;
pub use common::*;
