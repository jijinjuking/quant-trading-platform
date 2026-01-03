//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。
//!
//! ## 六边形架构说明
//! 在 main.rs 或 bootstrap 中完成：
//! - 创建 infrastructure adapter 实例
//! - 将 adapter 注入到 application service
//!
//! ## 风控架构
//! 风控逻辑已完全迁移到 risk-management 服务 (8085)
//! trading-engine 通过 RemoteRiskAdapter 调用远程风控
//!
//! ## 风控状态管理
//! - RiskStatePort: 本地维护账户状态（余额、持仓、未完成订单）
//! - 启动时从 ExchangeQueryPort 同步初始状态
//! - 运行期只有 ExecutionService 可以修改 RiskStatePort

use std::sync::Arc;

use anyhow::{anyhow, Context};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use crate::domain::port::execution_port::ExecutionPort;
use crate::domain::port::strategy_port::StrategyPort;
use crate::domain::port::order_risk_port::OrderRiskPort;
use crate::domain::port::order_execution_port::OrderExecutionPort;
use crate::domain::port::order_repository_port::OrderRepositoryPort;
use crate::domain::port::trade_audit_port::TradeAuditPort;
use crate::domain::port::risk_state_port::RiskStatePort;
use crate::infrastructure::execution::{BinanceExecution, NoopExecution, OrderExecutor};
use crate::infrastructure::strategy::{NoopStrategy, RemoteStrategy};
use crate::infrastructure::risk::RemoteRiskAdapter;
use crate::infrastructure::exchange::BinanceQueryAdapter;
use crate::infrastructure::repository::PostgresOrderRepository;
use crate::infrastructure::audit::NoopAuditAdapter;
use crate::infrastructure::messaging::MarketEventKafkaConsumer;
use crate::application::service::execution_service::ExecutionService;
use crate::application::service::market_event_consumer_service::MarketEventConsumerService;
use crate::application::service::risk_state_initializer::create_initialized_risk_state;

/// 创建 PostgreSQL 连接池
pub async fn create_postgres_pool() -> anyhow::Result<Pool> {
    let host = std::env::var("POSTGRES_HOST")
        .unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("POSTGRES_PORT")
        .unwrap_or_else(|_| "5432".to_string())
        .parse()
        .unwrap_or(5432);
    let user = std::env::var("POSTGRES_USER")
        .unwrap_or_else(|_| "postgres".to_string());
    let password = std::env::var("POSTGRES_PASSWORD")
        .unwrap_or_else(|_| "password".to_string());
    let dbname = std::env::var("POSTGRES_DATABASE")
        .unwrap_or_else(|_| "trading_db".to_string());

    let mut cfg = Config::new();
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.dbname = Some(dbname);

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("创建 PostgreSQL 连接池失败")?;

    // 测试连接
    let _client = pool.get().await.context("无法连接到 PostgreSQL")?;
    tracing::info!("PostgreSQL 连接池创建成功");

    Ok(pool)
}

/// 创建行情事件消费服务（交易主链路）
///
/// 这是交易主链路的入口：
/// MarketEvent → ExecutionFlowService → Strategy → Risk(远程) → Execution
///
/// # 风控说明
/// 风控逻辑已完全迁移到 risk-management 服务 (8085)
/// trading-engine 只通过 HTTP 调用远程风控，不包含任何风控规则
///
/// # 风控状态管理
/// - 启动时从 ExchangeQueryPort 同步初始状态到 RiskStatePort
/// - 运行期只有 ExecutionService 可以修改 RiskStatePort
/// - OrderRiskAdapter 只读取 RiskStatePort，不写入
#[allow(clippy::too_many_arguments)]
pub async fn create_market_event_consumer(
    brokers: String,
    market_topic: String,
    group_id: String,
    strategy_mode: String,
    strategy_url: Option<String>,
    execution_mode: String,
    binance_api_key: Option<String>,
    binance_secret_key: Option<String>,
    binance_base_url: String,
    _risk_mode: String,  // 保留参数兼容性，但只使用远程模式
    risk_url: Option<String>,
    _risk_min_qty: Option<rust_decimal::Decimal>,  // 已迁移到 risk-management
    _risk_max_qty: Option<rust_decimal::Decimal>,  // 已迁移到 risk-management
    _risk_max_notional: Option<rust_decimal::Decimal>,  // 已迁移到 risk-management
    _risk_allow_symbols: Option<Vec<String>>,  // 已迁移到 risk-management
    storage_enabled: bool,
) -> anyhow::Result<MarketEventConsumerService> {
    // 1. 创建行情事件源
    let source = Arc::new(MarketEventKafkaConsumer::new(
        brokers,
        market_topic,
        group_id,
    )?);

    // 2. 创建策略端口
    let strategy: Arc<dyn StrategyPort> = match strategy_mode.trim().to_lowercase().as_str() {
        "remote" => {
            let url = strategy_url
                .ok_or_else(|| anyhow!("STRATEGY_ENGINE_URL is required for remote strategy"))?;
            Arc::new(RemoteStrategy::new(url))
        }
        _ => Arc::new(NoopStrategy::new()),
    };

    // 3. 创建风控端口（只使用远程模式）
    let url = risk_url.unwrap_or_else(|| "http://localhost:8085".to_string());
    tracing::info!(url = %url, "使用远程风控服务 (risk-management)");
    let risk: Arc<dyn OrderRiskPort> = Arc::new(RemoteRiskAdapter::new(url));

    // 4. 创建执行端口
    let inner_execution: Arc<dyn ExecutionPort> =
        match execution_mode.trim().to_lowercase().as_str() {
            "binance" => {
                let api_key = binance_api_key.clone()
                    .ok_or_else(|| anyhow!("BINANCE_API_KEY is required for binance execution"))?;
                let secret_key = binance_secret_key.clone()
                    .ok_or_else(|| anyhow!("BINANCE_SECRET_KEY is required for binance execution"))?;
                let base_url = binance_base_url.trim().to_string();
                if base_url.is_empty() {
                    return Err(anyhow!("BINANCE_BASE_URL is required for binance execution"));
                }
                Arc::new(BinanceExecution::new(api_key, secret_key, base_url))
            }
            _ => Arc::new(NoopExecution::new()),
        };
    let execution: Arc<dyn OrderExecutionPort> = Arc::new(OrderExecutor::new(inner_execution));

    // 5. 创建审计端口（v1 使用 Noop，只打日志）
    let audit: Arc<dyn TradeAuditPort> = Arc::new(NoopAuditAdapter::new());
    tracing::info!("交易审计已启用 (noop mode)");

    // 6. 创建并初始化 RiskStatePort
    // 如果配置了币安 API，则从交易所同步初始状态
    let risk_state: Option<Arc<dyn RiskStatePort>> = match (
        &binance_api_key,
        &binance_secret_key,
    ) {
        (Some(api_key), Some(secret_key)) => {
            let exchange_query = BinanceQueryAdapter::new(
                api_key.clone(),
                secret_key.clone(),
                binance_base_url.clone(),
            );
            let risk_state = create_initialized_risk_state(Some(&exchange_query)).await;
            tracing::info!("RiskStatePort 已初始化（从交易所同步）");
            Some(risk_state)
        }
        _ => {
            let risk_state = create_initialized_risk_state(None).await;
            tracing::info!("RiskStatePort 已初始化（空状态）");
            Some(risk_state)
        }
    };

    // 7. 创建 ExecutionService（核心调度）
    let execution_service = if storage_enabled {
        // 尝试创建数据库连接池
        match create_postgres_pool().await {
            Ok(pool) => {
                let order_repo: Arc<dyn OrderRepositoryPort> = 
                    Arc::new(PostgresOrderRepository::new(pool));
                tracing::info!("订单存储已启用");
                Arc::new(ExecutionService::with_full_config(
                    strategy, risk, execution, risk_state, Some(order_repo), Some(audit),
                ))
            }
            Err(e) => {
                tracing::warn!("无法连接数据库，订单存储已禁用: {}", e);
                Arc::new(ExecutionService::with_full_config(
                    strategy, risk, execution, risk_state, None, Some(audit),
                ))
            }
        }
    } else {
        tracing::info!("订单存储已禁用");
        Arc::new(ExecutionService::with_full_config(
            strategy, risk, execution, risk_state, None, Some(audit),
        ))
    };

    // 8. 创建 MarketEventConsumerService
    Ok(MarketEventConsumerService::new(source, execution_service))
}
