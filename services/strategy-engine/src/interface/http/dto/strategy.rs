//! # 策略管理 DTO
//!
//! 定义策略 CRUD API 的请求/响应结构。

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 策略信息 DTO
#[derive(Debug, Clone, Serialize)]
pub struct StrategyInfoDto {
    /// 策略实例 ID
    pub instance_id: Uuid,
    /// 策略类型
    pub strategy_type: String,
    /// 市场类型
    pub market_type: String,
    /// 交易对
    pub symbol: String,
    /// 是否激活
    pub is_active: bool,
}

/// 策略列表响应
#[derive(Debug, Clone, Serialize)]
pub struct StrategyListResponse {
    /// 策略列表
    pub strategies: Vec<StrategyInfoDto>,
    /// 总数
    pub total: usize,
}

/// 创建策略请求
#[derive(Debug, Clone, Deserialize)]
pub struct CreateStrategyRequest {
    /// 策略类型: "spot_grid", "spot_mean", "futures_grid", etc.
    pub strategy_type: String,
    /// 市场类型: "spot", "usdt_futures", "coin_futures"
    pub market_type: String,
    /// 交易对
    pub symbol: String,
    /// 策略配置（JSON）
    #[serde(default)]
    pub config: serde_json::Value,
}

/// 创建策略响应
#[derive(Debug, Clone, Serialize)]
pub struct CreateStrategyResponse {
    /// 策略实例 ID
    pub instance_id: Uuid,
    /// 消息
    pub message: String,
}
