//! # 趋势跟踪策略 (Trend Following Strategy)
//!
//! 基于双均线系统的趋势跟踪策略。
//! 使用快速均线和慢速均线的交叉来判断趋势方向。
//! 支持杠杆、双向持仓。

use std::collections::VecDeque;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType};
use crate::domain::model::signal::{Signal, SignalType};

/// 趋势跟踪策略配置
#[derive(Debug, Clone)]
pub struct TrendFollowingConfig {
    /// 快速均线周期
    pub fast_period: usize,
    /// 慢速均线周期
    pub slow_period: usize,
    /// 交易数量
    pub quantity: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
    /// 止损百分比（可选）
    pub stop_loss_percent: Option<Decimal>,
}

impl Default for TrendFollowingConfig {
    fn default() -> Self {
        Self {
            fast_period: 10,
            slow_period: 30,
            quantity: Decimal::new(1, 3), // 0.001
            leverage: LeverageConfig {
                leverage: 10,
                margin_type: crate::domain::model::market_type::MarginType::Cross,
            },
            stop_loss_percent: Some(Decimal::new(2, 2)), // 2%
        }
    }
}

/// 趋势跟踪策略状态
#[derive(Debug, Clone)]
pub struct TrendFollowingState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 快速均线
    pub fast_ma: Option<Decimal>,
    /// 慢速均线
    pub slow_ma: Option<Decimal>,
    /// 当前持仓方向
    pub current_position: Option<SignalType>,
    /// 入场价格
    pub entry_price: Option<Decimal>,
}

impl TrendFollowingState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            fast_ma: None,
            slow_ma: None,
            current_position: None,
            entry_price: None,
        }
    }
}

impl Default for TrendFollowingState {
    fn default() -> Self {
        Self::new()
    }
}

/// 趋势跟踪策略
pub struct TrendFollowingStrategy {
    meta: StrategyMeta,
    config: TrendFollowingConfig,
    state: TrendFollowingState,
}

impl TrendFollowingStrategy {
    /// 创建趋势跟踪策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: TrendFollowingConfig,
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
                strategy_type: "trend_following".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: TrendFollowingState::new(),
        }
    }

    /// 计算移动平均
    fn calculate_ma(&self, period: usize) -> Option<Decimal> {
        if self.state.price_history.len() < period {
            return None;
        }

        let start = self.state.price_history.len() - period;
        let sum: Decimal = self.state.price_history.iter().skip(start).sum();
        Some(sum / Decimal::from(period))
    }

    /// 检查止损
    fn check_stop_loss(&self, current_price: Decimal) -> bool {
        if let (Some(entry_price), Some(position), Some(stop_loss_percent)) = (
            self.state.entry_price,
            self.state.current_position,
            self.config.stop_loss_percent,
        ) {
            let price_change_percent = (current_price - entry_price) / entry_price;

            match position {
                SignalType::Buy => {
                    // 多仓，价格下跌超过止损线
                    price_change_percent < -stop_loss_percent
                }
                SignalType::Sell => {
                    // 空仓，价格上涨超过止损线
                    price_change_percent > stop_loss_percent
                }
                SignalType::Hold => false,
            }
        } else {
            false
        }
    }

    /// 计算趋势跟踪信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 更新价格历史
        self.state.price_history.push_back(price);
        let max_period = self.config.fast_period.max(self.config.slow_period);
        if self.state.price_history.len() > max_period {
            self.state.price_history.pop_front();
        }

        // 检查止损
        if self.check_stop_loss(price) {
            let signal_type = match self.state.current_position {
                Some(SignalType::Buy) => SignalType::Sell,  // 平多仓
                Some(SignalType::Sell) => SignalType::Buy,  // 平空仓
                Some(SignalType::Hold) | None => return None,
            };

            self.state.current_position = None;
            self.state.entry_price = None;

            let leverage_multiplier =
                Decimal::from_u32(self.config.leverage.leverage).unwrap_or(Decimal::ONE);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type,
                price,
                quantity: self.config.quantity * leverage_multiplier,
                confidence: 0.6, // 止损信号置信度较低
                created_at: event.timestamp,
            });
        }

        // 计算快速和慢速均线
        let fast_ma = self.calculate_ma(self.config.fast_period)?;
        let slow_ma = self.calculate_ma(self.config.slow_period)?;

        let prev_fast_ma = self.state.fast_ma;
        let prev_slow_ma = self.state.slow_ma;

        self.state.fast_ma = Some(fast_ma);
        self.state.slow_ma = Some(slow_ma);

        // 需要前一个周期的数据来判断交叉
        let (prev_fast, prev_slow) = match (prev_fast_ma, prev_slow_ma) {
            (Some(f), Some(s)) => (f, s),
            _ => return None,
        };

        // 判断信号
        let signal_type = if prev_fast <= prev_slow && fast_ma > slow_ma {
            // 金叉：快线上穿慢线，做多
            Some(SignalType::Buy)
        } else if prev_fast >= prev_slow && fast_ma < slow_ma {
            // 死叉：快线下穿慢线，做空
            Some(SignalType::Sell)
        } else {
            None
        };

        if let Some(sig_type) = signal_type {
            // 如果已有持仓且方向相同，不重复开仓
            if self.state.current_position == Some(sig_type) {
                return None;
            }

            // 更新持仓状态
            self.state.current_position = Some(sig_type);
            self.state.entry_price = Some(price);

            let leverage_multiplier =
                Decimal::from_u32(self.config.leverage.leverage).unwrap_or(Decimal::ONE);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type: sig_type,
                price,
                quantity: self.config.quantity * leverage_multiplier,
                confidence: 0.8,
                created_at: event.timestamp,
            });
        }

        None
    }
}

impl Strategy for TrendFollowingStrategy {
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
        self.state = TrendFollowingState::new();
    }
}
