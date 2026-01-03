//! # 订单执行端口 (Order Execution Port)
//!
//! 定义订单执行的抽象接口。
//! 执行已通过风控检查的 OrderIntent。

use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::model::order_intent::OrderIntent;

/// 执行结果
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// 订单 ID（交易所返回）
    pub order_id: String,
    /// 交易对
    pub symbol: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息（如果失败）
    pub error: Option<String>,
}

/// 订单执行端口
///
/// 定义订单执行的抽象接口。
#[async_trait]
pub trait OrderExecutionPort: Send + Sync {
    /// 执行交易意图
    ///
    /// # 参数
    /// - `intent`: 已通过风控检查的交易意图
    ///
    /// # 返回
    /// - `Ok(ExecutionResult)`: 执行结果
    /// - `Err`: 执行失败
    async fn execute(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionResult>;
}

#[async_trait]
impl<T: OrderExecutionPort> OrderExecutionPort for Arc<T> {
    async fn execute(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionResult> {
        (**self).execute(intent).await
    }
}
