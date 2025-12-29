//! # 行情数据服务 (Market Data Service)
//! 
//! 端口: 8082
//! 
//! ## 服务职责
//! - WebSocket 连接交易所拉取实时行情
//! - 统一不同交易所的数据格式
//! - 通过 Kafka 推送行情事件给其他服务
//! 
//! ## 架构层次
//! - interface: HTTP API 接口层
//! - application: 应用层（用例编排）
//! - domain: 领域层（核心模型和端口）
//! - infrastructure: 基础设施层（交易所连接、Kafka）

// ============================================================================
// 模块声明
// ============================================================================

/// 应用状态模块 - 管理配置和共享资源
mod state;

/// 接口层 - HTTP API 处理
mod interface;

/// 应用层 - 用例编排
mod application;

/// 领域层 - 核心业务模型和端口定义
mod domain;

/// 基础设施层 - 外部系统适配器
mod infrastructure;

// ============================================================================
// 外部依赖导入
// ============================================================================

use anyhow::Result;           // 错误处理 - 统一错误类型
use std::net::SocketAddr;     // 网络地址 - 服务绑定地址
use tracing::info;            // 日志 - 结构化日志输出

// ============================================================================
// 服务入口
// ============================================================================

/// 服务主入口函数
/// 
/// 启动流程：
/// 1. 初始化日志系统
/// 2. 创建应用状态（配置、连接池等）
/// 3. 创建 HTTP 路由
/// 4. 绑定端口并启动服务
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志订阅器
    tracing_subscriber::fmt::init();
    
    // 创建应用状态（包含配置和共享资源）
    let state = state::AppState::new().await?;
    
    // 创建 HTTP 路由器
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口，默认 8082
    let port: u16 = std::env::var("MARKET_DATA_PORT")
        .ok()                           // 转换为 Option
        .and_then(|p| p.parse().ok())   // 尝试解析为 u16
        .unwrap_or(8082);               // 默认端口
    
    // 构建监听地址（0.0.0.0 表示监听所有网卡）
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Market Data listening on {}", addr);
    
    // 绑定 TCP 监听器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // 启动 Axum HTTP 服务
    axum::serve(listener, app).await?;
    
    Ok(())
}
