//! # HTTP 路由配置模块
//!
//! 本模块负责配置 API Gateway 的 HTTP 路由。
//!
//! ## 职责
//! - 定义 API 端点
//! - 绑定处理器函数
//! - 配置中间件
//! - 注入应用状态
//!
//! ## 路由列表
//! | 方法 | 路径 | 处理器 | 说明 |
//! |------|------|--------|------|
//! | GET | /health | health_check | 健康检查 |

use axum::{routing::get, Router};
use crate::state::AppState;
use super::handlers;

/// 创建 HTTP 路由器
///
/// 配置所有 HTTP 端点并绑定应用状态。
///
/// # 参数
/// - `state`: 应用状态，包含配置和共享资源
///
/// # 返回值
/// 配置完成的 Axum 路由器
///
/// # 路由配置
/// - `GET /health` - 健康检查端点
///
/// # 示例
/// ```ignore
/// let state = AppState::new().await?;
/// let router = create_router(state);
/// ```
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查端点
        .route("/health", get(handlers::health::health_check))
        // 注入应用状态
        .with_state(state)
}
