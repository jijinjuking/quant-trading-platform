//! # 端口模块 (Domain Ports)
//! 
//! 定义行情数据服务的抽象接口。
//! 
//! ## Hexagonal 架构说明
//! 端口是 Domain 层定义的抽象接口（trait），
//! Infrastructure 层提供具体实现。
//! 
//! ## 子模块
//! - `exchange_port`: 交易所数据端口
//! - `message_port`: 消息推送端口

/// 交易所数据端口 - 获取行情数据
pub mod exchange_port;

/// 消息推送端口 - 发布行情事件
pub mod message_port;
