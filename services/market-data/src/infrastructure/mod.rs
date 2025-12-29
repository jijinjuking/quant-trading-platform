//! # 基础设施层 (Infrastructure Layer)
//! 
//! 提供与外部系统交互的具体实现。
//! 
//! ## 子模块
//! - `exchange`: 交易所连接器（实现 MarketExchangePort）
//! - `messaging`: 消息队列（实现 MessagePort）
//! 
//! ## Hexagonal 架构角色
//! Infrastructure 层是「适配器」(Adapter)，负责：
//! - 实现 Domain 层定义的 Port trait
//! - 处理 SDK/DTO ↔ Domain 的转换

/// 交易所连接器 - WebSocket 行情获取
pub mod exchange;

/// 消息队列 - Kafka 事件发布
pub mod messaging;
