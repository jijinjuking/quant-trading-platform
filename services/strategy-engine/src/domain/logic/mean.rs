//! # 均值回归策略逻辑 (Mean Reversion Strategy)
//!
//! 均值回归策略的核心算法实现。

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::model::signal::{Signal, SignalType};

/// 均值回归策略配置 (Mean Reversion Strategy Configuration)
#[derive(Debug, Clone)]
pub struct MeanReversionConfig {
    /// 移动平均窗口 (Window Size)
    pub window_size: usize,
    /// 偏离阈值百分比 (Threshold Percent)
    pub threshold_percent: Decimal,
    /// 交易数量 (Order Quantity)
    pub quantity: Decimal,
}

/// 均值回归策略运行状态 (Mean Reversion Strategy State)
#[derive(Debug, Clone)]
pub struct MeanReversionState {
    /// 历史价格队列 (Price History)
    pub price_history: Vec<Decimal>,
}

impl MeanReversionState {
    /// 创建均值回归状态 (Create Mean Reversion State)
    pub fn new() -> Self {
        Self {
            price_history: Vec::new(),
        }
    }
}

/// 根据行情事件计算均值回归信号 (Calculate Mean Reversion Signal)
pub fn calculate_mean_reversion_signal(
    event: &MarketEvent,
    config: &MeanReversionConfig,
    state: &mut MeanReversionState,
) -> Option<Signal> {
    let trade = match &event.data {
        MarketEventData::Trade(trade) => trade,
        _ => return None,
    };

    if config.window_size == 0 {
        return None;
    }
    if config.quantity <= Decimal::ZERO {
        return None;
    }

    state.price_history.push(trade.price);
    if state.price_history.len() > config.window_size {
        let overflow = state.price_history.len() - config.window_size;
        state.price_history.drain(0..overflow);
    }

    if state.price_history.len() < config.window_size {
        return None;
    }

    let sum = state
        .price_history
        .iter()
        .fold(Decimal::ZERO, |acc, price| acc + *price);
    let window_size = Decimal::from_usize(config.window_size)?;
    if window_size == Decimal::ZERO {
        return None;
    }

    let moving_average = sum / window_size;
    if moving_average == Decimal::ZERO {
        return None;
    }

    let deviation = (trade.price - moving_average) / moving_average;
    let signal_type = if deviation > config.threshold_percent {
        Some(SignalType::Sell)
    } else if deviation < -config.threshold_percent {
        Some(SignalType::Buy)
    } else {
        None
    };

    signal_type.map(|signal_type| Signal {
        id: Uuid::new_v4(),
        strategy_id: Uuid::nil(),
        symbol: event.symbol.clone(),
        signal_type,
        price: trade.price,
        quantity: config.quantity,
        confidence: 1.0,
        created_at: event.timestamp,
    })
}
