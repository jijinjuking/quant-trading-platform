//! # 订单执行适配器 (Order Executor Adapter)
//!
//! 实现 OrderExecutionPort，执行 OrderIntent。

use async_trait::async_trait;
use tracing::{debug, info};

use crate::domain::model::order_intent::{OrderIntent, OrderSide};
use crate::domain::port::execution_port::{ExecutionCommand, ExecutionPort};
use crate::domain::port::order_execution_port::{ExecutionResult, OrderExecutionPort};

/// 订单执行适配器
///
/// 将 OrderIntent 转换为 ExecutionCommand，调用底层 ExecutionPort 执行。
pub struct OrderExecutor {
    inner: std::sync::Arc<dyn ExecutionPort>,
}

impl OrderExecutor {
    /// 创建订单执行适配器
    pub fn new(inner: std::sync::Arc<dyn ExecutionPort>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl OrderExecutionPort for OrderExecutor {
    async fn execute(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionResult> {
        let side = match intent.side {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        };

        let command = ExecutionCommand {
            symbol: intent.symbol.clone(),
            side: side.to_string(),
            quantity: intent.quantity.to_string(),
        };

        debug!(
            symbol = %command.symbol,
            side = %command.side,
            quantity = %command.quantity,
            "Executing order"
        );

        match self.inner.execute(&command).await {
            Ok(()) => {
                info!(
                    symbol = %command.symbol,
                    side = %command.side,
                    "Order executed successfully"
                );
                Ok(ExecutionResult {
                    order_id: intent.id.to_string(),
                    symbol: intent.symbol.clone(),
                    success: true,
                    error: None,
                })
            }
            Err(err) => {
                let error_msg = err.to_string();
                Ok(ExecutionResult {
                    order_id: intent.id.to_string(),
                    symbol: intent.symbol.clone(),
                    success: false,
                    error: Some(error_msg),
                })
            }
        }
    }
}
