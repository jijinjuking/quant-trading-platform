//! # 突破策略 (Breakout Strategy)
//!
//! 基于价格突破历史高点/低点的策略。
//! 当价格突破N周期内的最高价时做多，突破最低价时做空。
//! 支持杠杆、双向持仓。

use std::collections::VecDeque;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType};
use crate::domain::model::signal::{Signal, SignalType};

/// 突破策略配置
#[derive(Debug, Clone)]
pub struct BreakoutConfig {
    /// 回溯周期（用于计算历史高低点）
    pub lookback_period: usize,
    /// 突破确认百分比（价格需要超过高低点的百分比才算突破）
    pub breakout_threshold_percent: Decimal,
    /// 交易数量
    pub quantity: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
    /// 止损百分比
    pub stop_loss_percent: Decimal,
}

impl Default for BreakoutConfig {
    fn default() -> Self {
        Self {
            lookback_period: 20,
            breakout_threshold_percent: Decimal::new(5, 3), // 0.5%
            quantity: Decimal::new(1, 3),                   // 0.001
            leverage: LeverageConfig {
                leverage: 10,
                margin_type: crate::domain::model::market_type::MarginType::Cross,
            },
            stop_loss_percent: Decimal::new(2, 2), // 2%
        }
    }
}

/// 突破策略状态
#[derive(Debug, Clone)]
pub struct BreakoutState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 历史最高价
    pub highest_high: Option<Decimal>,
    /// 历史最低价
    pub lowest_low: Option<Decimal>,
    /// 当前持仓方向
    pub current_position: Option<SignalType>,
    /// 入场价格
    pub entry_price: Option<Decimal>,
    /// 上次信号类型（防止重复信号）
    pub last_signal: Option<SignalType>,
}

impl BreakoutState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            highest_high: None,
            lowest_low: None,
            current_position: None,
            entry_price: None,
            last_signal: None,
        }
    }
}

impl Default for BreakoutState {
    fn default() -> Self {
        Self::new()
    }
}

/// 突破策略
pub struct BreakoutStrategy {
    meta: StrategyMeta,
    config: BreakoutConfig,
    state: BreakoutState,
}

impl BreakoutStrategy {
    /// 创建突破策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: BreakoutConfig,
        market_type: MarketType,
    ) -> Self {
        let market = if market_type.is_futures() {
            market_type
        } else {
            MarketType::UsdtFutures
        };

        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "breakout".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: BreakoutState::new(),
        }
    }

    /// 计算历史高低点
    fn calculate_high_low(&mut self) {
        if self.state.price_history.is_empty() {
            return;
        }

        self.state.highest_high = self.state.price_history.iter().max().copied();
        self.state.lowest_low = self.state.price_history.iter().min().copied();
    }

    /// 检查止损
    fn check_stop_loss(&self, current_price: Decimal) -> bool {
        if let (Some(entry_price), Some(position)) =
            (self.state.entry_price, self.state.current_position)
        {
            let price_change_percent = (current_price - entry_price) / entry_price;

            match position {
                SignalType::Buy => {
                    // 多仓，价格下跌超过止损线
                    price_change_percent < -self.config.stop_loss_percent
                }
                SignalType::Sell => {
                    // 空仓，价格上涨超过止损线
                    price_change_percent > self.config.stop_loss_percent
                }
                SignalType::Hold => false,
            }
        } else {
            false
        }
    }

    /// 计算突破信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 检查止损
        if self.check_stop_loss(price) {
            let signal_type = match self.state.current_position {
                Some(SignalType::Buy) => SignalType::Sell,  // 平多仓
                Some(SignalType::Sell) => SignalType::Buy,  // 平空仓
                Some(SignalType::Hold) | None => return None,
            };

            self.state.current_position = None;
            self.state.entry_price = None;
            self.state.last_signal = None;

            let leverage_multiplier =
                Decimal::from_u32(self.config.leverage.leverage).unwrap_or(Decimal::ONE);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type,
                price,
                quantity: self.config.quantity * leverage_multiplier,
                confidence: 0.5, // 止损信号置信度较低
                created_at: event.timestamp,
            });
        }

        // 更新价格历史
        self.state.price_history.push_back(price);
        if self.state.price_history.len() > self.config.lookback_period {
            self.state.price_history.pop_front();
        }

        // 需要足够的历史数据
        if self.state.price_history.len() < self.config.lookback_period {
            return None;
        }

        // 计算历史高低点
        self.calculate_high_low();

        let highest_high = self.state.highest_high?;
        let lowest_low = self.state.lowest_low?;

        // 计算突破阈值
        let upper_breakout = highest_high * (Decimal::ONE + self.config.breakout_threshold_percent);
        let lower_breakout = lowest_low * (Decimal::ONE - self.config.breakout_threshold_percent);

        // 判断突破信号
        let signal_type = if price > upper_breakout && self.state.last_signal != Some(SignalType::Buy) {
            // 向上突破，做多
            Some(SignalType::Buy)
        } else if price < lower_breakout && self.state.last_signal != Some(SignalType::Sell) {
            // 向下突破，做空
            Some(SignalType::Sell)
        } else {
            None
        };

        if let Some(sig_type) = signal_type {
            self.state.current_position = Some(sig_type);
            self.state.entry_price = Some(price);
            self.state.last_signal = Some(sig_type);

            let leverage_multiplier =
                Decimal::from_u32(self.config.leverage.leverage).unwrap_or(Decimal::ONE);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type: sig_type,
                price,
                quantity: self.config.quantity * leverage_multiplier,
                confidence: 0.75,
                created_at: event.timestamp,
            });
        }

        None
    }
}

impl Strategy for BreakoutStrategy {
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
        self.state = BreakoutState::new();
    }
}
