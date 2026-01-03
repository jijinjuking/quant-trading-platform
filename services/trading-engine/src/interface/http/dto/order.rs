//! # 订单 DTO
//!
//! 订单相关的请求/响应数据结构。

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 订单查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct OrderQueryParams {
    /// 交易对（可选）
    pub symbol: Option<String>,
    /// 订单状态（可选）
    pub status: Option<String>,
    /// 分页: 偏移量
    pub offset: Option<i64>,
    /// 分页: 数量限制
    pub limit: Option<i64>,
}

/// 订单响应
#[derive(Debug, Clone, Serialize)]
pub struct OrderResponse {
    /// 订单 ID
    pub order_id: String,
    /// 客户端订单 ID
    pub client_order_id: Option<String>,
    /// 交易对
    pub symbol: String,
    /// 方向: "BUY" / "SELL"
    pub side: String,
    /// 订单类型: "MARKET" / "LIMIT"
    pub order_type: String,
    /// 订单状态
    pub status: String,
    /// 委托价格
    pub price: Decimal,
    /// 委托数量
    pub quantity: Decimal,
    /// 已成交数量
    pub executed_qty: Decimal,
    /// 成交均价
    pub avg_price: Decimal,
    /// 创建时间（毫秒时间戳）
    pub created_at: i64,
    /// 更新时间（毫秒时间戳）
    pub updated_at: i64,
}

/// 撤单请求
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderRequest {
    /// 交易对
    pub symbol: String,
    /// 订单 ID
    pub order_id: String,
}

/// 撤单响应
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderResponse {
    /// 订单 ID
    pub order_id: String,
    /// 交易对
    pub symbol: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息（失败时）
    pub error: Option<String>,
}

/// 批量撤单请求
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersRequest {
    /// 交易对
    pub symbol: String,
}

/// 订单列表响应
#[derive(Debug, Clone, Serialize)]
pub struct OrderListResponse {
    /// 订单列表
    pub orders: Vec<OrderResponse>,
    /// 总数
    pub total: usize,
}
