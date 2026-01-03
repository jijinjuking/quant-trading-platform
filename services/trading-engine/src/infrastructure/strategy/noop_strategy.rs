//! # 空策略适配器 (Noop Strategy Adapter)
//!
//! 不产生任何交易意图的策略实现。
//! 用于测试和开发。

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;
use tracing::debug;

use crate::domain::model::order_intent::OrderIntent;
use crate::domain::port::strategy_port::StrategyPort;

/// 空策略适配器
pub struct NoopStrategy;

impl NoopStrategy {
    /// 创建空策略适配器
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopStrategy {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StrategyPort for NoopStrategy {
    async fn evaluate(&self, event: &MarketEvent) -> anyhow::Result<Option<OrderIntent>> {
        debug!(symbol = %event.symbol, "NoopStrategy: no intent generated");
        Ok(None)
    }
}
