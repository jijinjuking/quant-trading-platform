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
//!
//! ## v1.1 集成重构
//! - RiskState 统一由 RiskStateCoordinator 管理
//! - 所有后台服务通过 bootstrap::start_background_services 启动
//! - main.rs 只调用 bootstrap 函数，不直接 spawn 服务
//!
//! ## 启动链路图
//! ```text
//! main
//!   ├── bootstrap::create_risk_state() → 创建唯一 RiskState
//!   ├── RiskStateCoordinator::new() → 创建协调器
//!   ├── coordinator.rebuild(Startup) → 初始化状态
//!   ├── bootstrap::create_market_event_consumer_with_state() → 创建消费者
//!   ├── bootstrap::start_background_services() → 启动后台服务
//!   │     └── OrderLifecycleService (tokio::spawn)
//!   └── axum::serve() → 启动 HTTP 服务
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
use crate::application::service::risk_state_coordinator::{RiskStateCoordinator, RebuildReason};
use crate::bootstrap::{
    create_risk_state,
    create_market_event_consumer_with_state,
    start_background_services,
    ConsumerConfig,
    ConsumerConfigWithState,
};

/// # 主函数 - 服务启动入口
///
/// ## 执行流程 (v1.1 集成重构):
/// 1. 初始化日志系统
/// 2. 创建应用状态（配置）
/// 3. 创建唯一的 RiskState 实例
/// 4. 创建 RiskStateCoordinator 并初始化状态
/// 5. 创建交易主链路消费者（共享 RiskState）
/// 6. 通过 bootstrap 启动所有后台服务
/// 7. 创建交易所查询适配器
/// 8. 创建 HTTP 路由
/// 9. 绑定端口并启动服务
#[tokio::main]
async fn main() -> Result<()> {
    // 加载 .env 文件
    dotenvy::dotenv().ok();

    // 初始化日志订阅器
    tracing_subscriber::fmt::init();

    println!("[trading-engine] Starting...");
    info!("Trading Engine starting...");

    // ========================================
    // Step 1: 创建应用状态（包含配置）
    // ========================================
    let state = state::AppState::new().await?;
    info!(
        kafka_brokers = %state.config.kafka_brokers,
        kafka_market_topic = %state.config.kafka_market_topic,
        "Config loaded"
    );
    let config = state.config.as_ref().clone();

    // ========================================
    // Step 2: 创建唯一的 RiskState 实例
    // ========================================
    let risk_state = create_risk_state(
        config.binance_api_key.clone(),
        config.binance_secret_key.clone(),
        config.binance_base_url.clone(),
    ).await;
    info!("RiskState 实例已创建（唯一）");

    // ========================================
    // Step 3: 创建 RiskStateCoordinator
    // ========================================
    let exchange_query: Option<Arc<dyn ExchangeQueryPort>> = match (
        &config.binance_api_key,
        &config.binance_secret_key,
    ) {
        (Some(api_key), Some(secret_key)) => {
            Some(Arc::new(BinanceQueryAdapter::new(
                api_key.clone(),
                secret_key.clone(),
                config.binance_base_url.clone(),
            )))
        }
        _ => None,
    };

    let coordinator = Arc::new(RiskStateCoordinator::new(
        Arc::clone(&risk_state),
        exchange_query.clone(),
    ));

    // 执行启动时状态初始化
    if let Err(e) = coordinator.rebuild(RebuildReason::Startup).await {
        warn!(error = %e, "启动时状态初始化失败，继续运行");
    }
    info!("RiskStateCoordinator 已初始化");

    // ========================================
    // Step 4: 创建交易主链路消费者（共享 RiskState）
    // ========================================
    let consumer_config = ConsumerConfigWithState {
        config: ConsumerConfig {
            kafka_brokers: config.kafka_brokers.clone(),
            kafka_market_topic: config.kafka_market_topic.clone(),
            kafka_consumer_group: config.kafka_consumer_group.clone(),
            strategy_mode: config.strategy_mode.clone(),
            strategy_url: config.strategy_engine_url.clone(),
            execution_mode: config.execution_mode.clone(),
            binance_api_key: config.binance_api_key.clone(),
            binance_secret_key: config.binance_secret_key.clone(),
            binance_base_url: config.binance_base_url.clone(),
            risk_url: config.risk_management_url.clone(),
            storage_enabled: config.storage_enabled,
        },
        risk_state: Arc::clone(&risk_state),
    };

    let market_consumer = create_market_event_consumer_with_state(consumer_config).await?;
    info!("交易主链路消费者已创建");

    // 启动消费者
    tokio::spawn(async move {
        if let Err(err) = market_consumer.run().await {
            error!(error = %err, "market event consumer stopped");
        }
    });
    info!("交易主链路消费者已启动");

    // ========================================
    // Step 5: 通过 bootstrap 启动所有后台服务
    // ========================================
    let _background_handles = start_background_services(Arc::clone(&risk_state));
    info!("所有后台服务已通过 bootstrap 启动");

    // ========================================
    // Step 6: 创建交易所查询适配器（用于 HTTP API）
    // ========================================
    let http_exchange_query: Arc<dyn ExchangeQueryPort> = match exchange_query {
        Some(eq) => {
            info!("交易所查询适配器已启用 (Binance)");
            eq
        }
        None => {
            warn!("未配置币安 API 密钥，交易所查询功能将不可用");
            Arc::new(NoopExchangeQuery)
        }
    };

    // ========================================
    // Step 7: 创建 HTTP 路由
    // ========================================
    let app = interface::http::routes::create_router(http_exchange_query);

    // ========================================
    // Step 8: 启动 HTTP 服务
    // ========================================
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
