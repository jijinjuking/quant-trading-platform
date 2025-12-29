//! # 认证处理器
//!
//! 本文件定义认证相关的 HTTP 处理器。
//!
//! ## 所属层
//! Interface Layer > HTTP > Handlers
//!
//! ## 端点
//! - `POST /api/v1/auth/login`: 用户登录
//! - `POST /api/v1/auth/register`: 用户注册

use axum::Json;
use serde_json::Value;

/// 用户登录处理器
///
/// 处理用户登录请求，验证凭证并返回访问令牌。
///
/// # 返回值
/// JSON 格式的登录响应
///
/// # TODO
/// - 接收登录请求 DTO
/// - 调用 AuthService 进行认证
/// - 返回 JWT Token
///
/// # 响应示例
/// ```json
/// {
///     "message": "login endpoint"
/// }
/// ```
pub async fn login() -> Json<Value> {
    // TODO: 实现登录逻辑
    // 1. 解析请求体（email, password）
    // 2. 调用 AuthService.login()
    // 3. 返回 JWT Token
    Json(serde_json::json!({
        "message": "login endpoint"
    }))
}

/// 用户注册处理器
///
/// 处理用户注册请求，创建新用户账户。
///
/// # 返回值
/// JSON 格式的注册响应
///
/// # TODO
/// - 接收注册请求 DTO
/// - 验证输入数据
/// - 调用 UserService 创建用户
/// - 返回创建结果
///
/// # 响应示例
/// ```json
/// {
///     "message": "register endpoint"
/// }
/// ```
pub async fn register() -> Json<Value> {
    // TODO: 实现注册逻辑
    // 1. 解析请求体（username, email, password）
    // 2. 验证输入数据
    // 3. 调用 UserService.create_user()
    // 4. 返回创建结果
    Json(serde_json::json!({
        "message": "register endpoint"
    }))
}
