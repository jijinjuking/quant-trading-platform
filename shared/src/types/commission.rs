//! # Commission 类型定义
//!
//! 分佣系统的纯数据结构。
//! 按 DDD 规范，只放数据结构，不放 Port trait。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// 分佣记录
// ============================================================================

/// 分佣状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommissionStatus {
    /// 待结算
    Pending,
    /// 已结算
    Settled,
    /// 已取消
    Cancelled,
}

impl Default for CommissionStatus {
    fn default() -> Self {
        CommissionStatus::Pending
    }
}

/// 分佣记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionRecord {
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
    /// 结算时间
    pub settled_at: Option<DateTime<Utc>>,
}

// ============================================================================
// 结算
// ============================================================================

/// 结算周期
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettlementPeriod {
    /// 实时
    Realtime,
    /// 每日
    Daily,
    /// 每周
    Weekly,
    /// 每月
    Monthly,
    /// 手动
    Manual,
}

impl Default for SettlementPeriod {
    fn default() -> Self {
        SettlementPeriod::Realtime
    }
}

/// 结算批次状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettlementBatchStatus {
    /// 待处理
    Pending,
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 失败
    Failed,
}

impl Default for SettlementBatchStatus {
    fn default() -> Self {
        SettlementBatchStatus::Pending
    }
}

/// 结算批次
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementBatch {
    /// 批次 ID
    pub id: Uuid,
    /// 结算周期
    pub period: SettlementPeriod,
    /// 周期开始时间
    pub period_start: DateTime<Utc>,
    /// 周期结束时间
    pub period_end: DateTime<Utc>,
    /// 状态
    pub status: SettlementBatchStatus,
    /// 包含的分佣记录数
    pub record_count: u64,
    /// Leader 总分佣（USDT）
    pub total_leader_commission: Decimal,
    /// 平台总分佣（USDT）
    pub total_platform_commission: Decimal,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 错误信息
    pub error: Option<String>,
}

// ============================================================================
// 高水位线
// ============================================================================

/// 高水位线
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighWaterMark {
    /// 跟单关系 ID
    pub follow_id: Uuid,
    /// 当前高水位线（USDT）
    pub value: Decimal,
    /// 上次更新时间
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// 配置
// ============================================================================

/// 平台分佣配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCommissionConfig {
    /// 平台分佣比例
    pub platform_rate: Decimal,
    /// 默认 Leader 分佣比例
    pub default_leader_rate: Decimal,
    /// 最大 Leader 分佣比例
    pub max_leader_rate: Decimal,
    /// 最小 Leader 分佣比例
    pub min_leader_rate: Decimal,
    /// 默认结算周期
    pub default_settlement_period: SettlementPeriod,
    /// 是否启用高水位线
    pub high_water_mark_enabled: bool,
}

impl Default for PlatformCommissionConfig {
    fn default() -> Self {
        Self {
            platform_rate: Decimal::new(5, 2),        // 5%
            default_leader_rate: Decimal::new(20, 2), // 20%
            max_leader_rate: Decimal::new(50, 2),     // 50%
            min_leader_rate: Decimal::new(5, 2),      // 5%
            default_settlement_period: SettlementPeriod::Realtime,
            high_water_mark_enabled: true,
        }
    }
}
