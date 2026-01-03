//! # 现货 MACD 策略 (Spot MACD Strategy)
//!
//! 基于 MACD 指标的趋势跟踪策略实现。(MACD Trend Following Strategy)

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// 现货 MACD 配置 (Spot MACD Configuration)
#[derive(Debug, Clone)]
pub struct SpotMacdConfig {
    /// 快线周期 (Fast Period)
    pub fast_period: usize,
    /// 慢线周期 (Slow Period)
    pub slow_period: usize,
    /// 信号线周期 (Signal Period)
    pub signal_period: usize,
    /// 交易数量 (Order Quantity)
    pub quantity: Decimal,
}

/// 现货 MACD 状态 (Spot MACD State)
#[derive(Debug, Clone)]
pub struct SpotMacdState {
    /// 价格历史 (Price History)
    pub price_history: Vec<Decimal>,
    /// MACD 历史 (MACD History)
    pub macd_history: Vec<Decimal>,
    /// 快线 EMA (Fast EMA)
    pub fast_ema: Option<Decimal>,
    /// 慢线 EMA (Slow EMA)
    pub slow_ema: Option<Decimal>,
    /// 信号线 (Signal Line)
    pub signal_line: Option<Decimal>,
    /// 上一次柱状图值 (Last Histogram)
    pub last_histogram: Option<Decimal>,
}

impl SpotMacdState {
    /// 创建初始状态 (Create Initial State)
    pub fn new() -> Self {
        Self {
            price_history: Vec::new(),
            macd_history: Vec::new(),
            fast_ema: None,
            slow_ema: None,
            signal_line: None,
            last_histogram: None,
        }
    }
}

impl Default for SpotMacdState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货 MACD 策略 (Spot MACD Strategy)
pub struct SpotMacdStrategy {
    meta: StrategyMeta,
    config: SpotMacdConfig,
    state: SpotMacdState,
}

impl SpotMacdStrategy {
    /// 创建现货 MACD 策略实例 (Create Spot MACD Strategy)
    pub fn new(instance_id: Uuid, symbol: String, config: SpotMacdConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_macd".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotMacdState::new(),
        }
    }

    /// 计算信号 (Calculate Signal)
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        if self.config.fast_period == 0
            || self.config.slow_period == 0
            || self.config.signal_period == 0
        {
            return None;
        }
        if self.config.quantity <= Decimal::ZERO {
            return None;
        }

        let price = trade.price;
        let max_period = self.config.fast_period.max(self.config.slow_period);
        self.state.price_history.push(price);
        if self.state.price_history.len() > max_period {
            let overflow = self.state.price_history.len() - max_period;
            self.state.price_history.drain(0..overflow);
        }

        let fast_ema = match self.state.fast_ema {
            Some(prev) => update_ema(price, prev, self.config.fast_period)?,
            None => {
                if self.state.price_history.len() < self.config.fast_period {
                    return None;
                }
                let start = self.state.price_history.len() - self.config.fast_period;
                average(&self.state.price_history[start..])?
            }
        };

        let slow_ema = match self.state.slow_ema {
            Some(prev) => update_ema(price, prev, self.config.slow_period)?,
            None => {
                if self.state.price_history.len() < self.config.slow_period {
                    return None;
                }
                let start = self.state.price_history.len() - self.config.slow_period;
                average(&self.state.price_history[start..])?
            }
        };

        self.state.fast_ema = Some(fast_ema);
        self.state.slow_ema = Some(slow_ema);

        let macd_line = fast_ema - slow_ema;
        self.state.macd_history.push(macd_line);
        if self.state.macd_history.len() > self.config.signal_period {
            let overflow = self.state.macd_history.len() - self.config.signal_period;
            self.state.macd_history.drain(0..overflow);
        }

        let signal_line = match self.state.signal_line {
            Some(prev) => update_ema(macd_line, prev, self.config.signal_period)?,
            None => {
                if self.state.macd_history.len() < self.config.signal_period {
                    return None;
                }
                average(&self.state.macd_history)?
            }
        };
        self.state.signal_line = Some(signal_line);

        let histogram = macd_line - signal_line;
        let last_histogram = self.state.last_histogram;
        self.state.last_histogram = Some(histogram);

        let signal_type = match last_histogram {
            Some(last) if last <= Decimal::ZERO && histogram > Decimal::ZERO => Some(SignalType::Buy),
            Some(last) if last >= Decimal::ZERO && histogram < Decimal::ZERO => Some(SignalType::Sell),
            _ => None,
        };

        signal_type.map(|signal_type| Signal {
            id: Uuid::new_v4(),
            strategy_id: self.meta.instance_id,
            symbol: event.symbol.clone(),
            signal_type,
            price,
            quantity: self.config.quantity,
            confidence: 1.0,
            created_at: event.timestamp,
        })
    }
}

impl Strategy for SpotMacdStrategy {
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
        self.state = SpotMacdState::new();
    }
}

fn average(values: &[Decimal]) -> Option<Decimal> {
    if values.is_empty() {
        return None;
    }
    let sum = values.iter().fold(Decimal::ZERO, |acc, value| acc + *value);
    let count = Decimal::from_usize(values.len())?;
    if count == Decimal::ZERO {
        return None;
    }
    Some(sum / count)
}

fn update_ema(current: Decimal, previous: Decimal, period: usize) -> Option<Decimal> {
    if period == 0 {
        return None;
    }
    let period_decimal = Decimal::from_usize(period)?;
    let multiplier = Decimal::from_u32(2)? / (period_decimal + Decimal::ONE);
    Some((current - previous) * multiplier + previous)
}
