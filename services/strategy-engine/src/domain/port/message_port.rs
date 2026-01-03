//! # 消息推送端口 (Message Port)
//! 
//! 定义发布交易信号的抽象接口。

use std::sync::Arc;

use crate::domain::model::signal::Signal;

/// 消息推送端口 - Domain 层定义的抽象接口
pub trait SignalMessagePort: Send + Sync {
    /// 发布交易信号到消息队列
    fn publish_signal(&self, signal: &Signal) -> bool;
}

// Arc<T> 自动实现 SignalMessagePort
impl<T: SignalMessagePort> SignalMessagePort for Arc<T> {
    fn publish_signal(&self, signal: &Signal) -> bool {
        (**self).publish_signal(signal)
    }
}
