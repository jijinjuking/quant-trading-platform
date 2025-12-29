//! # AI 服务入口
//!
//! AI 服务是量化交易平台的智能分析核心，提供以下能力：
//! - 智能市场分析：基于 AI 模型分析市场趋势
//! - 策略推荐：根据市场状况生成交易策略建议
//! - 市场预测：预测价格走势和市场变化
//!
//! ## 服务信息
//! - **端口**: 8087
//! - **AI 后端**: DeepSeek API
//!
//! ## 架构层次
//! 本服务遵循 DDD + 六边形架构：
//! - `interface`: 接口层（HTTP API）
//! - `application`: 应用层（用例编排）
//! - `domain`: 领域层（核心业务模型和端口定义）
//! - `infrastructure`: 基础设施层（DeepSeek 适配器实现）

mod state;
mod interface;
mod application;
mod domain;
mod infrastructure;
mod bootstrap;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;

/// AI 服务主入口函数
///
/// 初始化服务并启动 HTTP 服务器，监听端口 8087
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志追踪
    tracing_subscriber::fmt::init();
    
    // 创建应用状态（包含配置信息）
    let state = state::AppState::new().await?;
    // 创建路由
    let app = interface::http::routes::create_router(state);
    
    // 从环境变量读取端口，默认 8087
    let port: u16 = std::env::var("AI_SERVICE_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8087);
    
    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("AI Service listening on {}", addr);
    
    // 启动 HTTP 服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
