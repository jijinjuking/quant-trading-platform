//! # 风控端口工厂
//!
//! 路径: services/trading-engine/src/bootstrap/risk.rs
//!
//! ## 职责
//! 创建 OrderRiskPort 和 RiskStatePort 实现
//!
//! ## 风控架构
//! - OrderRiskPort: 调用远程 risk-management 服务 (8085)
//! - RiskStatePort: 本地维护账户状态（余额、持仓、未完成订单）
//!
//! ## v1.1 集成重构
//! - RiskState 创建时不再自动初始化
//! - 初始化统一由 RiskStateCoordinator.rebuild() 触发
//! - 禁止在此模块调用 RiskStateInitializer

use std::sync::Arc;

use crate::domain::port::order_risk_port::OrderRiskPort;
use crate::domain::port::risk_state_port::RiskStatePort;
use crate::infrastructure::risk::{InMemoryRiskStateAdapter, RemoteRiskAdapter};

/// 创建风控端口（远程模式）
///
/// # 参数
/// - `url`: risk-management 服务 URL
///
/// # 返回
/// - `Arc<dyn OrderRiskPort>`: 风控端口实例
pub fn create_risk_port(url: Option<String>) -> Arc<dyn OrderRiskPort> {
    let url = url.unwrap_or_else(|| "http://localhost:8085".to_string());
    tracing::info!(url = %url, "使用远程风控服务 (risk-management)");
    Arc::new(RemoteRiskAdapter::new(url))
}

/// 创建 RiskStatePort（空状态）
///
/// # 重要变更 (v1.1 集成重构)
/// - 此函数只创建空的 RiskState，不执行初始化
/// - 初始化必须由 RiskStateCoordinator.rebuild(Startup) 触发
/// - 禁止在此函数内调用 RiskStateInitializer
///
/// # 参数
/// - `_binance_api_key`: 保留参数（不再使用，初始化由 Coordinator 负责）
/// - `_binance_secret_key`: 保留参数（不再使用）
/// - `_binance_base_url`: 保留参数（不再使用）
///
/// # 返回
/// - `Arc<dyn RiskStatePort>`: 空的风控状态端口实例
pub async fn create_risk_state(
    _binance_api_key: Option<String>,
    _binance_secret_key: Option<String>,
    _binance_base_url: String,
) -> Arc<dyn RiskStatePort> {
    // v1.1 集成重构: 只创建空状态，不初始化
    // 初始化统一由 RiskStateCoordinator.rebuild(Startup) 触发
    let risk_state: Arc<dyn RiskStatePort> = Arc::new(InMemoryRiskStateAdapter::new());
    tracing::info!("RiskStatePort 已创建（空状态，等待 Coordinator 初始化）");
    risk_state
}
