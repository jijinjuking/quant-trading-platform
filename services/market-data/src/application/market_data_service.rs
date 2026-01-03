//! # 行情数据服务 (Market Data Service)
//!
//! 应用层服务，负责编排行情采集流程。
//!
//! ## 职责
//! - 连接交易所
//! - 获取行情事件
//! - 发布到消息队列
//! - 存储到 ClickHouse
//!
//! ## 依赖规则
//! - ✅ 只依赖 `domain::port` 中的 trait
//! - ❌ 不直接依赖 infrastructure

use std::time::Duration;

use tracing::{debug, error, info, warn};

use crate::domain::port::{MarketExchangePort, MarketStoragePort, MessagePort};

/// 行情数据服务
pub struct MarketDataService<E, M, S>
where
    E: MarketExchangePort,
    M: MessagePort,
    S: MarketStoragePort,
{
    exchange: E,
    message: M,
    storage: Option<S>,
}

impl<E, M, S> MarketDataService<E, M, S>
where
    E: MarketExchangePort,
    M: MessagePort,
    S: MarketStoragePort,
{
    /// 创建服务实例（由 bootstrap 调用）
    pub fn new(exchange: E, message: M, storage: Option<S>) -> Self {
        Self { exchange, message, storage }
    }

    /// 运行行情采集循环
    pub async fn run(&self, symbols: Vec<String>) -> anyhow::Result<()> {
        info!("MarketDataService starting with symbols: {:?}", symbols);

        // 1. 连接交易所
        self.exchange.connect().await?;

        // 2. 订阅现货行情
        if !symbols.is_empty() {
            self.exchange.subscribe_spot(symbols).await?;
        }

        // 3. 循环获取并发布行情
        loop {
            match self.exchange.next_event().await {
                Ok(event) => {
                    // 发布到 Kafka
                    if let Err(e) = self.message.publish(event.clone()).await {
                        warn!(error = %e, "Failed to publish event to Kafka");
                    }

                    // 存储到 ClickHouse（如果启用）
                    if let Some(ref storage) = self.storage {
                        if let Err(e) = storage.save_event(&event).await {
                            debug!(error = %e, "Failed to save event to storage");
                        }
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to get event");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
