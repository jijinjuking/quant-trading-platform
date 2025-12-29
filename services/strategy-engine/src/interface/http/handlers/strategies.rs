//! # 策略管理处理器 (Strategy Handlers)
//! 
//! 处理策略的 CRUD 操作。

use axum::Json;
use serde_json::Value;

/// 获取策略列表
pub async fn list_strategies() -> Json<Value> {
    // TODO: 调用 Application 层获取策略列表
    Json(serde_json::json!({
        "strategies": []
    }))
}

/// 创建新策略
pub async fn create_strategy() -> Json<Value> {
    // TODO: 调用 Application 层创建策略
    Json(serde_json::json!({
        "message": "strategy created"
    }))
}
