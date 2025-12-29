//! # 风险管理服务 (Risk Management Service)
//!
//! 本服务是量化交易平台的风险管理微服务，负责订单前的风险检查。
//!
//! ## 服务信息
//! - **端口**: 8085
//! - **职责**: 订单前检查 → 仓位/敞口/回撤校验
//!
//! ## 架构说明
//! 本服务采用 DDD + 六边形架构（端口-适配器模式）：
//! - `interface`: 接口层（HTTP API）
//! - `application`: 应用层（用例编排）
//! - `domain`: 领域层（核心业务逻辑）
//! - `infrastructure`: 基础设施层（适配器实现）

mod state;
mod interface;
mod application;
mod domain;
mod infrastructure;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;

/// 服务入口函数
///
/// 初始化日志、应用状态和 HTTP 服务器，启动风险管理服务。
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志订阅器
    tracing_subscriber::fmt::init();
    
    // 创建应用状态（包含配置、数据库连接等）
    let state = state::AppState::new().await?;
    
    // 创建 HTTP 路由
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口，默认 8085
    let port: u16 = std::env::var("RISK_MANAGEMENT_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8085);
    
    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Risk Management listening on {}", addr);
    
    // 启动 HTTP 服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
