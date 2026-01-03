//! # Noop 审计适配器 (Noop Audit Adapter)
//!
//! 路径: services/trading-engine/src/infrastructure/audit/noop_audit.rs
//!
//! ## 职责
//! TradeAuditPort 的空实现，只打日志不落库。
//! v1 阶段使用，后续可替换为 PostgreSQL/Kafka 实现。

use anyhow::Result;
use async_trait::async_trait;
use tracing::info;

use crate::domain::model::audit_event::{ExecutionResultEvent, RiskRejectedEvent};
use crate::domain::port::trade_audit_port::TradeAuditPort;

/// Noop 审计适配器
///
/// 只打日志，不落库。用于 v1 阶段。
pub struct NoopAuditAdapter;

impl NoopAuditAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopAuditAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TradeAuditPort for NoopAuditAdapter {
    async fn record_risk_rejected(&self, event: &RiskRejectedEvent) -> Result<()> {
        info!(
            event_id = %event.event_id,
            strategy_id = %event.strategy_id,
            symbol = %event.symbol,
            side = ?event.side,
            quantity = %event.quantity,
            reject_code = %event.reject_code,
            reject_reason = %event.reject_reason,
            timestamp = %event.timestamp,
            "[AUDIT] Risk rejected event recorded (noop)"
        );
        Ok(())
    }

    async fn record_execution_result(&self, event: &ExecutionResultEvent) -> Result<()> {
        if event.success {
            info!(
                event_id = %event.event_id,
                strategy_id = %event.strategy_id,
                symbol = %event.symbol,
                side = ?event.side,
                quantity = %event.quantity,
                exchange_order_id = ?event.exchange_order_id,
                timestamp = %event.timestamp,
                "[AUDIT] Execution success event recorded (noop)"
            );
        } else {
            info!(
                event_id = %event.event_id,
                strategy_id = %event.strategy_id,
                symbol = %event.symbol,
                side = ?event.side,
                quantity = %event.quantity,
                error = ?event.error_message,
                timestamp = %event.timestamp,
                "[AUDIT] Execution failure event recorded (noop)"
            );
        }
        Ok(())
    }
}
