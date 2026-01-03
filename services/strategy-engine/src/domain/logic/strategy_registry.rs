//! # 策略注册表 (Strategy Registry)
//!
//! 管理策略实例的生命周期，支持动态切换。

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::Strategy;
use crate::domain::model::market_type::MarketType;

/// 策略注册表
///
/// 管理所有运行中的策略实例，支持：
/// - 注册/注销策略
/// - 按 ID 查找策略
/// - 按市场类型筛选策略
/// - 动态切换策略状态
pub struct StrategyRegistry {
    /// 策略实例映射：instance_id -> Strategy
    instances: HashMap<Uuid, Box<dyn Strategy>>,
}

impl StrategyRegistry {
    /// 创建空的策略注册表
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }

    /// 注册策略实例
    ///
    /// # 参数
    /// - `strategy`: 策略实例（已配置好）
    ///
    /// # 返回
    /// - `Ok(Uuid)`: 策略实例 ID
    /// - `Err`: 注册失败
    pub fn register(&mut self, strategy: Box<dyn Strategy>) -> Result<Uuid> {
        let instance_id = strategy.meta().instance_id;
        
        if self.instances.contains_key(&instance_id) {
            return Err(anyhow!("strategy instance {} already exists", instance_id));
        }

        self.instances.insert(instance_id, strategy);
        Ok(instance_id)
    }

    /// 注销策略实例
    ///
    /// # 参数
    /// - `instance_id`: 策略实例 ID
    ///
    /// # 返回
    /// - `Ok(())`: 注销成功
    /// - `Err`: 实例不存在
    pub fn unregister(&mut self, instance_id: &Uuid) -> Result<()> {
        self.instances
            .remove(instance_id)
            .map(|_| ())
            .ok_or_else(|| anyhow!("strategy instance {} not found", instance_id))
    }

    /// 获取策略实例（不可变）
    pub fn get(&self, instance_id: &Uuid) -> Option<&dyn Strategy> {
        self.instances.get(instance_id).map(|s| s.as_ref())
    }

    /// 获取策略实例（可变）
    pub fn get_mut(&mut self, instance_id: &Uuid) -> Option<&mut Box<dyn Strategy>> {
        self.instances.get_mut(instance_id)
    }

    /// 激活策略
    pub fn activate(&mut self, instance_id: &Uuid) -> Result<()> {
        let strategy = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| anyhow!("strategy instance {} not found", instance_id))?;
        strategy.activate();
        Ok(())
    }

    /// 停用策略
    pub fn deactivate(&mut self, instance_id: &Uuid) -> Result<()> {
        let strategy = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| anyhow!("strategy instance {} not found", instance_id))?;
        strategy.deactivate();
        Ok(())
    }

    /// 重置策略状态
    pub fn reset(&mut self, instance_id: &Uuid) -> Result<()> {
        let strategy = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| anyhow!("strategy instance {} not found", instance_id))?;
        strategy.reset();
        Ok(())
    }

    /// 获取所有策略实例 ID
    pub fn list_ids(&self) -> Vec<Uuid> {
        self.instances.keys().copied().collect()
    }

    /// 按市场类型筛选策略
    pub fn filter_by_market(&self, market_type: MarketType) -> Vec<Uuid> {
        self.instances
            .iter()
            .filter(|(_, s)| s.meta().market_type == market_type)
            .map(|(id, _)| *id)
            .collect()
    }

    /// 获取所有激活的策略
    pub fn list_active(&self) -> Vec<Uuid> {
        self.instances
            .iter()
            .filter(|(_, s)| s.is_active())
            .map(|(id, _)| *id)
            .collect()
    }

    /// 按交易对筛选策略
    pub fn filter_by_symbol(&self, symbol: &str) -> Vec<Uuid> {
        let symbol_upper = symbol.to_uppercase();
        self.instances
            .iter()
            .filter(|(_, s)| s.meta().symbol.to_uppercase() == symbol_upper)
            .map(|(id, _)| *id)
            .collect()
    }

    /// 策略实例数量
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// 迭代所有策略（可变）
    ///
    /// 用于批量处理行情事件
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Uuid, &mut Box<dyn Strategy>)> {
        self.instances.iter_mut()
    }
}

impl Default for StrategyRegistry {
    fn default() -> Self {
        Self::new()
    }
}
