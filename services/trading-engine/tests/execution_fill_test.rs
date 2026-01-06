//! # 成交闭环集成测试
//!
//! 测试 Execution → 成交 → RiskState 的事实闭环

use std::sync::Arc;

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

use trading_engine::application::service::execution_service::ExecutionService;
use trading_engine::domain::model::execution_fill::{ExecutionFill, FillSide, FillType};
use trading_engine::domain::model::order_intent::{OrderIntent, OrderSide as IntentSide};
use trading_engine::domain::port::order_execution_port::{ExecutionResult, OrderExecutionPort};
use trading_engine::domain::port::risk_state_port::RiskStatePort;
use trading_engine::domain::port::strategy_port::StrategyPort;
use trading_engine::infrastructure::risk::inmemory_risk_state::InMemoryRiskStateAdapter;
use trading_engine::infrastructure::risk::order_risk_adapter::OrderRiskAdapter;
use trading_engine::infrastructure::risk::risk_limits::{OrderRiskConfig, RiskLimits};

use shared::event::market_event::{MarketEvent, MarketEventData, MarketEventType, TradeData};

fn dec(s: &str) -> Decimal {
    s.parse().unwrap_or_default()
}

// ========== Mock Strategy ==========

struct MockStrategy {
    intent: Option<OrderIntent>,
}

impl MockStrategy {
    fn with_intent(intent: OrderIntent) -> Self {
        Self { intent: Some(intent) }
    }

    fn no_signal() -> Self {
        Self { intent: None }
    }
}

#[async_trait]
impl StrategyPort for MockStrategy {
    async fn evaluate(&self, _event: &MarketEvent) -> anyhow::Result<Option<OrderIntent>> {
        Ok(self.intent.clone())
    }
}

// ========== Mock Execution ==========

struct MockExecution {
    should_succeed: bool,
}

impl MockExecution {
    fn success() -> Self {
        Self { should_succeed: true }
    }
}

#[async_trait]
impl OrderExecutionPort for MockExecution {
    async fn execute(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionResult> {
        if self.should_succeed {
            Ok(ExecutionResult {
                success: true,
                order_id: format!("mock_order_{}", Uuid::new_v4()),
                symbol: intent.symbol.clone(),
                error: None,
            })
        } else {
            Ok(ExecutionResult {
                success: false,
                order_id: String::new(),
                symbol: intent.symbol.clone(),
                error: Some("Mock execution failed".to_string()),
            })
        }
    }
}

// ========== 测试辅助函数 ==========

fn create_market_event(symbol: &str, price: Decimal) -> MarketEvent {
    MarketEvent {
        event_type: MarketEventType::Trade,
        exchange: "binance".to_string(),
        symbol: symbol.to_string(),
        timestamp: chrono::Utc::now(),
        data: MarketEventData::Trade(TradeData {
            trade_id: "1".to_string(),
            price,
            quantity: dec("1.0"),
            is_buyer_maker: false,
        }),
    }
}

fn create_buy_intent(symbol: &str, quantity: Decimal, price: Decimal) -> OrderIntent {
    OrderIntent {
        id: Uuid::new_v4(),
        strategy_id: Uuid::new_v4(),
        symbol: symbol.to_string(),
        side: IntentSide::Buy,
        quantity,
        price: Some(price),
        confidence: 0.9,
        created_at: chrono::Utc::now(),
    }
}

fn create_risk_config(allowed_symbols: Vec<String>, max_position: Decimal) -> OrderRiskConfig {
    let mut config = OrderRiskConfig::default();
    config.allowed_symbols = allowed_symbols.into_iter().collect();
    config.trading_enabled = true;
    config.limits = RiskLimits {
        max_order_notional: dec("100000"),
        max_position_per_symbol: max_position,
        max_total_exposure: dec("500000"),
        max_market_order_notional: dec("50000"),
        ..Default::default()
    };
    config
}

// ========== 测试 1: 成交后仓位与余额一致性 ==========

#[tokio::test]
async fn test_fill_updates_position_and_balance_correctly() {
    // 1. 初始化 RiskState（10000 USDT，空仓）
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("10000"), dec("0"));

    // 2. 创建 OrderRiskAdapter
    let config = create_risk_config(vec!["BTCUSDT".to_string()], dec("10"));
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));

    // 3. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy::with_intent(
        create_buy_intent("BTCUSDT", dec("0.1"), dec("50000")),
    ));
    let execution = Arc::new(MockExecution::success());

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter.clone(),
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 4. 执行交易
    let event = create_market_event("BTCUSDT", dec("50000"));
    service.on_market_event(&event).await.unwrap();

    // 5. 验证 RiskState
    let snapshot = risk_state.get_snapshot().await.unwrap();

    // 验证持仓：应该有 0.1 BTC
    let position_qty = snapshot.get_position_qty("BTCUSDT");
    assert_eq!(position_qty, dec("0.1"), "Position should be 0.1 BTC");

    // 验证余额：应该扣减 5000 USDT + 手续费 (5 USDT)
    // 初始 10000 - 5000 (notional) - 5 (commission 0.1%) = 4995
    let free_balance = snapshot.get_free_balance("USDT");
    assert_eq!(free_balance, dec("4995"), "Balance should be 4995 USDT after buy");

    // 验证 open_orders：应该为空（已成交）
    assert_eq!(snapshot.open_orders.len(), 0, "Open orders should be empty after fill");

    println!("✅ 测试通过: 成交后仓位={}, 余额={}", position_qty, free_balance);
}

// ========== 测试 2: 成交后风控生效 ==========

#[tokio::test]
async fn test_risk_rejects_after_position_limit_reached() {
    // 1. 初始化 RiskState（100000 USDT，空仓）
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("100000"), dec("0"));

    // 2. 创建 OrderRiskAdapter（限制单交易对最大持仓 0.15 BTC）
    let config = create_risk_config(vec!["BTCUSDT".to_string()], dec("0.15"));
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));

    // 3. 第一笔交易：买入 0.1 BTC
    let strategy1 = Arc::new(MockStrategy::with_intent(
        create_buy_intent("BTCUSDT", dec("0.1"), dec("50000")),
    ));
    let execution = Arc::new(MockExecution::success());

    let service1 = ExecutionService::with_full_config(
        strategy1,
        risk_adapter.clone(),
        execution.clone(),
        Some(risk_state.clone()),
        None,
        None,
    );

    let event = create_market_event("BTCUSDT", dec("50000"));
    service1.on_market_event(&event).await.unwrap();

    // 验证第一笔成交
    let snapshot = risk_state.get_snapshot().await.unwrap();
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.1"));
    println!("第一笔成交后持仓: {}", snapshot.get_position_qty("BTCUSDT"));

    // 4. 第二笔交易：尝试再买入 0.1 BTC（应该被风控拒绝）
    // 因为 0.1 + 0.1 = 0.2 > 0.15 (max_position_per_symbol)
    let strategy2 = Arc::new(MockStrategy::with_intent(
        create_buy_intent("BTCUSDT", dec("0.1"), dec("50000")),
    ));

    let service2 = ExecutionService::with_full_config(
        strategy2,
        risk_adapter.clone(),
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 执行第二笔交易
    service2.on_market_event(&event).await.unwrap();

    // 5. 验证：持仓应该仍然是 0.1（第二笔被拒绝）
    let snapshot = risk_state.get_snapshot().await.unwrap();
    let final_position = snapshot.get_position_qty("BTCUSDT");
    
    // 由于风控拒绝，持仓应该保持 0.1
    assert_eq!(final_position, dec("0.1"), 
        "Position should remain 0.1 BTC after risk rejection");

    println!("✅ 测试通过: 风控拒绝后持仓保持={}", final_position);
}

// ========== 测试 3: apply_execution_fill 直接调用测试 ==========

#[tokio::test]
async fn test_apply_execution_fill_directly() {
    // 1. 初始化 RiskState
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("10000"), dec("0"));

    // 2. 创建 ExecutionService（只需要 risk_state）
    let strategy = Arc::new(MockStrategy::no_signal());
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution::success());

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 先添加一个 open_order
    risk_state.add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
        order_id: "test_order_001".to_string(),
        symbol: "ETHUSDT".to_string(),
        side: "BUY".to_string(),
        quantity: dec("2.0"),
        price: dec("3000"),
        created_at: chrono::Utc::now().timestamp_millis(),
    }).await;

    // 4. 创建成交事件
    let fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "test_order_001".to_string(),
        trade_id: "trade_001".to_string(),
        client_order_id: None,
        symbol: "ETHUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full,
        filled_quantity: dec("2.0"),
        fill_price: dec("3000"),
        cumulative_quantity: dec("2.0"),
        original_quantity: dec("2.0"),
        commission: dec("6"), // 0.1% of 6000
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    // 5. 应用成交
    service.apply_execution_fill(&fill).await;

    // 6. 验证
    let snapshot = risk_state.get_snapshot().await.unwrap();

    // 持仓应该是 2.0 ETH
    assert_eq!(snapshot.get_position_qty("ETHUSDT"), dec("2.0"));

    // 余额应该是 10000 - 6000 - 6 = 3994 USDT
    assert_eq!(snapshot.get_free_balance("USDT"), dec("3994"));

    // open_orders 应该为空
    assert_eq!(snapshot.open_orders.len(), 0);

    println!("✅ 测试通过: apply_execution_fill 正确更新 RiskState");
}

// ========== 测试 4: 卖出后余额增加 ==========

#[tokio::test]
async fn test_sell_increases_balance() {
    // 1. 初始化 RiskState（有持仓）
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("5000"), dec("0"));
    risk_state.set_position("BTCUSDT", dec("0.2"), dec("50000"));

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy::no_signal());
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution::success());

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 添加卖单
    risk_state.add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
        order_id: "sell_order_001".to_string(),
        symbol: "BTCUSDT".to_string(),
        side: "SELL".to_string(),
        quantity: dec("0.1"),
        price: dec("52000"),
        created_at: chrono::Utc::now().timestamp_millis(),
    }).await;

    // 4. 创建卖出成交事件
    let fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "sell_order_001".to_string(),
        trade_id: "trade_sell_001".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Sell,
        fill_type: FillType::Full,
        filled_quantity: dec("0.1"),
        fill_price: dec("52000"),
        cumulative_quantity: dec("0.1"),
        original_quantity: dec("0.1"),
        commission: dec("5.2"), // 0.1% of 5200
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    // 5. 应用成交
    service.apply_execution_fill(&fill).await;

    // 6. 验证
    let snapshot = risk_state.get_snapshot().await.unwrap();

    // 持仓应该减少到 0.1 BTC
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.1"));

    // 余额应该增加：5000 + 5200 - 5.2 = 10194.8 USDT
    assert_eq!(snapshot.get_free_balance("USDT"), dec("10194.8"));

    // open_orders 应该为空
    assert_eq!(snapshot.open_orders.len(), 0);

    println!("✅ 测试通过: 卖出后余额正确增加");
}

// ========== 测试 5: 重复成交不应重复生效（幂等测试） ==========

#[tokio::test]
async fn test_duplicate_fill_should_not_apply_twice() {
    // 1. 初始化 RiskState（10000 USDT，空仓）
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("10000"), dec("0"));

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy::no_signal());
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution::success());

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 先添加一个 open_order
    risk_state.add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
        order_id: "dup_order_001".to_string(),
        symbol: "BTCUSDT".to_string(),
        side: "BUY".to_string(),
        quantity: dec("0.1"),
        price: dec("50000"),
        created_at: chrono::Utc::now().timestamp_millis(),
    }).await;

    // 4. 创建两个相同 trade_id 的成交事件
    let fill1 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "dup_order_001".to_string(),
        trade_id: "same_trade_id_123".to_string(), // 相同的 trade_id
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full,
        filled_quantity: dec("0.1"),
        fill_price: dec("50000"),
        cumulative_quantity: dec("0.1"),
        original_quantity: dec("0.1"),
        commission: dec("5"), // 0.1% of 5000
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    let fill2 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "dup_order_001".to_string(),
        trade_id: "same_trade_id_123".to_string(), // 相同的 trade_id
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full,
        filled_quantity: dec("0.1"),
        fill_price: dec("50000"),
        cumulative_quantity: dec("0.1"),
        original_quantity: dec("0.1"),
        commission: dec("5"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    // 5. 调用 apply_execution_fill 两次
    service.apply_execution_fill(&fill1).await;
    service.apply_execution_fill(&fill2).await;

    // 6. 验证：仓位只变化一次
    let snapshot = risk_state.get_snapshot().await.unwrap();
    
    // 持仓应该是 0.1 BTC（不是 0.2）
    let position = snapshot.get_position_qty("BTCUSDT");
    assert_eq!(position, dec("0.1"), 
        "Position should be 0.1 BTC (not 0.2), duplicate fill should be ignored");

    // 余额应该是 10000 - 5000 - 5 = 4995 USDT（不是 -5 - 5000 - 5 = -10）
    let balance = snapshot.get_free_balance("USDT");
    assert_eq!(balance, dec("4995"), 
        "Balance should be 4995 USDT, duplicate fill should be ignored");

    // open_orders 应该为空（第一次成交已移除）
    assert_eq!(snapshot.open_orders.len(), 0, 
        "Open orders should be empty after first fill");

    println!("✅ 测试通过: 重复成交（相同 trade_id）不会重复生效");
    println!("   - 仓位: {} (预期 0.1)", position);
    println!("   - 余额: {} (预期 4995)", balance);
}
