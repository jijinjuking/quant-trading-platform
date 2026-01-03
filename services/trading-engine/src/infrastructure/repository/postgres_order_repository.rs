//! # PostgreSQL 订单仓储 (PostgreSQL Order Repository)
//!
//! 实现 OrderRepositoryPort，将订单数据存储到 PostgreSQL。

use anyhow::{Context, Result};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use rust_decimal::Decimal;
use tracing::{debug, info};
use uuid::Uuid;

use crate::domain::model::order::{Order, OrderSide, OrderStatus, OrderType};
use crate::domain::model::trade::Trade;
use crate::domain::port::order_repository_port::OrderRepositoryPort;

/// PostgreSQL 订单仓储
pub struct PostgresOrderRepository {
    pool: Pool,
}

impl PostgresOrderRepository {
    /// 创建仓储实例
    pub fn new(pool: Pool) -> Self {
        info!("PostgreSQL 订单仓储初始化");
        Self { pool }
    }
}

#[async_trait]
impl OrderRepositoryPort for PostgresOrderRepository {
    /// 保存订单
    async fn save_order(&self, order: &Order) -> Result<()> {
        let client = self.pool.get().await.context("获取数据库连接失败")?;

        let sql = r#"
            INSERT INTO orders (
                id, user_id, symbol, order_type, side, quantity, price,
                status, filled_quantity, average_price, created_at, updated_at,
                metadata
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            )
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                filled_quantity = EXCLUDED.filled_quantity,
                average_price = EXCLUDED.average_price,
                updated_at = EXCLUDED.updated_at
        "#;

        // 构建 metadata JSON
        let metadata = serde_json::json!({
            "strategy_id": order.strategy_id,
            "exchange_order_id": order.exchange_order_id,
        });

        client
            .execute(
                sql,
                &[
                    &order.id,
                    &order.user_id,
                    &order.symbol,
                    &order.order_type.as_str(),
                    &order.side.as_str(),
                    &order.quantity.to_string(),
                    &order.price.map(|p| p.to_string()),
                    &order.status.as_str(),
                    &order.filled_quantity.to_string(),
                    &order.average_price.map(|p| p.to_string()),
                    &order.created_at,
                    &order.updated_at,
                    &metadata,
                ],
            )
            .await
            .context("保存订单失败")?;

        debug!("订单已保存: {}", order.id);
        Ok(())
    }

    /// 更新订单状态
    async fn update_order_status(&self, order_id: Uuid, status: &str) -> Result<()> {
        let client = self.pool.get().await.context("获取数据库连接失败")?;

        let sql = r#"
            UPDATE orders 
            SET status = $1, updated_at = NOW()
            WHERE id = $2
        "#;

        client
            .execute(sql, &[&status, &order_id])
            .await
            .context("更新订单状态失败")?;

        debug!("订单状态已更新: {} -> {}", order_id, status);
        Ok(())
    }

    /// 根据 ID 查询订单
    async fn find_order_by_id(&self, order_id: Uuid) -> Result<Option<Order>> {
        let client = self.pool.get().await.context("获取数据库连接失败")?;

        let sql = r#"
            SELECT id, user_id, symbol, order_type, side, quantity, price,
                   status, filled_quantity, average_price, created_at, updated_at,
                   metadata
            FROM orders
            WHERE id = $1
        "#;

        let row = client
            .query_opt(sql, &[&order_id])
            .await
            .context("查询订单失败")?;

        match row {
            Some(row) => {
                let order = row_to_order(&row)?;
                Ok(Some(order))
            }
            None => Ok(None),
        }
    }

    /// 保存成交记录
    async fn save_trade(&self, trade: &Trade) -> Result<()> {
        let client = self.pool.get().await.context("获取数据库连接失败")?;

        let sql = r#"
            INSERT INTO trades (
                id, order_id, user_id, symbol, side, quantity, price,
                fee, fee_currency, trade_time, exchange_trade_id, is_maker,
                metadata
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            )
        "#;

        let metadata = serde_json::json!({});

        client
            .execute(
                sql,
                &[
                    &trade.id,
                    &trade.order_id,
                    &trade.user_id,
                    &trade.symbol,
                    &trade.side.as_str(),
                    &trade.quantity.to_string(),
                    &trade.price.to_string(),
                    &trade.fee.to_string(),
                    &trade.fee_currency,
                    &trade.trade_time,
                    &trade.exchange_trade_id,
                    &trade.is_maker,
                    &metadata,
                ],
            )
            .await
            .context("保存成交记录失败")?;

        debug!("成交记录已保存: {}", trade.id);
        Ok(())
    }

    /// 查询用户订单列表
    async fn find_orders_by_user(&self, user_id: Uuid, limit: i64) -> Result<Vec<Order>> {
        let client = self.pool.get().await.context("获取数据库连接失败")?;

        let sql = r#"
            SELECT id, user_id, symbol, order_type, side, quantity, price,
                   status, filled_quantity, average_price, created_at, updated_at,
                   metadata
            FROM orders
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
        "#;

        let rows = client
            .query(sql, &[&user_id, &limit])
            .await
            .context("查询用户订单失败")?;

        let mut orders = Vec::with_capacity(rows.len());
        for row in rows {
            let order = row_to_order(&row)?;
            orders.push(order);
        }

        Ok(orders)
    }
}

/// 将数据库行转换为 Order
fn row_to_order(row: &tokio_postgres::Row) -> Result<Order> {
    let id: Uuid = row.get("id");
    let user_id: Uuid = row.get("user_id");
    let symbol: String = row.get("symbol");
    let order_type_str: String = row.get("order_type");
    let side_str: String = row.get("side");
    let quantity_str: String = row.get("quantity");
    let price_str: Option<String> = row.get("price");
    let status_str: String = row.get("status");
    let filled_quantity_str: String = row.get("filled_quantity");
    let average_price_str: Option<String> = row.get("average_price");
    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
    let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
    let metadata: serde_json::Value = row.get("metadata");

    let order_type = match order_type_str.as_str() {
        "MARKET" => OrderType::Market,
        "LIMIT" => OrderType::Limit,
        _ => OrderType::Market,
    };

    let side = match side_str.as_str() {
        "BUY" => OrderSide::Buy,
        "SELL" => OrderSide::Sell,
        _ => OrderSide::Buy,
    };

    let status = OrderStatus::from_str(&status_str).unwrap_or(OrderStatus::Pending);

    let quantity = quantity_str
        .parse::<Decimal>()
        .unwrap_or(Decimal::ZERO);
    let price = price_str.and_then(|s| s.parse::<Decimal>().ok());
    let filled_quantity = filled_quantity_str
        .parse::<Decimal>()
        .unwrap_or(Decimal::ZERO);
    let average_price = average_price_str.and_then(|s| s.parse::<Decimal>().ok());

    let strategy_id = metadata
        .get("strategy_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());
    let exchange_order_id = metadata
        .get("exchange_order_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(Order {
        id,
        user_id,
        strategy_id,
        symbol,
        order_type,
        side,
        quantity,
        price,
        status,
        filled_quantity,
        average_price,
        exchange_order_id,
        created_at,
        updated_at,
    })
}
