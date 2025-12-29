//! # 消息推送端口 (Message Port)
//! 
//! 定义推送行情事件的抽象接口。
//! 
//! ## Hexagonal 架构说明
//! - 这是一个「出站端口」(Outbound Port)
//! - Domain 层通过此 trait 发布行情事件
//! - Infrastructure 层提供具体实现（如 KafkaProducer）

// ============================================================================
// 领域模型导入
// ============================================================================

use crate::domain::model::tick::Tick;  // Tick 模型

// ============================================================================
// 消息推送端口 Trait 定义
// ============================================================================

/// 消息推送端口 - Domain 层定义的抽象接口
/// 
/// 定义了发布行情事件所需的所有操作。
/// 
/// # 实现要求
/// - `Send + Sync`: 支持跨线程安全使用
/// - 所有方法只接收 Domain 对象，不暴露 Kafka 类型
/// 
/// # 示例实现
/// ```ignore
/// impl MessagePort for KafkaProducer {
///     fn publish_tick(&self, tick: &Tick) -> bool {
///         // Domain Tick → Kafka 消息
///     }
/// }
/// ```
pub trait MessagePort: Send + Sync {
    /// 发布 Tick 数据
    /// 
    /// 将 Tick 数据发布到消息队列，供其他服务消费。
    /// 
    /// # 参数
    /// - `tick`: 要发布的 Tick 数据
    /// 
    /// # 返回
    /// - `true`: 发布成功
    /// - `false`: 发布失败
    fn publish_tick(&self, tick: &Tick) -> bool;
    
    /// 发布市场事件
    /// 
    /// 发布通用的市场事件消息。
    /// 
    /// # 参数
    /// - `event_type`: 事件类型（如 "price_alert", "volume_spike"）
    /// - `payload`: 事件负载（JSON 字符串）
    /// 
    /// # 返回
    /// - `true`: 发布成功
    /// - `false`: 发布失败
    fn publish_event(&self, event_type: &str, payload: &str) -> bool;
}
