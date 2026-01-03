//! # Mock 消费者适配器 (Mock Consumer Adapter)
//!
//! 用于测试的 Mock 实现。

use async_trait::async_trait;
use anyhow::anyhow;
use shared::event::market_event::MarketEvent;
use crate::domain::port::market_event_port::MarketEventPort;

/// Mock 消费者适配器
///
/// 用于测试，不连接真实 Kafka。
pub struct MockConsumer {
    // TODO: 可添加预设的测试数据
}

impl MockConsumer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MockConsumer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketEventPort for MockConsumer {
    async fn next_event(&self) -> anyhow::Result<MarketEvent> {
        // Mock 场景：未配置数据时直接返回错误
        Err(anyhow!("MockConsumer: no test data configured"))
    }
}
