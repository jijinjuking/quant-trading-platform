//! # 订单生命周期服务工厂 (v1.1)
//!
//! 路径: services/trading-engine/src/bootstrap/lifecycle.rs
//!
//! ## 职责
//! 创建 OrderLifecycleService 实例
//!
//! ## 安全修补 v1.1
//! 订单超时处理：防止长期未成交订单永久占用 RiskState.open_orders

use std::sync::Arc;

use crate::domain::port::risk_state_port::RiskStatePort;
use crate::application::service::order_lifecycle_service::{
    OrderLifecycleConfig,
    OrderLifecycleService,
};

/// 创建订单生命周期服务
///
/// # 参数
/// - `risk_state`: 风控状态端口
///
/// # 返回
/// - `OrderLifecycleService`: 订单生命周期服务实例
pub fn create_order_lifecycle_service(
    risk_state: Arc<dyn RiskStatePort>,
) -> OrderLifecycleService {
    let config = OrderLifecycleConfig::from_env();
    tracing::info!(
        ttl_ms = config.order_ttl_ms,
        interval_ms = config.check_interval_ms,
        enabled = config.enabled,
        "创建订单生命周期服务"
    );
    OrderLifecycleService::new(config, risk_state)
}
