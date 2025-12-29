//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::messaging::websocket::WebSocketManager;
use crate::application::service::notification_service::NotificationService;

/// 创建通知服务实例
#[allow(dead_code)]
pub fn create_notification_service() -> NotificationService<WebSocketManager> {
    let ws_manager = WebSocketManager::new();
    NotificationService::new(ws_manager)
}
