//! # 消息推送端口
//!
//! 本文件定义消息推送的抽象接口（Port），是领域层对外部消息服务的依赖抽象。
//!
//! ## 架构位置
//! 属于领域层（Domain Layer）的端口模块。
//!
//! ## 设计原则
//! - 只定义 trait，不包含任何实现
//! - 入参和出参只使用领域对象或基础类型
//! - 不依赖任何外部框架或库的类型

use crate::domain::model::notification::Notification;

/// 通知推送端口
///
/// 定义通知服务对外部消息推送能力的抽象接口。
/// 具体实现由基础设施层的适配器提供。
///
/// # 实现要求
/// - 必须是线程安全的（Send + Sync）
/// - 实现类应处理所有外部服务的错误
#[allow(dead_code)]
pub trait NotificationPort: Send + Sync {
    /// 发送 WebSocket 消息
    ///
    /// 通过 WebSocket 连接向指定用户推送实时通知。
    ///
    /// # 参数
    /// - `user_id`: 目标用户ID
    /// - `notification`: 要发送的通知内容
    ///
    /// # 返回值
    /// - `true`: 发送成功
    /// - `false`: 发送失败
    fn send_websocket(&self, user_id: &str, notification: &Notification) -> bool;
    
    /// 发送邮件通知
    ///
    /// 通过邮件服务向指定邮箱发送通知。
    ///
    /// # 参数
    /// - `email`: 目标邮箱地址
    /// - `notification`: 要发送的通知内容
    ///
    /// # 返回值
    /// - `true`: 发送成功
    /// - `false`: 发送失败
    fn send_email(&self, email: &str, notification: &Notification) -> bool;
    
    /// 广播消息
    ///
    /// 向所有在线用户广播通知消息。
    ///
    /// # 参数
    /// - `notification`: 要广播的通知内容
    ///
    /// # 返回值
    /// - `true`: 广播成功
    /// - `false`: 广播失败
    fn broadcast(&self, notification: &Notification) -> bool;
}
