//! Disabled execution adapter.

use async_trait::async_trait;

use crate::domain::port::execution_port::{ExecutionCommand, ExecutionPort, ExecutionResult};

/// Safety adapter: refuses to place any order when real execution is not configured.
pub struct NoopExecution;

impl NoopExecution {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopExecution {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ExecutionPort for NoopExecution {
    async fn execute(&self, command: &ExecutionCommand) -> anyhow::Result<ExecutionResult> {
        Err(anyhow::anyhow!(
            "execution is disabled for symbol={} side={:?}; set TRADING_EXECUTION_MODE=binance and configure BINANCE_*",
            command.symbol,
            command.side
        ))
    }
}
