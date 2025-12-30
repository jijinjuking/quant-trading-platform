//! # 策略端口 (Strategy Port)
//!
//! 定义策略入口的抽象接口。
//!
//! ## 架构位置
//! Domain Layer > Port
//!
//! ## 职责
//! - 接收 MarketEvent（只读引用）
//! - 作为"行情事件 → 策略入口"的结构性通道
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 不允许引入任何 infrastructure 依赖
//! - 不允许包含业务判断逻辑

use std::sync::Arc;
use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

/// 策略端口
///
/// 定义策略接收行情事件的抽象接口。
/// Infrastructure 层的具体策略实现必须实现此 trait。
///
/// # 设计说明
/// 当前仅作为结构性通道，不包含任何策略逻辑。
/// 未来可替换为真实策略实现。
#[async_trait]
pub trait StrategyPort: Send + Sync {
    /// 接收行情事件
    ///
    /// # 参数
    /// - `event`: 行情事件（只读引用）
    ///
    /// # 返回
    /// - `Ok(())`: 处理成功
    /// - `Err`: 处理失败
    async fn on_market_event(&self, event: &MarketEvent) -> anyhow::Result<()>;
}

// Arc<T> 自动实现 StrategyPort
#[async_trait]
impl<T: StrategyPort> StrategyPort for Arc<T> {
    async fn on_market_event(&self, event: &MarketEvent) -> anyhow::Result<()> {
        (**self).on_market_event(event).await
    }
}
