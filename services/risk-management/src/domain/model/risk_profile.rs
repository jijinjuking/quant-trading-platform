//! # 风控配置模型 (Risk Profile Model)
//!
//! 本模块定义用户的风控配置，是风险管理领域的核心实体。
//!
//! ## 职责
//! - 存储用户的风险参数限制
//! - 作为风险评估的输入数据

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 风控配置
///
/// 定义用户的风险管理参数，用于订单前的风险检查。
/// 每个用户可以有不同的风控配置，以适应不同的风险偏好。
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskProfile {
    /// 用户唯一标识
    pub user_id: Uuid,
    /// 最大杠杆倍数（如 10.0 表示 10 倍杠杆）
    pub max_leverage: Decimal,
    /// 最大回撤比例（如 0.2 表示 20% 回撤）
    pub max_drawdown: Decimal,
    /// 最大持仓规模（以基础货币计）
    pub max_position_size: Decimal,
    /// 每日亏损限额（以基础货币计）
    pub daily_loss_limit: Decimal,
}
