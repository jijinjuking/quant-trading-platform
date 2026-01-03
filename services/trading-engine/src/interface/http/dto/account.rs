//! # 账户 DTO
//!
//! 账户相关的请求/响应数据结构。

use rust_decimal::Decimal;
use serde::Serialize;

/// 账户余额响应
#[derive(Debug, Clone, Serialize)]
pub struct BalanceResponse {
    /// 资产名称 (USDT, BTC, ETH...)
    pub asset: String,
    /// 可用余额
    pub free: Decimal,
    /// 冻结余额
    pub locked: Decimal,
    /// 总余额
    pub total: Decimal,
}

/// 账户余额列表响应
#[derive(Debug, Clone, Serialize)]
pub struct BalanceListResponse {
    /// 余额列表
    pub balances: Vec<BalanceResponse>,
    /// 总资产数
    pub total: usize,
}
