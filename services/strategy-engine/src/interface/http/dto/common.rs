//! # 通用 DTO
//!
//! 定义通用的 API 响应结构。

use serde::Serialize;

/// 通用 API 响应
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    /// 是否成功
    pub success: bool,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    /// 成功响应
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// 错误响应
    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}
