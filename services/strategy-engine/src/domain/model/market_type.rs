//! # 市场类型定义 (Market Type)
//!
//! 区分现货和合约市场，为策略分类提供基础。

use serde::{Deserialize, Serialize};

/// 市场类型
///
/// 用于区分不同市场的交易规则和策略逻辑。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketType {
    /// 现货市场
    Spot,
    /// U本位永续合约
    UsdtFutures,
    /// 币本位永续合约
    CoinFutures,
}

impl MarketType {
    /// 是否为合约市场
    pub fn is_futures(&self) -> bool {
        matches!(self, MarketType::UsdtFutures | MarketType::CoinFutures)
    }

    /// 是否为现货市场
    pub fn is_spot(&self) -> bool {
        matches!(self, MarketType::Spot)
    }

    /// 获取市场类型名称
    pub fn as_str(&self) -> &'static str {
        match self {
            MarketType::Spot => "spot",
            MarketType::UsdtFutures => "usdt_futures",
            MarketType::CoinFutures => "coin_futures",
        }
    }
}

impl std::fmt::Display for MarketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 合约方向（仅合约市场使用）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionSide {
    /// 多头
    Long,
    /// 空头
    Short,
    /// 双向持仓模式下的净仓位
    Both,
}

/// 杠杆配置（仅合约市场使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageConfig {
    /// 杠杆倍数
    pub leverage: u32,
    /// 保证金模式
    pub margin_type: MarginType,
}

/// 保证金模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginType {
    /// 逐仓
    Isolated,
    /// 全仓
    Cross,
}
