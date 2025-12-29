//! # 用户管理服务 - 主入口
//!
//! 本文件是用户管理微服务的启动入口点。
//!
//! ## 服务信息
//! - **端口**: 8084
//! - **职责**: 用户认证、权限管理、会员系统
//!
//! ## 架构层次
//! 本服务遵循 DDD + 六边形架构（端口-适配器模式）：
//! - `interface`: 接口层（HTTP API）
//! - `application`: 应用层（用例编排）
//! - `domain`: 领域层（核心模型和端口）
//! - `infrastructure`: 基础设施层（适配器实现）

mod state;
mod interface;
mod application;
mod domain;
mod infrastructure;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;

/// 服务主函数
///
/// 初始化并启动用户管理服务：
/// 1. 初始化日志追踪
/// 2. 创建应用状态
/// 3. 配置路由
/// 4. 绑定端口并启动 HTTP 服务器
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志追踪订阅器
    tracing_subscriber::fmt::init();
    
    // 创建应用状态（包含配置信息）
    let state = state::AppState::new().await?;
    
    // 创建路由器
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口，默认 8084
    let port: u16 = std::env::var("USER_MANAGEMENT_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8084);
    
    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("User Management listening on {}", addr);
    
    // 启动 HTTP 服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
