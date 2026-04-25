//! # Commission 事件定义
//!
//! Kafka 事件，用于服务间通信。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::commission::CommissionStatus;

/// 分佣记录事件
///
/// Commission Processor → Accounting Service
/// Topic: commission.records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionRecordEvent {
    /// 记录 ID
    pub id: Uuid,
    /// 跟单关系 ID
    pub follow_id: Uuid,
    /// Leader ID
    pub leader_id: Uuid,
    /// Follower ID
    pub follower_id: Uuid,
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 关联的执行结果 ID
    pub execution_result_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 已实现盈亏（USDT）
    pub realized_pnl: Decimal,
    /// Leader 分佣金额（USDT）
    pub leader_commission: Decimal,
    /// 平台分佣金额（USDT）
    pub platform_commission: Decimal,
    /// Follower 净收益（USDT）
    pub follower_net: Decimal,
    /// Leader 分佣比例
    pub leader_rate: Decimal,
    /// 平台分佣比例
    pub platform_rate: Decimal,
    /// 状态
    pub status: CommissionStatus,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}
