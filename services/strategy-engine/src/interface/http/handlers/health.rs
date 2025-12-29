//! # 健康检查处理器 (Health Check Handler)

use axum::Json;
use serde::Serialize;

/// 健康检查响应
#[derive(Serialize)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    /// 服务名称
    pub service: String,
}

/// 健康检查处理器
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "strategy-engine".to_string(),
    })
}
