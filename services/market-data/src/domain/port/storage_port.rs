//! # 存储端口 (Storage Port)
//!
//! 定义行情数据存储的抽象接口。
//! Infrastructure 层实现此 trait。

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

/// 行情存储端口
///
/// 定义行情数据持久化的抽象接口。
/// 具体实现可以是 ClickHouse、PostgreSQL 等。
#[async_trait]
pub trait MarketStoragePort: Send + Sync {
    /// 存储单条行情事件
    async fn save_event(&self, event: &MarketEvent) -> Result<()>;

    /// 批量存储行情事件
    async fn save_events(&self, events: &[MarketEvent]) -> Result<()>;
}

/// 为 Arc<T> 实现 MarketStoragePort（blanket implementation）
#[async_trait]
impl<T: MarketStoragePort> MarketStoragePort for Arc<T> {
    async fn save_event(&self, event: &MarketEvent) -> Result<()> {
        (**self).save_event(event).await
    }

    async fn save_events(&self, events: &[MarketEvent]) -> Result<()> {
        (**self).save_events(events).await
    }
}
