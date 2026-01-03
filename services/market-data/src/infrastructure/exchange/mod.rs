//! # 交易所适配器模块
//!
//! 提供各交易所的行情接入实现。

mod binance_ws;

pub use binance_ws::BinanceWebSocket;
