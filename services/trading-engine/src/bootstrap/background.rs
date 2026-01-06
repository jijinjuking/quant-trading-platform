//! # 后台服务启动模块
//!
//! 路径: services/trading-engine/src/bootstrap/background.rs
//!
//! ## 职责
//! 统一启动所有后台服务，包括：
//! - OrderLifecycleService（订单超时检测）
//! - 其他后台任务
//!
//! ## 架构约束
//! - main.rs 禁止直接 tokio::spawn 具体服务
//! - 所有后台服务必须通过此模块启动
//! - 返回 JoinHandle 用于优雅关闭

use std::sync::Arc;

use tokio::task::JoinHandle;
use tracing::{info, error};

use crate::domain::port::risk_state_port::RiskStatePort;
use crate::application::service::order_lifecycle_service::{
    OrderLifecycleConfig,
    OrderLifecycleService,
};

/// 后台服务句柄集合
///
/// 用于管理所有后台服务的生命周期。
/// 可用于优雅关闭。
pub struct BackgroundHandles {
    /// 订单生命周期服务句柄
    pub order_lifecycle: JoinHandle<()>,
    // 未来可添加更多后台服务句柄
}

impl BackgroundHandles {
    /// 中止所有后台服务
    pub fn abort_all(&self) {
        self.order_lifecycle.abort();
        info!("所有后台服务已中止");
    }
}

/// 启动所有后台服务
///
/// # 参数
/// - `risk_state`: 风控状态端口（共享实例）
///
/// # 返回
/// - `BackgroundHandles`: 后台服务句柄集合
///
/// # 启动的服务
/// - OrderLifecycleService: 订单超时检测（v1.1 安全修补）
pub fn start_background_services(
    risk_state: Arc<dyn RiskStatePort>,
) -> BackgroundHandles {
    info!("启动后台服务...");

    // 1. 启动订单生命周期服务
    let order_lifecycle_handle = start_order_lifecycle_service(risk_state);

    info!("所有后台服务已启动");

    BackgroundHandles {
        order_lifecycle: order_lifecycle_handle,
    }
}

/// 启动订单生命周期服务
///
/// # 参数
/// - `risk_state`: 风控状态端口
///
/// # 返回
/// - `JoinHandle<()>`: 服务句柄
fn start_order_lifecycle_service(
    risk_state: Arc<dyn RiskStatePort>,
) -> JoinHandle<()> {
    let config = OrderLifecycleConfig::from_env();
    
    info!(
        ttl_ms = config.order_ttl_ms,
        interval_ms = config.check_interval_ms,
        enabled = config.enabled,
        "创建订单生命周期服务"
    );

    let service = OrderLifecycleService::new(config, risk_state);

    tokio::spawn(async move {
        info!("订单生命周期服务已启动 (v1.1)");
        service.run().await;
        error!("订单生命周期服务意外退出");
    })
}
