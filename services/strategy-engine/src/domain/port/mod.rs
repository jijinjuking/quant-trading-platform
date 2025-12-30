//! # 端口模块 (Domain Ports)
//! 
//! 定义策略引擎的抽象接口。

/// 策略仓储端口
pub mod strategy_repository_port;

/// 消息推送端口
pub mod message_port;

/// 行情事件消费端口
pub mod market_event_port;

pub use market_event_port::MarketEventPort;
