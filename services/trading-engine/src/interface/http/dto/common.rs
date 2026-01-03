//! # 通用响应 DTO
//!
//! 定义统一的 API 响应格式。

use serde::Serialize;

/// 通用 API 响应
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    /// 是否成功
    pub success: bool,
    /// 响应数据（成功时）
    pub data: Option<T>,
    /// 错误信息（失败时）
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

/// 分页响应
#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总数
    pub total: usize,
    /// 当前页
    pub page: usize,
    /// 每页大小
    pub page_size: usize,
}
