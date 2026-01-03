//! # 风控状态初始化服务 (Risk State Initializer)
//!
//! 路径: services/trading-engine/src/application/service/risk_state_initializer.rs
//!
//! ## 职责
//! 在 bootstrap 阶段从 ExchangeQueryPort 拉取账户状态，写入 RiskStatePort。
//! 初始化完成后，运行期不允许再直接查询交易所。
//!
//! ## 架构说明
//! - 只在启动时调用一次
//! - 使用 ExchangeQueryPort 获取初始状态
//! - 写入 RiskStatePort
//! - 初始化完成后，ExchangeQueryPort 不再被 ExecutionService 使用

use std::sync::Arc;

use anyhow::{Context, Result};
use tracing::{info, warn, debug};

use crate::domain::port::exchange_query_port::ExchangeQueryPort;
use crate::domain::port::risk_state_port::{RiskOpenOrder, RiskStatePort};

/// 风控状态初始化服务
pub struct RiskStateInitializer;

impl RiskStateInitializer {
    /// 从交易所同步初始状态到 RiskStatePort
    ///
    /// # 参数
    /// - `exchange`: 交易所查询端口
    /// - `risk_state`: 风控状态端口
    ///
    /// # 返回
    /// - `Ok(())`: 初始化成功
    /// - `Err`: 初始化失败（不影响服务启动，但风控状态可能不准确）
    pub async fn initialize(
        exchange: &dyn ExchangeQueryPort,
        risk_state: &dyn RiskStatePort,
    ) -> Result<()> {
        info!("开始初始化风控状态...");

        // 1. 同步现货余额
        let balances_result = Self::sync_balances(exchange, risk_state).await;
        if let Err(ref e) = balances_result {
            warn!(error = %e, "同步余额失败，继续初始化其他状态");
        }

        // 2. 同步合约持仓
        let positions_result = Self::sync_positions(exchange, risk_state).await;
        if let Err(ref e) = positions_result {
            warn!(error = %e, "同步持仓失败，继续初始化其他状态");
        }

        // 3. 同步未完成订单
        let orders_result = Self::sync_open_orders(exchange, risk_state).await;
        if let Err(ref e) = orders_result {
            warn!(error = %e, "同步未完成订单失败");
        }

        // 汇总结果
        let balance_count = balances_result.unwrap_or(0);
        let position_count = positions_result.unwrap_or(0);
        let order_count = orders_result.unwrap_or(0);

        info!(
            balance_count = balance_count,
            position_count = position_count,
            order_count = order_count,
            "风控状态初始化完成"
        );

        Ok(())
    }

    /// 同步余额
    async fn sync_balances(
        exchange: &dyn ExchangeQueryPort,
        risk_state: &dyn RiskStatePort,
    ) -> Result<usize> {
        let balances = exchange
            .get_spot_balances()
            .await
            .context("获取现货余额失败")?;

        let mut count = 0;
        for balance in balances {
            // 只同步有余额的资产
            if balance.free > rust_decimal::Decimal::ZERO
                || balance.locked > rust_decimal::Decimal::ZERO
            {
                risk_state
                    .update_balance(&balance.asset, balance.free, balance.locked)
                    .await;
                debug!(
                    asset = %balance.asset,
                    free = %balance.free,
                    locked = %balance.locked,
                    "同步余额"
                );
                count += 1;
            }
        }

        info!(count = count, "余额同步完成");
        Ok(count)
    }

    /// 同步持仓
    async fn sync_positions(
        exchange: &dyn ExchangeQueryPort,
        risk_state: &dyn RiskStatePort,
    ) -> Result<usize> {
        let positions = exchange
            .get_futures_positions()
            .await
            .context("获取合约持仓失败")?;

        let mut count = 0;
        for pos in positions {
            // 只同步有持仓的交易对
            if pos.quantity.abs() > rust_decimal::Decimal::ZERO {
                // 根据方向确定持仓数量的正负
                let qty = if pos.side.eq_ignore_ascii_case("SHORT") {
                    -pos.quantity.abs()
                } else {
                    pos.quantity.abs()
                };

                // 使用 update_position 设置初始持仓
                // 注意：这里 delta = qty，因为初始状态为 0
                risk_state
                    .update_position(&pos.symbol, qty, pos.entry_price)
                    .await;
                debug!(
                    symbol = %pos.symbol,
                    side = %pos.side,
                    quantity = %qty,
                    entry_price = %pos.entry_price,
                    "同步持仓"
                );
                count += 1;
            }
        }

        info!(count = count, "持仓同步完成");
        Ok(count)
    }

    /// 同步未完成订单
    async fn sync_open_orders(
        exchange: &dyn ExchangeQueryPort,
        risk_state: &dyn RiskStatePort,
    ) -> Result<usize> {
        let orders = exchange
            .get_open_orders(None)
            .await
            .context("获取未完成订单失败")?;

        let mut count = 0;
        for order in orders {
            let open_order = RiskOpenOrder {
                order_id: order.order_id.clone(),
                symbol: order.symbol.clone(),
                side: order.side.clone(),
                quantity: order.quantity - order.executed_qty, // 剩余数量
                price: order.price,
                created_at: order.created_at,
            };

            risk_state.add_open_order(open_order).await;
            debug!(
                order_id = %order.order_id,
                symbol = %order.symbol,
                side = %order.side,
                remaining_qty = %(order.quantity - order.executed_qty),
                "同步未完成订单"
            );
            count += 1;
        }

        info!(count = count, "未完成订单同步完成");
        Ok(count)
    }
}

/// 创建已初始化的 RiskStatePort
///
/// 便捷函数：创建 InMemoryRiskStateAdapter 并从交易所同步初始状态。
///
/// # 参数
/// - `exchange`: 交易所查询端口（可选，None 则跳过初始化）
///
/// # 返回
/// - 已初始化的 RiskStatePort
pub async fn create_initialized_risk_state(
    exchange: Option<&dyn ExchangeQueryPort>,
) -> Arc<dyn RiskStatePort> {
    use crate::infrastructure::risk::InMemoryRiskStateAdapter;

    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());

    if let Some(exchange) = exchange {
        if let Err(e) = RiskStateInitializer::initialize(exchange, risk_state.as_ref()).await {
            warn!(error = %e, "风控状态初始化失败，使用空状态启动");
        }
    } else {
        info!("未配置交易所查询，跳过风控状态初始化");
    }

    risk_state
}
