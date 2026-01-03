//! # 网格交易策略逻辑 (Grid Trading Strategy)
//!
//! 网格交易策略的核心算法实现。

use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::model::signal::{Signal, SignalType};

/// 网格策略配置 (Grid Strategy Configuration)
#[derive(Debug, Clone)]
pub struct GridConfig {
    /// 价格上界 (Upper Bound Price)
    pub upper_price: Decimal,
    /// 价格下界 (Lower Bound Price)
    pub lower_price: Decimal,
    /// 网格数量 (Grid Count)
    pub grid_count: u32,
    /// 每格数量 (Quantity Per Grid)
    pub quantity_per_grid: Decimal,
}

/// 网格策略运行状态 (Grid Strategy State)
#[derive(Debug, Clone)]
pub struct GridState {
    /// 上一次价格所在网格索引 (Last Grid Index)
    pub last_grid_index: Option<i32>,
    /// 上一次价格 (Last Price)
    pub last_price: Option<Decimal>,
}

impl GridState {
    /// 创建网格状态 (Create Grid State)
    pub fn new() -> Self {
        Self {
            last_grid_index: None,
            last_price: None,
        }
    }
}

/// 根据行情事件计算网格信号 (Calculate Grid Signal)
pub fn calculate_grid_signal(
    event: &MarketEvent,
    config: &GridConfig,
    state: &mut GridState,
) -> Option<Signal> {
    let trade = match &event.data {
        MarketEventData::Trade(trade) => trade,
        _ => return None,
    };

    if config.grid_count == 0 {
        return None;
    }
    if config.upper_price <= config.lower_price {
        return None;
    }
    if config.quantity_per_grid <= Decimal::ZERO {
        return None;
    }

    let price = trade.price;
    if price < config.lower_price || price > config.upper_price {
        state.last_price = Some(price);
        state.last_grid_index = None;
        return None;
    }

    let grid_count = Decimal::from_u32(config.grid_count)?;
    let grid_size = (config.upper_price - config.lower_price) / grid_count;
    if grid_size <= Decimal::ZERO {
        return None;
    }

    let position = (price - config.lower_price) / grid_size;
    let current_index = position.to_i32()?;

    let signal_type = match state.last_grid_index {
        Some(last_index) if current_index < last_index => Some(SignalType::Buy),
        Some(last_index) if current_index > last_index => Some(SignalType::Sell),
        _ => None,
    };

    state.last_grid_index = Some(current_index);
    state.last_price = Some(price);

    signal_type.map(|signal_type| Signal {
        id: Uuid::new_v4(),
        strategy_id: Uuid::nil(),
        symbol: event.symbol.clone(),
        signal_type,
        price,
        quantity: config.quantity_per_grid,
        confidence: 1.0,
        created_at: event.timestamp,
    })
}
