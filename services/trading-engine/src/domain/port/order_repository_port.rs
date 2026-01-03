//! # 订单仓储端口 (Order Repository Port)
//!
//! 定义订单持久化的抽象接口。

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::model::order::Order;
use crate::domain::model::trade::Trade;

/// 订单仓储端口
#[async_trait]
pub trait OrderRepositoryPort: Send + Sync {
    /// 保存订单
    async fn save_order(&self, order: &Order) -> Result<()>;

    /// 更新订单状态
    async fn update_order_status(&self, order_id: Uuid, status: &str) -> Result<()>;

    /// 根据 ID 查询订单
    async fn find_order_by_id(&self, order_id: Uuid) -> Result<Option<Order>>;

    /// 保存成交记录
    async fn save_trade(&self, trade: &Trade) -> Result<()>;

    /// 查询用户订单列表
    async fn find_orders_by_user(&self, user_id: Uuid, limit: i64) -> Result<Vec<Order>>;
}

/// 为 Arc<T> 实现 OrderRepositoryPort
#[async_trait]
impl<T: OrderRepositoryPort> OrderRepositoryPort for Arc<T> {
    async fn save_order(&self, order: &Order) -> Result<()> {
        (**self).save_order(order).await
    }

    async fn update_order_status(&self, order_id: Uuid, status: &str) -> Result<()> {
        (**self).update_order_status(order_id, status).await
    }

    async fn find_order_by_id(&self, order_id: Uuid) -> Result<Option<Order>> {
        (**self).find_order_by_id(order_id).await
    }

    async fn save_trade(&self, trade: &Trade) -> Result<()> {
        (**self).save_trade(trade).await
    }

    async fn find_orders_by_user(&self, user_id: Uuid, limit: i64) -> Result<Vec<Order>> {
        (**self).find_orders_by_user(user_id, limit).await
    }
}
