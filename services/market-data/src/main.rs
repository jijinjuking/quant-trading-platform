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

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    info!("Market Data Service starting...");

    // 构建服务（依赖注入在 bootstrap 中完成）
    let service = bootstrap::build();

    // 运行行情采集循环
    service.run().await
}
