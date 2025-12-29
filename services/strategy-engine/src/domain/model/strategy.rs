//! # 策略模型 (Strategy Model)
//! 
//! 定义交易策略的领域实体。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 策略实体 - 聚合根
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    /// 策略唯一标识符
    pub id: Uuid,
    /// 用户ID
    pub user_id: Uuid,
    /// 策略名称
    pub name: String,
    /// 策略类型
    pub strategy_type: StrategyType,
    /// 策略参数（JSON 格式）
    pub parameters: serde_json::Value,
    /// 是否激活
    pub is_active: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 策略类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    /// 网格交易
    Grid,
    /// 均值回归
    MeanReversion,
    /// 动量策略
    Momentum,
    /// 套利策略
    Arbitrage,
    /// 自定义策略
    Custom,
}
