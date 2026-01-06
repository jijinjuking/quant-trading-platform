//! # 风控状态协调器 (Risk State Coordinator)
//!
//! 路径: services/trading-engine/src/application/service/risk_state_coordinator.rs
//!
//! ## 职责
//! RiskState 的唯一协调者（不是业务判断者）。
//! 统一管理以下行为：
//! - RiskState 初始化
//! - 获取 snapshot
//! - 启动 OrderLifecycleService
//! - 交易所 WS 重连后状态修复
//!
//! ## 架构约束
//! - 不修改任何 Port trait
//! - 不引入数据库
//! - 不改变业务语义
//! - 只做结构性收口
//!
//! ## 禁止事项
//! - 任何 Service 自己 spawn OrderLifecycleService
//! - 在 reconnect 回调中直接操作 RiskState

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::domain::port::exchange_query_port::ExchangeQueryPort;
use crate::domain::port::risk_state_port::{RiskStatePort, RiskStateSnapshot};
use crate::application::service::risk_state_initializer::RiskStateInitializer;

/// 重建原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebuildReason {
    /// 服务启动
    Startup,
    /// WebSocket 重连
    WsReconnect,
    /// 手动触发
    Manual,
}

impl std::fmt::Display for RebuildReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RebuildReason::Startup => write!(f, "startup"),
            RebuildReason::WsReconnect => write!(f, "ws_reconnect"),
            RebuildReason::Manual => write!(f, "manual"),
        }
    }
}

/// 风控状态协调器
///
/// RiskState 的唯一协调者。
/// 所有对 RiskState 的初始化、snapshot 获取、状态修复
/// 都必须经由此协调器。
pub struct RiskStateCoordinator {
    /// 风控状态端口（共享实例）
    risk_state: Arc<dyn RiskStatePort>,
    /// 交易所查询端口（用于同步状态）
    exchange_query: Option<Arc<dyn ExchangeQueryPort>>,
    /// 是否已完成初始化
    initialized: RwLock<bool>,
}

impl RiskStateCoordinator {
    /// 创建协调器
    ///
    /// # 参数
    /// - `risk_state`: 风控状态端口
    /// - `exchange_query`: 交易所查询端口（用于同步状态）
    pub fn new(
        risk_state: Arc<dyn RiskStatePort>,
        exchange_query: Option<Arc<dyn ExchangeQueryPort>>,
    ) -> Self {
        Self {
            risk_state,
            exchange_query,
            initialized: RwLock::new(false),
        }
    }

    // =========================================================================
    // 【统一入口 1】状态重建
    // =========================================================================

    /// 重建风控状态（统一入口）
    ///
    /// 从交易所同步最新状态到 RiskState。
    /// 可在启动时或 WS 重连后调用。
    ///
    /// # 参数
    /// - `reason`: 重建原因（用于日志）
    ///
    /// # 返回
    /// - `Ok(())`: 重建成功
    /// - `Err`: 重建失败（不影响服务运行）
    pub async fn rebuild(&self, reason: RebuildReason) -> Result<()> {
        info!(reason = %reason, "开始重建风控状态");

        let exchange = match &self.exchange_query {
            Some(eq) => eq.as_ref(),
            None => {
                warn!("未配置交易所查询端口，跳过状态重建");
                return Ok(());
            }
        };

        // 调用 RiskStateInitializer 执行实际初始化
        if let Err(e) = RiskStateInitializer::initialize(exchange, self.risk_state.as_ref()).await {
            error!(error = %e, reason = %reason, "风控状态重建失败");
            return Err(e);
        }

        // 标记已初始化
        {
            let mut initialized = self.initialized.write().await;
            *initialized = true;
        }

        info!(reason = %reason, "风控状态重建完成");
        Ok(())
    }

    // =========================================================================
    // 【统一入口 2】获取 Snapshot
    // =========================================================================

    /// 获取风控状态快照（统一入口）
    ///
    /// 所有需要读取 RiskState 的地方都应该通过此方法。
    /// 这确保了 snapshot 获取的一致性。
    pub async fn get_snapshot(&self) -> Result<RiskStateSnapshot> {
        self.risk_state.get_snapshot().await
    }

    // =========================================================================
    // 【统一入口 3】WS 重连后状态修复
    // =========================================================================

    /// WS 重连后状态修复（统一入口）
    ///
    /// 当 BinanceFillStream 重连成功后，调用此方法。
    /// 内部会重新从交易所同步状态。
    ///
    /// # 注意
    /// 此方法不会 panic，失败只记录日志。
    pub async fn notify_reconnect(&self) {
        info!("收到 WS 重连通知，开始状态修复");

        if let Err(e) = self.rebuild(RebuildReason::WsReconnect).await {
            error!(error = %e, "WS 重连后状态修复失败");
        } else {
            info!("WS 重连后状态修复完成");
        }
    }

    // =========================================================================
    // 【辅助方法】
    // =========================================================================

    /// 获取 RiskState 引用（用于传递给其他服务）
    ///
    /// # 使用场景
    /// - 传递给 ExecutionService
    /// - 传递给 OrderLifecycleService
    /// - 传递给 OrderRiskAdapter
    pub fn risk_state(&self) -> Arc<dyn RiskStatePort> {
        Arc::clone(&self.risk_state)
    }

    /// 检查是否已初始化
    pub async fn is_initialized(&self) -> bool {
        *self.initialized.read().await
    }
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::risk::InMemoryRiskStateAdapter;

    #[tokio::test]
    async fn test_coordinator_without_exchange() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let coordinator = RiskStateCoordinator::new(risk_state.clone(), None);

        // 没有交易所查询端口时，重建应该成功但跳过
        let result = coordinator.rebuild(RebuildReason::Startup).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_coordinator_risk_state_shared() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let coordinator = RiskStateCoordinator::new(risk_state.clone(), None);

        // 验证返回的是同一个实例
        let state1 = coordinator.risk_state();
        let state2 = coordinator.risk_state();
        
        // 通过 Arc::ptr_eq 验证是同一个实例
        assert!(Arc::ptr_eq(&state1, &state2));
    }

    #[test]
    fn test_rebuild_reason_display() {
        assert_eq!(RebuildReason::Startup.to_string(), "startup");
        assert_eq!(RebuildReason::WsReconnect.to_string(), "ws_reconnect");
        assert_eq!(RebuildReason::Manual.to_string(), "manual");
    }

    #[tokio::test]
    async fn test_coordinator_initialization_flag() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let coordinator = RiskStateCoordinator::new(risk_state.clone(), None);

        // 初始状态应该是未初始化
        assert!(!coordinator.is_initialized().await);

        // 执行重建后应该标记为已初始化
        coordinator.rebuild(RebuildReason::Startup).await.ok();
        assert!(coordinator.is_initialized().await);
    }

    #[tokio::test]
    async fn test_get_snapshot() {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let coordinator = RiskStateCoordinator::new(risk_state.clone(), None);

        // 应该能获取空快照
        let snapshot = coordinator.get_snapshot().await;
        assert!(snapshot.is_ok());
        
        let snapshot = snapshot.unwrap();
        assert!(snapshot.balances.is_empty());
        assert!(snapshot.open_orders.is_empty());
    }
}
