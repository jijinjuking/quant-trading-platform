//! # 持仓管理处理器
//!
//! 代理转发持仓相关请求到 trading-engine 服务

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::Value;
use crate::state::AppState;

/// 获取持仓列表
///
/// 代理请求到 trading-engine 服务
pub async fn list_positions(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let url = format!("{}/api/v1/positions", state.config.services.trading_engine);
    
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
