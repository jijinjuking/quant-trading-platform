//! # 行情消费服务组装
//!
//! 路径: services/trading-engine/src/bootstrap/consumer.rs
//!
//! ## 职责
//! 组装 MarketEventConsumerService（交易主链路入口）
//!
//! ## 交易主链路
//! ```text
//! MarketEvent → ExecutionFlowService → Strategy → Risk(远程) → Execution
//! ```
//!
//! ## 重要变更 (v1.1 集成重构)
//! - RiskState 必须从外部传入，不再内部创建
//! - 确保整个系统使用同一个 RiskState 实例

use std::sync::Arc;

use crate::domain::port::order_repository_port::OrderRepositoryPort;
use crate::domain::port::risk_state_port::RiskStatePort;
use crate::domain::port::trade_audit_port::TradeAuditPort;
use crate::infrastructure::messaging::MarketEventKafkaConsumer;
use crate::infrastructure::repository::PostgresOrderRepository;
use crate::infrastructure::audit::NoopAuditAdapter;
use crate::application::service::execution_service::ExecutionService;
use crate::application::service::market_event_consumer_service::MarketEventConsumerService;

use super::database::create_postgres_pool;
use super::strategy::create_strategy_port;
use super::risk::create_risk_port;
use super::execution::{create_execution_port, create_order_execution_port};

/// 行情消费服务配置
#[derive(Debug, Clone)]
pub struct ConsumerConfig {
    // Kafka 配置
    pub kafka_brokers: String,
    pub kafka_market_topic: String,
    pub kafka_consumer_group: String,
    // 策略配置
    pub strategy_mode: String,
    pub strategy_url: Option<String>,
    // 执行配置
    pub execution_mode: String,
    pub binance_api_key: Option<String>,
    pub binance_secret_key: Option<String>,
    pub binance_base_url: String,
    // 风控配置
    pub risk_url: Option<String>,
    // 存储配置
    pub storage_enabled: bool,
}

/// 行情消费服务配置（带 RiskState）
#[derive(Clone)]
pub struct ConsumerConfigWithState {
    /// 基础配置
    pub config: ConsumerConfig,
    /// 风控状态端口（共享实例，必须从外部传入）
    pub risk_state: Arc<dyn RiskStatePort>,
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
/// # 风控状态管理 (v1.1 集成重构)
/// - RiskState 必须从外部传入（通过 ConsumerConfigWithState）
/// - 确保整个系统使用同一个 RiskState 实例
/// - 启动时由 RiskStateCoordinator 负责初始化
pub async fn create_market_event_consumer(
    config: ConsumerConfig,
) -> anyhow::Result<MarketEventConsumerService> {
    // 注意：此函数已废弃，请使用 create_market_event_consumer_with_state
    // 为保持向后兼容，内部创建一个空的 RiskState
    tracing::warn!("create_market_event_consumer 已废弃，请使用 create_market_event_consumer_with_state");
    
    use crate::infrastructure::risk::InMemoryRiskStateAdapter;
    let risk_state: Arc<dyn RiskStatePort> = Arc::new(InMemoryRiskStateAdapter::new());
    
    let config_with_state = ConsumerConfigWithState {
        config,
        risk_state,
    };
    
    create_market_event_consumer_with_state(config_with_state).await
}

/// 创建行情事件消费服务（带共享 RiskState）
///
/// # 参数
/// - `config`: 配置（包含共享的 RiskState）
///
/// # 返回
/// - `MarketEventConsumerService`: 行情消费服务
///
/// # 重要
/// RiskState 必须是整个系统共享的同一个实例，
/// 由 RiskStateCoordinator 统一管理。
pub async fn create_market_event_consumer_with_state(
    config: ConsumerConfigWithState,
) -> anyhow::Result<MarketEventConsumerService> {
    let ConsumerConfigWithState { config, risk_state } = config;
    // 1. 创建行情事件源
    let source = Arc::new(MarketEventKafkaConsumer::new(
        config.kafka_brokers,
        config.kafka_market_topic,
        config.kafka_consumer_group,
    )?);

    // 2. 创建策略端口
    let strategy = create_strategy_port(&config.strategy_mode, config.strategy_url)?;

    // 3. 创建风控端口（远程模式）
    let risk = create_risk_port(config.risk_url);

    // 4. 创建执行端口
    let inner_execution = create_execution_port(
        &config.execution_mode,
        config.binance_api_key.clone(),
        config.binance_secret_key.clone(),
        config.binance_base_url.clone(),
    )?;
    let execution = create_order_execution_port(inner_execution);

    // 5. 创建审计端口（v1 使用 Noop，只打日志）
    let audit: Arc<dyn TradeAuditPort> = Arc::new(NoopAuditAdapter::new());
    tracing::info!("交易审计已启用 (noop mode)");

    // 6. 使用外部传入的 RiskState（v1.1 集成重构）
    // 不再内部创建，确保整个系统使用同一个实例

    // 7. 创建 ExecutionService（核心调度）
    let execution_service = if config.storage_enabled {
        // 尝试创建数据库连接池
        match create_postgres_pool().await {
            Ok(pool) => {
                let order_repo: Arc<dyn OrderRepositoryPort> = 
                    Arc::new(PostgresOrderRepository::new(pool));
                tracing::info!("订单存储已启用");
                Arc::new(ExecutionService::with_full_config(
                    strategy, risk, execution, Some(risk_state), Some(order_repo), Some(audit),
                ))
            }
            Err(e) => {
                tracing::warn!("无法连接数据库，订单存储已禁用: {}", e);
                Arc::new(ExecutionService::with_full_config(
                    strategy, risk, execution, Some(risk_state), None, Some(audit),
                ))
            }
        }
    } else {
        tracing::info!("订单存储已禁用");
        Arc::new(ExecutionService::with_full_config(
            strategy, risk, execution, Some(risk_state), None, Some(audit),
        ))
    };

    // 8. 创建 MarketEventConsumerService
    Ok(MarketEventConsumerService::new(source, execution_service))
}

// ============================================================================
// 向后兼容的旧接口（保留参数签名）
// ============================================================================

/// 创建行情事件消费服务（向后兼容接口）
///
/// 此函数保留原有的参数签名，内部转换为新的 ConsumerConfig 结构。
/// 
/// # 注意
/// 此函数会内部创建一个独立的 RiskState，不与其他服务共享。
/// 推荐使用 `create_market_event_consumer_with_state` 并传入共享的 RiskState。
#[allow(clippy::too_many_arguments)]
pub async fn create_market_event_consumer_compat(
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
    // 创建独立的 RiskState（向后兼容，但不推荐）
    use super::risk::create_risk_state;
    
    let risk_state = create_risk_state(
        binance_api_key.clone(),
        binance_secret_key.clone(),
        binance_base_url.clone(),
    ).await;
    
    let config = ConsumerConfigWithState {
        config: ConsumerConfig {
            kafka_brokers: brokers,
            kafka_market_topic: market_topic,
            kafka_consumer_group: group_id,
            strategy_mode,
            strategy_url,
            execution_mode,
            binance_api_key,
            binance_secret_key,
            binance_base_url,
            risk_url,
            storage_enabled,
        },
        risk_state,
    };
    create_market_event_consumer_with_state(config).await
}
