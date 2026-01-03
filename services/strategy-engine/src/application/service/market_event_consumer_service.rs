//! # 行情事件消费服务 (Market Event Consumer Service)
//!
//! 应用层服务，负责接收 MarketEvent 并转交给策略。
//!
//! ## 职责
//! - 从消息队列消费行情事件
//! - 转发给 StrategyPort 处理
//!
//! ## 依赖规则
//! - ✅ 只依赖 `domain::port` 中的 trait
//! - ❌ 不直接依赖 infrastructure
//! - ❌ 不在 service 内 new adapter
//! - ❌ 不实现任何策略计算

use std::time::Duration;

use crate::domain::port::market_event_port::MarketEventPort;
use crate::domain::port::strategy_port::StrategyPort;
use tracing::{error, info, warn};

/// 行情事件消费服务
pub struct MarketEventConsumerService<E, S>
where
    E: MarketEventPort,
    S: StrategyPort,
{
    event_source: E,
    strategy: S,
}

impl<E, S> MarketEventConsumerService<E, S>
where
    E: MarketEventPort,
    S: StrategyPort,
{
    /// 创建服务实例（由 bootstrap 调用）
    pub fn new(event_source: E, strategy: S) -> Self {
        Self { event_source, strategy }
    }

    /// 运行事件消费循环
    ///
    /// # 流程
    /// 1. 循环获取行情事件
    /// 2. 记录日志
    /// 3. 转交给 StrategyPort
    ///
    /// # 返回
    /// - `Ok(())`: 正常退出（不会发生）
    /// - `Err`: 消费失败
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("MarketEventConsumerService started");

        loop {
            match self.event_source.next_event().await {
                Ok(event) => {
                    // 记录日志
                    info!(
                        symbol = %event.symbol,
                        exchange = %event.exchange,
                        "Received MarketEvent"
                    );

                    // 转交给 StrategyPort
                    if let Err(err) = self.strategy.on_market_event(&event).await {
                        warn!(error = %err, "strategy processing failed");
                    }
                }
                Err(err) => {
                    error!(error = %err, "market event source error");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
