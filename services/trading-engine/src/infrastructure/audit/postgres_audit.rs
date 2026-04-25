//! PostgreSQL audit adapter.

use anyhow::Result;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::types::Json;

use crate::domain::model::audit_event::{ExecutionResultEvent, RiskRejectedEvent};
use crate::domain::port::trade_audit_port::TradeAuditPort;

pub struct PostgresTradeAuditAdapter {
    pool: Pool,
}

impl PostgresTradeAuditAdapter {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    async fn ensure_table(&self) -> Result<()> {
        let client = self.pool.get().await?;
        client
            .batch_execute(
                "
                CREATE TABLE IF NOT EXISTS trade_audit_events (
                    id UUID PRIMARY KEY,
                    event_type TEXT NOT NULL,
                    strategy_id UUID NOT NULL,
                    symbol TEXT NOT NULL,
                    side TEXT NOT NULL,
                    quantity NUMERIC NOT NULL,
                    order_id TEXT NULL,
                    reject_code TEXT NULL,
                    message TEXT NOT NULL,
                    payload JSONB NOT NULL,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
                );
                CREATE INDEX IF NOT EXISTS idx_trade_audit_events_strategy_id
                    ON trade_audit_events(strategy_id);
                CREATE INDEX IF NOT EXISTS idx_trade_audit_events_created_at
                    ON trade_audit_events(created_at);
                "
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl TradeAuditPort for PostgresTradeAuditAdapter {
    async fn record_risk_rejected(&self, event: &RiskRejectedEvent) -> Result<()> {
        self.ensure_table().await?;
        let client = self.pool.get().await?;
        let payload = serde_json::to_value(event)?;

        client
            .execute(
                "
                INSERT INTO trade_audit_events
                    (id, event_type, strategy_id, symbol, side, quantity, order_id, reject_code, message, payload)
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ",
                &[
                    &event.event_id,
                    &"risk_rejected",
                    &event.strategy_id,
                    &event.symbol,
                    &format!("{:?}", event.side),
                    &event.quantity.to_string(),
                    &Option::<String>::None,
                    &Some(event.reject_code.clone()),
                    &event.reject_reason,
                    &Json(payload),
                ],
            )
            .await?;
        Ok(())
    }

    async fn record_execution_result(&self, event: &ExecutionResultEvent) -> Result<()> {
        self.ensure_table().await?;
        let client = self.pool.get().await?;
        let payload = serde_json::to_value(event)?;
        let message = if event.success {
            "execution_success".to_string()
        } else {
            event
                .error_message
                .clone()
                .unwrap_or_else(|| "execution_failed".to_string())
        };

        client
            .execute(
                "
                INSERT INTO trade_audit_events
                    (id, event_type, strategy_id, symbol, side, quantity, order_id, reject_code, message, payload)
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ",
                &[
                    &event.event_id,
                    &"execution_result",
                    &event.strategy_id,
                    &event.symbol,
                    &format!("{:?}", event.side),
                    &event.quantity.to_string(),
                    &event.exchange_order_id,
                    &Option::<String>::None,
                    &message,
                    &Json(payload),
                ],
            )
            .await?;
        Ok(())
    }
}
