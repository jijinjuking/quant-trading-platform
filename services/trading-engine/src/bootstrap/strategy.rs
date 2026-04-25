//! Strategy port factory

use std::sync::Arc;

use anyhow::anyhow;

use crate::domain::port::strategy_port::StrategyPort;
use crate::infrastructure::strategy::RemoteStrategy;

pub fn create_strategy_port(mode: &str, url: Option<String>) -> anyhow::Result<Arc<dyn StrategyPort>> {
    match mode.trim().to_lowercase().as_str() {
        "remote" => {
            let url = url.ok_or_else(|| anyhow!("STRATEGY_ENGINE_URL is required for remote strategy"))?;
            tracing::info!(url = %url, "using remote strategy service");
            Ok(Arc::new(RemoteStrategy::new(url)))
        }
        other => Err(anyhow!("unsupported TRADING_STRATEGY_MODE={} (allowed: remote)", other)),
    }
}
