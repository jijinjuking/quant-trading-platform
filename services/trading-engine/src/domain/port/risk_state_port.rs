//! # 风控状态端口 (Risk State Port)
//!
//! 路径: services/trading-engine/src/domain/port/risk_state_port.rs
//!
//! ## 职责
//! 为风控适配器提供账户状态查询接口，包括：
//! - 账户余额
//! - 当前持仓
//! - 未完成订单
//!
//! ## 架构说明
//! - ExecutionService 不允许直接访问 ExchangeQueryPort
//! - OrderRiskAdapter 只能依赖 RiskStatePort
//! - 实现位于 infrastructure/risk/

use std::sync::Arc;

use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 账户余额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskBalance {
    /// 资产名称 (USDT, BTC, ETH...)
    pub asset: String,
    /// 可用余额
    pub free: Decimal,
    /// 冻结余额
    pub locked: Decimal,
}

/// 持仓信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskPosition {
    /// 交易对
    pub symbol: String,
    /// 持仓数量（正数多头，负数空头）
    pub quantity: Decimal,
    /// 开仓均价
    pub entry_price: Decimal,
    /// 未实现盈亏
    pub unrealized_pnl: Decimal,
}

/// 未完成订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskOpenOrder {
    /// 订单 ID
    pub order_id: String,
    /// 交易对
    pub symbol: String,
    /// 方向: "BUY" / "SELL"
    pub side: String,
    /// 委托数量
    pub quantity: Decimal,
    /// 委托价格
    pub price: Decimal,
    /// 创建时间（毫秒时间戳）
    pub created_at: i64,
}

/// 风控状态快照
#[derive(Debug, Clone, Default)]
pub struct RiskStateSnapshot {
    /// 账户余额列表
    pub balances: Vec<RiskBalance>,
    /// 持仓列表
    pub positions: Vec<RiskPosition>,
    /// 未完成订单列表
    pub open_orders: Vec<RiskOpenOrder>,
}

impl RiskStateSnapshot {
    /// 获取指定资产的可用余额
    pub fn get_free_balance(&self, asset: &str) -> Decimal {
        self.balances
            .iter()
            .find(|b| b.asset.eq_ignore_ascii_case(asset))
            .map(|b| b.free)
            .unwrap_or(Decimal::ZERO)
    }

    /// 获取指定交易对的持仓数量
    pub fn get_position_qty(&self, symbol: &str) -> Decimal {
        self.positions
            .iter()
            .find(|p| p.symbol.eq_ignore_ascii_case(symbol))
            .map(|p| p.quantity)
            .unwrap_or(Decimal::ZERO)
    }

    /// 获取指定交易对的未完成订单数量
    pub fn get_open_order_count(&self, symbol: &str) -> usize {
        self.open_orders
            .iter()
            .filter(|o| o.symbol.eq_ignore_ascii_case(symbol))
            .count()
    }

    /// 获取指定交易对的未完成买单总量
    pub fn get_pending_buy_qty(&self, symbol: &str) -> Decimal {
        self.open_orders
            .iter()
            .filter(|o| o.symbol.eq_ignore_ascii_case(symbol) && o.side == "BUY")
            .map(|o| o.quantity)
            .sum()
    }

    /// 获取指定交易对的未完成卖单总量
    pub fn get_pending_sell_qty(&self, symbol: &str) -> Decimal {
        self.open_orders
            .iter()
            .filter(|o| o.symbol.eq_ignore_ascii_case(symbol) && o.side == "SELL")
            .map(|o| o.quantity)
            .sum()
    }
}

/// 风控状态端口
///
/// 为风控适配器提供账户状态查询接口。
/// 实现位于 infrastructure/risk/
#[async_trait]
pub trait RiskStatePort: Send + Sync {
    /// 获取当前风控状态快照
    async fn get_snapshot(&self) -> anyhow::Result<RiskStateSnapshot>;

    /// 更新持仓（下单成功后调用）
    ///
    /// # 参数
    /// - `symbol`: 交易对
    /// - `delta`: 持仓变化量（正数加仓，负数减仓）
    /// - `price`: 成交价格
    async fn update_position(&self, symbol: &str, delta: Decimal, price: Decimal);

    /// 添加未完成订单
    async fn add_open_order(&self, order: RiskOpenOrder);

    /// 移除未完成订单
    async fn remove_open_order(&self, order_id: &str);

    /// 更新余额
    async fn update_balance(&self, asset: &str, free: Decimal, locked: Decimal);
}

/// Arc 包装实现
#[async_trait]
impl<T: RiskStatePort> RiskStatePort for Arc<T> {
    async fn get_snapshot(&self) -> anyhow::Result<RiskStateSnapshot> {
        (**self).get_snapshot().await
    }

    async fn update_position(&self, symbol: &str, delta: Decimal, price: Decimal) {
        (**self).update_position(symbol, delta, price).await
    }

    async fn add_open_order(&self, order: RiskOpenOrder) {
        (**self).add_open_order(order).await
    }

    async fn remove_open_order(&self, order_id: &str) {
        (**self).remove_open_order(order_id).await
    }

    async fn update_balance(&self, asset: &str, free: Decimal, locked: Decimal) {
        (**self).update_balance(asset, free, locked).await
    }
}
