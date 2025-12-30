//! # 行情数据服务 (Market Data Service)
//!
//! 应用层服务，负责编排行情采集流程。
//!
//! ## 职责
//! - 连接交易所
//! - 获取行情事件
//! - 发布到消息队列
//!
//! ## 依赖规则
//! - ✅ 只依赖 `domain::port` 中的 trait
//! - ❌ 不直接依赖 infrastructure
//! - ❌ 不在 service 内 new adapter

use crate::domain::port::{MarketExchangePort, MessagePort};

/// 行情数据服务
pub struct MarketDataService<E, M>
where
    E: MarketExchangePort,
    M: MessagePort,
{
    exchange: E,
    message: M,
}

impl<E, M> MarketDataService<E, M>
where
    E: MarketExchangePort,
    M: MessagePort,
{
    /// 创建服务实例（由 bootstrap 调用）
    pub fn new(exchange: E, message: M) -> Self {
        Self { exchange, message }
    }

    /// 运行行情采集循环
    pub async fn run(&self) -> anyhow::Result<()> {
        // 1. 连接交易所
        self.exchange.connect().await?;

        // 2. 循环获取并发布行情
        loop {
            let event = self.exchange.next_event().await?;
            self.message.publish(event).await?;
        }
    }

    /// 订阅现货行情
    pub async fn subscribe_spot(&self, symbols: Vec<String>) -> anyhow::Result<()> {
        self.exchange.subscribe_spot(symbols).await
    }

    /// 订阅合约行情
    pub async fn subscribe_futures(&self, symbols: Vec<String>) -> anyhow::Result<()> {
        self.exchange.subscribe_futures(symbols).await
    }
}
