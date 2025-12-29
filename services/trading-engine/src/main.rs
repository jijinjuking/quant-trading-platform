//! # 交易引擎服务 - 入口文件
//! 
//! ## 功能层级: 【入口层】
//! ## 服务端口: 8081
//! ## 职责: 接收交易信号 → 下单/撤单 → 同步交易所

// ============================================================
// 模块声明 - 引入各层模块
// ============================================================
mod state;          // 应用状态模块
mod interface;      // 接口层 - HTTP/gRPC入口
mod application;    // 应用层 - 用例编排
mod domain;         // 领域层 - 核心业务逻辑
mod infrastructure; // 基础设施层 - 外部依赖实现
mod bootstrap;      // 依赖注入模块

// ============================================================
// 外部依赖导入
// ============================================================
use anyhow::Result;           // 错误处理
use std::net::SocketAddr;     // 网络地址
use tracing::info;            // 日志记录

/// # 主函数 - 服务启动入口
/// 
/// ## 执行流程:
/// 1. 初始化日志系统
/// 2. 创建应用状态
/// 3. 创建路由
/// 4. 绑定端口并启动服务
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志订阅器
    tracing_subscriber::fmt::init();
    
    // 创建应用状态（包含配置、数据库连接等）
    let state = state::AppState::new().await?;
    
    // 创建HTTP路由
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口，默认8081
    // 遵循开发规范：不硬编码端口号
    let port: u16 = std::env::var("TRADING_ENGINE_PORT")
        .ok()                           // 转换为Option
        .and_then(|p| p.parse().ok())   // 尝试解析为u16
        .unwrap_or(8081);               // 默认值
    
    // 构建监听地址 0.0.0.0:port
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    // 记录启动日志
    info!("Trading Engine listening on {}", addr);
    
    // 绑定TCP监听器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // 启动Axum服务
    axum::serve(listener, app).await?;
    
    Ok(())
}
