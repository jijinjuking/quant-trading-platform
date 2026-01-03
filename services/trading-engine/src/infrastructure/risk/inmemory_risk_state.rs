//! # 内存风控状态适配器 (InMemory Risk State Adapter)
//!
//! 路径: services/trading-engine/src/infrastructure/risk/inmemory_risk_state.rs
//!
//! ## 职责
//! 实现 RiskStatePort，在内存中维护账户状态。
//! 用于测试和开发环境，不依赖数据库或真实交易所。
//!
//! ## 架构位置
//! - 所属层级: Infrastructure Layer
//! - 实现端口: domain/port/risk_state_port.rs

use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::domain::port::risk_state_port::{
    RiskBalance, RiskOpenOrder, RiskPosition, RiskStatePort, RiskStateSnapshot,
};

/// 内存风控状态适配器
pub struct InMemoryRiskStateAdapter {
    /// 账户余额 (asset -> balance)
    balances: RwLock<HashMap<String, RiskBalance>>,
    /// 持仓 (symbol -> position)
    positions: RwLock<HashMap<String, RiskPosition>>,
    /// 未完成订单 (order_id -> order)
    open_orders: RwLock<HashMap<String, RiskOpenOrder>>,
}

impl InMemoryRiskStateAdapter {
    /// 创建空的内存状态
    pub fn new() -> Self {
        Self {
            balances: RwLock::new(HashMap::new()),
            positions: RwLock::new(HashMap::new()),
            open_orders: RwLock::new(HashMap::new()),
        }
    }

    /// 创建带初始余额的内存状态
    pub fn with_balances(initial_balances: Vec<RiskBalance>) -> Self {
        let mut balances = HashMap::new();
        for b in initial_balances {
            balances.insert(b.asset.clone(), b);
        }
        Self {
            balances: RwLock::new(balances),
            positions: RwLock::new(HashMap::new()),
            open_orders: RwLock::new(HashMap::new()),
        }
    }

    /// 设置初始余额（用于测试）
    pub fn set_balance(&self, asset: &str, free: Decimal, locked: Decimal) {
        if let Ok(mut balances) = self.balances.write() {
            balances.insert(
                asset.to_uppercase(),
                RiskBalance {
                    asset: asset.to_uppercase(),
                    free,
                    locked,
                },
            );
        }
    }

    /// 设置初始持仓（用于测试）
    pub fn set_position(&self, symbol: &str, quantity: Decimal, entry_price: Decimal) {
        if let Ok(mut positions) = self.positions.write() {
            positions.insert(
                symbol.to_uppercase(),
                RiskPosition {
                    symbol: symbol.to_uppercase(),
                    quantity,
                    entry_price,
                    unrealized_pnl: Decimal::ZERO,
                },
            );
        }
    }
}

impl Default for InMemoryRiskStateAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiskStatePort for InMemoryRiskStateAdapter {
    async fn get_snapshot(&self) -> anyhow::Result<RiskStateSnapshot> {
        let balances = self
            .balances
            .read()
            .map_err(|e| anyhow::anyhow!("读取余额失败: {}", e))?
            .values()
            .cloned()
            .collect();

        let positions = self
            .positions
            .read()
            .map_err(|e| anyhow::anyhow!("读取持仓失败: {}", e))?
            .values()
            .cloned()
            .collect();

        let open_orders = self
            .open_orders
            .read()
            .map_err(|e| anyhow::anyhow!("读取订单失败: {}", e))?
            .values()
            .cloned()
            .collect();

        Ok(RiskStateSnapshot {
            balances,
            positions,
            open_orders,
        })
    }

    async fn update_position(&self, symbol: &str, delta: Decimal, price: Decimal) {
        if let Ok(mut positions) = self.positions.write() {
            let symbol_upper = symbol.to_uppercase();
            let current = positions.get(&symbol_upper).cloned();

            match current {
                Some(mut pos) => {
                    // 更新现有持仓
                    let old_qty = pos.quantity;
                    let new_qty = old_qty + delta;

                    if new_qty.abs() < Decimal::new(1, 10) {
                        // 持仓接近零，移除
                        positions.remove(&symbol_upper);
                    } else {
                        // 计算新的开仓均价（简化：加权平均）
                        if delta > Decimal::ZERO && old_qty >= Decimal::ZERO {
                            // 加仓
                            let total_cost = old_qty * pos.entry_price + delta * price;
                            pos.entry_price = total_cost / new_qty;
                        }
                        // 减仓不改变均价
                        pos.quantity = new_qty;
                        positions.insert(symbol_upper, pos);
                    }
                }
                None => {
                    // 新建持仓
                    if delta.abs() > Decimal::ZERO {
                        positions.insert(
                            symbol_upper.clone(),
                            RiskPosition {
                                symbol: symbol_upper,
                                quantity: delta,
                                entry_price: price,
                                unrealized_pnl: Decimal::ZERO,
                            },
                        );
                    }
                }
            }
        }
    }

    async fn add_open_order(&self, order: RiskOpenOrder) {
        if let Ok(mut orders) = self.open_orders.write() {
            orders.insert(order.order_id.clone(), order);
        }
    }

    async fn remove_open_order(&self, order_id: &str) {
        if let Ok(mut orders) = self.open_orders.write() {
            orders.remove(order_id);
        }
    }

    async fn update_balance(&self, asset: &str, free: Decimal, locked: Decimal) {
        if let Ok(mut balances) = self.balances.write() {
            let asset_upper = asset.to_uppercase();
            balances.insert(
                asset_upper.clone(),
                RiskBalance {
                    asset: asset_upper,
                    free,
                    locked,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap_or_default()
    }

    #[tokio::test]
    async fn test_empty_snapshot() {
        let adapter = InMemoryRiskStateAdapter::new();
        let snapshot = adapter.get_snapshot().await.unwrap();

        assert!(snapshot.balances.is_empty());
        assert!(snapshot.positions.is_empty());
        assert!(snapshot.open_orders.is_empty());
    }

    #[tokio::test]
    async fn test_set_balance() {
        let adapter = InMemoryRiskStateAdapter::new();
        adapter.set_balance("USDT", dec("10000"), dec("0"));

        let snapshot = adapter.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_free_balance("USDT"), dec("10000"));
    }

    #[tokio::test]
    async fn test_update_position() {
        let adapter = InMemoryRiskStateAdapter::new();

        // 开仓
        adapter.update_position("BTCUSDT", dec("0.1"), dec("50000")).await;
        let snapshot = adapter.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.1"));

        // 加仓
        adapter.update_position("BTCUSDT", dec("0.1"), dec("51000")).await;
        let snapshot = adapter.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.2"));

        // 减仓
        adapter.update_position("BTCUSDT", dec("-0.15"), dec("52000")).await;
        let snapshot = adapter.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.05"));

        // 平仓
        adapter.update_position("BTCUSDT", dec("-0.05"), dec("52000")).await;
        let snapshot = adapter.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0"));
    }

    #[tokio::test]
    async fn test_open_orders() {
        let adapter = InMemoryRiskStateAdapter::new();

        // 添加订单
        adapter
            .add_open_order(RiskOpenOrder {
                order_id: "order1".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "BUY".to_string(),
                quantity: dec("0.1"),
                price: dec("50000"),
                created_at: 0,
            })
            .await;

        let snapshot = adapter.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_open_order_count("BTCUSDT"), 1);
        assert_eq!(snapshot.get_pending_buy_qty("BTCUSDT"), dec("0.1"));

        // 移除订单
        adapter.remove_open_order("order1").await;
        let snapshot = adapter.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_open_order_count("BTCUSDT"), 0);
    }
}
