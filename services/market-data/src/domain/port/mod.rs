//! # 端口模块 (Port Module)
//!
//! Domain 层定义的抽象接口（端口）。
//!
//! ## 包含端口
//! - `MarketExchangePort`: 行情交易所端口
//! - `MessagePort`: 消息推送端口

pub mod market_exchange_port;
pub mod message_port;

pub use market_exchange_port::MarketExchangePort;
pub use message_port::MessagePort;
