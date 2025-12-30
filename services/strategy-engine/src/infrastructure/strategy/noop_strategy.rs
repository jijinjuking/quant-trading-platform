//! # Noop 策略适配器 (Noop Strategy Adapter)
//!
//! 占位实现，仅记录日志，不执行任何策略逻辑。
//!
//! ## 架构位置
//! Infrastructure Layer > Strategy Adapter
//!
//! ## 职责
//! - 实现 StrategyPort trait
//! - 仅记录事件信息（日志输出）
//!
//! ## 规则
//! - 不允许保存状态
//! - 不允许写缓存
//! - 不允许写业务逻辑

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;
use crate::domain::port::strategy_port::StrategyPort;
use tracing::info;

/// Noop 策略适配器
///
/// 占位实现，用于架构验证。
/// 未来可替换为真实策略实现。
pub struct NoopStrategy;

impl NoopStrategy {
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
    async fn on_market_event(&self, event: &MarketEvent) -> anyhow::Result<()> {
        // 仅记录日志，不执行任何策略逻辑
        info!(
            event_type = ?event.event_type,
            symbol = %event.symbol,
            exchange = %event.exchange,
            timestamp = %event.timestamp,
            "NoopStrategy received MarketEvent"
        );
        Ok(())
    }
}
