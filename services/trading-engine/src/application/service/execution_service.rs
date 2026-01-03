//! # 交易主链路调度服务 (Execution Service)
//!
//! 这是交易系统的核心调度层。
//! 所有交易流程必须经过此服务，不允许绕过。
//!
//! ## 职责
//! 1. 接收 MarketEvent
//! 2. 调用 StrategyPort → 获取 OrderIntent
//! 3. 调用 OrderRiskPort → 校验 OrderIntent
//! 4. 调用 OrderExecutionPort → 执行 OrderIntent
//! 5. 下单成功后 → 更新风控状态 + 落库 + 审计记录
//!
//! ## 风控状态管理
//! ExecutionService 是唯一允许修改 RiskStatePort 的地方：
//! - 成功下单 → add_open_order
//! - 成交回报 → update_position / update_balance / remove_open_order
//! - 失败或取消 → remove_open_order
//!
//! ## 禁止
//! - 禁止包含任何策略逻辑
//! - 禁止包含任何风控规则
//! - 禁止包含任何执行实现
//! - 禁止直接调用交易所 API

use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use shared::event::market_event::MarketEvent;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use crate::domain::model::order::{Order, OrderSide, OrderStatus, OrderType};
use crate::domain::model::audit_event::{ExecutionResultEvent, RiskRejectedEvent};
use crate::domain::model::execution_fill::{ExecutionFill, FillSide, FillType};
use crate::domain::port::order_execution_port::OrderExecutionPort;
use crate::domain::port::order_repository_port::OrderRepositoryPort;
use crate::domain::port::order_risk_port::OrderRiskPort;
use crate::domain::port::risk_state_port::{RiskStatePort, RiskOpenOrder};
use crate::domain::port::strategy_port::StrategyPort;
use crate::domain::port::trade_audit_port::TradeAuditPort;

/// 交易主链路调度服务
///
/// 统一调度 Strategy → Risk → Execution 的唯一入口。
/// "谁掌权谁负责" - 中枢大脑负责所有后处理工作。
///
/// ## 风控状态管理
/// ExecutionService 是唯一允许修改 RiskStatePort 的地方：
/// - 成功下单 → add_open_order
/// - 成交回报 → update_position / update_balance / remove_open_order
/// - 失败或取消 → remove_open_order
pub struct ExecutionService {
    strategy: Arc<dyn StrategyPort>,
    risk: Arc<dyn OrderRiskPort>,
    execution: Arc<dyn OrderExecutionPort>,
    /// 风控状态端口（用于状态回写）
    risk_state: Option<Arc<dyn RiskStatePort>>,
    /// 订单仓储（可选，用于落库）
    order_repo: Option<Arc<dyn OrderRepositoryPort>>,
    /// 交易审计（可选，用于记录风控决策和执行结果）
    audit: Option<Arc<dyn TradeAuditPort>>,
}

impl ExecutionService {
    /// 创建交易主链路调度服务
    ///
    /// # 参数
    /// - `strategy`: 策略端口
    /// - `risk`: 风控端口
    /// - `execution`: 执行端口
    pub fn new(
        strategy: Arc<dyn StrategyPort>,
        risk: Arc<dyn OrderRiskPort>,
        execution: Arc<dyn OrderExecutionPort>,
    ) -> Self {
        Self {
            strategy,
            risk,
            execution,
            risk_state: None,
            order_repo: None,
            audit: None,
        }
    }

    /// 创建带订单仓储的交易主链路调度服务
    ///
    /// # 参数
    /// - `strategy`: 策略端口
    /// - `risk`: 风控端口
    /// - `execution`: 执行端口
    /// - `order_repo`: 订单仓储端口
    pub fn with_repository(
        strategy: Arc<dyn StrategyPort>,
        risk: Arc<dyn OrderRiskPort>,
        execution: Arc<dyn OrderExecutionPort>,
        order_repo: Arc<dyn OrderRepositoryPort>,
    ) -> Self {
        Self {
            strategy,
            risk,
            execution,
            risk_state: None,
            order_repo: Some(order_repo),
            audit: None,
        }
    }

    /// 创建完整配置的交易主链路调度服务
    ///
    /// # 参数
    /// - `strategy`: 策略端口
    /// - `risk`: 风控端口
    /// - `execution`: 执行端口
    /// - `risk_state`: 风控状态端口（可选，用于状态回写）
    /// - `order_repo`: 订单仓储端口（可选）
    /// - `audit`: 交易审计端口（可选）
    pub fn with_full_config(
        strategy: Arc<dyn StrategyPort>,
        risk: Arc<dyn OrderRiskPort>,
        execution: Arc<dyn OrderExecutionPort>,
        risk_state: Option<Arc<dyn RiskStatePort>>,
        order_repo: Option<Arc<dyn OrderRepositoryPort>>,
        audit: Option<Arc<dyn TradeAuditPort>>,
    ) -> Self {
        Self {
            strategy,
            risk,
            execution,
            risk_state,
            order_repo,
            audit,
        }
    }

    /// 处理行情事件
    ///
    /// 这是交易主链路的唯一入口。
    /// 流程：MarketEvent → Strategy → Risk → Execution → 后处理（落库+更新风控）
    ///
    /// # 参数
    /// - `event`: 行情事件
    ///
    /// # 返回
    /// - `Ok(())`: 处理完成（不代表一定有交易）
    /// - `Err`: 处理失败
    pub async fn on_market_event(&self, event: &MarketEvent) -> anyhow::Result<()> {
        // Step 1: 调用策略，获取交易意图
        let intent = match self.strategy.evaluate(event).await {
            Ok(Some(intent)) => {
                info!(
                    symbol = %intent.symbol,
                    side = ?intent.side,
                    quantity = %intent.quantity,
                    "Strategy generated order intent"
                );
                intent
            }
            Ok(None) => {
                // 策略无交易意图，正常情况
                return Ok(());
            }
            Err(err) => {
                warn!(error = %err, "Strategy evaluation failed");
                return Err(err);
            }
        };

        // Step 2: 调用风控，校验交易意图
        // 明确区分：Strategy None vs Risk Rejected
        if let Err(risk_err) = self.risk.check(&intent).await {
            // 风控拒绝 - 明确记录拒绝原因，不触发 Execution
            let reject_reason = risk_err.to_string();
            let reject_code = Self::extract_reject_code(&reject_reason);
            
            info!(
                symbol = %intent.symbol,
                side = ?intent.side,
                quantity = %intent.quantity,
                reject_reason = %reject_reason,
                reject_code = %reject_code,
                outcome = "RISK_REJECTED",
                "Order intent rejected by risk check - execution skipped"
            );

            // 记录风控拒绝事件（审计）
            if let Some(ref audit) = self.audit {
                let reject_event = RiskRejectedEvent::new(
                    intent.strategy_id,
                    intent.symbol.clone(),
                    intent.side,
                    intent.quantity,
                    intent.price,
                    reject_reason,
                    reject_code,
                );
                if let Err(e) = audit.record_risk_rejected(&reject_event).await {
                    error!(error = %e, "Failed to record risk rejected event");
                }
            }

            // 风控拒绝不是系统错误，只是不执行，返回 Ok
            return Ok(());
        }

        info!(
            symbol = %intent.symbol,
            side = ?intent.side,
            outcome = "RISK_PASSED",
            "Order intent passed risk check, proceeding to execution"
        );

        // Step 3: 调用执行，执行交易意图
        let result = match self.execution.execute(&intent).await {
            Ok(result) => result,
            Err(err) => {
                // 记录执行失败事件
                if let Some(ref audit) = self.audit {
                    let fail_event = ExecutionResultEvent::failure(
                        intent.strategy_id,
                        intent.symbol.clone(),
                        intent.side,
                        intent.quantity,
                        err.to_string(),
                    );
                    if let Err(e) = audit.record_execution_result(&fail_event).await {
                        error!(error = %e, "Failed to record execution failure event");
                    }
                }
                warn!(
                    symbol = %intent.symbol,
                    error = %err,
                    "Order execution error"
                );
                return Err(err);
            }
        };

        // Step 4: 后处理（中枢大脑负责）
        if result.success {
            info!(
                symbol = %result.symbol,
                order_id = %result.order_id,
                "Order executed successfully, starting post-processing"
            );

            // 4.1 更新 RiskStatePort - 添加未完成订单
            let fill_side = match intent.side {
                crate::domain::model::order_intent::OrderSide::Buy => FillSide::Buy,
                crate::domain::model::order_intent::OrderSide::Sell => FillSide::Sell,
            };
            
            if let Some(ref risk_state) = self.risk_state {
                let open_order = RiskOpenOrder {
                    order_id: result.order_id.clone(),
                    symbol: intent.symbol.clone(),
                    side: fill_side.as_str().to_string(),
                    quantity: intent.quantity,
                    price: intent.price.unwrap_or_default(),
                    created_at: Utc::now().timestamp_millis(),
                };
                risk_state.add_open_order(open_order).await;
                debug!(
                    order_id = %result.order_id,
                    symbol = %intent.symbol,
                    "Added open order to RiskStatePort"
                );
            }

            // 4.2 v1 阶段：模拟立即成交（100% 成交）
            // 将来可替换为 WebSocket 成交事件
            let fill_price = intent.price.unwrap_or_else(|| {
                // 市价单使用一个默认价格（实际应从成交回报获取）
                Decimal::ZERO
            });
            
            // 计算模拟手续费（0.1%）
            let commission = intent.quantity * fill_price * Decimal::new(1, 3);
            
            self.simulate_immediate_fill(
                &result.order_id,
                &intent.symbol,
                fill_side,
                intent.quantity,
                fill_price,
                commission,
            ).await;

            // 4.3 更新风控状态 - 下单时间（通过 OrderRiskPort）
            self.risk.record_order_time(&intent.symbol).await;

            // 4.4 记录执行成功事件（审计）
            if let Some(ref audit) = self.audit {
                let success_event = ExecutionResultEvent::success(
                    intent.strategy_id,
                    intent.symbol.clone(),
                    intent.side,
                    intent.quantity,
                    result.order_id.clone(),
                );
                if let Err(e) = audit.record_execution_result(&success_event).await {
                    error!(error = %e, "Failed to record execution success event");
                }
            }

            // 4.5 落库（如果配置了仓储）
            if let Some(ref repo) = self.order_repo {
                // 根据是否有价格判断订单类型
                let order_type = if intent.price.is_some() {
                    OrderType::Limit
                } else {
                    OrderType::Market
                };

                let order = Order {
                    id: Uuid::new_v4(),
                    user_id: intent.strategy_id, // 暂用 strategy_id 作为 user_id
                    strategy_id: Some(intent.strategy_id),
                    symbol: intent.symbol.clone(),
                    side: match intent.side {
                        crate::domain::model::order_intent::OrderSide::Buy => OrderSide::Buy,
                        crate::domain::model::order_intent::OrderSide::Sell => OrderSide::Sell,
                    },
                    order_type,
                    quantity: intent.quantity,
                    price: intent.price,
                    status: OrderStatus::Filled, // 成功执行的订单
                    exchange_order_id: Some(result.order_id.clone()),
                    filled_quantity: intent.quantity,
                    average_price: intent.price,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                if let Err(e) = repo.save_order(&order).await {
                    // 落库失败不影响主流程，只记录错误
                    error!(
                        order_id = %order.id,
                        error = %e,
                        "Failed to save order to database"
                    );
                } else {
                    info!(
                        order_id = %order.id,
                        exchange_order_id = %result.order_id,
                        "Order saved to database"
                    );
                }
            }
        } else {
            // 执行失败 - 如果之前添加了 open_order，需要移除
            // 注意：实际上执行失败时不会有 order_id，这里是防御性编程
            if let Some(ref risk_state) = self.risk_state {
                if !result.order_id.is_empty() {
                    risk_state.remove_open_order(&result.order_id).await;
                    debug!(
                        order_id = %result.order_id,
                        "Removed failed order from RiskStatePort"
                    );
                }
            }

            // 记录执行失败事件
            if let Some(ref audit) = self.audit {
                let fail_event = ExecutionResultEvent::failure(
                    intent.strategy_id,
                    intent.symbol.clone(),
                    intent.side,
                    intent.quantity,
                    result.error.clone().unwrap_or_else(|| "Unknown error".to_string()),
                );
                if let Err(e) = audit.record_execution_result(&fail_event).await {
                    error!(error = %e, "Failed to record execution failure event");
                }
            }
            warn!(
                symbol = %result.symbol,
                error = ?result.error,
                "Order execution failed"
            );
        }

        Ok(())
    }

    /// 从拒绝原因中提取拒绝代码
    fn extract_reject_code(reason: &str) -> String {
        // 简单实现：取第一个冒号前的部分作为代码
        if let Some(pos) = reason.find(':') {
            reason[..pos].trim().to_uppercase().replace(' ', "_")
        } else {
            reason.trim().to_uppercase().replace(' ', "_")
        }
    }

    /// 处理订单成交回报
    ///
    /// 当订单完全成交时调用，更新 RiskStatePort：
    /// - 移除未完成订单
    /// - 更新持仓
    ///
    /// # 参数
    /// - `order_id`: 订单 ID
    /// - `symbol`: 交易对
    /// - `side`: 方向 ("BUY" / "SELL")
    /// - `filled_qty`: 成交数量
    /// - `avg_price`: 成交均价
    pub async fn on_order_filled(
        &self,
        order_id: &str,
        symbol: &str,
        side: &str,
        filled_qty: rust_decimal::Decimal,
        avg_price: rust_decimal::Decimal,
    ) {
        if let Some(ref risk_state) = self.risk_state {
            // 1. 移除未完成订单
            risk_state.remove_open_order(order_id).await;
            
            // 2. 更新持仓
            let delta = if side.eq_ignore_ascii_case("BUY") {
                filled_qty
            } else {
                -filled_qty
            };
            risk_state.update_position(symbol, delta, avg_price).await;
            
            info!(
                order_id = %order_id,
                symbol = %symbol,
                side = %side,
                filled_qty = %filled_qty,
                avg_price = %avg_price,
                "Order filled, RiskStatePort updated"
            );
        }
    }

    /// 处理订单取消
    ///
    /// 当订单被取消时调用，从 RiskStatePort 移除未完成订单。
    ///
    /// # 参数
    /// - `order_id`: 订单 ID
    pub async fn on_order_canceled(&self, order_id: &str) {
        if let Some(ref risk_state) = self.risk_state {
            risk_state.remove_open_order(order_id).await;
            info!(
                order_id = %order_id,
                "Order canceled, removed from RiskStatePort"
            );
        }
    }

    /// 处理订单部分成交
    ///
    /// 当订单部分成交时调用，更新 RiskStatePort 持仓。
    /// 注意：未完成订单不移除，因为还有剩余数量。
    ///
    /// # 参数
    /// - `order_id`: 订单 ID
    /// - `symbol`: 交易对
    /// - `side`: 方向 ("BUY" / "SELL")
    /// - `filled_qty`: 本次成交数量
    /// - `price`: 成交价格
    pub async fn on_order_partially_filled(
        &self,
        _order_id: &str,
        symbol: &str,
        side: &str,
        filled_qty: rust_decimal::Decimal,
        price: rust_decimal::Decimal,
    ) {
        if let Some(ref risk_state) = self.risk_state {
            // 只更新持仓，不移除未完成订单
            let delta = if side.eq_ignore_ascii_case("BUY") {
                filled_qty
            } else {
                -filled_qty
            };
            risk_state.update_position(symbol, delta, price).await;
            
            debug!(
                symbol = %symbol,
                side = %side,
                filled_qty = %filled_qty,
                "Order partially filled, position updated"
            );
        }
    }

    /// 应用成交回报事件
    ///
    /// 这是成交闭环的核心方法。当收到成交事件时调用此方法更新 RiskState。
    /// 
    /// ## 处理顺序（不可乱）
    /// 1. 更新持仓 (update_position)
    /// 2. 更新余额 (update_balance) - 扣减/增加 free balance，应用手续费
    /// 3. 更新未完成订单：
    ///    - PARTIAL: 保留 open_order（剩余数量）
    ///    - FILLED/CANCELED: remove_open_order
    ///
    /// ## 禁止
    /// - 不在这里重新跑风控
    /// - 不访问 ExchangeQueryPort
    /// - 不引入行情价格
    ///
    /// # 参数
    /// - `fill`: 成交回报事件
    pub async fn apply_execution_fill(&self, fill: &ExecutionFill) {
        let Some(ref risk_state) = self.risk_state else {
            warn!("RiskStatePort not configured, skipping fill application");
            return;
        };

        info!(
            order_id = %fill.order_id,
            symbol = %fill.symbol,
            side = ?fill.side,
            filled_qty = %fill.filled_quantity,
            fill_price = %fill.fill_price,
            fill_type = ?fill.fill_type,
            "Applying execution fill to RiskState"
        );

        // Step 1: 更新持仓
        let position_delta = fill.position_delta();
        risk_state
            .update_position(&fill.symbol, position_delta, fill.fill_price)
            .await;
        
        debug!(
            symbol = %fill.symbol,
            delta = %position_delta,
            "Position updated from fill"
        );

        // Step 2: 更新余额
        // 计算成交金额和手续费
        let notional = fill.notional();
        let commission = fill.commission;
        
        // 获取当前余额快照
        if let Ok(snapshot) = risk_state.get_snapshot().await {
            // 假设使用 USDT 作为计价货币
            let quote_asset = "USDT";
            let current_free = snapshot.get_free_balance(quote_asset);
            
            // 计算新余额
            // BUY: 扣减 USDT (notional + commission)
            // SELL: 增加 USDT (notional - commission)
            let new_free = match fill.side {
                FillSide::Buy => current_free - notional - commission,
                FillSide::Sell => current_free + notional - commission,
            };
            
            // 更新余额（locked 保持不变，简化处理）
            let current_locked = snapshot.balances
                .iter()
                .find(|b| b.asset.eq_ignore_ascii_case(quote_asset))
                .map(|b| b.locked)
                .unwrap_or(Decimal::ZERO);
            
            risk_state.update_balance(quote_asset, new_free, current_locked).await;
            
            debug!(
                asset = %quote_asset,
                old_free = %current_free,
                new_free = %new_free,
                notional = %notional,
                commission = %commission,
                "Balance updated from fill"
            );
        }

        // Step 3: 更新未完成订单
        match fill.fill_type {
            FillType::Partial => {
                // 部分成交：保留 open_order，剩余数量由外部管理
                debug!(
                    order_id = %fill.order_id,
                    remaining = %fill.remaining_quantity(),
                    "Partial fill, open order retained"
                );
            }
            FillType::Full => {
                // 全部成交：移除 open_order
                risk_state.remove_open_order(&fill.order_id).await;
                info!(
                    order_id = %fill.order_id,
                    "Full fill, open order removed"
                );
            }
        }

        info!(
            order_id = %fill.order_id,
            symbol = %fill.symbol,
            "Execution fill applied successfully"
        );
    }

    /// 模拟立即成交（v1 阶段使用）
    ///
    /// 在 v1 阶段，下单成功后立即模拟 100% 成交。
    /// 将来可直接替换为 WebSocket 成交事件。
    ///
    /// # 参数
    /// - `order_id`: 订单 ID
    /// - `symbol`: 交易对
    /// - `side`: 方向
    /// - `quantity`: 成交数量
    /// - `price`: 成交价格
    /// - `commission`: 手续费
    pub async fn simulate_immediate_fill(
        &self,
        order_id: &str,
        symbol: &str,
        side: FillSide,
        quantity: Decimal,
        price: Decimal,
        commission: Decimal,
    ) {
        let fill = ExecutionFill {
            id: Uuid::new_v4(),
            order_id: order_id.to_string(),
            client_order_id: None,
            symbol: symbol.to_string(),
            side,
            fill_type: FillType::Full,
            filled_quantity: quantity,
            fill_price: price,
            cumulative_quantity: quantity,
            original_quantity: quantity,
            commission,
            commission_asset: "USDT".to_string(),
            fill_time: Utc::now(),
            created_at: Utc::now(),
        };

        self.apply_execution_fill(&fill).await;
    }
}
