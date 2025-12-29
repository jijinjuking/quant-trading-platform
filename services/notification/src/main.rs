//! # 通知服务入口
//!
//! 本文件是通知服务（Notification Service）的主入口点。
//!
//! ## 服务信息
//! - **端口**: 8086
//! - **职责**: 实时通知、WebSocket推送、邮件短信
//!
//! ## 架构层级
//! 本文件属于服务启动层，负责初始化应用状态并启动HTTP服务器。

// 模块声明
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
/// 初始化日志、应用状态，并启动HTTP服务器监听指定端口。
///
/// # 返回值
/// - `Ok(())`: 服务正常退出
/// - `Err`: 启动过程中发生错误
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志订阅器
    tracing_subscriber::fmt::init();
    
    // 创建应用状态（包含配置、数据库连接等）
    let state = state::AppState::new().await?;
    
    // 创建路由器并绑定应用状态
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口，默认为 8086
    let port: u16 = std::env::var("NOTIFICATION_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8086);
    
    // 绑定地址（监听所有网络接口）
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Notification Service listening on {}", addr);
    
    // 创建TCP监听器并启动服务
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
