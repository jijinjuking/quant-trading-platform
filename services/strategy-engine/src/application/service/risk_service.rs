//! # 风控服务 (Risk Service) - v1 占位骨架
//!
//! 应用层服务，负责转发风控检查。
//!
//! ## 版本说明
//! v1 仅作为结构性通道，不做任何判断。
//!
//! ## 职责
//! - 接收上游调用
//! - 转发给 RiskPort
//!
//! ## 依赖规则
//! - ✅ 只依赖 `domain::port` 中的 trait
//! - ❌ 不直接依赖 infrastructure
//! - ❌ 不在 service 内 new adapter
//! - ❌ 不做判断
//! - ❌ 不保存状态

use crate::domain::port::risk_port::RiskPort;
use shared::event::market_event::MarketEvent;
use tracing::debug;

/// 风控服务 - v1 占位骨架
///
/// 应用层风控服务，仅负责转发。
/// 构造函数由 bootstrap 注入。
pub struct RiskService<R>
where
    R: RiskPort,
{
    risk: R,
}

impl<R> RiskService<R>
where
    R: RiskPort,
{
    /// 创建服务实例（由 bootstrap 调用）
    pub fn new(risk: R) -> Self {
        Self { risk }
    }

    /// 执行风控检查（v1 仅转发）
    ///
    /// # 参数
    /// - `event`: 行情事件
    ///
    /// # 返回
    /// - `Ok(())`: 风控通过
    /// - `Err`: 风控失败
    pub async fn check(&self, event: &MarketEvent) -> anyhow::Result<()> {
        debug!(
            symbol = %event.symbol,
            "RiskService forwarding to RiskPort"
        );
        self.risk.check(event).await
    }
}
