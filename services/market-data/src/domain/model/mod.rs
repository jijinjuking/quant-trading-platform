//! # 领域模型 (Domain Models)
//! 
//! 定义行情数据的核心领域模型。
//! 
//! ## 子模块
//! - `tick`: 逐笔成交数据
//! - `kline`: K 线数据

/// Tick 模型 - 逐笔成交数据
pub mod tick;

/// Kline 模型 - K 线（蜡烛图）数据
pub mod kline;
