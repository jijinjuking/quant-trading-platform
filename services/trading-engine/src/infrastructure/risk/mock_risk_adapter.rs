//! # Mock 风控适配器 (Mock Risk Adapter)
//!
//! 路径: services/trading-engine/src/infrastructure/risk/mock_risk_adapter.rs
//!
//! ## 职责
//! 用于测试的本地风控适配器，不依赖远程服务。
//! 基于 RiskStatePort 实现风控规则。
//!
//! ## 架构说明
//! - 依赖 RiskStatePort 获取账户状态
//! - 不直接访问 ExchangeQueryPort
//! - 不依赖数据库或真实交易所

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use async_trait::async_trait;
use parking_lot::RwLock;
use rust_decimal::Decimal;

use crate::domain::model::order_intent::OrderIntent;
use crate::domain::port::order_risk_port::OrderRiskPort;
use crate::domain::port::risk_state_port::RiskStatePort;

/// Mock 风控配置
#[derive(Debug, Clone)]
pub struct MockRiskConfig {
    /// 允许的交易对（空表示允许所有）
    pub allowed_symbols: HashSet<String>,
    /// 最小数量
    pub min_qty: Decimal,
    /// 最大数量
    pub max_qty: Decimal,
    /// 单交易对最大持仓
    pub max_position_per_symbol: Decimal,
    /// 最大未完成订单数
    pub max_open_orders: usize,
    /// 最小下单间隔（毫秒）
    pub min_order_interval_ms: u64,
    /// 是否启用交易
    pub trading_enabled: bool,
}

impl Default for MockRiskConfig {
    fn default() -> Self {
        Self {
            allowed_symbols: HashSet::new(),
            min_qty: Decimal::ZERO,
            max_qty: Decimal::new(100, 0),
            max_position_per_symbol: Decimal::new(10, 0),
            max_open_orders: 10,
            min_order_interval_ms: 0,
            trading_enabled: true,
        }
    }
}

/// Mock 风控适配器
///
/// 基于 RiskStatePort 实现风控规则。
pub struct MockRiskAdapter {
    config: MockRiskConfig,
    /// 风控状态端口
    risk_state: Arc<dyn RiskStatePort>,
    /// 上次下单时间（用于频率限制）
    last_order_times: RwLock<std::collections::HashMap<String, Instant>>,
}

impl MockRiskAdapter {
    /// 创建 Mock 风控适配器
    pub fn new(config: MockRiskConfig, risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self {
            config,
            risk_state,
            last_order_times: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// 使用默认配置创建
    pub fn with_default_config(risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self::new(MockRiskConfig::default(), risk_state)
    }
}

#[async_trait]
impl OrderRiskPort for MockRiskAdapter {
    async fn check(&self, intent: &OrderIntent) -> Result<()> {
        // 规则 1: 交易是否启用
        if !self.config.trading_enabled {
            anyhow::bail!("TRADING_DISABLED: 交易已禁用");
        }

        // 规则 2: Symbol 白名单
        if !self.config.allowed_symbols.is_empty()
            && !self.config.allowed_symbols.contains(&intent.symbol.to_uppercase())
        {
            anyhow::bail!(
                "SYMBOL_NOT_ALLOWED: 交易对 {} 不在白名单中",
                intent.symbol
            );
        }

        // 规则 3: Symbol 不能为空
        if intent.symbol.is_empty() {
            anyhow::bail!("INVALID_SYMBOL: 交易对不能为空");
        }

        // 规则 4: 数量范围检查
        if intent.quantity <= Decimal::ZERO {
            anyhow::bail!("INVALID_QUANTITY: 数量必须大于 0");
        }
        if intent.quantity < self.config.min_qty {
            anyhow::bail!(
                "QUANTITY_TOO_SMALL: 数量 {} 小于最小值 {}",
                intent.quantity,
                self.config.min_qty
            );
        }
        if intent.quantity > self.config.max_qty {
            anyhow::bail!(
                "QUANTITY_TOO_LARGE: 数量 {} 大于最大值 {}",
                intent.quantity,
                self.config.max_qty
            );
        }

        // 获取风控状态快照
        let snapshot = self.risk_state.get_snapshot().await?;

        // 规则 5: 仓位检查（基于 RiskStatePort）
        let current_position = snapshot.get_position_qty(&intent.symbol);
        let pending_buy = snapshot.get_pending_buy_qty(&intent.symbol);
        let pending_sell = snapshot.get_pending_sell_qty(&intent.symbol);

        let position_delta = match intent.side {
            crate::domain::model::order_intent::OrderSide::Buy => intent.quantity + pending_buy,
            crate::domain::model::order_intent::OrderSide::Sell => -intent.quantity - pending_sell,
        };

        let new_position = current_position + position_delta;
        if new_position.abs() > self.config.max_position_per_symbol {
            anyhow::bail!(
                "POSITION_EXCEEDS_LIMIT: 预计仓位 {} 超过最大限制 {}",
                new_position,
                self.config.max_position_per_symbol
            );
        }

        // 规则 6: 未完成订单数检查
        let open_order_count = snapshot.get_open_order_count(&intent.symbol);
        if open_order_count >= self.config.max_open_orders {
            anyhow::bail!(
                "TOO_MANY_OPEN_ORDERS: 未完成订单数 {} 已达上限 {}",
                open_order_count,
                self.config.max_open_orders
            );
        }

        // 规则 7: 下单频率检查
        if self.config.min_order_interval_ms > 0 {
            let last_times = self.last_order_times.read();
            if let Some(last_time) = last_times.get(&intent.symbol) {
                let elapsed = last_time.elapsed().as_millis() as u64;
                if elapsed < self.config.min_order_interval_ms {
                    anyhow::bail!(
                        "ORDER_TOO_FREQUENT: 距上次下单仅 {}ms，需间隔 {}ms",
                        elapsed,
                        self.config.min_order_interval_ms
                    );
                }
            }
        }

        Ok(())
    }

    async fn update_position(&self, symbol: &str, delta: Decimal) {
        // 委托给 RiskStatePort
        // 注意：这里没有价格信息，使用 0 作为占位
        self.risk_state
            .update_position(symbol, delta, Decimal::ZERO)
            .await;
    }

    async fn record_order_time(&self, symbol: &str) {
        let mut last_times = self.last_order_times.write();
        last_times.insert(symbol.to_string(), Instant::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::order_intent::OrderSide;
    use crate::infrastructure::risk::InMemoryRiskStateAdapter;
    use uuid::Uuid;

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap_or_default()
    }

    fn create_adapter() -> (MockRiskAdapter, Arc<InMemoryRiskStateAdapter>) {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let adapter = MockRiskAdapter::with_default_config(risk_state.clone());
        (adapter, risk_state)
    }

    #[tokio::test]
    async fn test_pass_basic_check() {
        let (adapter, _) = create_adapter();
        let intent = OrderIntent::new(
            Uuid::new_v4(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            dec("0.1"),
            Some(dec("50000")),
            0.9,
        );

        let result = adapter.check(&intent).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reject_empty_symbol() {
        let (adapter, _) = create_adapter();
        let intent = OrderIntent::new(
            Uuid::new_v4(),
            "".to_string(),
            OrderSide::Buy,
            dec("0.1"),
            Some(dec("50000")),
            0.9,
        );

        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("INVALID_SYMBOL"));
    }

    #[tokio::test]
    async fn test_reject_invalid_quantity() {
        let (adapter, _) = create_adapter();
        let intent = OrderIntent::new(
            Uuid::new_v4(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            dec("0"),
            Some(dec("50000")),
            0.9,
        );

        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("INVALID_QUANTITY"));
    }

    #[tokio::test]
    async fn test_reject_position_limit() {
        let config = MockRiskConfig {
            max_position_per_symbol: dec("1"),
            ..Default::default()
        };
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        // 设置初始持仓
        risk_state.set_position("BTCUSDT", dec("0.8"), dec("50000"));

        let adapter = MockRiskAdapter::new(config, risk_state);

        // 尝试加仓超限
        let intent = OrderIntent::new(
            Uuid::new_v4(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            dec("0.3"),
            Some(dec("50000")),
            0.9,
        );

        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("POSITION_EXCEEDS_LIMIT"));
    }

    #[tokio::test]
    async fn test_reject_too_many_open_orders() {
        let config = MockRiskConfig {
            max_open_orders: 2,
            ..Default::default()
        };
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());

        // 添加 2 个未完成订单
        use crate::domain::port::risk_state_port::RiskOpenOrder;
        risk_state
            .add_open_order(RiskOpenOrder {
                order_id: "order1".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "BUY".to_string(),
                quantity: dec("0.1"),
                price: dec("50000"),
                created_at: 0,
            })
            .await;
        risk_state
            .add_open_order(RiskOpenOrder {
                order_id: "order2".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "BUY".to_string(),
                quantity: dec("0.1"),
                price: dec("50000"),
                created_at: 0,
            })
            .await;

        let adapter = MockRiskAdapter::new(config, risk_state);

        let intent = OrderIntent::new(
            Uuid::new_v4(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            dec("0.1"),
            Some(dec("50000")),
            0.9,
        );

        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("TOO_MANY_OPEN_ORDERS"));
    }

    #[tokio::test]
    async fn test_symbol_whitelist() {
        let mut allowed = HashSet::new();
        allowed.insert("ETHUSDT".to_string());

        let config = MockRiskConfig {
            allowed_symbols: allowed,
            ..Default::default()
        };
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let adapter = MockRiskAdapter::new(config, risk_state);

        // BTCUSDT 不在白名单
        let intent = OrderIntent::new(
            Uuid::new_v4(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            dec("0.1"),
            Some(dec("50000")),
            0.9,
        );

        let result = adapter.check(&intent).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SYMBOL_NOT_ALLOWED"));
    }
}
