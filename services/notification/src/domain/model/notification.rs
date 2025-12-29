//! # 通知领域模型
//!
//! 本文件定义通知服务的核心领域模型，包括通知实体及其相关枚举。
//!
//! ## 模型说明
//! - `Notification`: 通知实体，表示一条待发送或已发送的通知
//! - `NotificationChannel`: 通知渠道枚举
//! - `NotificationStatus`: 通知状态枚举
//!
//! ## 架构位置
//! 属于领域层（Domain Layer）的模型模块，是业务核心。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 通知实体
///
/// 表示系统中的一条通知消息，包含发送目标、内容和状态信息。
///
/// # 字段说明
/// - `id`: 通知唯一标识
/// - `user_id`: 接收用户ID
/// - `channel`: 发送渠道
/// - `title`: 通知标题
/// - `content`: 通知内容
/// - `status`: 当前状态
/// - `created_at`: 创建时间
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// 通知唯一标识符
    pub id: Uuid,
    /// 接收通知的用户ID
    pub user_id: Uuid,
    /// 通知发送渠道
    pub channel: NotificationChannel,
    /// 通知标题
    pub title: String,
    /// 通知正文内容
    pub content: String,
    /// 通知当前状态
    pub status: NotificationStatus,
    /// 通知创建时间（UTC）
    pub created_at: DateTime<Utc>,
}

/// 通知渠道枚举
///
/// 定义通知可以通过哪些渠道发送给用户。
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// WebSocket 实时推送
    WebSocket,
    /// 电子邮件
    Email,
    /// 短信
    Sms,
    /// 移动端推送通知
    Push,
}

/// 通知状态枚举
///
/// 表示通知在生命周期中的当前状态。
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationStatus {
    /// 待发送
    Pending,
    /// 已发送成功
    Sent,
    /// 发送失败
    Failed,
}
