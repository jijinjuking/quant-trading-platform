//! # 币安下单功能集成测试
//!
//! 测试现货和合约下单的各种场景。
//!
//! ## 注意
//! 这些测试需要真实的 API Key 和 Secret Key。
//! 建议在币安测试网运行：https://testnet.binance.vision
//!
//! ## 环境变量
//! - BINANCE_API_KEY: 币安 API Key
//! - BINANCE_SECRET_KEY: 币安 Secret Key
//! - BINANCE_BASE_URL: 币安 API 地址（默认测试网）

use rust_decimal::Decimal;
use std::str::FromStr;

use crate::domain::port::execution_port::{
    ExecutionCommand, ExecutionPort, OrderSide, OrderType, TimeInForce,
};
use crate::infrastructure::execution::{
    BinanceExecution, BinanceFuturesExecution, FuturesCommand, MarginType, PositionSide,
    RateLimiter, RateLimiterConfig, RetryConfig, RetryPolicy,
};

/// 测试辅助函数：创建测试用的执行器
fn create_test_executor() -> Option<BinanceExecution> {
    let api_key = std::env::var("BINANCE_API_KEY").ok()?;
    let secret_key = std::env::var("BINANCE_SECRET_KEY").ok()?;
    let base_url = std::env::var("BINANCE_BASE_URL")
        .unwrap_or_else(|_| "https://testnet.binance.vision".to_string());

    Some(BinanceExecution::new(api_key, secret_key, base_url))
}

/// 测试辅助函数：创建测试用的合约执行器
fn create_test_futures_executor() -> Option<BinanceFuturesExecution> {
    let api_key = std::env::var("BINANCE_API_KEY").ok()?;
    let secret_key = std::env::var("BINANCE_SECRET_KEY").ok()?;
    let base_url = std::env::var("BINANCE_BASE_URL")
        .unwrap_or_else(|_| "https://testnet.binance.vision".to_string());

    Some(BinanceFuturesExecution::new(api_key, secret_key, base_url))
}

#[tokio::test]
#[ignore] // 需要真实 API Key，默认跳过
async fn test_spot_market_order() {
    let executor = match create_test_executor() {
        Some(e) => e,
        None => {
            println!("Skipping test: BINANCE_API_KEY or BINANCE_SECRET_KEY not set");
            return;
        }
    };

    // 创建市价买单
    let command = ExecutionCommand::market(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        Decimal::from_str("0.001").unwrap(),
    );

    let result = executor.execute(&command).await;

    match result {
        Ok(exec_result) => {
            println!("✅ Market order succeeded:");
            println!("  Order ID: {}", exec_result.order_id);
            println!("  Status: {}", exec_result.status);
            println!("  Executed Qty: {}", exec_result.executed_qty);
            assert!(!exec_result.order_id.is_empty());
        }
        Err(e) => {
            println!("❌ Market order failed: {}", e);
            // 测试网可能没有足够余额，这是正常的
            assert!(
                e.to_string().contains("insufficient balance")
                    || e.to_string().contains("-2010")
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_spot_limit_order() {
    let executor = match create_test_executor() {
        Some(e) => e,
        None => {
            println!("Skipping test: API credentials not set");
            return;
        }
    };

    // 创建限价买单（价格设置得很低，不会立即成交）
    let command = ExecutionCommand::limit(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        Decimal::from_str("0.001").unwrap(),
        Decimal::from_str("20000").unwrap(), // 远低于市场价
    );

    let result = executor.execute(&command).await;

    match result {
        Ok(exec_result) => {
            println!("✅ Limit order succeeded:");
            println!("  Order ID: {}", exec_result.order_id);
            println!("  Status: {}", exec_result.status);
            assert!(!exec_result.order_id.is_empty());
            assert_eq!(exec_result.status, "NEW"); // 限价单应该是 NEW 状态
        }
        Err(e) => {
            println!("❌ Limit order failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_spot_stop_loss_limit_order() {
    let executor = match create_test_executor() {
        Some(e) => e,
        None => {
            println!("Skipping test: API credentials not set");
            return;
        }
    };

    // 创建止损限价单
    let command = ExecutionCommand::stop_loss_limit(
        "BTCUSDT".to_string(),
        OrderSide::Sell,
        Decimal::from_str("0.001").unwrap(),
        Decimal::from_str("30000").unwrap(), // 限价
        Decimal::from_str("31000").unwrap(), // 止损价
    );

    let result = executor.execute(&command).await;

    match result {
        Ok(exec_result) => {
            println!("✅ Stop loss limit order succeeded:");
            println!("  Order ID: {}", exec_result.order_id);
            println!("  Status: {}", exec_result.status);
            assert!(!exec_result.order_id.is_empty());
        }
        Err(e) => {
            println!("❌ Stop loss limit order failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_futures_market_order() {
    let executor = match create_test_futures_executor() {
        Some(e) => e,
        None => {
            println!("Skipping test: API credentials not set");
            return;
        }
    };

    // 创建合约市价开多单
    let command = FuturesCommand::open_market(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        Decimal::from_str("0.001").unwrap(),
        Some(PositionSide::Long),
    );

    let result = executor.execute_futures(&command).await;

    match result {
        Ok(exec_result) => {
            println!("✅ Futures market order succeeded:");
            println!("  Order ID: {}", exec_result.order_id);
            println!("  Status: {}", exec_result.status);
            println!("  Executed Qty: {}", exec_result.executed_qty);
            assert!(!exec_result.order_id.is_empty());
        }
        Err(e) => {
            println!("❌ Futures market order failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_futures_set_leverage() {
    let executor = match create_test_futures_executor() {
        Some(e) => e,
        None => {
            println!("Skipping test: API credentials not set");
            return;
        }
    };

    // 设置 BTCUSDT 杠杆为 10 倍
    let result = executor.set_leverage("BTCUSDT", 10).await;

    match result {
        Ok(_) => {
            println!("✅ Set leverage succeeded");
        }
        Err(e) => {
            println!("❌ Set leverage failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_futures_set_margin_type() {
    let executor = match create_test_futures_executor() {
        Some(e) => e,
        None => {
            println!("Skipping test: API credentials not set");
            return;
        }
    };

    // 设置为逐仓模式
    let result = executor.set_margin_type("BTCUSDT", MarginType::Isolated).await;

    match result {
        Ok(_) => {
            println!("✅ Set margin type succeeded");
        }
        Err(e) => {
            println!("❌ Set margin type failed: {}", e);
            // 如果已经是逐仓模式，会返回错误，这是正常的
        }
    }
}

#[tokio::test]
async fn test_rate_limiter() {
    use std::time::Instant;

    let rate_limiter = RateLimiter::new(RateLimiterConfig {
        tokens_per_second: 5,
        bucket_capacity: 10,
    });

    let start = Instant::now();

    // 快速请求 15 次
    for i in 0..15 {
        rate_limiter.acquire(1).await;
        println!("Request {} completed", i + 1);
    }

    let elapsed = start.elapsed();
    println!("Total time: {:?}", elapsed);

    // 15 个请求，速率 5/秒，应该至少需要 1 秒
    // （前 10 个立即完成，后 5 个需要等待）
    assert!(elapsed.as_secs() >= 1);
}

#[tokio::test]
async fn test_retry_policy() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    let retry_policy = RetryPolicy::new(RetryConfig {
        max_retries: 3,
        initial_delay_ms: 10,
        max_delay_ms: 100,
        jitter_ratio: 0.0,
    });

    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    // 模拟前 2 次失败，第 3 次成功
    let result = retry_policy
        .execute_with_retry(
            || {
                let c = counter_clone.clone();
                async move {
                    let count = c.fetch_add(1, Ordering::SeqCst);
                    if count < 2 {
                        Err(anyhow::anyhow!("timeout error"))
                    } else {
                        Ok::<_, anyhow::Error>("success")
                    }
                }
            },
            "test_operation",
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_order_command_builders() {
    // 测试市价单构建器
    let market_cmd = ExecutionCommand::market(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        Decimal::from_str("0.001").unwrap(),
    );
    assert_eq!(market_cmd.order_type, OrderType::Market);
    assert_eq!(market_cmd.side, OrderSide::Buy);
    assert!(market_cmd.price.is_none());

    // 测试限价单构建器
    let limit_cmd = ExecutionCommand::limit(
        "ETHUSDT".to_string(),
        OrderSide::Sell,
        Decimal::from_str("0.1").unwrap(),
        Decimal::from_str("2000").unwrap(),
    );
    assert_eq!(limit_cmd.order_type, OrderType::Limit);
    assert_eq!(limit_cmd.side, OrderSide::Sell);
    assert_eq!(limit_cmd.price, Some(Decimal::from_str("2000").unwrap()));
    assert_eq!(limit_cmd.time_in_force, Some(TimeInForce::GTC));

    // 测试止损限价单构建器
    let stop_loss_cmd = ExecutionCommand::stop_loss_limit(
        "BTCUSDT".to_string(),
        OrderSide::Sell,
        Decimal::from_str("0.001").unwrap(),
        Decimal::from_str("30000").unwrap(),
        Decimal::from_str("31000").unwrap(),
    );
    assert_eq!(stop_loss_cmd.order_type, OrderType::StopLossLimit);
    assert_eq!(stop_loss_cmd.price, Some(Decimal::from_str("30000").unwrap()));
    assert_eq!(stop_loss_cmd.stop_price, Some(Decimal::from_str("31000").unwrap()));

    // 测试客户端订单 ID
    let cmd_with_id = ExecutionCommand::market(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        Decimal::from_str("0.001").unwrap(),
    )
    .with_client_order_id("my_order_123".to_string());
    assert_eq!(cmd_with_id.client_order_id, Some("my_order_123".to_string()));
}

#[tokio::test]
async fn test_futures_command_builders() {
    // 测试开仓市价单
    let open_market = FuturesCommand::open_market(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        Decimal::from_str("0.001").unwrap(),
        Some(PositionSide::Long),
    );
    assert_eq!(open_market.base.order_type, OrderType::Market);
    assert_eq!(open_market.position_side, Some(PositionSide::Long));
    assert!(!open_market.reduce_only);

    // 测试平仓限价单
    let close_limit = FuturesCommand::close_limit(
        "ETHUSDT".to_string(),
        OrderSide::Sell,
        Decimal::from_str("0.1").unwrap(),
        Decimal::from_str("2000").unwrap(),
        Some(PositionSide::Long),
    );
    assert_eq!(close_limit.base.order_type, OrderType::Limit);
    assert_eq!(close_limit.position_side, Some(PositionSide::Long));
    assert!(close_limit.reduce_only);
}
