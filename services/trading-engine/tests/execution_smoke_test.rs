//! # Execution 主链路 Smoke Test
//!
//! 验证 Port + Adapter + Service 能跑通。
//!
//! ## 测试范围
//! - ExecutionPort trait 可被实现
//! - NoopExecution 能正常执行
//! - ExecutionService 能调度完整链路
//! - 风控拦截验证（Symbol 白名单、仓位超限）

use std::collections::HashSet;
use std::sync::Arc;

// 导入 trading-engine 模块
use trading_engine::domain::port::execution_port::{ExecutionCommand, ExecutionPort};
use trading_engine::domain::port::strategy_port::StrategyPort;
use trading_engine::domain::port::order_risk_port::OrderRiskPort;
use trading_engine::domain::port::order_execution_port::OrderExecutionPort;
use trading_engine::domain::port::risk_state_port::RiskStatePort;
use trading_engine::domain::model::order_intent::{OrderIntent, OrderSide};
use trading_engine::infrastructure::execution::{NoopExecution, OrderExecutor};
use trading_engine::infrastructure::strategy::NoopStrategy;
use trading_engine::infrastructure::risk::{MockRiskAdapter, MockRiskConfig, InMemoryRiskStateAdapter};
use trading_engine::application::service::execution_service::ExecutionService;

use rust_decimal::Decimal;
use std::str::FromStr;
use shared::event::market_event::{MarketEvent, MarketEventType, MarketEventData, TradeData};
use chrono::Utc;
use uuid::Uuid;

/// 辅助函数：创建 Decimal
fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap_or_default()
}

/// Test 1: ExecutionPort + NoopExecution 基础验证
#[tokio::test]
async fn test_noop_execution_port() {
    let executor = NoopExecution::new();
    
    let command = ExecutionCommand {
        symbol: "BTCUSDT".to_string(),
        side: "buy".to_string(),
        quantity: "0.001".to_string(),
    };
    
    let result = executor.execute(&command).await;
    
    // NoopExecution 应该始终成功
    assert!(result.is_ok(), "NoopExecution should always succeed");
}

/// Test 2: OrderExecutor 适配器验证
#[tokio::test]
async fn test_order_executor_adapter() {
    let inner: Arc<dyn ExecutionPort> = Arc::new(NoopExecution::new());
    let executor = OrderExecutor::new(inner);
    
    let intent = OrderIntent::new(
        Uuid::new_v4(),
        "ETHUSDT".to_string(),
        OrderSide::Sell,
        dec("0.1"),
        Some(dec("2000.0")),
        0.8,
    );
    
    let result = executor.execute(&intent).await;
    
    assert!(result.is_ok(), "OrderExecutor should succeed with NoopExecution");
    let exec_result = result.unwrap();
    assert!(exec_result.success, "Execution result should be success");
    assert_eq!(exec_result.symbol, "ETHUSDT");
}

/// Test 3: ExecutionService 完整链路验证
#[tokio::test]
async fn test_execution_service_full_chain() {
    // 1. 创建所有依赖
    let strategy: Arc<dyn StrategyPort> = Arc::new(NoopStrategy::new());
    
    // 使用默认配置（允许所有交易对）+ InMemoryRiskStateAdapter
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    let risk: Arc<dyn OrderRiskPort> = Arc::new(
        MockRiskAdapter::new(MockRiskConfig::default(), risk_state.clone())
    );
    
    let inner_exec: Arc<dyn ExecutionPort> = Arc::new(NoopExecution::new());
    let execution: Arc<dyn OrderExecutionPort> = Arc::new(OrderExecutor::new(inner_exec));
    
    // 2. 创建 ExecutionService（带 RiskStatePort）
    let service = ExecutionService::with_full_config(
        strategy, 
        risk, 
        execution, 
        Some(risk_state as Arc<dyn RiskStatePort>),  // 传入 RiskStatePort
        None, 
        None
    );
    
    // 3. 构造 MarketEvent
    let event = MarketEvent {
        event_type: MarketEventType::Trade,
        exchange: "binance".to_string(),
        symbol: "BTCUSDT".to_string(),
        timestamp: Utc::now(),
        data: MarketEventData::Trade(TradeData {
            trade_id: "12345".to_string(),
            price: dec("50000.0"),
            quantity: dec("0.001"),
            is_buyer_maker: false,
        }),
    };
    
    // 4. 调用主链路
    let result = service.on_market_event(&event).await;
    
    // NoopStrategy 不产生意图，所以链路应该正常结束
    assert!(result.is_ok(), "ExecutionService should handle event without error");
}

/// Test 4: 风控拦截验证 - Symbol 白名单
#[tokio::test]
async fn test_risk_rejection_symbol_whitelist() {
    // 创建一个只允许 BTCUSDT 的风控配置
    let mut allowed_symbols = HashSet::new();
    allowed_symbols.insert("BTCUSDT".to_string());
    
    let config = MockRiskConfig {
        allowed_symbols,
        min_qty: Decimal::ZERO,
        max_qty: Decimal::new(100, 0),
        max_position_per_symbol: Decimal::new(10, 0),
        max_open_orders: 10,
        min_order_interval_ms: 0,
        trading_enabled: true,
    };
    
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    let risk = MockRiskAdapter::new(config, risk_state);
    
    // 创建一个 ETHUSDT 的意图
    let intent = OrderIntent::new(
        Uuid::new_v4(),
        "ETHUSDT".to_string(), // 不在白名单
        OrderSide::Buy,
        dec("0.1"),
        Some(dec("2000.0")),
        0.9,
    );
    
    let result = risk.check(&intent).await;
    
    // 应该被拒绝
    assert!(result.is_err(), "Risk should reject non-whitelisted symbol");
}

/// Test 5: 风控拦截验证 - 仓位超限
#[tokio::test]
async fn test_risk_rejection_position_limit() {
    // 创建一个仓位限制为 1 的风控配置
    let config = MockRiskConfig {
        allowed_symbols: HashSet::new(), // 允许所有
        min_qty: Decimal::ZERO,
        max_qty: Decimal::new(100, 0),
        max_position_per_symbol: Decimal::new(1, 0), // 最大仓位 1
        max_open_orders: 10,
        min_order_interval_ms: 0,
        trading_enabled: true,
    };
    
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    // 设置初始持仓为 0.5
    risk_state.set_position("BTCUSDT", dec("0.5"), dec("50000"));
    
    let risk = MockRiskAdapter::new(config, risk_state);
    
    // 第一笔订单：买入 0.4，总仓位将达到 0.9，应该通过
    let intent1 = OrderIntent::new(
        Uuid::new_v4(),
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        dec("0.4"),
        Some(dec("50000.0")),
        0.9,
    );
    
    let result1 = risk.check(&intent1).await;
    assert!(result1.is_ok(), "First order should pass (0.5 + 0.4 = 0.9 < 1.0 limit)");
    
    // 第二笔订单：买入 0.6，总仓位将达到 1.1，应该被拒绝
    let intent2 = OrderIntent::new(
        Uuid::new_v4(),
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        dec("0.6"),
        Some(dec("50000.0")),
        0.9,
    );
    
    let result2 = risk.check(&intent2).await;
    assert!(result2.is_err(), "Second order should be rejected (0.5 + 0.6 = 1.1 > 1.0 limit)");
    
    // 验证错误信息包含仓位超限
    let err_msg = result2.unwrap_err().to_string();
    assert!(
        err_msg.contains("POSITION_EXCEEDS_LIMIT"),
        "Error should indicate position limit exceeded, got: {}",
        err_msg
    );
}

/// Test 6: 风控基于 RiskStatePort 的未完成订单检查
#[tokio::test]
async fn test_risk_open_orders_limit() {
    use trading_engine::domain::port::risk_state_port::RiskOpenOrder;
    
    let config = MockRiskConfig {
        allowed_symbols: HashSet::new(),
        min_qty: Decimal::ZERO,
        max_qty: Decimal::new(100, 0),
        max_position_per_symbol: Decimal::new(10, 0),
        max_open_orders: 2, // 最多 2 个未完成订单
        min_order_interval_ms: 0,
        trading_enabled: true,
    };
    
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    
    // 添加 2 个未完成订单
    risk_state.add_open_order(RiskOpenOrder {
        order_id: "order1".to_string(),
        symbol: "BTCUSDT".to_string(),
        side: "BUY".to_string(),
        quantity: dec("0.1"),
        price: dec("50000"),
        created_at: 0,
    }).await;
    
    risk_state.add_open_order(RiskOpenOrder {
        order_id: "order2".to_string(),
        symbol: "BTCUSDT".to_string(),
        side: "BUY".to_string(),
        quantity: dec("0.1"),
        price: dec("50000"),
        created_at: 0,
    }).await;
    
    let risk = MockRiskAdapter::new(config, risk_state);
    
    // 尝试下第 3 个订单，应该被拒绝
    let intent = OrderIntent::new(
        Uuid::new_v4(),
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        dec("0.1"),
        Some(dec("50000.0")),
        0.9,
    );
    
    let result = risk.check(&intent).await;
    assert!(result.is_err(), "Should reject when open orders limit reached");
    assert!(result.unwrap_err().to_string().contains("TOO_MANY_OPEN_ORDERS"));
}
