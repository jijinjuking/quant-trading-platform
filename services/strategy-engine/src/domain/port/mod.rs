//! # 端口模块 (Domain Ports)
//! 
//! 定义策略引擎的抽象接口。

/// 策略仓储端口
pub mod strategy_repository_port;

/// 消息推送端口
pub mod message_port;

/// 行情事件消费端口
pub mod market_event_port;

/// 策略端口
pub mod strategy_port;

/// 风控端口
pub mod risk_port;

/// 策略状态端口（Redis 缓存）
pub mod strategy_state_port;

pub use strategy_state_port::{GridStateData, MeanReversionStateData, StrategyStatePort};
