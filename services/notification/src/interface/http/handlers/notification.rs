//! # 通知处理器
//!
//! 本文件实现通知相关的 HTTP 接口处理器。
//!
//! ## 接口
//! - `POST /api/v1/notifications`: 发送通知
//!
//! ## 架构位置
//! 属于接口层（Interface Layer）的处理器模块。

use axum::Json;
use serde_json::Value;

/// 发送通知处理函数
///
/// 接收通知请求并发送通知（当前为骨架实现）。
///
/// # 返回值
/// JSON 格式的发送结果响应
///
/// # TODO
/// - 解析请求体中的通知参数
/// - 调用应用层服务发送通知
/// - 返回详细的发送结果
pub async fn send_notification() -> Json<Value> {
    Json(serde_json::json!({
        "message": "notification sent"
    }))
}
