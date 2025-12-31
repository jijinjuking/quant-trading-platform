//! # Noop 风控适配器 (Noop Risk Adapter) - v1 占位骨架
//!
//! 占位实现，仅记录日志，不执行任何风控逻辑。
//!
//! ## 架构位置
//! Infrastructure Layer > Risk Adapter
//!
//! ## 版本说明
//! v1 仅作为结构性通道，不拦截、不拒绝、不修改任何数据。
//!
//! ## 职责
//! - 实现 RiskPort trait
//! - 仅记录事件信息（日志输出）
//!
//! ## 规则
//! - 不允许保存状态
//! - 不允许写缓存
//! - 不允许访问数据库
//! - 不允许访问网络
//! - 不允许拦截或拒绝

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;
use crate::domain::port::risk_port::RiskPort;
use tracing::info;

/// Noop 风控适配器 - v1 占位骨架
///
/// 占位实现，用于架构验证。
/// 未来可替换为真实风控实现。
pub struct NoopRisk;

impl NoopRisk {
    /// 创建 NoopRisk 实例
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopRisk {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiskPort for NoopRisk {
    async fn check(&self, event: &MarketEvent) -> anyhow::Result<()> {
        // v1: 仅记录日志，始终通过
        info!(
            symbol = %event.symbol,
            exchange = %event.exchange,
            "NoopRisk: check passed (v1 always passes)"
        );
        Ok(())
    }
}
