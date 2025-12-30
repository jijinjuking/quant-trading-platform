//! # 行情交易所端口 (Market Exchange Port)
//!
//! 定义从交易所获取行情数据的抽象接口。
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 入参/出参只能是领域对象或基础类型
//! - 不暴露任何 SDK / WebSocket / Stream 类型

use std::sync::Arc;
use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

/// 行情交易所端口
#[async_trait]
pub trait MarketExchangePort: Send + Sync {
    /// 连接交易所
    async fn connect(&self) -> anyhow::Result<()>;

    /// 订阅现货行情
    async fn subscribe_spot(&self, symbols: Vec<String>) -> anyhow::Result<()>;

    /// 订阅合约行情
    async fn subscribe_futures(&self, symbols: Vec<String>) -> anyhow::Result<()>;

    /// 获取下一个行情事件
    async fn next_event(&self) -> anyhow::Result<MarketEvent>;
}

// Arc<T> 自动实现 MarketExchangePort
#[async_trait]
impl<T: MarketExchangePort> MarketExchangePort for Arc<T> {
    async fn connect(&self) -> anyhow::Result<()> {
        (**self).connect().await
    }

    async fn subscribe_spot(&self, symbols: Vec<String>) -> anyhow::Result<()> {
        (**self).subscribe_spot(symbols).await
    }

    async fn subscribe_futures(&self, symbols: Vec<String>) -> anyhow::Result<()> {
        (**self).subscribe_futures(symbols).await
    }

    async fn next_event(&self) -> anyhow::Result<MarketEvent> {
        (**self).next_event().await
    }
}
