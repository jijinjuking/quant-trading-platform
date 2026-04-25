//! # 策略元数据 (Strategy Metadata)
//!
//! 定义策略实例的元数据信息，用于标识、分类和管理策略。
//!
//! ## 工程约束
//! - 元数据创建后不可变（除版本号）
//! - 支持多市场、多用户、多版本

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 市场类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketType {
    /// 现货
    Spot,
    /// U 本位合约
    UsdtFutures,
    /// 币本位合约
    CoinFutures,
}

impl std::fmt::Display for MarketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketType::Spot => write!(f, "spot"),
            MarketType::UsdtFutures => write!(f, "usdt_futures"),
            MarketType::CoinFutures => write!(f, "coin_futures"),
        }
    }
}

/// 策略类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyKind {
    /// 网格策略
    Grid,
    /// 均值回归
    MeanReversion,
    /// MACD
    Macd,
    /// RSI
    Rsi,
    /// 布林带
    Bollinger,
    /// 资金费率套利
    FundingArbitrage,
    /// 自定义策略
    Custom(String),
}

impl std::fmt::Display for StrategyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrategyKind::Grid => write!(f, "grid"),
            StrategyKind::MeanReversion => write!(f, "mean_reversion"),
            StrategyKind::Macd => write!(f, "macd"),
            StrategyKind::Rsi => write!(f, "rsi"),
            StrategyKind::Bollinger => write!(f, "bollinger"),
            StrategyKind::FundingArbitrage => write!(f, "funding_arbitrage"),
            StrategyKind::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// 策略元数据
///
/// 描述策略实例的静态信息，创建后不可变（除版本号）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyMetadata {
    /// 策略实例 ID（全局唯一）
    pub instance_id: Uuid,
    /// 策略类型
    pub kind: StrategyKind,
    /// 市场类型
    pub market_type: MarketType,
    /// 交易对（如 BTCUSDT）
    pub symbol: String,
    /// 所有者用户 ID
    pub owner_id: Uuid,
    /// 策略版本号（支持热更新）
    pub version: u32,
    /// 策略名称（用户自定义）
    pub name: String,
    /// 策略描述
    pub description: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 标签（用于分组和筛选）
    pub tags: Vec<String>,
    /// 初始资金（用于风控和统计）
    pub initial_capital: Option<Decimal>,
}

impl StrategyMetadata {
    /// 创建策略元数据
    pub fn new(
        kind: StrategyKind,
        market_type: MarketType,
        symbol: impl Into<String>,
        owner_id: Uuid,
        name: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            instance_id: Uuid::new_v4(),
            kind,
            market_type,
            symbol: symbol.into(),
            owner_id,
            version: 1,
            name: name.into(),
            description: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            initial_capital: None,
        }
    }

    /// 使用指定 ID 创建
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.instance_id = id;
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// 设置初始资金
    pub fn with_initial_capital(mut self, capital: Decimal) -> Self {
        self.initial_capital = Some(capital);
        self
    }

    /// 增加版本号
    pub fn increment_version(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }

    /// 更新时间戳
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// 生成唯一标识字符串
    pub fn unique_key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.owner_id, self.market_type, self.symbol, self.instance_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let owner_id = Uuid::new_v4();
        let metadata = StrategyMetadata::new(
            StrategyKind::Grid,
            MarketType::Spot,
            "BTCUSDT",
            owner_id,
            "我的网格策略",
        );

        assert_eq!(metadata.kind, StrategyKind::Grid);
        assert_eq!(metadata.market_type, MarketType::Spot);
        assert_eq!(metadata.symbol, "BTCUSDT");
        assert_eq!(metadata.owner_id, owner_id);
        assert_eq!(metadata.version, 1);
    }

    #[test]
    fn test_version_increment() {
        let mut metadata = StrategyMetadata::new(
            StrategyKind::MeanReversion,
            MarketType::UsdtFutures,
            "ETHUSDT",
            Uuid::new_v4(),
            "均值回归",
        );

        assert_eq!(metadata.version, 1);
        metadata.increment_version();
        assert_eq!(metadata.version, 2);
    }

    #[test]
    fn test_unique_key() {
        let owner_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let instance_id = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();

        let metadata = StrategyMetadata::new(
            StrategyKind::Grid,
            MarketType::Spot,
            "BTCUSDT",
            owner_id,
            "测试",
        )
        .with_id(instance_id);

        let key = metadata.unique_key();
        assert!(key.contains("550e8400"));
        assert!(key.contains("spot"));
        assert!(key.contains("BTCUSDT"));
    }
}
