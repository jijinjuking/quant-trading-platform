//! # 健康检查处理器
//! 
//! ## 功能层级: 【接口层 Interface】
//! ## 职责: 提供服务健康状态检查端点

// ============================================================
// 外部依赖导入
// ============================================================
use axum::Json;           // JSON响应
use serde::Serialize;     // 序列化trait

// ============================================================
// 响应结构体
// ============================================================

/// # HealthResponse - 健康检查响应
/// 
/// ## 字段:
/// - status: 服务状态（healthy/unhealthy）
/// - service: 服务名称
#[derive(Serialize)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    /// 服务名称
    pub service: String,
}

// ============================================================
// Handler函数
// ============================================================

/// # 健康检查端点
/// 
/// ## 路由: GET /health
/// ## 返回: JSON格式的健康状态
/// 
/// ## 用途:
/// - 负载均衡器健康检查
/// - Kubernetes存活探针
/// - 监控系统状态检测
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "trading-engine".to_string(),
    })
}
