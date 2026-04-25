//! # 策略引擎服务 (Strategy Engine Service)
//! 
//! 端口: 8083
//! 
//! ## 服务职责
//! - 接收行情数据和用户策略配置
//! - 运行策略算法计算交易信号
//! - 输出信号事件到消息队列
//! 
//! ## 支持的策略类型
//! - 网格交易 (Grid Trading)
//! - 均值回归 (Mean Reversion)
//! - 更多策略可扩展...

// ============================================================================
// 模块声明
// ============================================================================

/// 应用状态模块 - 管理配置和共享资源
mod state;

/// 接口层 - HTTP API 处理
mod interface;

/// 应用层 - 用例编排
mod application;

/// 领域层 - 核心策略模型和算法
mod domain;

/// 基础设施层 - 外部系统适配器
mod infrastructure;

/// 依赖注入模块
mod bootstrap;

// ============================================================================
// 外部依赖导入
// ============================================================================

use anyhow::Result;           // 错误处理
use std::net::SocketAddr;     // 网络地址
use std::sync::Arc;           // 原子引用计数
use tracing::{error, info};   // 日志
use tracing_subscriber::EnvFilter;

// ============================================================================
// 服务入口
// ============================================================================

/// 服务主入口函数
#[tokio::main]
async fn main() -> Result<()> {
    // 加载 .env 文件
    dotenvy::dotenv().ok();

    // 初始化日志（默认 INFO 级别）
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    info!("Strategy Engine starting...");

    // 创建应用状态
    let mut state = state::AppState::new().await?;
    let config = state.config.as_ref().clone();

    // 创建策略调度器（新版本）
    let (registry, scheduler, loader) = bootstrap::create_strategy_scheduler(
        config.kafka_brokers.clone(),
        config.kafka_market_topic.clone(),
        config.kafka_signal_topic.clone(),
        config.kafka_consumer_group.clone(),
    ).await?;

    // 注入运行时组件到 AppState，供 HTTP handler 直接使用
    let loader = Arc::new(loader);
    state.strategy_registry = Some(Arc::clone(&registry));
    state.strategy_loader = Some(Arc::clone(&loader));

    // 加载示例策略
    info!("Loading example strategies...");
    let example_configs = application::scheduler::StrategyLoader::load_example_strategies();
    let instance_ids = loader.load_strategies(example_configs).await?;
    info!("Loaded {} strategies", instance_ids.len());

    // 启动调度器（在后台任务中运行）
    let scheduler_clone = Arc::clone(&scheduler);
    let scheduler_handle = tokio::spawn(async move {
        if let Err(err) = scheduler_clone.run().await {
            error!(error = %err, "Strategy scheduler stopped");
        }
    });

    // 创建路由
    let app = interface::http::routes::create_router(state);

    // 从环境变量读取端口，默认 8083
    let port: u16 = std::env::var("STRATEGY_ENGINE_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8083);

    // 构建监听地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Strategy Engine listening on {}", addr);

    // 启动服务
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    scheduler_handle.abort();
    info!("Strategy Engine 已优雅关闭");

    Ok(())
}

async fn shutdown_signal() {
    if let Err(err) = tokio::signal::ctrl_c().await {
        error!(error = %err, "监听 Ctrl+C 失败");
        return;
    }

    info!("收到 Ctrl+C，开始优雅关闭");
}
