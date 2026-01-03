//! # API Gateway 服务入口
//!
//! 本模块是 API Gateway 服务的主入口点。
//!
//! ## 服务信息
//! - **端口**: 8080
//! - **职责**: API 网关 - 请求路由、认证鉴权、限流、负载均衡
//!
//! ## 架构层级
//! 本文件属于服务启动层，负责：
//! - 初始化日志系统
//! - 创建应用状态
//! - 配置 HTTP 路由
//! - 启动 HTTP 服务器
//!
//! ## 依赖方向
//! ```text
//! main.rs → state → interface → application → domain ← infrastructure
//! ```

mod state;
mod interface;
mod application;
mod domain;
mod infrastructure;
mod bootstrap;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;

/// 服务主入口函数
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt::init();
    
    // 创建应用状态
    let state = state::AppState::new().await?;
    
    // 打印配置信息
    println!("========================================");
    println!("       API Gateway 启动中...");
    println!("========================================");
    println!("后端服务配置:");
    println!("  - strategy-engine: {}", state.config.services.strategy_engine);
    println!("  - trading-engine:  {}", state.config.services.trading_engine);
    println!("  - market-data:     {}", state.config.services.market_data);
    println!("========================================");
    
    // 创建 HTTP 路由器
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口
    let port: u16 = std::env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Gateway listening on {}", addr);
    println!("Gateway 监听地址: http://{}", addr);
    println!("========================================");
    println!("API 端点:");
    println!("  GET  /health              - 健康检查");
    println!("  GET  /api/v1/services     - 服务状态");
    println!("  GET  /api/v1/strategies   - 策略列表");
    println!("  POST /api/v1/strategies   - 创建策略");
    println!("  GET  /api/v1/orders       - 订单列表");
    println!("  POST /api/v1/orders       - 创建订单");
    println!("  GET  /api/v1/positions    - 持仓列表");
    println!("========================================");
    
    // 启动 HTTP 服务
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
