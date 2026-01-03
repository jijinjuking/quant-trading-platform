//! # 风控检查 DTO
//!
//! 定义风控检查的请求和响应数据结构。

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 风控检查请求
#[derive(Debug, Clone, Deserialize)]
pub struct RiskCheckRequest {
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向: "buy" / "sell"
    pub side: String,
    /// 数量
    pub quantity: Decimal,
    /// 价格
    pub price: Decimal,
    /// 订单类型: "market" / "limit"
    pub order_type: String,
}

/// 风控检查响应
#[derive(Debug, Clone, Serialize)]
pub struct RiskCheckResponse {
    /// 是否通过
    pub approved: bool,
    /// 拒绝原因（如果被拒绝）
    pub reason: Option<String>,
    /// 检查详情
    pub checks: Vec<RiskCheckDetail>,
}

/// 单项检查详情
#[derive(Debug, Clone, Serialize)]
pub struct RiskCheckDetail {
    /// 检查项名称
    pub name: String,
    /// 是否通过
    pub passed: bool,
    /// 说明
    pub message: Option<String>,
}

impl RiskCheckResponse {
    /// 创建通过响应
    pub fn approved(checks: Vec<RiskCheckDetail>) -> Self {
        Self {
            approved: true,
            reason: None,
            checks,
        }
    }

    /// 创建拒绝响应
    pub fn rejected(reason: impl Into<String>, checks: Vec<RiskCheckDetail>) -> Self {
        Self {
            approved: false,
            reason: Some(reason.into()),
            checks,
        }
    }
}

impl RiskCheckDetail {
    /// 创建通过的检查项
    pub fn passed(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: true,
            message: None,
        }
    }

    /// 创建失败的检查项
    pub fn failed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: false,
            message: Some(message.into()),
        }
    }
}
