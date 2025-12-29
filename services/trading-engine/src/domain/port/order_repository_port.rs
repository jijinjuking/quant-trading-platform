//! # 订单仓储端口 (Order Repository Port)
//! 
//! 定义订单持久化的抽象接口。
//! 
//! ## Hexagonal 架构说明
//! - 这是一个「出站端口」(Outbound Port)
//! - Domain 层通过此 trait 进行订单的持久化操作
//! - Infrastructure 层提供具体实现（如 PostgreSQL、Redis）
//! 
//! ## Repository 模式
//! Repository 是 DDD 中的重要模式，用于：
//! - 封装数据访问逻辑
//! - 提供类似集合的接口操作聚合根
//! - 隔离 Domain 与具体存储技术

// ============================================================================
// 外部依赖导入
// ============================================================================

use uuid::Uuid;  // UUID - 用于订单和用户标识
use crate::domain::model::order::Order;  // 订单模型

// ============================================================================
// 订单仓储端口 Trait 定义
// ============================================================================

/// 订单仓储端口 - Domain 层定义的抽象接口
/// 
/// 定义了订单持久化所需的所有操作。
/// Infrastructure 层的具体仓储实现必须实现此 trait。
/// 
/// # 实现要求
/// - `Send + Sync`: 支持跨线程安全使用
/// - 所有方法只使用 Domain 对象，不暴露 DB 类型
/// 
/// # 示例实现
/// ```ignore
/// impl OrderRepositoryPort for PostgresOrderRepository {
///     fn save(&self, order: &Order) -> bool {
///         // Domain → DTO → SQL 执行
///     }
/// }
/// ```
pub trait OrderRepositoryPort: Send + Sync {
    /// 保存订单 - 持久化订单到存储
    /// 
    /// # 参数
    /// - `order`: 要保存的订单
    /// 
    /// # 返回
    /// - `true`: 保存成功
    /// - `false`: 保存失败
    fn save(&self, order: &Order) -> bool;
    
    /// 根据ID查询订单
    /// 
    /// # 参数
    /// - `id`: 订单唯一标识符
    /// 
    /// # 返回
    /// - `Some(Order)`: 找到订单
    /// - `None`: 订单不存在
    fn find_by_id(&self, id: Uuid) -> Option<Order>;
    
    /// 查询用户所有订单
    /// 
    /// # 参数
    /// - `user_id`: 用户唯一标识符
    /// 
    /// # 返回
    /// - 该用户的所有订单列表（可能为空）
    fn find_by_user_id(&self, user_id: Uuid) -> Vec<Order>;
}
