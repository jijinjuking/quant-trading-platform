//! # 交易所查询端口 (Exchange Query Port)
//!
//! 路径: services/trading-engine/src/domain/port/exchange_query_port.rs
//!
//! ## 职责
//! 定义交易所查询相关的端口接口，包括：
//! - 账户余额查询
//! - 持仓查询
//! - 订单状态查询
//! - 撤单操作
//!
//! ## 架构位置
//! - 所属层级: Domain Layer (Port)
//! - 实现位置: Infrastructure Layer (exchange/)

use anyhow::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 账户余额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    /// 资产名称 (USDT, BTC, ETH...)
    pub asset: String,
    /// 可用余额
    pub free: Decimal,
    /// 冻结余额
    pub locked: Decimal,
}

/// 持仓信息（合约）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
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

/// 订单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeOrder {
    /// 交易所订单 ID
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
    pub status: ExchangeOrderStatus,
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

/// 交易所订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExchangeOrderStatus {
    /// 新订单
    New,
    /// 部分成交
    PartiallyFilled,
    /// 完全成交
    Filled,
    /// 已撤销
    Canceled,
    /// 已拒绝
    Rejected,
    /// 已过期
    Expired,
}

impl ExchangeOrderStatus {
    /// 从字符串解析
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "NEW" => Self::New,
            "PARTIALLY_FILLED" => Self::PartiallyFilled,
            "FILLED" => Self::Filled,
            "CANCELED" => Self::Canceled,
            "REJECTED" => Self::Rejected,
            "EXPIRED" => Self::Expired,
            _ => Self::New,
        }
    }
}

/// 撤单结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderResult {
    /// 订单 ID
    pub order_id: String,
    /// 交易对
    pub symbol: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息（失败时）
    pub error: Option<String>,
}

/// 交易所查询端口
///
/// 定义与交易所交互的查询接口。
/// 实现位于 infrastructure/exchange/
#[async_trait]
pub trait ExchangeQueryPort: Send + Sync {
    /// 查询现货账户余额
    async fn get_spot_balances(&self) -> Result<Vec<AccountBalance>>;

    /// 查询合约持仓
    async fn get_futures_positions(&self) -> Result<Vec<Position>>;

    /// 查询单个订单状态
    async fn get_order(&self, symbol: &str, order_id: &str) -> Result<Option<ExchangeOrder>>;

    /// 查询未完成订单
    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<ExchangeOrder>>;

    /// 撤销订单
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<CancelOrderResult>;

    /// 撤销某交易对所有订单
    async fn cancel_all_orders(&self, symbol: &str) -> Result<Vec<CancelOrderResult>>;
}
