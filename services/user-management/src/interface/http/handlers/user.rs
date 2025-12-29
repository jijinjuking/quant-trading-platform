//! # 用户处理器
//!
//! 本文件定义用户相关的 HTTP 处理器。
//!
//! ## 所属层
//! Interface Layer > HTTP > Handlers
//!
//! ## 端点
//! - `GET /api/v1/user/profile`: 获取用户资料

use axum::Json;
use serde_json::Value;

/// 获取用户资料处理器
///
/// 返回当前登录用户的资料信息。
///
/// # 返回值
/// JSON 格式的用户资料响应
///
/// # TODO
/// - 从请求中提取用户身份（JWT）
/// - 调用 UserService 获取用户信息
/// - 返回用户资料 DTO
///
/// # 响应示例
/// ```json
/// {
///     "message": "profile endpoint"
/// }
/// ```
pub async fn get_profile() -> Json<Value> {
    // TODO: 实现获取用户资料逻辑
    // 1. 从请求头提取 JWT Token
    // 2. 验证 Token 并获取用户 ID
    // 3. 调用 UserService.get_user()
    // 4. 返回用户资料 DTO
    Json(serde_json::json!({
        "message": "profile endpoint"
    }))
}
