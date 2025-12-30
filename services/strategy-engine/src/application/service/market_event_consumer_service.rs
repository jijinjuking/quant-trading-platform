//! # 行情事件消费服务 (Market Event Consumer Service)
//!
//! 应用层服务，负责接收 MarketEvent。
//!
//! ## 职责
//! - 从消息队列消费行情事件
//! - 转发给策略处理（当前为空实现）
//!
//! ## 依赖规则
//! - ✅ 只依赖 `domain::port` 中的 trait
//! - ❌ 不直接依赖 infrastructure
//! - ❌ 不在 service 内 new adapter
//! - ❌ 不实现任何策略计算

use crate::domain::port::market_event_port::MarketEventPort;
use tracing::info;

/// 行情事件消费服务
pub struct MarketEventConsumerService<E>
where
    E: MarketEventPort,
{
    event_source: E,
}

impl<E> MarketEventConsumerService<E>
where
    E: MarketEventPort,
{
    /// 创建服务实例（由 bootstrap 调用）
    pub fn new(event_source: E) -> Self {
        Self { event_source }
    }

    /// 运行事件消费循环
    ///
    /// # 流程
    /// 1. 循环获取行情事件
    /// 2. 记录日志（当前不做任何处理）
    ///
    /// # 返回
    /// - `Ok(())`: 正常退出（不会发生）
    /// - `Err`: 消费失败
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("MarketEventConsumerService started");

        loop {
            let event = self.event_source.next_event().await?;

            // TODO: 转发给策略处理
            // 当前只记录日志，不做任何计算
            info!(
                symbol = %event.symbol,
                exchange = %event.exchange,
                "Received MarketEvent"
            );
        }
    }
}
