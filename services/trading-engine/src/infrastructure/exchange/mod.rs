//! # 交易所连接器 (Exchange Connectors)
//!
//! 提供与各交易所 API 交互的具体实现。
//!
//! ## 模块说明
//! - `binance_query`: 币安查询适配器（账户余额、持仓、订单查询）
//! - `binance_fill_stream`: 币安成交事件流（User Data Stream）
//!
//! ## 架构位置
//! - 所属层级: Infrastructure Layer
//! - 实现端口: domain/port/exchange_query_port.rs

/// 币安查询适配器
pub mod binance_query;

/// 币安成交事件流
pub mod binance_fill_stream;

pub use binance_query::BinanceQueryAdapter;
pub use binance_fill_stream::{BinanceFillStream, FillStreamConfig, create_fill_stream};
