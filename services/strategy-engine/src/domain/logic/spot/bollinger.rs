//! # 现货布林带策略 (Spot Bollinger Bands Strategy)
//!
//! 基于布林带指标的突破策略。

use std::collections::VecDeque;

use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// 布林带策略配置
#[derive(Debug, Clone)]
pub struct SpotBollingerConfig {
    /// 周期
    pub period: usize,
    /// 标准差倍数
    pub std_dev_multiplier: Decimal,
    /// 交易数量
    pub quantity: Decimal,
}

impl Default for SpotBollingerConfig {
    fn default() -> Self {
        Self {
            period: 20,
            std_dev_multiplier: Decimal::from(2),
            quantity: Decimal::new(1, 3),
        }
    }
}

/// 布林带策略状态
#[derive(Debug, Clone)]
pub struct SpotBollingerState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 上次信号
    pub last_signal: Option<SignalType>,
}

impl SpotBollingerState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            last_signal: None,
        }
    }
}

impl Default for SpotBollingerState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货布林带策略
pub struct SpotBollingerStrategy {
    meta: StrategyMeta,
    config: SpotBollingerConfig,
    state: SpotBollingerState,
}

impl SpotBollingerStrategy {
    pub fn new(instance_id: Uuid, symbol: String, config: SpotBollingerConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_bollinger".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotBollingerState::new(),
        }
    }

    /// 计算标准差
    fn calculate_std_dev(&self, prices: &VecDeque<Decimal>, mean: Decimal) -> Option<Decimal> {
        if prices.is_empty() {
            return None;
        }

        let variance: Decimal = prices
            .iter()
            .map(|p| {
                let diff = *p - mean;
                diff * diff
            })
            .sum::<Decimal>()
            / Decimal::from(prices.len());

        // 简化的平方根计算（使用迭代法）
        let mut x = variance;
        for _ in 0..10 {
            if x == Decimal::ZERO {
                break;
            }
            x = (x + variance / x) / Decimal::from(2);
        }

        Some(x)
    }

    /// 计算布林带信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 更新价格历史
        self.state.price_history.push_back(price);
        if self.state.price_history.len() > self.config.period {
            self.state.price_history.pop_front();
        }

        // 需要足够的数据
        if self.state.price_history.len() < self.config.period {
            return None;
        }

        // 计算中轨（移动平均）
        let middle_band: Decimal = self.state.price_history.iter().sum::<Decimal>()
            / Decimal::from(self.state.price_history.len());

        // 计算标准差
        let std_dev = self.calculate_std_dev(&self.state.price_history, middle_band)?;

        // 计算上轨和下轨
        let upper_band = middle_band + std_dev * self.config.std_dev_multiplier;
        let lower_band = middle_band - std_dev * self.config.std_dev_multiplier;

        // 判断信号
        let signal_type = if price < lower_band && self.state.last_signal != Some(SignalType::Buy) {
            Some(SignalType::Buy) // 突破下轨，买入
        } else if price > upper_band && self.state.last_signal != Some(SignalType::Sell) {
            Some(SignalType::Sell) // 突破上轨，卖出
        } else {
            None
        };

        if let Some(sig_type) = signal_type {
            self.state.last_signal = Some(sig_type);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type: sig_type,
                price,
                quantity: self.config.quantity,
                confidence: 0.75,
                created_at: event.timestamp,
            });
        }

        None
    }
}

impl Strategy for SpotBollingerStrategy {
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
        self.state = SpotBollingerState::new();
    }
}
