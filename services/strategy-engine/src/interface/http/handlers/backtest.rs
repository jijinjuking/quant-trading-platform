//! # 回测处理器 (Backtest Handlers)
//! 
//! 处理策略回测请求。

use axum::Json;
use serde_json::Value;

/// 运行回测
pub async fn run_backtest() -> Json<Value> {
    // TODO: 调用 Application 层运行回测
    Json(serde_json::json!({
        "message": "backtest started"
    }))
}
