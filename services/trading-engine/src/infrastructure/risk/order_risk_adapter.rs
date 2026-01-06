//! # 订单风控适配器 v1 (Order Risk Adapter v1)
//!
//! 路径: services/trading-engine/src/infrastructure/risk/order_risk_adapter.rs
//!
//! ## ⚠️ 版本冻结声明 (Version Freeze Notice)
//!
//! **当前版本: v1 (FROZEN)**
//! **冻结日期: 2026-01-04**
//!
//! 本文件为 OrderRiskAdapter v1 实现，已冻结。
//! - ❌ 禁止在 v1 中继续堆加新规则
//! - ❌ 禁止修改现有规则语义
//! - ✅ 新规则必须通过 OrderRiskAdapter v2 实现
//! - ✅ 仅允许 bug 修复和文档完善
//!
//! ## 职责
//! 实盘级本地风控适配器，基于 RiskStatePort 实现完整风控规则。
//!
//! ## 风控规则 (v1)
//! A. 单笔下单金额上限（基于 balance.free）
//! B. symbol 维度最大仓位限制（含未完成订单）
//! C. 未完成订单总名义敞口限制（⚠️ 仅限制未完成订单，不含持仓市值）
//! D. 市价单保护（⚠️ v1 估算保护，使用固定估算价格 100000）
//!
//! ## v1 限制声明（重要）
//!
//! ### Sell/空头行为约束
//! - v1 不支持裸卖/空头风控校验
//! - Sell 单默认假设调用方已确保有足够的 base position
//! - v2 应实现: 检查 base asset 余额是否足够卖出
//!
//! ### 市价单保护语义
//! - v1 使用固定估算价格（100000 USDT）计算名义金额
//! - 这是保守估算，非真实行情价格
//! - v2 应实现: 接入行情获取真实价格
//!
//! ### 总风险敞口语义
//! - v1 仅计算未完成订单的名义金额之和
//! - 不包含当前持仓的市值
//! - v2 应实现: 持仓市值 + 未完成订单敞口
//!
//! ## 架构约束
//! - 依赖 RiskStatePort 获取账户状态（只读）
//! - 不调用 ExchangeQueryPort
//! - 不引入数据库
//! - 返回结构化 RiskRejectReason
//! - 所有风控数值参数通过 OrderRiskConfig 注入，Adapter 内无 hardcode

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use async_trait::async_trait;
use parking_lot::RwLock;
use rust_decimal::Decimal;
use tracing::{debug, info};

use crate::domain::model::order_intent::{OrderIntent, OrderSide};
use crate::domain::port::order_risk_port::OrderRiskPort;
use crate::domain::port::risk_state_port::{RiskStatePort, RiskStateSnapshot};
use crate::domain::risk::result::{RiskCheckResult, RiskRejectReason};

// 从 risk_limits 模块导入配置结构体
use super::risk_limits::OrderRiskConfig;

/// 订单风控适配器 v1
///
/// 实盘级本地风控适配器，基于 RiskStatePort 实现完整风控规则。
/// 所有风控数值参数通过 OrderRiskConfig 注入。
pub struct OrderRiskAdapter {
    config: OrderRiskConfig,
    risk_state: Arc<dyn RiskStatePort>,
    /// 上次下单时间（用于频率限制）
    last_order_times: RwLock<std::collections::HashMap<String, Instant>>,
}

impl OrderRiskAdapter {
    /// 创建订单风控适配器
    pub fn new(config: OrderRiskConfig, risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self {
            config,
            risk_state,
            last_order_times: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// 从环境变量创建
    pub fn from_env(risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self::new(OrderRiskConfig::from_env(), risk_state)
    }

    /// 使用默认配置创建
    pub fn with_default(risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self::new(OrderRiskConfig::default(), risk_state)
    }

    /// 执行所有风控检查
    async fn check_all_rules(&self, intent: &OrderIntent) -> RiskCheckResult {
        // 获取风控状态快照
        let snapshot = match self.risk_state.get_snapshot().await {
            Ok(s) => s,
            Err(e) => {
                return RiskCheckResult::rejected(RiskRejectReason::Custom {
                    rule_name: "snapshot".to_string(),
                    message: format!("获取风控状态失败: {}", e),
                });
            }
        };

        // 基础检查
        if let result @ RiskCheckResult::Rejected(_) = self.check_basic(intent) {
            return result;
        }

        // 规则 A: 单笔下单金额上限
        if let result @ RiskCheckResult::Rejected(_) = self.check_order_amount(intent, &snapshot) {
            return result;
        }

        // 规则 B: Symbol 维度最大仓位
        if let result @ RiskCheckResult::Rejected(_) = self.check_symbol_position(intent, &snapshot) {
            return result;
        }

        // 规则 C: 账户总风险敞口
        if let result @ RiskCheckResult::Rejected(_) = self.check_total_exposure(intent, &snapshot) {
            return result;
        }

        // 规则 D: 市价单保护
        if let result @ RiskCheckResult::Rejected(_) = self.check_market_order(intent) {
            return result;
        }

        // 规则 E: 强平前安全风控 (v1.1 安全修补)
        if let result @ RiskCheckResult::Rejected(_) = self.check_margin_safety(intent, &snapshot) {
            return result;
        }

        // 频率限制
        if let result @ RiskCheckResult::Rejected(_) = self.check_order_frequency(intent) {
            return result;
        }

        RiskCheckResult::passed()
    }

    /// 基础检查：交易开关、Symbol 白名单、数量有效性
    fn check_basic(&self, intent: &OrderIntent) -> RiskCheckResult {
        // 交易开关
        if !self.config.trading_enabled {
            return RiskCheckResult::rejected(RiskRejectReason::Custom {
                rule_name: "trading_enabled".to_string(),
                message: "交易已禁用".to_string(),
            });
        }

        // Symbol 不能为空
        if intent.symbol.is_empty() {
            return RiskCheckResult::rejected(RiskRejectReason::SymbolNotAllowed {
                symbol: "(empty)".to_string(),
            });
        }

        // Symbol 白名单
        if !self.config.allowed_symbols.is_empty()
            && !self.config.allowed_symbols.contains(&intent.symbol.to_uppercase())
        {
            return RiskCheckResult::rejected(RiskRejectReason::SymbolNotAllowed {
                symbol: intent.symbol.clone(),
            });
        }

        let limits = &self.config.limits;

        // 数量有效性
        if intent.quantity <= Decimal::ZERO {
            return RiskCheckResult::rejected(RiskRejectReason::OrderQuantityTooSmall {
                requested: intent.quantity,
                min_required: limits.min_order_qty,
            });
        }

        if intent.quantity < limits.min_order_qty {
            return RiskCheckResult::rejected(RiskRejectReason::OrderQuantityTooSmall {
                requested: intent.quantity,
                min_required: limits.min_order_qty,
            });
        }

        if intent.quantity > limits.max_order_qty {
            return RiskCheckResult::rejected(RiskRejectReason::OrderQuantityExceeded {
                requested: intent.quantity,
                max_allowed: limits.max_order_qty,
            });
        }

        RiskCheckResult::passed()
    }

    /// 规则 A: 单笔下单金额上限
    fn check_order_amount(&self, intent: &OrderIntent, snapshot: &RiskStateSnapshot) -> RiskCheckResult {
        let limits = &self.config.limits;

        // 计算名义金额
        let price = intent.price.unwrap_or(Decimal::ZERO);
        let notional = intent.quantity * price;

        // 检查单笔名义金额上限
        if notional > limits.max_order_notional {
            return RiskCheckResult::rejected(RiskRejectReason::NotionalValueExceeded {
                notional,
                max_allowed: limits.max_order_notional,
            });
        }

        // 检查余额占用比例（仅对买单）
        if intent.side == OrderSide::Buy && price > Decimal::ZERO {
            let available_balance = snapshot.get_free_balance(&self.config.quote_asset);
            let max_allowed = available_balance * limits.max_balance_usage_ratio;

            if notional > max_allowed && available_balance > Decimal::ZERO {
                return RiskCheckResult::rejected(RiskRejectReason::InsufficientBalance {
                    asset: self.config.quote_asset.clone(),
                    available: available_balance,
                    required: notional,
                });
            }
        }

        RiskCheckResult::passed()
    }

    /// 规则 B: Symbol 维度最大仓位限制
    fn check_symbol_position(&self, intent: &OrderIntent, snapshot: &RiskStateSnapshot) -> RiskCheckResult {
        let limits = &self.config.limits;

        // 当前持仓
        let current_position = snapshot.get_position_qty(&intent.symbol);
        // 未完成买单总量
        let pending_buy = snapshot.get_pending_buy_qty(&intent.symbol);
        // 未完成卖单总量
        let pending_sell = snapshot.get_pending_sell_qty(&intent.symbol);

        // 计算预期持仓（含未完成订单）
        let effective_position = current_position + pending_buy - pending_sell;

        // 计算新订单后的预期持仓
        let new_position = match intent.side {
            OrderSide::Buy => effective_position + intent.quantity,
            OrderSide::Sell => effective_position - intent.quantity,
        };

        // 检查持仓限制（使用绝对值）
        if new_position.abs() > limits.max_position_per_symbol {
            return RiskCheckResult::rejected(RiskRejectReason::PositionLimitExceeded {
                current: effective_position,
                requested: intent.quantity,
                max_allowed: limits.max_position_per_symbol,
            });
        }

        // 检查未完成订单数
        let open_order_count = snapshot.get_open_order_count(&intent.symbol);
        if open_order_count >= limits.max_open_orders_per_symbol {
            return RiskCheckResult::rejected(RiskRejectReason::OpenOrderLimitExceeded {
                current: open_order_count,
                max_allowed: limits.max_open_orders_per_symbol,
            });
        }

        RiskCheckResult::passed()
    }

    /// 规则 C: 账户总名义风险敞口限制
    fn check_total_exposure(&self, intent: &OrderIntent, snapshot: &RiskStateSnapshot) -> RiskCheckResult {
        let limits = &self.config.limits;

        // v1: 仅计算未完成订单的名义金额（不含持仓市值）
        let current_exposure: Decimal = snapshot
            .open_orders
            .iter()
            .map(|o| o.quantity * o.price)
            .sum();

        // 新订单的名义金额
        let order_notional = intent.quantity * intent.price.unwrap_or(Decimal::ZERO);

        // 新总敞口
        let new_exposure = current_exposure + order_notional;

        if new_exposure > limits.max_total_exposure {
            return RiskCheckResult::rejected(RiskRejectReason::NotionalValueExceeded {
                notional: new_exposure,
                max_allowed: limits.max_total_exposure,
            });
        }

        // 检查全局未完成订单数
        let total_open_orders = snapshot.open_orders.len();
        if total_open_orders >= limits.max_total_open_orders {
            return RiskCheckResult::rejected(RiskRejectReason::OpenOrderLimitExceeded {
                current: total_open_orders,
                max_allowed: limits.max_total_open_orders,
            });
        }

        RiskCheckResult::passed()
    }

    /// 规则 D: 市价单保护
    ///
    /// ⚠️ v1 语义说明：
    /// - 市价单无法精确计算名义金额（无委托价格）
    /// - v1 使用固定估算价格 100000 USDT 作为保守估算
    /// - v2 应实现: 接入行情服务获取真实价格
    fn check_market_order(&self, intent: &OrderIntent) -> RiskCheckResult {
        let limits = &self.config.limits;

        // 市价单判断：price 为 None
        if intent.price.is_none() {
            // v1 估算逻辑：使用固定价格 100000 USDT
            const V1_ESTIMATED_PRICE: i64 = 100000;
            let estimated_notional = intent.quantity * Decimal::new(V1_ESTIMATED_PRICE, 0);

            if estimated_notional > limits.max_market_order_notional {
                return RiskCheckResult::rejected(RiskRejectReason::MarketOrderNotionalExceeded {
                    symbol: intent.symbol.clone(),
                    estimated_notional,
                    max_allowed: limits.max_market_order_notional,
                });
            }
        }

        RiskCheckResult::passed()
    }

    /// 规则 E: 强平前安全风控 (v1.1 安全修补)
    ///
    /// ⚠️ v1.1 语义说明：
    /// - 当保证金率低于临界阈值时，禁止新开仓，允许减仓/平仓
    /// - v1 使用近似计算: margin_ratio ≈ free_balance / total_position_value
    /// - 如果没有持仓，跳过此检查
    /// - 只对开仓方向的订单进行检查
    ///
    /// ## 开仓判断逻辑
    /// - 当前无持仓: Buy = 开仓, Sell = 开仓
    /// - 当前多头持仓: Buy = 加仓(开仓), Sell = 减仓(允许)
    /// - 当前空头持仓: Buy = 减仓(允许), Sell = 加仓(开仓)
    fn check_margin_safety(&self, intent: &OrderIntent, snapshot: &RiskStateSnapshot) -> RiskCheckResult {
        let limits = &self.config.limits;

        // 如果临界阈值为 0，跳过检查
        if limits.critical_margin_ratio <= Decimal::ZERO {
            return RiskCheckResult::passed();
        }

        // 计算总持仓价值（使用 entry_price 近似）
        let total_position_value: Decimal = snapshot
            .positions
            .iter()
            .map(|p| p.quantity.abs() * p.entry_price)
            .sum();

        // 如果没有持仓，跳过检查（无强平风险）
        if total_position_value <= Decimal::ZERO {
            return RiskCheckResult::passed();
        }

        // 获取可用余额
        let free_balance = snapshot.get_free_balance(&self.config.quote_asset);

        // 计算近似保证金率
        // margin_ratio = free_balance / total_position_value
        let margin_ratio = if total_position_value > Decimal::ZERO {
            free_balance / total_position_value
        } else {
            Decimal::new(1, 0) // 无持仓时，保证金率视为 100%
        };

        // 如果保证金率高于临界阈值，通过
        if margin_ratio >= limits.critical_margin_ratio {
            return RiskCheckResult::passed();
        }

        // 保证金率过低，检查是否为开仓操作
        // 获取当前交易对的持仓
        let current_position = snapshot.get_position_qty(&intent.symbol);

        // 判断是否为开仓操作
        let is_opening_position = match intent.side {
            OrderSide::Buy => {
                // 买入: 如果当前无持仓或多头，则为开仓/加仓
                current_position >= Decimal::ZERO
            }
            OrderSide::Sell => {
                // 卖出: 如果当前无持仓或空头，则为开仓/加仓
                current_position <= Decimal::ZERO
            }
        };

        // 如果是开仓操作，拒绝
        if is_opening_position {
            return RiskCheckResult::rejected(RiskRejectReason::MarginRatioTooLow {
                current_ratio: margin_ratio,
                critical_threshold: limits.critical_margin_ratio,
            });
        }

        // 减仓/平仓操作，允许通过
        RiskCheckResult::passed()
    }

    /// 频率限制检查
    fn check_order_frequency(&self, intent: &OrderIntent) -> RiskCheckResult {
        let limits = &self.config.limits;

        if limits.min_order_interval_ms == 0 {
            return RiskCheckResult::passed();
        }

        let last_times = self.last_order_times.read();
        if let Some(last_time) = last_times.get(&intent.symbol) {
            let elapsed = last_time.elapsed().as_millis() as u64;
            if elapsed < limits.min_order_interval_ms {
                return RiskCheckResult::rejected(RiskRejectReason::CooldownNotExpired {
                    remaining_seconds: (limits.min_order_interval_ms - elapsed) / 1000,
                });
            }
        }

        RiskCheckResult::passed()
    }
}

#[async_trait]
impl OrderRiskPort for OrderRiskAdapter {
    async fn check(&self, intent: &OrderIntent) -> Result<()> {
        debug!(
            symbol = %intent.symbol,
            side = ?intent.side,
            quantity = %intent.quantity,
            price = ?intent.price,
            "开始风控检查"
        );

        let result = self.check_all_rules(intent).await;

        match &result {
            RiskCheckResult::Passed => {
                debug!(symbol = %intent.symbol, "风控检查通过");
                // ⚠️ v1 频率限制生效保证：检查通过时立即记录下单时间
                self.record_order_time(&intent.symbol).await;
                Ok(())
            }
            RiskCheckResult::Rejected(reason) => {
                info!(
                    symbol = %intent.symbol,
                    code = %reason.code(),
                    message = %reason.message(),
                    "风控检查拒绝"
                );
                Err(anyhow::anyhow!("{}: {}", reason.code(), reason.message()))
            }
        }
    }

    async fn update_position(&self, symbol: &str, delta: Decimal) {
        self.risk_state
            .update_position(symbol, delta, Decimal::ZERO)
            .await;
    }

    async fn record_order_time(&self, symbol: &str) {
        let mut last_times = self.last_order_times.write();
        last_times.insert(symbol.to_string(), Instant::now());
    }
}


// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::risk::InMemoryRiskStateAdapter;
    use crate::infrastructure::risk::risk_limits::RiskLimits;
    use crate::domain::port::risk_state_port::RiskOpenOrder;
    use std::collections::HashSet;
    use uuid::Uuid;

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap_or_default()
    }

    fn create_adapter(config: OrderRiskConfig) -> (OrderRiskAdapter, Arc<InMemoryRiskStateAdapter>) {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let adapter = OrderRiskAdapter::new(config, risk_state.clone());
        (adapter, risk_state)
    }

    fn create_intent(symbol: &str, side: OrderSide, qty: &str, price: Option<&str>) -> OrderIntent {
        OrderIntent::new(
            Uuid::new_v4(),
            symbol.to_string(),
            side,
            dec(qty),
            price.map(dec),
            0.9,
        )
    }

    // === 基础检查测试 ===

    #[tokio::test]
    async fn test_pass_basic_check() {
        let (adapter, _) = create_adapter(OrderRiskConfig::default());
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000"));
        assert!(adapter.check(&intent).await.is_ok());
    }

    #[tokio::test]
    async fn test_reject_trading_disabled() {
        let config = OrderRiskConfig {
            trading_enabled: false,
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("交易已禁用"));
    }

    #[tokio::test]
    async fn test_reject_symbol_not_allowed() {
        let mut allowed = HashSet::new();
        allowed.insert("ETHUSDT".to_string());
        let config = OrderRiskConfig {
            allowed_symbols: allowed,
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SYMBOL_NOT_ALLOWED"));
    }

    #[tokio::test]
    async fn test_reject_quantity_too_small() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                min_order_qty: dec("0.001"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.0001", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ORDER_QTY_TOO_SMALL"));
    }

    #[tokio::test]
    async fn test_reject_quantity_too_large() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_order_qty: dec("1"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "2", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ORDER_QTY_EXCEEDED"));
    }

    // === 规则 A: 单笔下单金额上限测试 ===

    #[tokio::test]
    async fn test_reject_notional_exceeded() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_order_notional: dec("1000"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.1", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("NOTIONAL_EXCEEDED"));
    }

    #[tokio::test]
    async fn test_reject_insufficient_balance() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_balance_usage_ratio: dec("0.5"),
                ..Default::default()
            },
            quote_asset: "USDT".to_string(),
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.set_balance("USDT", dec("1000"), dec("0"));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.02", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("INSUFFICIENT_BALANCE"));
    }

    #[tokio::test]
    async fn test_pass_balance_check_for_sell() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_balance_usage_ratio: dec("0.5"),
                ..Default::default()
            },
            quote_asset: "USDT".to_string(),
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.set_balance("USDT", dec("100"), dec("0"));
        let intent = create_intent("BTCUSDT", OrderSide::Sell, "0.02", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_ok());
    }

    // === 规则 B: Symbol 维度最大仓位测试 ===

    #[tokio::test]
    async fn test_reject_position_limit_exceeded() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_position_per_symbol: dec("1"),
                max_order_notional: dec("100000"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.set_position("BTCUSDT", dec("0.8"), dec("50000"));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.5", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err(), "应该被拒绝");
        assert!(result.unwrap_err().to_string().contains("POSITION_LIMIT_EXCEEDED"));
    }

    #[tokio::test]
    async fn test_pass_position_within_limit() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_position_per_symbol: dec("1"),
                max_order_notional: dec("100000"),
                critical_margin_ratio: dec("0"), // 禁用保证金率检查
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.set_balance("USDT", dec("100000"), dec("0")); // 设置足够余额
        risk_state.set_position("BTCUSDT", dec("0.5"), dec("50000"));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.3", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_ok(), "应该通过");
    }

    #[tokio::test]
    async fn test_reject_open_orders_per_symbol_exceeded() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_open_orders_per_symbol: 2,
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.add_open_order(RiskOpenOrder {
            order_id: "order1".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("0.01"),
            price: dec("50000"),
            created_at: 0,
        }).await;
        risk_state.add_open_order(RiskOpenOrder {
            order_id: "order2".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("0.01"),
            price: dec("50000"),
            created_at: 0,
        }).await;
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("OPEN_ORDER_LIMIT_EXCEEDED"));
    }

    // === 规则 C: 账户总风险敞口测试 ===

    #[tokio::test]
    async fn test_reject_total_exposure_exceeded() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_total_exposure: dec("10000"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.add_open_order(RiskOpenOrder {
            order_id: "order1".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("0.16"),
            price: dec("50000"),
            created_at: 0,
        }).await;
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.1", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("NOTIONAL_EXCEEDED"));
    }

    #[tokio::test]
    async fn test_reject_total_open_orders_exceeded() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_total_open_orders: 2,
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.add_open_order(RiskOpenOrder {
            order_id: "order1".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("0.01"),
            price: dec("50000"),
            created_at: 0,
        }).await;
        risk_state.add_open_order(RiskOpenOrder {
            order_id: "order2".to_string(),
            symbol: "ETHUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("0.1"),
            price: dec("3000"),
            created_at: 0,
        }).await;
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000"));
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("OPEN_ORDER_LIMIT_EXCEEDED"));
    }

    // === 规则 D: 市价单保护测试 ===

    #[tokio::test]
    async fn test_reject_market_order_too_large() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_market_order_notional: dec("1000"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.1", None);
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MARKET_ORDER_NOTIONAL_EXCEEDED"));
    }

    #[tokio::test]
    async fn test_pass_limit_order_with_large_notional() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_market_order_notional: dec("1000"),
                max_order_notional: dec("100000"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.1", Some("50000"));
        assert!(adapter.check(&intent).await.is_ok());
    }

    // === 频率限制测试 ===

    #[tokio::test]
    async fn test_pass_frequency_check_disabled() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                min_order_interval_ms: 0,
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, _) = create_adapter(config);
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000"));
        assert!(adapter.check(&intent).await.is_ok());
        assert!(adapter.check(&intent).await.is_ok());
    }

    // === 综合测试 ===

    #[tokio::test]
    async fn test_multiple_rules_pass() {
        let config = OrderRiskConfig {
            limits: RiskLimits {
                max_order_qty: dec("1"),
                max_order_notional: dec("100000"),
                max_position_per_symbol: dec("10"),
                max_total_exposure: dec("1000000"),
                max_open_orders_per_symbol: 10,
                max_total_open_orders: 50,
                min_order_interval_ms: 0,
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.set_balance("USDT", dec("100000"), dec("0"));
        risk_state.set_position("BTCUSDT", dec("1"), dec("50000"));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.5", Some("50000"));
        assert!(adapter.check(&intent).await.is_ok());
    }

    #[tokio::test]
    async fn test_config_from_env() {
        let config = OrderRiskConfig::from_env();
        assert!(config.trading_enabled);
        assert!(config.limits.min_order_qty > Decimal::ZERO);
    }

    // === 规则 E: 强平前安全风控测试 (v1.1 安全修补) ===

    #[tokio::test]
    async fn test_margin_safety_pass_no_position() {
        // 无持仓时，跳过保证金率检查
        let config = OrderRiskConfig {
            limits: RiskLimits {
                critical_margin_ratio: dec("0.1"),
                max_order_notional: dec("100000"),
                max_balance_usage_ratio: dec("1.0"), // 允许使用 100% 余额
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.set_balance("USDT", dec("10000"), dec("0")); // 足够的余额
        // 无持仓
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000")); // 500 USDT
        assert!(adapter.check(&intent).await.is_ok());
    }

    #[tokio::test]
    async fn test_margin_safety_pass_high_margin_ratio() {
        // 保证金率高于临界阈值，允许开仓
        let config = OrderRiskConfig {
            limits: RiskLimits {
                critical_margin_ratio: dec("0.1"), // 10%
                max_order_notional: dec("100000"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        // 余额 1000 USDT，持仓价值 5000 USDT，保证金率 = 1000/5000 = 20% > 10%
        risk_state.set_balance("USDT", dec("1000"), dec("0"));
        risk_state.set_position("BTCUSDT", dec("0.1"), dec("50000")); // 0.1 * 50000 = 5000
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000"));
        assert!(adapter.check(&intent).await.is_ok());
    }

    #[tokio::test]
    async fn test_margin_safety_reject_low_margin_ratio_buy() {
        // 保证金率低于临界阈值，禁止开仓（买入加仓）
        let config = OrderRiskConfig {
            limits: RiskLimits {
                critical_margin_ratio: dec("0.1"), // 10%
                max_order_notional: dec("100000"),
                max_balance_usage_ratio: dec("1.0"), // 允许使用 100% 余额
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        // 余额 100 USDT，持仓价值 5000 USDT，保证金率 = 100/5000 = 2% < 10%
        // 订单金额 = 0.01 * 50000 = 500 USDT，需要余额 >= 500
        risk_state.set_balance("USDT", dec("600"), dec("0")); // 足够支付订单，但保证金率仍然低
        risk_state.set_position("BTCUSDT", dec("0.1"), dec("50000")); // 0.1 * 50000 = 5000
        // 保证金率 = 600 / 5000 = 12% > 10%，需要调整使其低于阈值
        // 改为: 余额 400 USDT，持仓价值 5000 USDT，保证金率 = 400/5000 = 8% < 10%
        risk_state.set_balance("USDT", dec("400"), dec("0"));
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.001", Some("50000")); // 50 USDT 订单
        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MARGIN_RATIO_TOO_LOW"));
    }

    #[tokio::test]
    async fn test_margin_safety_allow_close_position() {
        // 保证金率低于临界阈值，但允许减仓/平仓
        let config = OrderRiskConfig {
            limits: RiskLimits {
                critical_margin_ratio: dec("0.1"), // 10%
                max_order_notional: dec("100000"),
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        // 余额 100 USDT，持仓价值 5000 USDT，保证金率 = 100/5000 = 2% < 10%
        risk_state.set_balance("USDT", dec("100"), dec("0"));
        risk_state.set_position("BTCUSDT", dec("0.1"), dec("50000")); // 多头持仓
        // 卖出减仓，应该允许
        let intent = create_intent("BTCUSDT", OrderSide::Sell, "0.05", Some("50000"));
        assert!(adapter.check(&intent).await.is_ok());
    }

    #[tokio::test]
    async fn test_margin_safety_disabled_when_threshold_zero() {
        // 临界阈值为 0 时，跳过检查
        let config = OrderRiskConfig {
            limits: RiskLimits {
                critical_margin_ratio: dec("0"), // 禁用
                max_order_notional: dec("100000"),
                max_balance_usage_ratio: dec("1.0"), // 允许使用 100% 余额
                ..Default::default()
            },
            ..Default::default()
        };
        let (adapter, risk_state) = create_adapter(config);
        risk_state.set_balance("USDT", dec("1000"), dec("0")); // 足够的余额
        risk_state.set_position("BTCUSDT", dec("1"), dec("50000")); // 大持仓
        let intent = create_intent("BTCUSDT", OrderSide::Buy, "0.01", Some("50000")); // 500 USDT
        assert!(adapter.check(&intent).await.is_ok());
    }
}
