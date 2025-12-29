//! # Analytics Service - 数据分析服务入口
//!
//! ## 服务信息
//! - **端口**: 8088
//! - **职责**: 数据分析服务 - 交易绩效分析、统计报表、时序数据查询
//!
//! ## 架构层级
//! 本文件为服务启动入口，负责：
//! - 初始化日志系统
//! - 创建应用状态（AppState）
//! - 配置HTTP路由
//! - 启动HTTP服务器
//!
//! ## 依赖方向
//! ```text
//! main.rs → state.rs → interface → application → domain ← infrastructure
//! ```

mod state;
mod interface;
mod application;
mod domain;
mod infrastructure;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;

/// 服务主入口函数
///
/// ## 启动流程
/// 1. 初始化日志订阅器
/// 2. 创建应用状态（包含配置和连接）
/// 3. 创建HTTP路由
/// 4. 绑定端口并启动服务
///
/// ## 环境变量
/// - `ANALYTICS_PORT`: 服务端口，默认 8088
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt::init();
    
    // 创建应用状态
    let state = state::AppState::new().await?;
    
    // 创建HTTP路由
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口，默认8088
    let port: u16 = std::env::var("ANALYTICS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8088);
    
    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Analytics Service listening on {}", addr);
    
    // 启动HTTP服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
