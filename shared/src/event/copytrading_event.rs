//! # CopyTrading 事件定义
//!
//! Kafka 事件，用于服务间通信。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::copytrading::CopyTradingMeta;

// ============================================================================
// ExecutionDraft 事件
// ============================================================================

/// 执行草稿来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DraftSource {
    /// 直接来自策略
    Strategy,
    /// 来自 CopyTrading
    CopyTrading,
}

impl Default for DraftSource {
    fn default() -> Self {
        DraftSource::Strategy
    }
}

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DraftOrderSide {
    Buy,
    Sell,
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DraftOrderType {
    Market,
    Limit,
}

impl Default for DraftOrderType {
    fn default() -> Self {
        DraftOrderType::Market
    }
}

/// 执行草稿事件
///
/// CopyTrading Processor → Trading Engine
/// Topic: execution.drafts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDraftEvent {
    /// 草稿 ID
    pub id: Uuid,
    /// 来源
    pub source: DraftSource,
    /// 用户 ID（实际执行者）
    pub user_id: Uuid,
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: DraftOrderSide,
    /// 数量
    pub quantity: Decimal,
    /// 价格（限价单）
    pub price: Option<Decimal>,
    /// 订单类型
    pub order_type: DraftOrderType,
    /// 置信度
    pub confidence: f64,
    /// CopyTrading 元数据
    pub copytrading_meta: Option<CopyTradingMeta>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
}

// ============================================================================
// StrategyResult 事件
// ============================================================================

/// 策略结果状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyResultStatus {
    /// 产生交易意图
    IntentGenerated,
    /// 无交易意图
    NoIntent,
    /// 错误
    Error,
}

/// 交易意图方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntentSide {
    Buy,
    Sell,
}

/// 交易意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIntent {
    /// 意图 ID
    pub id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: IntentSide,
    /// 数量
    pub quantity: Decimal,
    /// 价格
    pub price: Option<Decimal>,
    /// 置信度
    pub confidence: f64,
}

/// 触发事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerEvent {
    /// 交易对
    pub symbol: String,
    /// 价格
    pub price: Decimal,
    /// 数量
    pub quantity: Decimal,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

/// 策略结果事件
///
/// Strategy Engine → CopyTrading Processor
/// Topic: strategy.results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyResultEvent {
    /// 结果 ID
    pub id: Uuid,
    /// 策略实例 ID
    pub strategy_id: Uuid,
    /// 策略 Owner ID（= Leader ID）
    pub owner_id: Uuid,
    /// 状态
    pub status: StrategyResultStatus,
    /// 交易意图
    pub intent: Option<TradeIntent>,
    /// 触发行情
    pub trigger_event: Option<TriggerEvent>,
    /// 执行耗时（微秒）
    pub execution_time_us: u64,
    /// 错误信息
    pub error: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}
