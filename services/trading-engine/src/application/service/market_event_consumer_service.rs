//! # 行情事件消费服务 (Market Event Consumer Service)
//!
//! 消费行情事件，转发给 ExecutionService 处理。
//!
//! ## 职责
//! - 消费 MarketEvent
//! - 调用 ExecutionService.on_market_event()
//!
//! ## 禁止
//! - 禁止直接调用 StrategyPort
//! - 禁止直接调用 RiskPort
//! - 禁止直接调用 ExecutionPort
//! - 禁止包含任何业务逻辑

use std::sync::Arc;
use std::time::Duration;

use tracing::{error, info};

use crate::application::service::execution_service::ExecutionService;
use crate::domain::port::market_event_port::MarketEventPort;

/// 行情事件消费服务
pub struct MarketEventConsumerService {
    source: Arc<dyn MarketEventPort>,
    execution: Arc<ExecutionService>,
}

impl MarketEventConsumerService {
    /// 创建行情事件消费服务
    ///
    /// # 参数
    /// - `source`: 行情事件源
    /// - `execution`: 交易主链路调度服务
    pub fn new(source: Arc<dyn MarketEventPort>, execution: Arc<ExecutionService>) -> Self {
        Self { source, execution }
    }

    /// 运行消费循环
    ///
    /// 这是唯一的交易主循环入口。
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("MarketEventConsumerService started, waiting for market events...");

        loop {
            match self.source.next_event().await {
                Ok(event) => {
                    // 转发给 ExecutionService 处理
                    // 不在这里做任何业务判断
                    if let Err(err) = self.execution.on_market_event(&event).await {
                        error!(
                            symbol = %event.symbol,
                            error = %err,
                            "ExecutionService error"
                        );
                    }
                }
                Err(err) => {
                    error!(error = %err, "Market event source error");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
