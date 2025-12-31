//! # 风控端口 (Risk Port) - v1 占位骨架
//!
//! 定义风控入口的抽象接口。
//!
//! ## 架构位置
//! Domain Layer > Port
//!
//! ## 版本说明
//! v1 仅作为结构性通道，不包含任何风控逻辑。
//! 验证 Strategy → Risk 的结构通路是否干净。
//!
//! ## 职责
//! - 接收来自 Strategy 的事件或信号
//! - 作为"策略 → 风控"的结构性通道
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 不允许引入任何 infrastructure 依赖
//! - 不允许包含业务判断逻辑

use std::sync::Arc;
use async_trait::async_trait;
use shared::event::market_event::MarketEvent;

/// 风控端口 - v1 占位骨架
///
/// 定义风控接收事件的抽象接口。
/// Infrastructure 层的具体风控实现必须实现此 trait。
///
/// # 设计说明
/// v1 仅作为结构性通道，不包含任何风控逻辑。
/// 未来可替换为真实风控实现。
#[async_trait]
pub trait RiskPort: Send + Sync {
    /// 检查风控（v1 占位）
    ///
    /// # 参数
    /// - `event`: 行情事件（只读引用）
    ///
    /// # 返回
    /// - `Ok(())`: 通过（v1 始终通过）
    /// - `Err`: 风控失败
    ///
    /// # TODO
    /// v2 需要定义 RiskResult 类型，支持拒绝/警告/通过
    async fn check(&self, event: &MarketEvent) -> anyhow::Result<()>;
}

// Arc<T> 自动实现 RiskPort
#[async_trait]
impl<T: RiskPort> RiskPort for Arc<T> {
    async fn check(&self, event: &MarketEvent) -> anyhow::Result<()> {
        (**self).check(event).await
    }
}
