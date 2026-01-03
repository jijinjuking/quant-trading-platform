//! # 策略端口 (Strategy Port)
//!
//! 定义策略计算的抽象接口。
//! Trading Engine 通过此端口调用策略计算，获取交易意图。
//!
//! ## 架构位置
//! Domain Layer > Port
//!
//! ## 职责
//! - 接收 MarketEvent
//! - 返回 OrderIntent（交易意图，非执行指令）
//!
//! ## 规则
//! - Strategy 只能被 ExecutionFlowService 调用
//! - Strategy 不能直接发 Kafka
//! - Strategy 不能直接调用 Execution

use std::sync::Arc;

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

use crate::domain::model::order_intent::OrderIntent;

/// 策略端口
///
/// 定义策略计算的抽象接口。
/// 策略实现必须实现此 trait。
#[async_trait]
pub trait StrategyPort: Send + Sync {
    /// 评估行情事件，返回交易意图
    ///
    /// # 参数
    /// - `event`: 行情事件
    ///
    /// # 返回
    /// - `Ok(Some(OrderIntent))`: 产生交易意图
    /// - `Ok(None)`: 无交易意图
    /// - `Err`: 评估失败
    async fn evaluate(&self, event: &MarketEvent) -> anyhow::Result<Option<OrderIntent>>;
}

#[async_trait]
impl<T: StrategyPort> StrategyPort for Arc<T> {
    async fn evaluate(&self, event: &MarketEvent) -> anyhow::Result<Option<OrderIntent>> {
        (**self).evaluate(event).await
    }
}
