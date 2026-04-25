//! # 策略注册表 (Strategy Registry)
//!
//! 策略实例的句柄管理中心。
//!
//! ## DDD 定位
//! - 这是一个 **领域服务 (Domain Service)**
//! - 管理跨策略实例的规则
//! - 不包含业务逻辑，只做句柄管理
//!
//! ## 工程约束
//! - Registry 只管理句柄，不管理策略逻辑
//! - Registry 不调度、不遍历执行
//! - Registry 是轻量的、无业务逻辑的
//! - Registry 支持并发访问（使用 DashMap）

use std::sync::Arc;

use anyhow::{anyhow, Result};
use dashmap::DashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::domain::model::lifecycle_state::LifecycleState;
use crate::domain::model::strategy_handle::StrategyHandle;
use crate::domain::model::strategy_metadata::{MarketType, StrategyKind};
use crate::domain::model::{ExecutionRequest, ExecutionResult};

/// 注册表统计信息
#[derive(Debug, Clone, Default)]
pub struct RegistryStats {
    pub total: usize,
    pub running: usize,
    pub paused: usize,
    pub stopped: usize,
    pub faulted: usize,
    pub created: usize,
}

/// 策略查询条件
#[derive(Debug, Clone, Default)]
pub struct StrategyQuery {
    pub owner_id: Option<Uuid>,
    pub market_type: Option<MarketType>,
    pub symbol: Option<String>,
    pub kind: Option<StrategyKind>,
    pub lifecycle_state: Option<LifecycleState>,
    pub tags: Option<Vec<String>>,
}

impl StrategyQuery {
    pub fn all() -> Self { Self::default() }
    pub fn by_owner(owner_id: Uuid) -> Self { Self { owner_id: Some(owner_id), ..Default::default() } }
    pub fn by_symbol(symbol: impl Into<String>) -> Self { Self { symbol: Some(symbol.into()), ..Default::default() } }
    pub fn by_state(state: LifecycleState) -> Self { Self { lifecycle_state: Some(state), ..Default::default() } }

    pub fn with_owner(mut self, owner_id: Uuid) -> Self { self.owner_id = Some(owner_id); self }
    pub fn with_market_type(mut self, market_type: MarketType) -> Self { self.market_type = Some(market_type); self }
    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self { self.symbol = Some(symbol.into()); self }
    pub fn with_state(mut self, state: LifecycleState) -> Self { self.lifecycle_state = Some(state); self }

    fn matches(&self, handle: &StrategyHandle) -> bool {
        let metadata = handle.metadata();
        if let Some(owner_id) = self.owner_id { if metadata.owner_id != owner_id { return false; } }
        if let Some(market_type) = self.market_type { if metadata.market_type != market_type { return false; } }
        if let Some(ref symbol) = self.symbol { if metadata.symbol != *symbol { return false; } }
        if let Some(ref kind) = self.kind { if metadata.kind != *kind { return false; } }
        if let Some(state) = self.lifecycle_state { if handle.lifecycle_state() != state { return false; } }
        if let Some(ref tags) = self.tags {
            let has_any_tag = tags.iter().any(|t| metadata.tags.contains(t));
            if !has_any_tag && !tags.is_empty() { return false; }
        }
        true
    }
}

/// 策略注册表
pub struct StrategyRegistry {
    handles: DashMap<Uuid, Arc<StrategyHandle>>,
}


impl StrategyRegistry {
    pub fn new() -> Self {
        info!("创建策略注册表");
        Self { handles: DashMap::new() }
    }

    // =========================================================================
    // 注册与注销
    // =========================================================================

    pub fn register(&self, handle: Arc<StrategyHandle>) -> Result<Uuid> {
        let instance_id = handle.instance_id();
        if self.handles.contains_key(&instance_id) {
            return Err(anyhow!("策略实例 {} 已存在", instance_id));
        }
        self.handles.insert(instance_id, handle);
        info!(instance_id = %instance_id, "策略实例已注册");
        Ok(instance_id)
    }

    pub fn unregister(&self, instance_id: Uuid) -> Result<()> {
        let handle = self.handles.get(&instance_id)
            .ok_or_else(|| anyhow!("策略实例 {} 不存在", instance_id))?;
        if !handle.can_unregister() {
            return Err(anyhow!("策略实例 {} 当前状态 {} 不允许注销", instance_id, handle.lifecycle_state()));
        }
        drop(handle);
        self.handles.remove(&instance_id);
        info!(instance_id = %instance_id, "策略实例已注销");
        Ok(())
    }

    pub fn force_unregister(&self, instance_id: Uuid) -> bool {
        let removed = self.handles.remove(&instance_id).is_some();
        if removed { warn!(instance_id = %instance_id, "策略实例已强制注销"); }
        removed
    }

    // =========================================================================
    // 查找
    // =========================================================================

    pub fn get(&self, instance_id: Uuid) -> Option<Arc<StrategyHandle>> {
        self.handles.get(&instance_id).map(|r| Arc::clone(&r))
    }

    pub fn contains(&self, instance_id: Uuid) -> bool {
        self.handles.contains_key(&instance_id)
    }

    pub fn query(&self, query: &StrategyQuery) -> Vec<Arc<StrategyHandle>> {
        self.handles.iter().filter(|entry| query.matches(entry.value())).map(|entry| Arc::clone(entry.value())).collect()
    }

    pub fn list_ids(&self) -> Vec<Uuid> {
        self.handles.iter().map(|entry| *entry.key()).collect()
    }

    pub fn len(&self) -> usize { self.handles.len() }
    pub fn is_empty(&self) -> bool { self.handles.is_empty() }

    // =========================================================================
    // 执行路由
    // =========================================================================

    /// 执行策略（委托给 Handle）
    pub fn execute(&self, instance_id: Uuid, request: &ExecutionRequest) -> Result<ExecutionResult> {
        let handle = self.handles.get(&instance_id)
            .ok_or_else(|| anyhow!("策略实例 {} 不存在", instance_id))?;
        debug!(instance_id = %instance_id, request_id = %request.request_id, "路由执行请求");
        handle.execute(request)
    }

    /// 批量执行
    pub fn execute_batch(&self, query: &StrategyQuery, request: &ExecutionRequest) -> Vec<(Uuid, Result<ExecutionResult>)> {
        let handles = self.query(query);
        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            let id = handle.instance_id();
            let result = handle.execute(request);
            results.push((id, result));
        }
        results
    }


    // =========================================================================
    // 生命周期管理
    // =========================================================================

    pub fn start(&self, instance_id: Uuid) -> Result<()> {
        let handle = self.handles.get(&instance_id).ok_or_else(|| anyhow!("策略实例 {} 不存在", instance_id))?;
        handle.start().map_err(|e| anyhow!("{}", e))?;
        info!(instance_id = %instance_id, "策略已启动");
        Ok(())
    }

    pub fn pause(&self, instance_id: Uuid) -> Result<()> {
        let handle = self.handles.get(&instance_id).ok_or_else(|| anyhow!("策略实例 {} 不存在", instance_id))?;
        handle.pause().map_err(|e| anyhow!("{}", e))?;
        info!(instance_id = %instance_id, "策略已暂停");
        Ok(())
    }

    pub fn resume(&self, instance_id: Uuid) -> Result<()> {
        let handle = self.handles.get(&instance_id).ok_or_else(|| anyhow!("策略实例 {} 不存在", instance_id))?;
        handle.resume().map_err(|e| anyhow!("{}", e))?;
        info!(instance_id = %instance_id, "策略已恢复");
        Ok(())
    }

    pub fn stop(&self, instance_id: Uuid) -> Result<()> {
        let handle = self.handles.get(&instance_id).ok_or_else(|| anyhow!("策略实例 {} 不存在", instance_id))?;
        handle.stop().map_err(|e| anyhow!("{}", e))?;
        info!(instance_id = %instance_id, "策略已停止");
        Ok(())
    }

    pub fn restart(&self, instance_id: Uuid) -> Result<()> {
        let handle = self.handles.get(&instance_id).ok_or_else(|| anyhow!("策略实例 {} 不存在", instance_id))?;
        handle.restart().map_err(|e| anyhow!("{}", e))?;
        info!(instance_id = %instance_id, "策略已重启");
        Ok(())
    }

    pub fn stop_batch(&self, query: &StrategyQuery) -> Vec<(Uuid, Result<()>)> {
        self.query(query).into_iter().map(|handle| {
            let id = handle.instance_id();
            let result = handle.stop().map_err(|e| anyhow!("{}", e));
            (id, result)
        }).collect()
    }

    // =========================================================================
    // 统计与监控
    // =========================================================================

    pub fn stats(&self) -> RegistryStats {
        let mut stats = RegistryStats::default();
        for entry in self.handles.iter() {
            stats.total += 1;
            match entry.value().lifecycle_state() {
                LifecycleState::Created => stats.created += 1,
                LifecycleState::Running => stats.running += 1,
                LifecycleState::Paused => stats.paused += 1,
                LifecycleState::Stopped => stats.stopped += 1,
                LifecycleState::Faulted => stats.faulted += 1,
            }
        }
        stats
    }

    pub fn faulted_instances(&self) -> Vec<Arc<StrategyHandle>> {
        self.query(&StrategyQuery::by_state(LifecycleState::Faulted))
    }

    pub fn running_instances(&self) -> Vec<Arc<StrategyHandle>> {
        self.query(&StrategyQuery::by_state(LifecycleState::Running))
    }

    // =========================================================================
    // 清理
    // =========================================================================

    pub fn cleanup_stopped(&self) -> usize {
        let stopped_ids: Vec<Uuid> = self.handles.iter()
            .filter(|entry| entry.value().lifecycle_state() == LifecycleState::Stopped)
            .map(|entry| *entry.key()).collect();
        let count = stopped_ids.len();
        for id in stopped_ids { self.handles.remove(&id); }
        if count > 0 { info!(count = count, "已清理停止的策略实例"); }
        count
    }

    pub fn shutdown(&self) {
        info!("开始关闭所有策略实例");
        for entry in self.handles.iter() {
            if entry.value().lifecycle_state().can_stop() { let _ = entry.value().stop(); }
        }
        self.handles.clear();
        info!("所有策略实例已关闭");
    }
}

impl Default for StrategyRegistry {
    fn default() -> Self { Self::new() }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::strategy_metadata::StrategyMetadata;
    use crate::domain::port::strategy_executor_port::StrategyExecutorPort;

    /// 测试用的空执行器
    struct NoopExecutor;

    impl StrategyExecutorPort for NoopExecutor {
        fn execute(&self, request: &ExecutionRequest) -> Result<ExecutionResult> {
            Ok(ExecutionResult {
                request_id: request.request_id,
                has_intent: false,
                intent: None,
                execution_time_us: 0,
                error: None,
            })
        }

        fn reset(&self) -> Result<()> {
            Ok(())
        }

        fn state_snapshot(&self) -> Result<serde_json::Value> {
            Ok(serde_json::json!({}))
        }
    }

    fn create_test_handle(owner_id: Uuid, symbol: &str) -> Arc<StrategyHandle> {
        let metadata = StrategyMetadata::new(
            StrategyKind::Grid,
            MarketType::Spot,
            symbol,
            owner_id,
            format!("测试策略-{}", symbol),
        );
        let executor: Arc<dyn StrategyExecutorPort> = Arc::new(NoopExecutor);
        Arc::new(StrategyHandle::new(metadata, executor))
    }

    #[test]
    fn test_register_and_get() {
        let registry = StrategyRegistry::new();
        let owner_id = Uuid::new_v4();
        let handle = create_test_handle(owner_id, "BTCUSDT");
        let instance_id = handle.instance_id();
        assert!(registry.register(handle).is_ok());
        assert_eq!(registry.len(), 1);
        let retrieved = registry.get(instance_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().instance_id(), instance_id);
    }

    #[test]
    fn test_duplicate_register() {
        let registry = StrategyRegistry::new();
        let owner_id = Uuid::new_v4();
        let handle = create_test_handle(owner_id, "BTCUSDT");
        assert!(registry.register(Arc::clone(&handle)).is_ok());
        assert!(registry.register(handle).is_err());
    }

    #[test]
    fn test_unregister() {
        let registry = StrategyRegistry::new();
        let owner_id = Uuid::new_v4();
        let handle = create_test_handle(owner_id, "BTCUSDT");
        let instance_id = handle.instance_id();
        registry.register(handle).unwrap();
        // Created 状态可以注销
        assert!(registry.unregister(instance_id).is_ok());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_unregister_running_fails() {
        let registry = StrategyRegistry::new();
        let owner_id = Uuid::new_v4();
        let handle = create_test_handle(owner_id, "BTCUSDT");
        let instance_id = handle.instance_id();
        registry.register(Arc::clone(&handle)).unwrap();
        // 启动后不能注销
        registry.start(instance_id).unwrap();
        assert!(registry.unregister(instance_id).is_err());
    }

    #[test]
    fn test_query_by_owner() {
        let registry = StrategyRegistry::new();
        let owner1 = Uuid::new_v4();
        let owner2 = Uuid::new_v4();
        registry.register(create_test_handle(owner1, "BTCUSDT")).unwrap();
        registry.register(create_test_handle(owner1, "ETHUSDT")).unwrap();
        registry.register(create_test_handle(owner2, "BTCUSDT")).unwrap();
        let results = registry.query(&StrategyQuery::by_owner(owner1));
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_by_symbol() {
        let registry = StrategyRegistry::new();
        let owner = Uuid::new_v4();
        registry.register(create_test_handle(owner, "BTCUSDT")).unwrap();
        registry.register(create_test_handle(owner, "BTCUSDT")).unwrap();
        registry.register(create_test_handle(owner, "ETHUSDT")).unwrap();
        let results = registry.query(&StrategyQuery::by_symbol("BTCUSDT"));
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_lifecycle_management() {
        let registry = StrategyRegistry::new();
        let owner_id = Uuid::new_v4();
        let handle = create_test_handle(owner_id, "BTCUSDT");
        let instance_id = handle.instance_id();
        registry.register(handle).unwrap();
        assert!(registry.start(instance_id).is_ok());
        assert!(registry.pause(instance_id).is_ok());
        assert!(registry.resume(instance_id).is_ok());
        assert!(registry.stop(instance_id).is_ok());
    }

    #[test]
    fn test_stats() {
        let registry = StrategyRegistry::new();
        let owner = Uuid::new_v4();
        let h1 = create_test_handle(owner, "BTCUSDT");
        let h2 = create_test_handle(owner, "ETHUSDT");
        let h3 = create_test_handle(owner, "BNBUSDT");
        let id1 = h1.instance_id();
        let id2 = h2.instance_id();
        registry.register(h1).unwrap();
        registry.register(h2).unwrap();
        registry.register(h3).unwrap();
        registry.start(id1).unwrap();
        registry.start(id2).unwrap();
        registry.pause(id2).unwrap();
        let stats = registry.stats();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.running, 1);
        assert_eq!(stats.paused, 1);
        assert_eq!(stats.created, 1);
    }

    #[test]
    fn test_cleanup_stopped() {
        let registry = StrategyRegistry::new();
        let owner = Uuid::new_v4();
        let h1 = create_test_handle(owner, "BTCUSDT");
        let h2 = create_test_handle(owner, "ETHUSDT");
        let id1 = h1.instance_id();
        registry.register(h1).unwrap();
        registry.register(h2).unwrap();
        registry.start(id1).unwrap();
        registry.stop(id1).unwrap();
        assert_eq!(registry.len(), 2);
        let cleaned = registry.cleanup_stopped();
        assert_eq!(cleaned, 1);
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_shutdown() {
        let registry = StrategyRegistry::new();
        let owner = Uuid::new_v4();
        for i in 0..5 {
            let handle = create_test_handle(owner, &format!("SYM{}", i));
            let id = handle.instance_id();
            registry.register(handle).unwrap();
            registry.start(id).unwrap();
        }
        assert_eq!(registry.len(), 5);
        registry.shutdown();
        assert_eq!(registry.len(), 0);
    }
}
