//! # CopyTrading 类型定义
//!
//! 跟单交易系统的纯数据结构。
//! 按 DDD 规范，只放数据结构，不放 Port trait。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Leader（带单者）
// ============================================================================

/// Leader 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaderStatus {
    /// 活跃，接受新跟单
    Active,
    /// 暂停，不接受新跟单，现有跟单继续
    Paused,
    /// 关闭，不接受新跟单，现有跟单停止
    Closed,
}

impl Default for LeaderStatus {
    fn default() -> Self {
        LeaderStatus::Active
    }
}

/// Leader 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderConfig {
    /// Leader ID（= user_id）
    pub leader_id: Uuid,
    /// 公开的策略实例 ID
    pub strategy_id: Uuid,
    /// 分佣比例（0.0 - 1.0）
    pub commission_rate: Decimal,
    /// 最大跟单人数
    pub max_followers: u32,
    /// 最小跟单金额（USDT）
    pub min_follow_amount: Decimal,
    /// 状态
    pub status: LeaderStatus,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// Follower（跟单者）
// ============================================================================

/// Follower 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FollowerStatus {
    /// 活跃，正常跟单
    Active,
    /// 暂停，暂停跟单但保留关系
    Paused,
    /// 停止，终止跟单关系
    Stopped,
}

impl Default for FollowerStatus {
    fn default() -> Self {
        FollowerStatus::Active
    }
}

/// 风控覆盖配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RiskOverride {
    /// 单交易对最大持仓（USDT）
    pub max_position_per_symbol: Option<Decimal>,
    /// 总持仓上限（USDT）
    pub max_total_position: Option<Decimal>,
    /// 单笔订单上限（USDT）
    pub max_single_order: Option<Decimal>,
    /// 允许的交易对（白名单）
    pub allowed_symbols: Vec<String>,
    /// 禁止的交易对（黑名单）
    pub blocked_symbols: Vec<String>,
    /// 最大杠杆倍数
    pub max_leverage: Option<u32>,
}

/// 缩放模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalingMode {
    /// 固定比例
    FixedRatio,
    /// 固定金额
    FixedAmount,
    /// 按资金比例
    ProportionalToCapital,
}

impl Default for ScalingMode {
    fn default() -> Self {
        ScalingMode::FixedRatio
    }
}

/// 缩放配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// 缩放模式
    pub mode: ScalingMode,
    /// 固定比例
    pub ratio: Option<Decimal>,
    /// 固定金额上限（USDT）
    pub max_amount: Option<Decimal>,
    /// 最小交易金额（USDT）
    pub min_amount: Decimal,
    /// 最大单笔金额（USDT）
    pub max_single_amount: Decimal,
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            mode: ScalingMode::FixedRatio,
            ratio: Some(Decimal::new(1, 1)), // 0.1
            max_amount: None,
            min_amount: Decimal::new(10, 0),
            max_single_amount: Decimal::new(10000, 0),
        }
    }
}

/// Follower 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowerConfig {
    /// 跟单关系 ID
    pub id: Uuid,
    /// Follower ID（= user_id）
    pub follower_id: Uuid,
    /// Leader ID
    pub leader_id: Uuid,
    /// 跟随的策略 ID
    pub strategy_id: Uuid,
    /// 缩放配置
    pub scaling: ScalingConfig,
    /// 风控覆盖配置
    pub risk_override: RiskOverride,
    /// 跟单资金（USDT）
    pub follow_amount: Decimal,
    /// 状态
    pub status: FollowerStatus,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// CopyTrading 元数据
// ============================================================================

/// CopyTrading 元数据（附加在执行草稿上）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyTradingMeta {
    /// 跟单关系 ID
    pub follow_id: Uuid,
    /// Leader ID
    pub leader_id: Uuid,
    /// Follower ID
    pub follower_id: Uuid,
    /// 原始策略 ID
    pub original_strategy_id: Uuid,
    /// 原始意图 ID
    pub original_intent_id: Uuid,
    /// 应用的缩放比例
    pub applied_ratio: Decimal,
}
