//! # 持仓 DTO
//!
//! 持仓相关的请求/响应数据结构。

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 持仓查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct PositionQueryParams {
    /// 交易对（可选）
    pub symbol: Option<String>,
}

/// 持仓响应
#[derive(Debug, Clone, Serialize)]
pub struct PositionResponse {
    /// 交易对
    pub symbol: String,
    /// 持仓方向: "LONG" / "SHORT"
    pub side: String,
    /// 持仓数量
    pub quantity: Decimal,
    /// 开仓均价
    pub entry_price: Decimal,
    /// 标记价格
    pub mark_price: Decimal,
    /// 未实现盈亏
    pub unrealized_pnl: Decimal,
    /// 杠杆倍数
    pub leverage: u32,
    /// 保证金模式: "isolated" / "cross"
    pub margin_type: String,
}

/// 持仓列表响应
#[derive(Debug, Clone, Serialize)]
pub struct PositionListResponse {
    /// 持仓列表
    pub positions: Vec<PositionResponse>,
    /// 总数
    pub total: usize,
}
