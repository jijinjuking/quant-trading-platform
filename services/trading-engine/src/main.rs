//! # 交易引擎服务 - 入口文件
//!
//! ## 功能层级: 【入口层】
//! ## 服务端口: 8081
//! ## 职责: 消费行情 → 调用策略 → 风控检查 → 执行下单
//!
//! ## 交易主链路
//! ```text
//! MarketEvent → ExecutionFlowService → Strategy → Risk → Execution
//! ```

// ============================================================
// 模块声明 - 引入各层模块
// ============================================================
mod state;          // 应用状态模块
mod interface;      // 接口层 - HTTP/gRPC入口
mod application;    // 应用层 - 用例编排
mod domain;         // 领域层 - 核心业务逻辑
mod infrastructure; // 基础设施层 - 外部依赖实现
mod bootstrap;      // 依赖注入模块

// ============================================================
// 外部依赖导入
// ============================================================
use std::sync::Arc;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::{error, info, warn};

use crate::domain::port::exchange_query_port::ExchangeQueryPort;
use crate::infrastructure::exchange::BinanceQueryAdapter;

/// # 主函数 - 服务启动入口
///
/// ## 执行流程:
/// 1. 初始化日志系统
/// 2. 创建应用状态
/// 3. 启动交易主链路消费者
/// 4. 创建交易所查询适配器
/// 5. 创建 HTTP 路由
/// 6. 绑定端口并启动服务
#[tokio::main]
async fn main() -> Result<()> {
    // 加载 .env 文件
    dotenvy::dotenv().ok();

    // 初始化日志订阅器
    tracing_subscriber::fmt::init();

    println!("[trading-engine] Starting...");
    info!("Trading Engine starting...");

    // 创建应用状态（包含配置）
    let state = state::AppState::new().await?;
    info!(
        kafka_brokers = %state.config.kafka_brokers,
        kafka_market_topic = %state.config.kafka_market_topic,
        "Config loaded"
    );
    let config = state.config.as_ref().clone();

    // 启动交易主链路：MarketEvent → ExecutionFlowService → Strategy → Risk → Execution
    let market_consumer = bootstrap::create_market_event_consumer(
        config.kafka_brokers.clone(),
        config.kafka_market_topic.clone(),
        config.kafka_consumer_group.clone(),
        config.strategy_mode.clone(),
        config.strategy_engine_url.clone(),
        config.execution_mode.clone(),
        config.binance_api_key.clone(),
        config.binance_secret_key.clone(),
        config.binance_base_url.clone(),
        config.risk_mode.clone(),
        config.risk_management_url.clone(),
        config.risk_min_qty,
        config.risk_max_qty,
        config.risk_max_notional,
        config.risk_allow_symbols.clone(),
        config.storage_enabled,
    ).await?;

    tokio::spawn(async move {
        if let Err(err) = market_consumer.run().await {
            error!(error = %err, "market event consumer stopped");
        }
    });

    // 创建交易所查询适配器（用于 HTTP API）
    let exchange_query: Arc<dyn ExchangeQueryPort> = match (
        &config.binance_api_key,
        &config.binance_secret_key,
    ) {
        (Some(api_key), Some(secret_key)) => {
            info!("交易所查询适配器已启用 (Binance)");
            Arc::new(BinanceQueryAdapter::new(
                api_key.clone(),
                secret_key.clone(),
                config.binance_base_url.clone(),
            ))
        }
        _ => {
            warn!("未配置币安 API 密钥，交易所查询功能将不可用");
            // 使用 Noop 适配器（返回空数据）
            Arc::new(NoopExchangeQuery)
        }
    };

    // 创建 HTTP 路由
    let app = interface::http::routes::create_router(exchange_query);

    // 从环境变量读取端口，默认 8081
    let port: u16 = std::env::var("TRADING_ENGINE_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Trading Engine listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Noop 交易所查询适配器（未配置 API 时使用）
struct NoopExchangeQuery;

#[async_trait::async_trait]
impl ExchangeQueryPort for NoopExchangeQuery {
    async fn get_spot_balances(&self) -> anyhow::Result<Vec<domain::port::exchange_query_port::AccountBalance>> {
        Ok(vec![])
    }

    async fn get_futures_positions(&self) -> anyhow::Result<Vec<domain::port::exchange_query_port::Position>> {
        Ok(vec![])
    }

    async fn get_order(&self, _symbol: &str, _order_id: &str) -> anyhow::Result<Option<domain::port::exchange_query_port::ExchangeOrder>> {
        Ok(None)
    }

    async fn get_open_orders(&self, _symbol: Option<&str>) -> anyhow::Result<Vec<domain::port::exchange_query_port::ExchangeOrder>> {
        Ok(vec![])
    }

    async fn cancel_order(&self, symbol: &str, order_id: &str) -> anyhow::Result<domain::port::exchange_query_port::CancelOrderResult> {
        Ok(domain::port::exchange_query_port::CancelOrderResult {
            order_id: order_id.to_string(),
            symbol: symbol.to_string(),
            success: false,
            error: Some("交易所查询未配置".to_string()),
        })
    }

    async fn cancel_all_orders(&self, symbol: &str) -> anyhow::Result<Vec<domain::port::exchange_query_port::CancelOrderResult>> {
        Ok(vec![domain::port::exchange_query_port::CancelOrderResult {
            order_id: "all".to_string(),
            symbol: symbol.to_string(),
            success: false,
            error: Some("交易所查询未配置".to_string()),
        }])
    }
}
