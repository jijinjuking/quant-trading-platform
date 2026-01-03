//! # 交易审计端口 (Trade Audit Port)
//!
//! 路径: services/trading-engine/src/domain/port/trade_audit_port.rs
//!
//! ## 职责
//! 定义交易审计的端口接口，用于记录风控决策和执行结果。
//! 
//! ## 架构位置
//! - 所属层级: Domain Layer (Port)
//! - 实现位置: Infrastructure Layer
//! - 调用者: ExecutionService

use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::audit_event::{ExecutionResultEvent, RiskRejectedEvent};

/// 交易审计端口
///
/// 用于记录交易链路中的关键事件，包括：
/// - 风控拒绝事件
/// - 执行结果事件
///
/// v1 实现可以是 Noop（只打日志），后续可接入数据库或 Kafka。
#[async_trait]
pub trait TradeAuditPort: Send + Sync {
    /// 记录风控拒绝事件
    ///
    /// # 参数
    /// - `event`: 风控拒绝事件
    async fn record_risk_rejected(&self, event: &RiskRejectedEvent) -> Result<()>;

    /// 记录执行结果事件
    ///
    /// # 参数
    /// - `event`: 执行结果事件
    async fn record_execution_result(&self, event: &ExecutionResultEvent) -> Result<()>;
}
