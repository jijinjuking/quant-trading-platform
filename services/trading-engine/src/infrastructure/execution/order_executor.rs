//! Order executor adapter

use async_trait::async_trait;
use tracing::{debug, info};

use crate::domain::model::order_intent::{OrderIntent, OrderSide};
use crate::domain::port::execution_port::{ExecutionCommand, ExecutionPort, OrderSide as ExchangeOrderSide};
use crate::domain::port::order_execution_port::{ExecutionResult, OrderExecutionPort};

pub struct OrderExecutor {
    inner: std::sync::Arc<dyn ExecutionPort>,
}

impl OrderExecutor {
    pub fn new(inner: std::sync::Arc<dyn ExecutionPort>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl OrderExecutionPort for OrderExecutor {
    async fn execute(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionResult> {
        let side = match intent.side {
            OrderSide::Buy => ExchangeOrderSide::Buy,
            OrderSide::Sell => ExchangeOrderSide::Sell,
        };

        let command = match intent.price {
            Some(price) => ExecutionCommand::limit(intent.symbol.clone(), side, intent.quantity, price)
                .with_client_order_id(intent.id.to_string()),
            None => ExecutionCommand::market(intent.symbol.clone(), side, intent.quantity)
                .with_client_order_id(intent.id.to_string()),
        };

        debug!(symbol = %command.symbol, side = ?command.side, quantity = %command.quantity, "Executing order");

        match self.inner.execute(&command).await {
            Ok(exchange_result) => {
                let status = exchange_result.status.to_ascii_uppercase();
                let success = !matches!(status.as_str(), "REJECTED" | "EXPIRED" | "CANCELED");

                info!(symbol = %command.symbol, side = ?command.side, order_id = %exchange_result.order_id, status = %exchange_result.status, "Order execution completed");

                Ok(ExecutionResult {
                    order_id: exchange_result.order_id,
                    symbol: intent.symbol.clone(),
                    success,
                    error: if success {
                        None
                    } else {
                        Some(format!("exchange order failed, status={}", exchange_result.status))
                    },
                })
            }
            Err(err) => Ok(ExecutionResult {
                order_id: intent.id.to_string(),
                symbol: intent.symbol.clone(),
                success: false,
                error: Some(err.to_string()),
            }),
        }
    }
}
