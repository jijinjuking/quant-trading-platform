//! # HTTP 处理器 (HTTP Handlers)
//! 
//! 处理具体的 HTTP 请求。
//! 
//! ## 子模块
//! - `health`: 健康检查处理器
//! - `market`: 行情数据处理器

/// 健康检查处理器
pub mod health;

/// 行情数据处理器 - 交易对、Ticker、K线
pub mod market;
