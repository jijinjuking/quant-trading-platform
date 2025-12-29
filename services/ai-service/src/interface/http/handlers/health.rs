//! # 健康检查处理器
//!
//! 提供服务健康状态检查端点，用于：
//! - 负载均衡器健康探测
//! - Kubernetes 存活/就绪探针
//! - 服务监控

use axum::Json;
use serde::Serialize;

/// 健康检查响应结构
///
/// 返回服务的健康状态信息。
#[derive(Serialize)]
pub struct HealthResponse {
    /// 服务状态（healthy/unhealthy）
    pub status: String,
    /// 服务名称
    pub service: String,
}

/// 健康检查处理函数
///
/// 返回服务的当前健康状态。
///
/// # Returns
/// JSON 格式的健康状态响应
///
/// # Example Response
/// ```json
/// {
///   "status": "healthy",
///   "service": "ai-service"
/// }
/// ```
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "ai-service".to_string(),
    })
}
