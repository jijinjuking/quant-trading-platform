//! # 订单生命周期服务 (Order Lifecycle Service)
//!
//! 路径: services/trading-engine/src/application/service/order_lifecycle_service.rs
//!
//! ## 职责
//! 管理订单生命周期，包括：
//! - 订单超时检测与处理
//! - 过期订单从 RiskState 中移除
//!
//! ## 安全修补 v1.1
//! 防止长期未成交订单永久占用 RiskState.open_orders。
//!
//! ## 架构约束
//! - 只在 ExecutionService 层面操作
//! - 不修改任何 Port trait
//! - 不引入数据库

use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use tracing::{debug, info, warn};

use crate::domain::port::risk_state_port::RiskStatePort;

/// 订单生命周期配置
#[derive(Debug, Clone)]
pub struct OrderLifecycleConfig {
    /// 订单超时时间（毫秒）
    /// 默认: 5 分钟 = 300_000 ms
    pub order_ttl_ms: i64,
    /// 检查间隔（毫秒）
    /// 默认: 30 秒 = 30_000 ms
    pub check_interval_ms: u64,
    /// 是否启用超时检查
    pub enabled: bool,
}

impl Default for OrderLifecycleConfig {
    fn default() -> Self {
        Self {
            order_ttl_ms: 300_000,      // 5 分钟
            check_interval_ms: 30_000,  // 30 秒
            enabled: true,
        }
    }
}

impl OrderLifecycleConfig {
    /// 从环境变量创建配置
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(v) = std::env::var("ORDER_TTL_MS") {
            if let Ok(ttl) = v.parse::<i64>() {
                config.order_ttl_ms = ttl;
            }
        }

        if let Ok(v) = std::env::var("ORDER_CHECK_INTERVAL_MS") {
            if let Ok(interval) = v.parse::<u64>() {
                config.check_interval_ms = interval;
            }
        }

        if let Ok(v) = std::env::var("ORDER_LIFECYCLE_ENABLED") {
            config.enabled = v.to_lowercase() == "true" || v == "1";
        }

        config
    }
}

/// 过期订单信息
#[derive(Debug, Clone)]
pub struct ExpiredOrder {
    /// 订单 ID
    pub order_id: String,
    /// 交易对
    pub symbol: String,
    /// 方向
    pub side: String,
    /// 数量
    pub quantity: Decimal,
    /// 创建时间（毫秒时间戳）
    pub created_at: i64,
    /// 过期时间（毫秒时间戳）
    pub expired_at: i64,
}

/// 订单生命周期服务
///
/// 负责检测和处理超时订单。
/// 超时订单将被标记为 Expired 并从 RiskState.open_orders 中移除。
pub struct OrderLifecycleService {
    config: OrderLifecycleConfig,
    risk_state: Arc<dyn RiskStatePort>,
}

impl OrderLifecycleService {
    /// 创建订单生命周期服务
    pub fn new(config: OrderLifecycleConfig, risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self { config, risk_state }
    }

    /// 从环境变量创建
    pub fn from_env(risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self::new(OrderLifecycleConfig::from_env(), risk_state)
    }

    /// 检查并处理超时订单
    ///
    /// 遍历所有 open_orders，检查是否超时。
    /// 超时订单将被移除并返回。
    ///
    /// # 返回
    /// - 被移除的过期订单列表
    pub async fn check_expired_orders(&self) -> Vec<ExpiredOrder> {
        if !self.config.enabled {
            return Vec::new();
        }

        let now = Utc::now().timestamp_millis();
        let ttl = self.config.order_ttl_ms;

        // 获取当前快照
        let snapshot = match self.risk_state.get_snapshot().await {
            Ok(s) => s,
            Err(e) => {
                warn!(error = %e, "获取风控状态快照失败，跳过超时检查");
                return Vec::new();
            }
        };

        let mut expired_orders = Vec::new();

        for order in &snapshot.open_orders {
            let age = now - order.created_at;

            if age > ttl {
                // 订单已超时
                info!(
                    order_id = %order.order_id,
                    symbol = %order.symbol,
                    age_ms = age,
                    ttl_ms = ttl,
                    "订单超时，标记为 Expired"
                );

                // 从 RiskState 移除
                self.risk_state.remove_open_order(&order.order_id).await;

                expired_orders.push(ExpiredOrder {
                    order_id: order.order_id.clone(),
                    symbol: order.symbol.clone(),
                    side: order.side.clone(),
                    quantity: order.quantity,
                    created_at: order.created_at,
                    expired_at: now,
                });
            } else {
                debug!(
                    order_id = %order.order_id,
                    age_ms = age,
                    remaining_ms = ttl - age,
                    "订单未超时"
                );
            }
        }

        if !expired_orders.is_empty() {
            info!(
                count = expired_orders.len(),
                "已处理超时订单"
            );
        }

        expired_orders
    }

    /// 启动定时检查循环
    ///
    /// 在后台定期检查超时订单。
    /// 此方法会阻塞，应在 tokio::spawn 中调用。
    pub async fn run(&self) {
        if !self.config.enabled {
            info!("订单生命周期服务已禁用");
            return;
        }

        info!(
            ttl_ms = self.config.order_ttl_ms,
            interval_ms = self.config.check_interval_ms,
            "订单生命周期服务启动"
        );

        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(
                self.config.check_interval_ms,
            ))
            .await;

            let expired = self.check_expired_orders().await;

            if !expired.is_empty() {
                debug!(
                    count = expired.len(),
                    "本轮检查完成，移除 {} 个超时订单",
                    expired.len()
                );
            }
        }
    }

    /// 获取配置
    pub fn config(&self) -> &OrderLifecycleConfig {
        &self.config
    }
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::port::risk_state_port::RiskOpenOrder;
    use crate::infrastructure::risk::InMemoryRiskStateAdapter;

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap_or_default()
    }

    #[tokio::test]
    async fn test_no_expired_orders() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let config = OrderLifecycleConfig {
            order_ttl_ms: 300_000, // 5 分钟
            check_interval_ms: 1000,
            enabled: true,
        };
        let service = OrderLifecycleService::new(config, risk_state.clone());

        // 添加一个刚创建的订单
        let now = Utc::now().timestamp_millis();
        risk_state
            .add_open_order(RiskOpenOrder {
                order_id: "order1".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "BUY".to_string(),
                quantity: dec("0.1"),
                price: dec("50000"),
                created_at: now,
            })
            .await;

        // 检查超时订单
        let expired = service.check_expired_orders().await;
        assert!(expired.is_empty(), "新订单不应该超时");

        // 验证订单仍在
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.open_orders.len(), 1);
    }

    #[tokio::test]
    async fn test_expired_order_removed() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let config = OrderLifecycleConfig {
            order_ttl_ms: 1000, // 1 秒（测试用）
            check_interval_ms: 100,
            enabled: true,
        };
        let service = OrderLifecycleService::new(config, risk_state.clone());

        // 添加一个已经超时的订单（created_at 在 2 秒前）
        let now = Utc::now().timestamp_millis();
        risk_state
            .add_open_order(RiskOpenOrder {
                order_id: "expired_order".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "BUY".to_string(),
                quantity: dec("0.1"),
                price: dec("50000"),
                created_at: now - 2000, // 2 秒前
            })
            .await;

        // 检查超时订单
        let expired = service.check_expired_orders().await;
        assert_eq!(expired.len(), 1, "应该有 1 个超时订单");
        assert_eq!(expired[0].order_id, "expired_order");

        // 验证订单已被移除
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert!(snapshot.open_orders.is_empty(), "超时订单应该被移除");
    }

    #[tokio::test]
    async fn test_mixed_orders() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let config = OrderLifecycleConfig {
            order_ttl_ms: 1000, // 1 秒
            check_interval_ms: 100,
            enabled: true,
        };
        let service = OrderLifecycleService::new(config, risk_state.clone());

        let now = Utc::now().timestamp_millis();

        // 添加一个超时订单
        risk_state
            .add_open_order(RiskOpenOrder {
                order_id: "expired".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "BUY".to_string(),
                quantity: dec("0.1"),
                price: dec("50000"),
                created_at: now - 2000, // 超时
            })
            .await;

        // 添加一个未超时订单
        risk_state
            .add_open_order(RiskOpenOrder {
                order_id: "active".to_string(),
                symbol: "ETHUSDT".to_string(),
                side: "SELL".to_string(),
                quantity: dec("1.0"),
                price: dec("3000"),
                created_at: now, // 刚创建
            })
            .await;

        // 检查超时订单
        let expired = service.check_expired_orders().await;
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0].order_id, "expired");

        // 验证只有未超时订单保留
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.open_orders.len(), 1);
        assert_eq!(snapshot.open_orders[0].order_id, "active");
    }

    #[tokio::test]
    async fn test_disabled_service() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let config = OrderLifecycleConfig {
            order_ttl_ms: 1,
            check_interval_ms: 100,
            enabled: false, // 禁用
        };
        let service = OrderLifecycleService::new(config, risk_state.clone());

        let now = Utc::now().timestamp_millis();
        risk_state
            .add_open_order(RiskOpenOrder {
                order_id: "order1".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "BUY".to_string(),
                quantity: dec("0.1"),
                price: dec("50000"),
                created_at: now - 10000, // 很久以前
            })
            .await;

        // 禁用时不应该移除任何订单
        let expired = service.check_expired_orders().await;
        assert!(expired.is_empty());

        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.open_orders.len(), 1);
    }

    #[test]
    fn test_config_from_env() {
        // 测试默认配置
        let config = OrderLifecycleConfig::default();
        assert_eq!(config.order_ttl_ms, 300_000);
        assert_eq!(config.check_interval_ms, 30_000);
        assert!(config.enabled);
    }
}
