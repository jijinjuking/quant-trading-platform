//! # 风控上下文 (Risk Context)
//!
//! 定义风控检查所需的账户/仓位/订单快照结构。

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// 风控上下文 - 风控检查时的账户快照
#[derive(Debug, Clone)]
pub struct RiskContext {
    /// 账户状态
    pub account: AccountSnapshot,
    /// 当前持仓: symbol → PositionSnapshot
    pub positions: HashMap<String, PositionSnapshot>,
    /// 未完成订单: symbol → Vec<PendingOrder>
    pub pending_orders: HashMap<String, Vec<PendingOrder>>,
    /// 风控统计
    pub risk_stats: RiskStats,
}

impl RiskContext {
    /// 创建空的风控上下文
    pub fn empty() -> Self {
        Self {
            account: AccountSnapshot::default(),
            positions: HashMap::new(),
            pending_orders: HashMap::new(),
            risk_stats: RiskStats::default(),
        }
    }

    /// 获取指定 symbol 的持仓数量（无持仓返回 0）
    pub fn get_position_qty(&self, symbol: &str) -> Decimal {
        self.positions
            .get(&symbol.to_uppercase())
            .map(|p| p.quantity)
            .unwrap_or(Decimal::ZERO)
    }

    /// 获取指定 symbol 的未完成订单
    pub fn get_pending_orders(&self, symbol: &str) -> Vec<&PendingOrder> {
        self.pending_orders
            .get(&symbol.to_uppercase())
            .map(|orders| orders.iter().collect())
            .unwrap_or_default()
    }

    /// 检查是否有同方向的未完成订单
    pub fn has_pending_order_same_side(&self, symbol: &str, side: OrderSide) -> bool {
        self.get_pending_orders(symbol)
            .iter()
            .any(|o| o.side == side)
    }
}

/// 账户快照
#[derive(Debug, Clone, Default)]
pub struct AccountSnapshot {
    /// 账户是否启用交易
    pub trading_enabled: bool,
    /// 可用余额 (USDT)
    pub available_balance: Decimal,
    /// 总余额 (USDT)
    pub total_balance: Decimal,
}

/// 持仓快照
#[derive(Debug, Clone)]
pub struct PositionSnapshot {
    /// 交易对
    pub symbol: String,
    /// 持仓方向
    pub side: OrderSide,
    /// 持仓数量
    pub quantity: Decimal,
}

/// 未完成订单
#[derive(Debug, Clone)]
pub struct PendingOrder {
    /// 订单 ID
    pub order_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 订单方向
    pub side: OrderSide,
    /// 订单数量
    pub quantity: Decimal,
}

/// 风控统计 - 用于熔断判断
#[derive(Debug, Clone, Default)]
pub struct RiskStats {
    /// 连续拒绝次数
    pub consecutive_rejects: u32,
    /// 是否处于熔断状态
    pub circuit_breaker_active: bool,
    /// 熔断解除时间
    pub circuit_breaker_until: Option<DateTime<Utc>>,
}

impl RiskStats {
    /// 记录一次拒绝
    pub fn record_reject(&mut self) {
        self.consecutive_rejects += 1;
    }

    /// 记录一次成功
    pub fn record_success(&mut self) {
        self.consecutive_rejects = 0;
    }

    /// 触发熔断
    pub fn trigger_circuit_breaker(&mut self, duration_secs: i64) {
        self.circuit_breaker_active = true;
        self.circuit_breaker_until = Some(Utc::now() + chrono::Duration::seconds(duration_secs));
    }

    /// 检查熔断是否已解除
    pub fn check_circuit_breaker(&mut self) -> bool {
        if !self.circuit_breaker_active {
            return false;
        }
        if let Some(until) = self.circuit_breaker_until {
            if Utc::now() >= until {
                self.circuit_breaker_active = false;
                self.circuit_breaker_until = None;
                self.consecutive_rejects = 0;
                return false;
            }
        }
        true
    }
}
