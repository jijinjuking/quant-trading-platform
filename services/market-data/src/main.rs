//! # 行情数据服务 (Market Data Service)
//!
//! 端口: 8082 (未使用 - 无 HTTP API)
//!
//! ## 服务职责
//! - 连接交易所 WebSocket 获取实时行情
//! - 标准化行情数据格式
//! - 发布到 Kafka 供其他服务消费
//!
//! ## 架构说明
//! market-data 是纯粹的行情采集器（Market Ingestor）
//! ❌ 不提供 HTTP API
//! ❌ 不存储数据
//! ❌ 不做业务判断

mod state;
mod application;
mod domain;
mod infrastructure;
mod bootstrap;

use anyhow::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::state::MarketDataConfig;

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

    info!("Market Data Service starting...");

    // 加载配置
    let config = MarketDataConfig::from_env();

    // 构建服务（依赖注入在 bootstrap 中完成）
    let service = bootstrap::build(config.clone())?;

    // 运行行情采集循环
    service.run(config.symbols).await
}
