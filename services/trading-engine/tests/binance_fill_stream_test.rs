//! # Binance Fill Stream 测试
//!
//! 测试 Binance 成交事件 → ExecutionFill 映射
//! 以及 ExecutionService 接收 fill → RiskState 正确更新

use std::sync::Arc;

use async_trait::async_trait;
use rust_decimal::Decimal;
use tokio::sync::mpsc;
use uuid::Uuid;

use trading_engine::application::service::execution_service::ExecutionService;
use trading_engine::domain::model::execution_fill::{ExecutionFill, FillSide, FillType};
use trading_engine::domain::model::order_intent::OrderIntent;
use trading_engine::domain::port::order_execution_port::{ExecutionResult, OrderExecutionPort};
use trading_engine::domain::port::risk_state_port::RiskStatePort;
use trading_engine::domain::port::strategy_port::StrategyPort;
use trading_engine::infrastructure::risk::inmemory_risk_state::InMemoryRiskStateAdapter;
use trading_engine::infrastructure::risk::order_risk_adapter::OrderRiskAdapter;
use trading_engine::infrastructure::risk::risk_limits::OrderRiskConfig;

use shared::event::market_event::MarketEvent;

fn dec(s: &str) -> Decimal {
    s.parse().unwrap_or_default()
}

// ========== Mock Strategy ==========

struct MockStrategy;

#[async_trait]
impl StrategyPort for MockStrategy {
    async fn evaluate(&self, _event: &MarketEvent) -> anyhow::Result<Option<OrderIntent>> {
        Ok(None)
    }
}

// ========== Mock Execution ==========

struct MockExecution;

#[async_trait]
impl OrderExecutionPort for MockExecution {
    async fn execute(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionResult> {
        Ok(ExecutionResult {
            success: true,
            order_id: format!("mock_order_{}", Uuid::new_v4()),
            symbol: intent.symbol.clone(),
            error: None,
        })
    }
}

// ========== 测试 1: Binance 成交事件 → ExecutionFill 映射 ==========

#[test]
fn test_execution_fill_from_binance_format() {
    // 模拟 Binance executionReport 字段映射
    let order_id = "12345678";
    let symbol = "BTCUSDT";
    let side = "BUY";
    let last_executed_qty = "0.1";
    let last_executed_price = "50000";
    let cumulative_qty = "0.1";
    let original_qty = "0.1";
    let commission = "5";
    let commission_asset = "USDT";

    // 转换为 ExecutionFill
    let fill_side = FillSide::from_str(side).expect("Invalid side");
    let filled_qty: Decimal = last_executed_qty.parse().unwrap();
    let fill_price: Decimal = last_executed_price.parse().unwrap();
    let cum_qty: Decimal = cumulative_qty.parse().unwrap();
    let orig_qty: Decimal = original_qty.parse().unwrap();
    let comm: Decimal = commission.parse().unwrap();

    let fill_type = if cum_qty >= orig_qty {
        FillType::Full
    } else {
        FillType::Partial
    };

    let fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: order_id.to_string(),
        trade_id: "test_trade_001".to_string(),
        client_order_id: None,
        symbol: symbol.to_string(),
        side: fill_side,
        fill_type,
        filled_quantity: filled_qty,
        fill_price,
        cumulative_quantity: cum_qty,
        original_quantity: orig_qty,
        commission: comm,
        commission_asset: commission_asset.to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    // 验证映射正确
    assert_eq!(fill.order_id, "12345678");
    assert_eq!(fill.symbol, "BTCUSDT");
    assert_eq!(fill.side, FillSide::Buy);
    assert_eq!(fill.fill_type, FillType::Full);
    assert_eq!(fill.filled_quantity, dec("0.1"));
    assert_eq!(fill.fill_price, dec("50000"));
    assert_eq!(fill.commission, dec("5"));
    assert_eq!(fill.commission_asset, "USDT");

    // 验证计算方法
    assert_eq!(fill.notional(), dec("5000")); // 0.1 * 50000
    assert_eq!(fill.position_delta(), dec("0.1")); // BUY = +qty
    assert_eq!(fill.remaining_quantity(), dec("0")); // Full fill
}

#[test]
fn test_execution_fill_partial_mapping() {
    // 部分成交场景
    let fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "67890".to_string(),
        trade_id: "test_trade_002".to_string(),
        client_order_id: Some("client_123".to_string()),
        symbol: "ETHUSDT".to_string(),
        side: FillSide::Sell,
        fill_type: FillType::Partial,
        filled_quantity: dec("1.0"),
        fill_price: dec("3000"),
        cumulative_quantity: dec("1.0"),
        original_quantity: dec("2.0"),
        commission: dec("3"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    assert_eq!(fill.fill_type, FillType::Partial);
    assert_eq!(fill.remaining_quantity(), dec("1.0")); // 2.0 - 1.0
    assert_eq!(fill.position_delta(), dec("-1.0")); // SELL = -qty
    assert_eq!(fill.notional(), dec("3000")); // 1.0 * 3000
}

// ========== 测试 2: ExecutionService 接收 fill → RiskState 正确更新 ==========

#[tokio::test]
async fn test_execution_service_on_execution_fill() {
    // 1. 初始化 RiskState
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("10000"), dec("0"));

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy);
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution);

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 先添加一个 open_order（模拟下单成功）
    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "ws_order_001".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("0.1"),
            price: dec("50000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    // 验证 open_order 已添加
    let snapshot = risk_state.get_snapshot().await.unwrap();
    assert_eq!(snapshot.open_orders.len(), 1);

    // 4. 创建来自 WebSocket 的成交事件
    let fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "ws_order_001".to_string(),
        trade_id: "ws_trade_001".to_string(),
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

    // 5. 通过 on_execution_fill 处理
    service.on_execution_fill(fill).await;

    // 6. 验证 RiskState 更新
    let snapshot = risk_state.get_snapshot().await.unwrap();

    // 持仓应该是 0.1 BTC
    assert_eq!(
        snapshot.get_position_qty("BTCUSDT"),
        dec("0.1"),
        "Position should be 0.1 BTC"
    );

    // 余额应该是 10000 - 5000 - 5 = 4995 USDT
    assert_eq!(
        snapshot.get_free_balance("USDT"),
        dec("4995"),
        "Balance should be 4995 USDT"
    );

    // open_orders 应该为空（已成交）
    assert_eq!(
        snapshot.open_orders.len(),
        0,
        "Open orders should be empty after full fill"
    );

    println!("✅ 测试通过: on_execution_fill 正确更新 RiskState");
}

// ========== 测试 3: 通过 channel 接收成交事件 ==========

#[tokio::test]
async fn test_fill_consumer_via_channel() {
    // 1. 初始化 RiskState
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("20000"), dec("0"));

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy);
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution);

    let service = Arc::new(ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    ));

    // 3. 创建 channel
    let (fill_tx, fill_rx) = mpsc::channel::<ExecutionFill>(100);

    // 4. 先添加 open_orders
    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "channel_order_001".to_string(),
            symbol: "ETHUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("2.0"),
            price: dec("3000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "channel_order_002".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "SELL".to_string(),
            quantity: dec("0.05"),
            price: dec("52000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    // 5. 启动 fill consumer（在后台）
    let service_clone = Arc::clone(&service);
    let consumer_handle = tokio::spawn(async move {
        service_clone.run_fill_consumer(fill_rx).await;
    });

    // 6. 发送成交事件
    let fill1 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "channel_order_001".to_string(),
        trade_id: "channel_trade_001".to_string(),
        client_order_id: None,
        symbol: "ETHUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full,
        filled_quantity: dec("2.0"),
        fill_price: dec("3000"),
        cumulative_quantity: dec("2.0"),
        original_quantity: dec("2.0"),
        commission: dec("6"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    fill_tx.send(fill1).await.unwrap();

    // 等待处理
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 7. 验证第一笔成交
    let snapshot = risk_state.get_snapshot().await.unwrap();
    assert_eq!(snapshot.get_position_qty("ETHUSDT"), dec("2.0"));
    // 20000 - 6000 - 6 = 13994
    assert_eq!(snapshot.get_free_balance("USDT"), dec("13994"));
    assert_eq!(snapshot.open_orders.len(), 1); // 还剩一个

    // 8. 发送第二笔成交（卖出）
    // 先设置 BTC 持仓
    risk_state.set_position("BTCUSDT", dec("0.1"), dec("50000"));

    let fill2 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "channel_order_002".to_string(),
        trade_id: "channel_trade_002".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Sell,
        fill_type: FillType::Full,
        filled_quantity: dec("0.05"),
        fill_price: dec("52000"),
        cumulative_quantity: dec("0.05"),
        original_quantity: dec("0.05"),
        commission: dec("2.6"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    fill_tx.send(fill2).await.unwrap();

    // 等待处理
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 9. 验证第二笔成交
    let snapshot = risk_state.get_snapshot().await.unwrap();
    // BTC 持仓: 0.1 - 0.05 = 0.05
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.05"));
    // USDT: 13994 + 2600 - 2.6 = 16591.4
    assert_eq!(snapshot.get_free_balance("USDT"), dec("16591.4"));
    // 所有订单都已成交
    assert_eq!(snapshot.open_orders.len(), 0);

    // 10. 关闭 channel
    drop(fill_tx);

    // 等待 consumer 退出
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    consumer_handle.abort();

    println!("✅ 测试通过: channel 方式接收成交事件正确更新 RiskState");
}

// ========== 测试 4: 部分成交不移除 open_order ==========

#[tokio::test]
async fn test_partial_fill_keeps_open_order() {
    // 1. 初始化 RiskState
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("10000"), dec("0"));

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy);
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution);

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 添加 open_order
    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "partial_order_001".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("1.0"),
            price: dec("50000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    // 4. 发送部分成交
    let partial_fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "partial_order_001".to_string(),
        trade_id: "partial_trade_001".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Partial,
        filled_quantity: dec("0.3"),
        fill_price: dec("50000"),
        cumulative_quantity: dec("0.3"),
        original_quantity: dec("1.0"),
        commission: dec("15"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };

    service.on_execution_fill(partial_fill).await;

    // 5. 验证
    let snapshot = risk_state.get_snapshot().await.unwrap();

    // 持仓应该是 0.3 BTC
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.3"));

    // open_order 应该保留（部分成交）
    assert_eq!(
        snapshot.open_orders.len(),
        1,
        "Open order should be retained for partial fill"
    );

    println!("✅ 测试通过: 部分成交保留 open_order");
}

// ========== 测试 5: 单订单多次部分成交 → 累计持仓/余额正确 ==========

#[tokio::test]
async fn test_multiple_partial_fills_cumulative() {
    // 1. 初始化 RiskState
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("100000"), dec("0"));

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy);
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution);

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 添加 open_order（原始订单 1.0 BTC）
    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "multi_fill_order".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("1.0"),
            price: dec("50000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    // 4. 第一次部分成交: 0.3 BTC @ 50000
    let fill1 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "multi_fill_order".to_string(),
        trade_id: "trade_001".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Partial,
        filled_quantity: dec("0.3"),
        fill_price: dec("50000"),
        cumulative_quantity: dec("0.3"),
        original_quantity: dec("1.0"),
        commission: dec("15"), // 0.1% of 15000
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(fill1).await;

    // 验证第一次成交后状态
    let snapshot = risk_state.get_snapshot().await.unwrap();
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.3"), "After fill1: position should be 0.3");
    // 100000 - 15000 - 15 = 84985
    assert_eq!(snapshot.get_free_balance("USDT"), dec("84985"), "After fill1: balance should be 84985");
    assert_eq!(snapshot.open_orders.len(), 1, "After fill1: open_order should remain");

    // 5. 第二次部分成交: 0.4 BTC @ 50100
    let fill2 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "multi_fill_order".to_string(),
        trade_id: "trade_002".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Partial,
        filled_quantity: dec("0.4"),
        fill_price: dec("50100"),
        cumulative_quantity: dec("0.7"),
        original_quantity: dec("1.0"),
        commission: dec("20.04"), // 0.1% of 20040
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(fill2).await;

    // 验证第二次成交后状态
    let snapshot = risk_state.get_snapshot().await.unwrap();
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.7"), "After fill2: position should be 0.7");
    // 84985 - 20040 - 20.04 = 64924.96
    assert_eq!(snapshot.get_free_balance("USDT"), dec("64924.96"), "After fill2: balance should be 64924.96");
    assert_eq!(snapshot.open_orders.len(), 1, "After fill2: open_order should remain");

    // 6. 第三次成交（最后一笔，全部成交）: 0.3 BTC @ 50200
    let fill3 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "multi_fill_order".to_string(),
        trade_id: "trade_003".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full, // 最后一笔，标记为 Full
        filled_quantity: dec("0.3"),
        fill_price: dec("50200"),
        cumulative_quantity: dec("1.0"),
        original_quantity: dec("1.0"),
        commission: dec("15.06"), // 0.1% of 15060
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(fill3).await;

    // 7. 验证最终状态
    let snapshot = risk_state.get_snapshot().await.unwrap();
    
    // 持仓: 0.3 + 0.4 + 0.3 = 1.0 BTC
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("1.0"), "Final: position should be 1.0");
    
    // 余额: 64924.96 - 15060 - 15.06 = 49849.90
    assert_eq!(snapshot.get_free_balance("USDT"), dec("49849.90"), "Final: balance should be 49849.90");
    
    // open_order 应该被移除（最后一笔是 Full）
    assert_eq!(snapshot.open_orders.len(), 0, "Final: open_order should be removed after full fill");

    println!("✅ 测试通过: 单订单多次部分成交累计正确，最后一笔移除 open_order");
}

// ========== 测试 6: trade_id 幂等性 - 重复 trade_id 被忽略 ==========

#[tokio::test]
async fn test_trade_id_idempotency() {
    // 1. 初始化 RiskState
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("50000"), dec("0"));

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy);
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution);

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 添加 open_order
    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "idempotent_order".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("0.5"),
            price: dec("50000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    // 4. 第一次成交
    let fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "idempotent_order".to_string(),
        trade_id: "unique_trade_123".to_string(), // 唯一 trade_id
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full,
        filled_quantity: dec("0.5"),
        fill_price: dec("50000"),
        cumulative_quantity: dec("0.5"),
        original_quantity: dec("0.5"),
        commission: dec("25"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(fill.clone()).await;

    // 验证第一次成交后状态
    let snapshot = risk_state.get_snapshot().await.unwrap();
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.5"), "After first fill: position should be 0.5");
    // 50000 - 25000 - 25 = 24975
    assert_eq!(snapshot.get_free_balance("USDT"), dec("24975"), "After first fill: balance should be 24975");
    assert_eq!(snapshot.open_orders.len(), 0, "After first fill: open_order should be removed");

    // 5. 重复发送相同 trade_id 的成交事件（模拟网络重传）
    let duplicate_fill = ExecutionFill {
        id: Uuid::new_v4(), // 不同的事件 ID
        order_id: "idempotent_order".to_string(),
        trade_id: "unique_trade_123".to_string(), // 相同的 trade_id
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full,
        filled_quantity: dec("0.5"),
        fill_price: dec("50000"),
        cumulative_quantity: dec("0.5"),
        original_quantity: dec("0.5"),
        commission: dec("25"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(duplicate_fill).await;

    // 6. 验证状态未变化（幂等）
    let snapshot = risk_state.get_snapshot().await.unwrap();
    
    // 持仓应该仍然是 0.5（不是 1.0）
    assert_eq!(
        snapshot.get_position_qty("BTCUSDT"), 
        dec("0.5"), 
        "After duplicate: position should still be 0.5 (idempotent)"
    );
    
    // 余额应该仍然是 24975（不是 -25）
    assert_eq!(
        snapshot.get_free_balance("USDT"), 
        dec("24975"), 
        "After duplicate: balance should still be 24975 (idempotent)"
    );

    // 7. 发送不同 trade_id 的成交事件（应该被处理）
    // 先添加新的 open_order
    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "another_order".to_string(),
            symbol: "ETHUSDT".to_string(),
            side: "BUY".to_string(),
            quantity: dec("1.0"),
            price: dec("3000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    let new_fill = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "another_order".to_string(),
        trade_id: "different_trade_456".to_string(), // 不同的 trade_id
        client_order_id: None,
        symbol: "ETHUSDT".to_string(),
        side: FillSide::Buy,
        fill_type: FillType::Full,
        filled_quantity: dec("1.0"),
        fill_price: dec("3000"),
        cumulative_quantity: dec("1.0"),
        original_quantity: dec("1.0"),
        commission: dec("3"),
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(new_fill).await;

    // 8. 验证新成交被处理
    let snapshot = risk_state.get_snapshot().await.unwrap();
    assert_eq!(snapshot.get_position_qty("ETHUSDT"), dec("1.0"), "New fill should be processed");
    // 24975 - 3000 - 3 = 21972
    assert_eq!(snapshot.get_free_balance("USDT"), dec("21972"), "Balance should reflect new fill");

    println!("✅ 测试通过: trade_id 幂等性正确，重复事件被忽略");
}

// ========== 测试 7: 卖出订单多次部分成交 ==========

#[tokio::test]
async fn test_sell_order_multiple_partial_fills() {
    // 1. 初始化 RiskState（有 BTC 持仓）
    let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
    risk_state.set_balance("USDT", dec("10000"), dec("0"));
    risk_state.set_position("BTCUSDT", dec("2.0"), dec("48000")); // 持有 2 BTC

    // 2. 创建 ExecutionService
    let strategy = Arc::new(MockStrategy);
    let config = OrderRiskConfig::default();
    let risk_adapter = Arc::new(OrderRiskAdapter::new(config, risk_state.clone()));
    let execution = Arc::new(MockExecution);

    let service = ExecutionService::with_full_config(
        strategy,
        risk_adapter,
        execution,
        Some(risk_state.clone()),
        None,
        None,
    );

    // 3. 添加卖出 open_order（卖出 1.5 BTC）
    risk_state
        .add_open_order(trading_engine::domain::port::risk_state_port::RiskOpenOrder {
            order_id: "sell_order".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "SELL".to_string(),
            quantity: dec("1.5"),
            price: dec("52000"),
            created_at: chrono::Utc::now().timestamp_millis(),
        })
        .await;

    // 4. 第一次部分成交: 卖出 0.5 BTC @ 52000
    let fill1 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "sell_order".to_string(),
        trade_id: "sell_trade_001".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Sell,
        fill_type: FillType::Partial,
        filled_quantity: dec("0.5"),
        fill_price: dec("52000"),
        cumulative_quantity: dec("0.5"),
        original_quantity: dec("1.5"),
        commission: dec("26"), // 0.1% of 26000
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(fill1).await;

    // 验证
    let snapshot = risk_state.get_snapshot().await.unwrap();
    // 持仓: 2.0 - 0.5 = 1.5 BTC
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("1.5"), "After sell fill1: position should be 1.5");
    // 余额: 10000 + 26000 - 26 = 35974
    assert_eq!(snapshot.get_free_balance("USDT"), dec("35974"), "After sell fill1: balance should be 35974");
    assert_eq!(snapshot.open_orders.len(), 1, "After sell fill1: open_order should remain");

    // 5. 第二次部分成交: 卖出 1.0 BTC @ 52100（最后一笔）
    let fill2 = ExecutionFill {
        id: Uuid::new_v4(),
        order_id: "sell_order".to_string(),
        trade_id: "sell_trade_002".to_string(),
        client_order_id: None,
        symbol: "BTCUSDT".to_string(),
        side: FillSide::Sell,
        fill_type: FillType::Full, // 最后一笔
        filled_quantity: dec("1.0"),
        fill_price: dec("52100"),
        cumulative_quantity: dec("1.5"),
        original_quantity: dec("1.5"),
        commission: dec("52.1"), // 0.1% of 52100
        commission_asset: "USDT".to_string(),
        fill_time: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
    };
    service.on_execution_fill(fill2).await;

    // 6. 验证最终状态
    let snapshot = risk_state.get_snapshot().await.unwrap();
    
    // 持仓: 1.5 - 1.0 = 0.5 BTC
    assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.5"), "Final: position should be 0.5");
    
    // 余额: 35974 + 52100 - 52.1 = 88021.9
    assert_eq!(snapshot.get_free_balance("USDT"), dec("88021.9"), "Final: balance should be 88021.9");
    
    // open_order 应该被移除
    assert_eq!(snapshot.open_orders.len(), 0, "Final: open_order should be removed");

    println!("✅ 测试通过: 卖出订单多次部分成交正确");
}
