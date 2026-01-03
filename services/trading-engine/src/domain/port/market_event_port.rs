//! # 行情事件端口 (Market Event Port)
//!
//! 定义行情事件消费的抽象接口。

use std::sync::Arc;

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

/// 行情事件端口
///
/// 定义行情事件消费的抽象接口。
#[async_trait]
pub trait MarketEventPort: Send + Sync {
    /// 获取下一个行情事件
    async fn next_event(&self) -> anyhow::Result<MarketEvent>;
}

#[async_trait]
impl<T: MarketEventPort> MarketEventPort for Arc<T> {
    async fn next_event(&self) -> anyhow::Result<MarketEvent> {
        (**self).next_event().await
    }
}
