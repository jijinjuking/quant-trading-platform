//! # 健康检查处理器 (Health Check Handler)
//!
//! 本模块提供服务健康检查端点。
//!
//! ## 端点
//! - `GET /health`: 返回服务健康状态

use axum::Json;
use serde::Serialize;

/// 健康检查响应
///
/// 返回服务的健康状态信息。
#[derive(Serialize)]
pub struct HealthResponse {
    /// 服务状态（如 "healthy"）
    pub status: String,
    /// 服务名称
    pub service: String,
}

/// 健康检查处理器
///
/// 返回服务的健康状态，用于负载均衡器和监控系统。
///
/// # 返回值
/// 返回 JSON 格式的健康状态响应
///
/// # 示例响应
/// ```json
/// {
///   "status": "healthy",
///   "service": "risk-management"
/// }
/// ```
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "risk-management".to_string(),
    })
}
