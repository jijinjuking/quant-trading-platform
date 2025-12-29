//! # 健康检查处理器
//!
//! 本文件定义服务健康检查的 HTTP 处理器。
//!
//! ## 所属层
//! Interface Layer > HTTP > Handlers
//!
//! ## 端点
//! - `GET /health`: 返回服务健康状态

use axum::Json;
use serde::Serialize;

/// 健康检查响应 DTO
///
/// 返回服务的健康状态信息。
#[derive(Serialize)]
pub struct HealthResponse {
    /// 服务状态（healthy/unhealthy）
    pub status: String,
    /// 服务名称
    pub service: String,
}

/// 健康检查处理器
///
/// 返回服务的健康状态，用于负载均衡器和监控系统。
///
/// # 返回值
/// JSON 格式的健康状态响应
///
/// # 响应示例
/// ```json
/// {
///     "status": "healthy",
///     "service": "user-management"
/// }
/// ```
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "user-management".to_string(),
    })
}
