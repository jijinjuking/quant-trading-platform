//! # 订单风控端口 (Order Risk Port)
//!
//! 定义订单风控检查的抽象接口。
//! 检查 OrderIntent 是否符合风控要求。

use std::sync::Arc;

use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::domain::model::order_intent::OrderIntent;

/// 订单风控端口
///
/// 定义订单风控检查的抽象接口。
#[async_trait]
pub trait OrderRiskPort: Send + Sync {
    /// 检查交易意图是否符合风控要求
    ///
    /// # 参数
    /// - `intent`: 交易意图
    ///
    /// # 返回
    /// - `Ok(())`: 风控通过
    /// - `Err`: 风控拒绝，包含拒绝原因
    async fn check(&self, intent: &OrderIntent) -> anyhow::Result<()>;

    /// 更新持仓（下单成功后调用）
    ///
    /// # 参数
    /// - `symbol`: 交易对
    /// - `delta`: 持仓变化量（正数加仓，负数减仓）
    async fn update_position(&self, symbol: &str, delta: Decimal);

    /// 记录下单时间（下单成功后调用）
    ///
    /// # 参数
    /// - `symbol`: 交易对
    async fn record_order_time(&self, symbol: &str);
}

#[async_trait]
impl<T: OrderRiskPort> OrderRiskPort for Arc<T> {
    async fn check(&self, intent: &OrderIntent) -> anyhow::Result<()> {
        (**self).check(intent).await
    }

    async fn update_position(&self, symbol: &str, delta: Decimal) {
        (**self).update_position(symbol, delta).await
    }

    async fn record_order_time(&self, symbol: &str) {
        (**self).record_order_time(symbol).await
    }
}
