//! # 交易所端口 (Exchange Port)
//! 
//! 定义与交易所交互的抽象接口。
//! 
//! ## Hexagonal 架构说明
//! - 这是一个「出站端口」(Outbound Port)
//! - Domain 层通过此 trait 与外部交易所交互
//! - Infrastructure 层提供具体实现（如 BinanceConnector）
//! 
//! ## 依赖方向
//! ```text
//! Application → Domain::Port ← Infrastructure
//!                   ↑
//!              (trait 定义)
//! ```

// ============================================================================
// 领域模型导入
// ============================================================================

use crate::domain::model::order::Order;  // 订单模型
use crate::domain::model::trade::Trade;  // 成交模型

// ============================================================================
// 交易所端口 Trait 定义
// ============================================================================

/// 交易所端口 - Domain 层定义的抽象接口
/// 
/// 定义了与交易所交互所需的所有操作。
/// Infrastructure 层的具体交易所连接器必须实现此 trait。
/// 
/// # 实现要求
/// - `Send + Sync`: 支持跨线程安全使用
/// - 所有方法只使用 Domain 对象，不暴露 SDK 类型
/// 
/// # 示例实现
/// ```ignore
/// impl ExchangePort for BinanceConnector {
///     fn place_order(&self, order: &Order) -> Option<Trade> {
///         // SDK 调用 → DTO → Domain 转换
///     }
/// }
/// ```
pub trait ExchangePort: Send + Sync {
    /// 下单 - 向交易所提交订单
    /// 
    /// # 参数
    /// - `order`: 要提交的订单
    /// 
    /// # 返回
    /// - `Some(Trade)`: 下单成功，返回成交记录
    /// - `None`: 下单失败
    fn place_order(&self, order: &Order) -> Option<Trade>;
    
    /// 取消订单 - 撤销已提交的订单
    /// 
    /// # 参数
    /// - `order_id`: 要取消的订单ID
    /// 
    /// # 返回
    /// - `true`: 取消成功
    /// - `false`: 取消失败
    fn cancel_order(&self, order_id: &str) -> bool;
    
    /// 查询订单状态 - 获取订单当前状态
    /// 
    /// # 参数
    /// - `order_id`: 要查询的订单ID
    /// 
    /// # 返回
    /// - `Some(Order)`: 查询成功，返回订单信息
    /// - `None`: 订单不存在或查询失败
    fn query_order(&self, order_id: &str) -> Option<Order>;
}
