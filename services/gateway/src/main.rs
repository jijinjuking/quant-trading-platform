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
///
/// 初始化并启动 API Gateway 服务，执行以下步骤：
/// 1. 初始化日志订阅器
/// 2. 创建应用状态（包含配置信息）
/// 3. 创建 HTTP 路由器
/// 4. 绑定端口并启动服务
///
/// # 返回值
/// - `Ok(())` - 服务正常退出
/// - `Err(anyhow::Error)` - 启动或运行过程中发生错误
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt::init();
    
    // 创建应用状态，包含配置和共享资源
    let state = state::AppState::new().await?;
    
    // 创建 HTTP 路由器
    let app = interface::http::routes::create_router(state);
    
    // 配置服务监听地址（0.0.0.0:8080）
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Gateway listening on {}", addr);
    
    // 绑定 TCP 监听器并启动 HTTP 服务
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
