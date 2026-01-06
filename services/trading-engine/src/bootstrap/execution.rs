//! # 执行端口工厂
//!
//! 路径: services/trading-engine/src/bootstrap/execution.rs
//!
//! ## 职责
//! 创建 ExecutionPort 和 OrderExecutionPort 实现

use std::sync::Arc;

use anyhow::anyhow;

use crate::domain::port::execution_port::ExecutionPort;
use crate::domain::port::order_execution_port::OrderExecutionPort;
use crate::infrastructure::execution::{BinanceExecution, NoopExecution, OrderExecutor};

/// 创建执行端口
///
/// # 参数
/// - `mode`: 执行模式 ("binance" | "noop")
/// - `api_key`: 币安 API Key（binance 模式必需）
/// - `secret_key`: 币安 Secret Key（binance 模式必需）
/// - `base_url`: 币安 API Base URL（binance 模式必需）
///
/// # 返回
/// - `Arc<dyn ExecutionPort>`: 底层执行端口
pub fn create_execution_port(
    mode: &str,
    api_key: Option<String>,
    secret_key: Option<String>,
    base_url: String,
) -> anyhow::Result<Arc<dyn ExecutionPort>> {
    match mode.trim().to_lowercase().as_str() {
        "binance" => {
            let api_key = api_key
                .ok_or_else(|| anyhow!("BINANCE_API_KEY is required for binance execution"))?;
            let secret_key = secret_key
                .ok_or_else(|| anyhow!("BINANCE_SECRET_KEY is required for binance execution"))?;
            if base_url.trim().is_empty() {
                return Err(anyhow!("BINANCE_BASE_URL is required for binance execution"));
            }
            tracing::info!(base_url = %base_url, "使用币安执行端口");
            Ok(Arc::new(BinanceExecution::new(api_key, secret_key, base_url)))
        }
        _ => {
            tracing::info!("使用 Noop 执行端口（不实际下单）");
            Ok(Arc::new(NoopExecution::new()))
        }
    }
}

/// 创建订单执行端口（包装底层执行端口）
///
/// # 参数
/// - `inner`: 底层执行端口
///
/// # 返回
/// - `Arc<dyn OrderExecutionPort>`: 订单执行端口
pub fn create_order_execution_port(
    inner: Arc<dyn ExecutionPort>,
) -> Arc<dyn OrderExecutionPort> {
    Arc::new(OrderExecutor::new(inner))
}
