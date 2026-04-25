//! # 策略加载器 (Strategy Loader)
//!
//! 负责：
//! 1. 从配置文件或数据库加载策略配置
//! 2. 创建策略实例
//! 3. 注册到策略注册表

use std::sync::Arc;

use anyhow::{Context, Result};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tracing::{info, warn};
use uuid::Uuid;

use crate::domain::logic::spot::{
    SpotGridStrategy, SpotMeanReversionStrategy,
};
use crate::domain::logic::spot::grid::SpotGridConfig;
use crate::domain::logic::spot::mean::SpotMeanReversionConfig;
use crate::domain::model::strategy_handle::StrategyHandle;
use crate::domain::model::strategy_metadata::StrategyMetadata;
use crate::domain::port::strategy_executor_port::StrategyExecutorPort;
use crate::domain::service::strategy_registry::StrategyRegistry;
use crate::infrastructure::strategy::StrategyExecutorAdapter;

/// 策略配置
#[derive(Debug, Clone)]
pub struct StrategyConfig {
    /// 策略ID
    pub instance_id: Uuid,
    /// 策略类型
    pub strategy_type: String,
    /// 交易对
    pub symbol: String,
    /// 用户ID
    pub owner_id: Uuid,
    /// 策略名称
    pub name: String,
    /// 策略参数（JSON格式）
    pub params: serde_json::Value,
    /// 是否自动启动
    pub auto_start: bool,
}

/// 策略加载器
pub struct StrategyLoader {
    /// 策略注册表
    registry: Arc<StrategyRegistry>,
}

impl StrategyLoader {
    /// 创建加载器
    pub fn new(registry: Arc<StrategyRegistry>) -> Self {
        Self { registry }
    }

    /// 从配置列表加载策略
    pub async fn load_strategies(&self, configs: Vec<StrategyConfig>) -> Result<Vec<Uuid>> {
        let mut instance_ids = Vec::new();

        for config in configs {
            match self.load_strategy(&config).await {
                Ok(instance_id) => {
                    info!(
                        instance_id = %instance_id,
                        strategy_type = %config.strategy_type,
                        symbol = %config.symbol,
                        "Strategy loaded"
                    );
                    instance_ids.push(instance_id);
                }
                Err(e) => {
                    warn!(
                        strategy_type = %config.strategy_type,
                        symbol = %config.symbol,
                        error = %e,
                        "Failed to load strategy"
                    );
                }
            }
        }

        Ok(instance_ids)
    }

    /// 加载单个策略
    async fn load_strategy(&self, config: &StrategyConfig) -> Result<Uuid> {
        // 创建策略实例
        let executor = self.create_strategy_executor(config)?;

        // 创建元数据
        let metadata = StrategyMetadata::new(
            crate::domain::model::strategy_metadata::StrategyKind::Custom(config.strategy_type.clone()),
            crate::domain::model::strategy_metadata::MarketType::Spot,
            &config.symbol,
            config.owner_id,
            config.name.clone(),
        )
        .with_id(config.instance_id);

        // 创建策略句柄
        let handle = Arc::new(StrategyHandle::new(metadata, executor));

        // 注册到注册表
        let instance_id = self.registry.register(handle.clone())?;

        // 如果配置了自动启动，则启动策略
        if config.auto_start {
            self.registry.start(instance_id)?;
            info!(instance_id = %instance_id, "Strategy auto-started");
        }

        Ok(instance_id)
    }

    /// 创建策略执行器
    fn create_strategy_executor(
        &self,
        config: &StrategyConfig,
    ) -> Result<Arc<dyn StrategyExecutorPort>> {
        match config.strategy_type.as_str() {
            "spot_grid" => {
                let params = self.parse_grid_config(&config.params)?;
                let strategy = SpotGridStrategy::new(
                    config.instance_id,
                    config.symbol.clone(),
                    params,
                );
                Ok(Arc::new(StrategyExecutorAdapter::new(strategy)))
            }
            "spot_mean_reversion" => {
                let params = self.parse_mean_reversion_config(&config.params)?;
                let strategy = SpotMeanReversionStrategy::new(
                    config.instance_id,
                    config.symbol.clone(),
                    params,
                );
                Ok(Arc::new(StrategyExecutorAdapter::new(strategy)))
            }
            _ => Err(anyhow::anyhow!(
                "Unsupported strategy type: {}",
                config.strategy_type
            )),
        }
    }

    /// 解析网格策略配置
    fn parse_grid_config(&self, params: &serde_json::Value) -> Result<SpotGridConfig> {
        let upper_price = params
            .get("upper_price")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<Decimal>().ok())
            .context("Missing or invalid upper_price")?;

        let lower_price = params
            .get("lower_price")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<Decimal>().ok())
            .context("Missing or invalid lower_price")?;

        let grid_count = params
            .get("grid_count")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32)
            .context("Missing or invalid grid_count")?;

        let quantity_per_grid = params
            .get("quantity_per_grid")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<Decimal>().ok())
            .context("Missing or invalid quantity_per_grid")?;

        Ok(SpotGridConfig {
            upper_price,
            lower_price,
            grid_count,
            quantity_per_grid,
        })
    }

    /// 解析均值回归策略配置
    fn parse_mean_reversion_config(
        &self,
        params: &serde_json::Value,
    ) -> Result<SpotMeanReversionConfig> {
        let window_size = params
            .get("window_size")
            .or_else(|| params.get("period"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(20);

        let threshold_percent = params
            .get("threshold_percent")
            .or_else(|| params.get("std_dev_multiplier"))
            .and_then(|v| {
                v.as_str()
                    .and_then(|s| s.parse::<Decimal>().ok())
                    .or_else(|| v.as_f64().and_then(Decimal::from_f64))
            })
            .unwrap_or(Decimal::new(2, 2));

        let quantity = params
            .get("quantity")
            .and_then(|v| {
                v.as_str()
                    .and_then(|s| s.parse::<Decimal>().ok())
                    .or_else(|| v.as_f64().and_then(Decimal::from_f64))
            })
            .unwrap_or(Decimal::new(1, 3));

        Ok(SpotMeanReversionConfig {
            window_size,
            threshold_percent,
            quantity,
        })
    }

    /// 从环境变量加载示例策略
    pub fn load_example_strategies() -> Vec<StrategyConfig> {
        vec![
            // 示例：BTC网格策略
            StrategyConfig {
                instance_id: Uuid::new_v4(),
                strategy_type: "spot_grid".to_string(),
                symbol: "BTCUSDT".to_string(),
                owner_id: Uuid::new_v4(),
                name: "BTC Grid Strategy".to_string(),
                params: serde_json::json!({
                    "upper_price": "50000",
                    "lower_price": "40000",
                    "grid_count": 10,
                    "quantity_per_grid": "0.001"
                }),
                auto_start: true,
            },
            // 示例：ETH均值回归策略
            StrategyConfig {
                instance_id: Uuid::new_v4(),
                strategy_type: "spot_mean_reversion".to_string(),
                symbol: "ETHUSDT".to_string(),
                owner_id: Uuid::new_v4(),
                name: "ETH Mean Reversion Strategy".to_string(),
                params: serde_json::json!({
                    "period": 20,
                    "std_dev_multiplier": "2.0",
                    "quantity": "0.01"
                }),
                auto_start: true,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_example_strategies() {
        let configs = StrategyLoader::load_example_strategies();
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].strategy_type, "spot_grid");
        assert_eq!(configs[1].strategy_type, "spot_mean_reversion");
    }
}
