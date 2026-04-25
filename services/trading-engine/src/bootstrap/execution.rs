//! Execution port factory

use std::sync::Arc;

use anyhow::anyhow;

use crate::domain::port::execution_port::ExecutionPort;
use crate::domain::port::order_execution_port::OrderExecutionPort;
use crate::infrastructure::execution::{BinanceExecution, BinanceFuturesExecution, OrderExecutor};

pub fn create_execution_port(
    mode: &str,
    api_key: Option<String>,
    secret_key: Option<String>,
    base_url: String,
) -> anyhow::Result<Arc<dyn ExecutionPort>> {
    match mode.trim().to_lowercase().as_str() {
        "binance" => {
            let api_key = api_key.ok_or_else(|| anyhow!("BINANCE_API_KEY is required for binance execution"))?;
            let secret_key = secret_key.ok_or_else(|| anyhow!("BINANCE_SECRET_KEY is required for binance execution"))?;
            if base_url.trim().is_empty() {
                return Err(anyhow!("BINANCE_BASE_URL is required for binance execution"));
            }
            tracing::info!(base_url = %base_url, "using binance execution adapter");
            Ok(Arc::new(BinanceExecution::new(api_key, secret_key, base_url)))
        }
        "binance_futures" | "futures" => {
            let api_key = api_key.ok_or_else(|| anyhow!("BINANCE_API_KEY is required for binance_futures execution"))?;
            let secret_key = secret_key.ok_or_else(|| anyhow!("BINANCE_SECRET_KEY is required for binance_futures execution"))?;
            if base_url.trim().is_empty() {
                return Err(anyhow!("BINANCE_BASE_URL is required for binance_futures execution"));
            }
            tracing::info!(base_url = %base_url, "using binance futures execution adapter");
            Ok(Arc::new(BinanceFuturesExecution::new(api_key, secret_key, base_url)))
        }
        other => Err(anyhow!(
            "unsupported TRADING_EXECUTION_MODE={} (allowed: binance, binance_futures)",
            other
        )),
    }
}

pub fn create_order_execution_port(inner: Arc<dyn ExecutionPort>) -> Arc<dyn OrderExecutionPort> {
    Arc::new(OrderExecutor::new(inner))
}
