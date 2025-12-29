//! # Health Check Handler - 健康检查处理器
//!
//! ## 模块职责
//! 提供服务健康检查端点，用于：
//! - Kubernetes 存活探针（Liveness Probe）
//! - Kubernetes 就绪探针（Readiness Probe）
//! - 负载均衡器健康检查

use axum::Json;
use serde::Serialize;

/// 健康检查响应
///
/// 返回服务的健康状态信息
#[derive(Serialize)]
pub struct HealthResponse {
    /// 服务状态（healthy/unhealthy）
    pub status: String,
    /// 服务名称
    pub service: String,
}

/// 健康检查处理器
///
/// ## 端点
/// `GET /health`
///
/// ## 响应示例
/// ```json
/// {
///     "status": "healthy",
///     "service": "analytics"
/// }
/// ```
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "analytics".to_string(),
    })
}
