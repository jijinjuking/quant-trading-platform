//! # 现货RSI策略 (Spot RSI Strategy)
//!
//! 基于RSI指标的超买超卖策略。

use std::collections::VecDeque;

use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// RSI策略配置
#[derive(Debug, Clone)]
pub struct SpotRsiConfig {
    /// RSI周期
    pub period: usize,
    /// 超卖阈值
    pub oversold_threshold: Decimal,
    /// 超买阈值
    pub overbought_threshold: Decimal,
    /// 交易数量
    pub quantity: Decimal,
}

impl Default for SpotRsiConfig {
    fn default() -> Self {
        Self {
            period: 14,
            oversold_threshold: Decimal::from(30),
            overbought_threshold: Decimal::from(70),
            quantity: Decimal::new(1, 3),
        }
    }
}

/// RSI策略状态
#[derive(Debug, Clone)]
pub struct SpotRsiState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 平均涨幅
    pub avg_gain: Option<Decimal>,
    /// 平均跌幅
    pub avg_loss: Option<Decimal>,
    /// 上次信号
    pub last_signal: Option<SignalType>,
}

impl SpotRsiState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            avg_gain: None,
            avg_loss: None,
            last_signal: None,
        }
    }
}

impl Default for SpotRsiState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货RSI策略
pub struct SpotRsiStrategy {
    meta: StrategyMeta,
    config: SpotRsiConfig,
    state: SpotRsiState,
}

impl SpotRsiStrategy {
    pub fn new(instance_id: Uuid, symbol: String, config: SpotRsiConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_rsi".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotRsiState::new(),
        }
    }

    /// 计算RSI信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 更新价格历史
        self.state.price_history.push_back(price);
        if self.state.price_history.len() > self.config.period + 1 {
            self.state.price_history.pop_front();
        }

        // 需要足够的数据
        if self.state.price_history.len() < 2 {
            return None;
        }

        // 计算价格变化
        let prev_price = self.state.price_history[self.state.price_history.len() - 2];
        let change = price - prev_price;

        let gain = if change > Decimal::ZERO { change } else { Decimal::ZERO };
        let loss = if change < Decimal::ZERO { -change } else { Decimal::ZERO };

        // 更新平均涨跌幅
        let period_decimal = Decimal::from(self.config.period);
        self.state.avg_gain = Some(match self.state.avg_gain {
            Some(avg) => (avg * (period_decimal - Decimal::ONE) + gain) / period_decimal,
            None => gain,
        });

        self.state.avg_loss = Some(match self.state.avg_loss {
            Some(avg) => (avg * (period_decimal - Decimal::ONE) + loss) / period_decimal,
            None => loss,
        });

        // 计算RSI
        let rsi = match (self.state.avg_gain, self.state.avg_loss) {
            (Some(avg_gain), Some(avg_loss)) if avg_loss != Decimal::ZERO => {
                let rs = avg_gain / avg_loss;
                Decimal::from(100) - (Decimal::from(100) / (Decimal::ONE + rs))
            }
            (Some(_), Some(avg_loss)) if avg_loss == Decimal::ZERO => Decimal::from(100),
            _ => return None,
        };

        // 判断信号
        let signal_type = if rsi < self.config.oversold_threshold
            && self.state.last_signal != Some(SignalType::Buy)
        {
            Some(SignalType::Buy) // 超卖，买入
        } else if rsi > self.config.overbought_threshold
            && self.state.last_signal != Some(SignalType::Sell)
        {
            Some(SignalType::Sell) // 超买，卖出
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
                confidence: 0.7,
                created_at: event.timestamp,
            });
        }

        None
    }
}

impl Strategy for SpotRsiStrategy {
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
        self.state = SpotRsiState::new();
    }
}
