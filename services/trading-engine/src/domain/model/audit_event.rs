//! # 审计事件模型 (Audit Event Model)
//!
//! 路径: services/trading-engine/src/domain/model/audit_event.rs
//!
//! ## 职责
//! 定义交易审计相关的事件模型，用于记录风控决策和执行结果。
//! 这些模型仅用于审计记录，不包含任何风控逻辑。
//!
//! ## 架构位置
//! - 所属层级: Domain Layer (Model)
//! - 使用者: TradeAuditPort, ExecutionService

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::model::order_intent::OrderSide;

/// 风控拒绝事件（审计用）
///
/// 记录被风控拒绝的订单意图，用于审计和分析。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRejectedEvent {
    /// 事件 ID
    pub event_id: Uuid,
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: OrderSide,
    /// 数量
    pub quantity: Decimal,
    /// 价格（可选）
    pub price: Option<Decimal>,
    /// 拒绝原因
    pub reject_reason: String,
    /// 拒绝代码
    pub reject_code: String,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

impl RiskRejectedEvent {
    /// 创建风控拒绝事件
    pub fn new(
        strategy_id: Uuid,
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Option<Decimal>,
        reject_reason: String,
        reject_code: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            strategy_id,
            symbol,
            side,
            quantity,
            price,
            reject_reason,
            reject_code,
            timestamp: Utc::now(),
        }
    }
}

/// 执行结果事件（审计用）
///
/// 记录订单执行的结果，用于审计和分析。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResultEvent {
    /// 事件 ID
    pub event_id: Uuid,
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: OrderSide,
    /// 数量
    pub quantity: Decimal,
    /// 是否成功
    pub success: bool,
    /// 交易所订单 ID（成功时）
    pub exchange_order_id: Option<String>,
    /// 错误信息（失败时）
    pub error_message: Option<String>,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

impl ExecutionResultEvent {
    /// 创建执行成功事件
    pub fn success(
        strategy_id: Uuid,
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        exchange_order_id: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            strategy_id,
            symbol,
            side,
            quantity,
            success: true,
            exchange_order_id: Some(exchange_order_id),
            error_message: None,
            timestamp: Utc::now(),
        }
    }

    /// 创建执行失败事件
    pub fn failure(
        strategy_id: Uuid,
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        error_message: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            strategy_id,
            symbol,
            side,
            quantity,
            success: false,
            exchange_order_id: None,
            error_message: Some(error_message),
            timestamp: Utc::now(),
        }
    }
}
