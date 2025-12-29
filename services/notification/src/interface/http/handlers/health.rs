//! # 健康检查处理器
//!
//! 本文件实现服务健康检查接口，用于监控和负载均衡器探测。
//!
//! ## 接口
//! - `GET /health`: 返回服务健康状态
//!
//! ## 架构位置
//! 属于接口层（Interface Layer）的处理器模块。

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
/// 返回服务的当前健康状态，用于：
/// - Kubernetes 存活探针（Liveness Probe）
/// - 负载均衡器健康检查
/// - 监控系统状态采集
///
/// # 返回值
/// JSON 格式的健康状态响应
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "notification".to_string(),
    })
}
