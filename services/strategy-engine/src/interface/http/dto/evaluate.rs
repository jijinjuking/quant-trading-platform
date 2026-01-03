//! # 策略评估 DTO
//!
//! 定义策略评估 API 的请求/响应结构。

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 策略评估请求
///
/// trading-engine 调用此 API 进行策略评估
#[derive(Debug, Clone, Deserialize)]
pub struct EvaluateRequest {
    /// 策略实例 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 当前价格
    pub price: Decimal,
    /// 成交量
    pub quantity: Decimal,
    /// 时间戳（毫秒）
    pub timestamp: i64,
    /// 是否买方主动
    #[serde(default)]
    pub is_buyer_maker: bool,
}

/// 策略评估响应
#[derive(Debug, Clone, Serialize)]
pub struct EvaluateResponse {
    /// 是否生成交易意图
    pub has_intent: bool,
    /// 交易意图（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<OrderIntentDto>,
}

/// 交易意图 DTO
#[derive(Debug, Clone, Serialize)]
pub struct OrderIntentDto {
    /// 意图 ID
    pub id: Uuid,
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向: "buy" / "sell" / "hold"
    pub side: String,
    /// 数量
    pub quantity: Decimal,
    /// 价格（限价单）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// 订单类型: "market" / "limit"
    pub order_type: String,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f64,
    /// 创建时间（毫秒）
    pub created_at: i64,
}
