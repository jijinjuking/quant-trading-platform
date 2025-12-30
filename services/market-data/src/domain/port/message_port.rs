//! # 消息推送端口 (Message Port)
//!
//! 定义发布行情事件的抽象接口。
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 入参/出参只能是领域对象或基础类型
//! - 不暴露任何 Kafka / MQ 类型

use std::sync::Arc;
use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

/// 消息推送端口
#[async_trait]
pub trait MessagePort: Send + Sync {
    /// 发布行情事件
    async fn publish(&self, event: MarketEvent) -> anyhow::Result<()>;
}

// Arc<T> 自动实现 MessagePort
#[async_trait]
impl<T: MessagePort> MessagePort for Arc<T> {
    async fn publish(&self, event: MarketEvent) -> anyhow::Result<()> {
        (**self).publish(event).await
    }
}
