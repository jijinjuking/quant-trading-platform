//! # 策略管理处理器
//!
//! 代理转发策略相关请求到 strategy-engine 服务

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::Value;
use crate::state::AppState;

/// 获取策略列表
///
/// 代理请求到 strategy-engine 服务
pub async fn list_strategies(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let url = format!("{}/api/v1/strategies", state.config.services.strategy_engine);
    
    println!("[Gateway] 代理请求: GET {}", url);
    
    let response = state.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            println!("[Gateway] 请求失败: {}", e);
            (StatusCode::BAD_GATEWAY, format!("请求后端服务失败: {}", e))
        })?;
    
    if response.status().is_success() {
        let body = response
            .json::<Value>()
            .await
            .map_err(|e| {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("解析响应失败: {}", e))
            })?;
        Ok(Json(body))
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        Err((StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY), text))
    }
}

/// 创建策略
///
/// 代理请求到 strategy-engine 服务
pub async fn create_strategy(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let url = format!("{}/api/v1/strategies", state.config.services.strategy_engine);
    
    println!("[Gateway] 代理请求: POST {} - {:?}", url, payload);
    
    let response = state.http_client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            println!("[Gateway] 请求失败: {}", e);
            (StatusCode::BAD_GATEWAY, format!("请求后端服务失败: {}", e))
        })?;
    
    if response.status().is_success() {
        let body = response
            .json::<Value>()
            .await
            .map_err(|e| {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("解析响应失败: {}", e))
            })?;
        Ok(Json(body))
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        Err((StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY), text))
    }
}
