//! # HTTP处理器模块
//!
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 包含所有HTTP请求处理函数

/// 健康检查处理器
pub mod health;

/// 订单处理器 - 处理订单相关请求
pub mod orders;

/// 持仓处理器 - 处理持仓相关请求
pub mod positions;

/// 账户处理器 - 处理账户余额查询
pub mod account;
