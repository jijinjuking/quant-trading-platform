//! # 策略配置模型 (Strategy Configuration)
//!
//! 定义策略类型与配置结构。

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 策略类型 (Strategy Type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    /// 网格策略 (Grid)
    Grid,
    /// 均值回归策略 (Mean Reversion)
    MeanReversion,
}

/// 统一策略配置 (Unified Strategy Configuration)
#[derive(Debug, Clone)]
pub enum StrategyConfig {
    /// 网格策略配置 (Grid Strategy Configuration)
    Grid(GridStrategyConfig),
    /// 均值回归策略配置 (Mean Reversion Strategy Configuration)
    MeanReversion(MeanReversionStrategyConfig),
}

/// 网格策略配置 (Grid Strategy Configuration)
#[derive(Debug, Clone)]
pub struct GridStrategyConfig {
    /// 价格上界 (Upper Bound Price)
    pub upper_price: Decimal,
    /// 价格下界 (Lower Bound Price)
    pub lower_price: Decimal,
    /// 网格数量 (Grid Count)
    pub grid_count: u32,
    /// 每格数量 (Quantity Per Grid)
    pub quantity_per_grid: Decimal,
}

/// 均值回归策略配置 (Mean Reversion Strategy Configuration)
#[derive(Debug, Clone)]
pub struct MeanReversionStrategyConfig {
    /// 移动平均窗口 (Window Size)
    pub window_size: usize,
    /// 偏离阈值百分比 (Threshold Percent)
    pub threshold_percent: Decimal,
    /// 交易数量 (Order Quantity)
    pub quantity: Decimal,
}
