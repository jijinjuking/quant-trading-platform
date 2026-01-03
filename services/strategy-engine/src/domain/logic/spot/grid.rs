//! # 现货网格策略 (Spot Grid Strategy)
//!
//! 现货市场的网格交易策略实现。

use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// 现货网格策略配置
#[derive(Debug, Clone)]
pub struct SpotGridConfig {
    /// 价格上界
    pub upper_price: Decimal,
    /// 价格下界
    pub lower_price: Decimal,
    /// 网格数量
    pub grid_count: u32,
    /// 每格交易数量
    pub quantity_per_grid: Decimal,
}

/// 现货网格策略状态
#[derive(Debug, Clone)]
pub struct SpotGridState {
    /// 上一次价格所在网格索引
    pub last_grid_index: Option<i32>,
    /// 上一次价格
    pub last_price: Option<Decimal>,
}

impl SpotGridState {
    /// 创建初始状态
    pub fn new() -> Self {
        Self {
            last_grid_index: None,
            last_price: None,
        }
    }
}

impl Default for SpotGridState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货网格策略
pub struct SpotGridStrategy {
    meta: StrategyMeta,
    config: SpotGridConfig,
    state: SpotGridState,
}

impl SpotGridStrategy {
    /// 创建现货网格策略实例
    pub fn new(instance_id: Uuid, symbol: String, config: SpotGridConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_grid".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotGridState::new(),
        }
    }

    /// 计算网格信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        // 参数校验
        if self.config.grid_count == 0 {
            return None;
        }
        if self.config.upper_price <= self.config.lower_price {
            return None;
        }
        if self.config.quantity_per_grid <= Decimal::ZERO {
            return None;
        }

        let price = trade.price;

        // 价格超出范围
        if price < self.config.lower_price || price > self.config.upper_price {
            self.state.last_price = Some(price);
            self.state.last_grid_index = None;
            return None;
        }

        // 计算当前网格索引
        let grid_count = Decimal::from_u32(self.config.grid_count)?;
        let grid_size = (self.config.upper_price - self.config.lower_price) / grid_count;
        if grid_size <= Decimal::ZERO {
            return None;
        }

        let position = (price - self.config.lower_price) / grid_size;
        let current_index = position.to_i32()?;

        // 判断信号类型
        let signal_type = match self.state.last_grid_index {
            Some(last_index) if current_index < last_index => Some(SignalType::Buy),
            Some(last_index) if current_index > last_index => Some(SignalType::Sell),
            _ => None,
        };

        // 更新状态
        self.state.last_grid_index = Some(current_index);
        self.state.last_price = Some(price);

        // 生成信号
        signal_type.map(|signal_type| Signal {
            id: Uuid::new_v4(),
            strategy_id: self.meta.instance_id,
            symbol: event.symbol.clone(),
            signal_type,
            price,
            quantity: self.config.quantity_per_grid,
            confidence: 1.0,
            created_at: event.timestamp,
        })
    }
}

impl Strategy for SpotGridStrategy {
    fn meta(&self) -> &StrategyMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut StrategyMeta {
        &mut self.meta
    }

    fn on_market_event(&mut self, event: &MarketEvent) -> Option<Signal> {
        if !self.is_active() {
            return None;
        }
        self.calculate_signal(event)
    }

    fn reset(&mut self) {
        self.state = SpotGridState::new();
    }
}
