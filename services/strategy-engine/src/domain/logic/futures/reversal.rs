//! # 反转策略 (Reversal Strategy)
//!
//! 基于价格反转信号的策略。
//! 使用价格动量和K线形态来识别趋势反转点。
//! 支持杠杆、双向持仓。

use std::collections::VecDeque;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType};
use crate::domain::model::signal::{Signal, SignalType};

/// 反转策略配置
#[derive(Debug, Clone)]
pub struct ReversalConfig {
    /// 动量周期
    pub momentum_period: usize,
    /// 反转阈值（动量变化百分比）
    pub reversal_threshold_percent: Decimal,
    /// 确认周期（连续N个周期确认反转）
    pub confirmation_periods: usize,
    /// 交易数量
    pub quantity: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
}

impl Default for ReversalConfig {
    fn default() -> Self {
        Self {
            momentum_period: 10,
            reversal_threshold_percent: Decimal::new(15, 2), // 1.5%
            confirmation_periods: 2,
            quantity: Decimal::new(1, 3), // 0.001
            leverage: LeverageConfig {
                leverage: 10,
                margin_type: crate::domain::model::market_type::MarginType::Cross,
            },
        }
    }
}

/// 反转策略状态
#[derive(Debug, Clone)]
pub struct ReversalState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 动量历史
    pub momentum_history: VecDeque<Decimal>,
    /// 反转确认计数器
    pub bullish_reversal_count: usize,
    pub bearish_reversal_count: usize,
    /// 上次信号
    pub last_signal: Option<SignalType>,
}

impl ReversalState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            momentum_history: VecDeque::new(),
            bullish_reversal_count: 0,
            bearish_reversal_count: 0,
            last_signal: None,
        }
    }
}

impl Default for ReversalState {
    fn default() -> Self {
        Self::new()
    }
}

/// 反转策略
pub struct ReversalStrategy {
    meta: StrategyMeta,
    config: ReversalConfig,
    state: ReversalState,
}

impl ReversalStrategy {
    /// 创建反转策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: ReversalConfig,
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
                strategy_type: "reversal".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: ReversalState::new(),
        }
    }

    /// 计算动量（当前价格相对于N周期前的变化百分比）
    fn calculate_momentum(&self) -> Option<Decimal> {
        if self.state.price_history.len() <= self.config.momentum_period {
            return None;
        }

        let current_price = *self.state.price_history.back()?;
        let past_price = self.state.price_history
            [self.state.price_history.len() - self.config.momentum_period - 1];

        if past_price == Decimal::ZERO {
            return None;
        }

        Some((current_price - past_price) / past_price)
    }

    /// 检测看涨反转信号
    fn detect_bullish_reversal(&self) -> bool {
        if self.state.momentum_history.len() < 2 {
            return false;
        }

        let current_momentum = *self.state.momentum_history.back().unwrap();
        let prev_momentum = self.state.momentum_history[self.state.momentum_history.len() - 2];

        // 动量从负转正，且变化幅度超过阈值
        prev_momentum < -self.config.reversal_threshold_percent
            && current_momentum > Decimal::ZERO
            && (current_momentum - prev_momentum) > self.config.reversal_threshold_percent
    }

    /// 检测看跌反转信号
    fn detect_bearish_reversal(&self) -> bool {
        if self.state.momentum_history.len() < 2 {
            return false;
        }

        let current_momentum = *self.state.momentum_history.back().unwrap();
        let prev_momentum = self.state.momentum_history[self.state.momentum_history.len() - 2];

        // 动量从正转负，且变化幅度超过阈值
        prev_momentum > self.config.reversal_threshold_percent
            && current_momentum < Decimal::ZERO
            && (prev_momentum - current_momentum) > self.config.reversal_threshold_percent
    }

    /// 计算反转信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 更新价格历史
        self.state.price_history.push_back(price);
        if self.state.price_history.len() > self.config.momentum_period + 10 {
            self.state.price_history.pop_front();
        }

        // 计算动量
        let momentum = self.calculate_momentum()?;
        self.state.momentum_history.push_back(momentum);
        if self.state.momentum_history.len() > 20 {
            self.state.momentum_history.pop_front();
        }

        // 检测反转信号
        let bullish_reversal = self.detect_bullish_reversal();
        let bearish_reversal = self.detect_bearish_reversal();

        // 更新确认计数器
        if bullish_reversal {
            self.state.bullish_reversal_count += 1;
            self.state.bearish_reversal_count = 0;
        } else if bearish_reversal {
            self.state.bearish_reversal_count += 1;
            self.state.bullish_reversal_count = 0;
        } else {
            self.state.bullish_reversal_count = 0;
            self.state.bearish_reversal_count = 0;
        }

        // 判断信号（需要连续确认）
        let signal_type = if self.state.bullish_reversal_count >= self.config.confirmation_periods
            && self.state.last_signal != Some(SignalType::Buy)
        {
            Some(SignalType::Buy) // 看涨反转，做多
        } else if self.state.bearish_reversal_count >= self.config.confirmation_periods
            && self.state.last_signal != Some(SignalType::Sell)
        {
            Some(SignalType::Sell) // 看跌反转，做空
        } else {
            None
        };

        if let Some(sig_type) = signal_type {
            self.state.last_signal = Some(sig_type);
            self.state.bullish_reversal_count = 0;
            self.state.bearish_reversal_count = 0;

            let leverage_multiplier =
                Decimal::from_u32(self.config.leverage.leverage).unwrap_or(Decimal::ONE);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type: sig_type,
                price,
                quantity: self.config.quantity * leverage_multiplier,
                confidence: 0.7,
                created_at: event.timestamp,
            });
        }

        None
    }
}

impl Strategy for ReversalStrategy {
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
        self.state = ReversalState::new();
    }
}
