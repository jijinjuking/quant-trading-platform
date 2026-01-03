//! # 策略状态端口 (Strategy State Port)
//!
//! 定义策略运行状态的存储接口。
//!
//! ## 职责
//! - 读取策略状态
//! - 保存策略状态
//!
//! ## 存储位置
//! 按架构规范，短期状态存储在 Redis

use anyhow::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 网格策略状态（可序列化）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GridStateData {
    /// 上一次价格所在网格索引
    pub last_grid_index: Option<i32>,
    /// 上一次价格
    pub last_price: Option<Decimal>,
}

/// 均值回归策略状态（可序列化）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeanReversionStateData {
    /// 历史价格队列
    pub price_history: Vec<Decimal>,
}

/// 策略状态端口
///
/// 定义策略运行状态的存储接口。
/// Infrastructure 层实现此 trait（Redis）。
#[async_trait]
pub trait StrategyStatePort: Send + Sync {
    /// 获取网格策略状态
    ///
    /// # 参数
    /// - `strategy_id`: 策略实例 ID
    ///
    /// # 返回
    /// - `Ok(Some(state))`: 找到状态
    /// - `Ok(None)`: 状态不存在（首次运行）
    /// - `Err`: 存储错误
    async fn get_grid_state(&self, strategy_id: &str) -> Result<Option<GridStateData>>;

    /// 保存网格策略状态
    ///
    /// # 参数
    /// - `strategy_id`: 策略实例 ID
    /// - `state`: 策略状态
    async fn save_grid_state(&self, strategy_id: &str, state: &GridStateData) -> Result<()>;

    /// 获取均值回归策略状态
    ///
    /// # 参数
    /// - `strategy_id`: 策略实例 ID
    ///
    /// # 返回
    /// - `Ok(Some(state))`: 找到状态
    /// - `Ok(None)`: 状态不存在（首次运行）
    /// - `Err`: 存储错误
    async fn get_mean_reversion_state(
        &self,
        strategy_id: &str,
    ) -> Result<Option<MeanReversionStateData>>;

    /// 保存均值回归策略状态
    ///
    /// # 参数
    /// - `strategy_id`: 策略实例 ID
    /// - `state`: 策略状态
    async fn save_mean_reversion_state(
        &self,
        strategy_id: &str,
        state: &MeanReversionStateData,
    ) -> Result<()>;

    /// 删除策略状态
    ///
    /// # 参数
    /// - `strategy_id`: 策略实例 ID
    async fn delete_state(&self, strategy_id: &str) -> Result<()>;
}
