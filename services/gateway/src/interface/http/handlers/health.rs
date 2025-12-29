//! # 健康检查处理器
//!
//! 本模块提供服务健康检查端点的实现。
//!
//! ## 用途
//! - Kubernetes 存活探针（Liveness Probe）
//! - Kubernetes 就绪探针（Readiness Probe）
//! - 负载均衡器健康检查
//! - 监控系统状态检测
//!
//! ## 端点
//! - `GET /health` - 返回服务健康状态

use axum::Json;
use serde::Serialize;

/// 健康检查响应结构
///
/// 返回服务的健康状态信息。
///
/// # 字段
/// - `status`: 服务状态（"healthy" / "unhealthy"）
/// - `service`: 服务名称
#[derive(Serialize)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    /// 服务名称
    pub service: String,
}

/// 健康检查处理器
///
/// 返回 Gateway 服务的健康状态。
///
/// # 返回值
/// JSON 格式的健康状态响应
///
/// # 响应示例
/// ```json
/// {
///     "status": "healthy",
///     "service": "gateway"
/// }
/// ```
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "gateway".to_string(),
    })
}
