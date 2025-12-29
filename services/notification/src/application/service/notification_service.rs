//! # 通知应用服务
//!
//! 本文件实现通知服务的应用层逻辑，负责编排通知发送流程。
//!
//! ## 架构位置
//! 属于应用层（Application Layer），负责：
//! - 接收接口层的请求
//! - 编排领域对象和端口完成业务流程
//! - 返回处理结果
//!
//! ## 依赖规则
//! - 只依赖领域层的 trait（NotificationPort）
//! - 通过泛型参数注入具体实现，实现依赖倒置

use crate::domain::model::notification::Notification;
use crate::domain::port::message_port::NotificationPort;

/// 通知应用服务
///
/// 封装通知发送的业务流程，通过泛型参数 `P` 接收端口实现，
/// 实现依赖倒置原则（DIP）。
///
/// # 类型参数
/// - `P`: 实现了 `NotificationPort` trait 的类型
///
/// # 示例
/// ```ignore
/// let ws_manager = WebSocketManager::new();
/// let service = NotificationService::new(ws_manager);
/// service.send("user_123", &notification);
/// ```
#[allow(dead_code)]
pub struct NotificationService<P: NotificationPort> {
    /// 通知推送端口（依赖注入）
    port: P,
}

#[allow(dead_code)]
impl<P: NotificationPort> NotificationService<P> {
    /// 创建新的通知服务实例
    ///
    /// # 参数
    /// - `port`: 实现了 NotificationPort 的适配器实例
    ///
    /// # 返回值
    /// 新创建的 NotificationService 实例
    pub fn new(port: P) -> Self {
        Self { port }
    }
    
    /// 发送通知给指定用户
    ///
    /// 通过 WebSocket 向指定用户发送通知。
    ///
    /// # 参数
    /// - `user_id`: 目标用户ID
    /// - `notification`: 要发送的通知
    ///
    /// # 返回值
    /// - `true`: 发送成功
    /// - `false`: 发送失败
    pub fn send(&self, user_id: &str, notification: &Notification) -> bool {
        self.port.send_websocket(user_id, notification)
    }
    
    /// 广播通知给所有用户
    ///
    /// 向所有在线用户广播通知消息。
    ///
    /// # 参数
    /// - `notification`: 要广播的通知
    ///
    /// # 返回值
    /// - `true`: 广播成功
    /// - `false`: 广播失败
    pub fn broadcast(&self, notification: &Notification) -> bool {
        self.port.broadcast(notification)
    }
}
