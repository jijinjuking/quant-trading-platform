//! # 行情事件消费端口 (Market Event Port)
//!
//! 定义消费 MarketEvent 的抽象接口。
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 入参/出参只能是领域对象或基础类型
//! - 不暴露任何 Kafka / MQ 类型

use std::sync::Arc;
use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

/// 行情事件消费端口
///
/// 定义从消息队列消费行情事件的抽象接口。
/// Infrastructure 层的具体消费者必须实现此 trait。
#[async_trait]
pub trait MarketEventPort: Send + Sync {
    /// 获取下一个行情事件
    ///
    /// 阻塞等待下一个行情事件。
    ///
    /// # 返回
    /// - `Ok(MarketEvent)`: 收到行情事件
    /// - `Err`: 消费失败或连接断开
    async fn next_event(&self) -> anyhow::Result<MarketEvent>;
}

// Arc<T> 自动实现 MarketEventPort
#[async_trait]
impl<T: MarketEventPort> MarketEventPort for Arc<T> {
    async fn next_event(&self) -> anyhow::Result<MarketEvent> {
        (**self).next_event().await
    }
}
