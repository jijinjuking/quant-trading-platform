//! # 跨期套利策略 (Calendar Spread Strategy)
//!
//! 利用不同到期日合约之间的价差进行套利。
//! 当近月合约和远月合约的价差偏离正常范围时进行套利交易。
//! 支持杠杆。

use std::collections::VecDeque;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType};
use crate::domain::model::signal::{Signal, SignalType};

/// 跨期套利策略配置
#[derive(Debug, Clone)]
pub struct CalendarSpreadConfig {
    /// 近月合约symbol（例如：BTCUSDT_PERP）
    pub near_contract: String,
    /// 远月合约symbol（例如：BTCUSDT_240329）
    pub far_contract: String,
    /// 价差均值计算周期
    pub spread_period: usize,
    /// 价差标准差倍数（触发套利的阈值）
    pub spread_std_multiplier: Decimal,
    /// 交易数量
    pub quantity: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
}

impl Default for CalendarSpreadConfig {
    fn default() -> Self {
        Self {
            near_contract: "BTCUSDT_PERP".to_string(),
            far_contract: "BTCUSDT_240329".to_string(),
            spread_period: 20,
            spread_std_multiplier: Decimal::from(2),
            quantity: Decimal::new(1, 3), // 0.001
            leverage: LeverageConfig {
                leverage: 5,
                margin_type: crate::domain::model::market_type::MarginType::Cross,
            },
        }
    }
}

/// 跨期套利策略状态
#[derive(Debug, Clone)]
pub struct CalendarSpreadState {
    /// 近月合约价格
    pub near_price: Option<Decimal>,
    /// 远月合约价格
    pub far_price: Option<Decimal>,
    /// 价差历史（远月 - 近月）
    pub spread_history: VecDeque<Decimal>,
    /// 是否有持仓
    pub has_position: bool,
    /// 持仓方向（true=做多价差，false=做空价差）
    pub is_long_spread: bool,
}

impl CalendarSpreadState {
    pub fn new() -> Self {
        Self {
            near_price: None,
            far_price: None,
            spread_history: VecDeque::new(),
            has_position: false,
            is_long_spread: false,
        }
    }
}

impl Default for CalendarSpreadState {
    fn default() -> Self {
        Self::new()
    }
}

/// 跨期套利策略
pub struct CalendarSpreadStrategy {
    meta: StrategyMeta,
    config: CalendarSpreadConfig,
    state: CalendarSpreadState,
}

impl CalendarSpreadStrategy {
    /// 创建跨期套利策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: CalendarSpreadConfig,
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
                strategy_type: "calendar_spread".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: CalendarSpreadState::new(),
        }
    }

    /// 更新合约价格
    ///
    /// 外部需要调用此方法更新近月和远月合约的价格
    pub fn update_near_price(&mut self, price: Decimal) {
        self.state.near_price = Some(price);
    }

    pub fn update_far_price(&mut self, price: Decimal) {
        self.state.far_price = Some(price);
    }

    /// 计算价差的均值和标准差
    fn calculate_spread_stats(&self) -> Option<(Decimal, Decimal)> {
        if self.state.spread_history.len() < self.config.spread_period {
            return None;
        }

        // 计算均值
        let sum: Decimal = self.state.spread_history.iter().sum();
        let mean = sum / Decimal::from(self.state.spread_history.len());

        // 计算标准差
        let variance: Decimal = self
            .state
            .spread_history
            .iter()
            .map(|s| {
                let diff = *s - mean;
                diff * diff
            })
            .sum::<Decimal>()
            / Decimal::from(self.state.spread_history.len());

        // 简化的平方根计算
        let mut std_dev = variance;
        for _ in 0..10 {
            if std_dev == Decimal::ZERO {
                break;
            }
            std_dev = (std_dev + variance / std_dev) / Decimal::from(2);
        }

        Some((mean, std_dev))
    }

    /// 计算套利信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        // 更新对应合约的价格
        if event.symbol == self.config.near_contract {
            self.state.near_price = Some(trade.price);
        } else if event.symbol == self.config.far_contract {
            self.state.far_price = Some(trade.price);
        }

        // 需要两个合约的价格
        let near_price = self.state.near_price?;
        let far_price = self.state.far_price?;

        // 计算价差（远月 - 近月）
        let spread = far_price - near_price;

        // 更新价差历史
        self.state.spread_history.push_back(spread);
        if self.state.spread_history.len() > self.config.spread_period {
            self.state.spread_history.pop_front();
        }

        // 需要足够的历史数据
        if self.state.spread_history.len() < self.config.spread_period {
            return None;
        }

        // 计算价差统计
        let (mean_spread, std_spread) = self.calculate_spread_stats()?;

        // 计算上下界
        let upper_bound = mean_spread + std_spread * self.config.spread_std_multiplier;
        let lower_bound = mean_spread - std_spread * self.config.spread_std_multiplier;

        let leverage_multiplier =
            Decimal::from_u32(self.config.leverage.leverage).unwrap_or(Decimal::ONE);

        // 无持仓时，判断开仓信号
        if !self.state.has_position {
            if spread > upper_bound {
                // 价差过大，做空价差（卖远月，买近月）
                self.state.has_position = true;
                self.state.is_long_spread = false;

                return Some(Signal {
                    id: Uuid::new_v4(),
                    strategy_id: self.meta.instance_id,
                    symbol: self.config.far_contract.clone(),
                    signal_type: SignalType::Sell, // 卖远月
                    price: far_price,
                    quantity: self.config.quantity * leverage_multiplier,
                    confidence: 0.8,
                    created_at: event.timestamp,
                });
            } else if spread < lower_bound {
                // 价差过小，做多价差（买远月，卖近月）
                self.state.has_position = true;
                self.state.is_long_spread = true;

                return Some(Signal {
                    id: Uuid::new_v4(),
                    strategy_id: self.meta.instance_id,
                    symbol: self.config.far_contract.clone(),
                    signal_type: SignalType::Buy, // 买远月
                    price: far_price,
                    quantity: self.config.quantity * leverage_multiplier,
                    confidence: 0.8,
                    created_at: event.timestamp,
                });
            }
        } else {
            // 有持仓时，判断平仓信号（价差回归均值）
            let should_close = if self.state.is_long_spread {
                // 做多价差，价差回归或超过均值时平仓
                spread >= mean_spread
            } else {
                // 做空价差，价差回归或低于均值时平仓
                spread <= mean_spread
            };

            if should_close {
                self.state.has_position = false;

                let signal_type = if self.state.is_long_spread {
                    SignalType::Sell // 平多仓
                } else {
                    SignalType::Buy // 平空仓
                };

                return Some(Signal {
                    id: Uuid::new_v4(),
                    strategy_id: self.meta.instance_id,
                    symbol: self.config.far_contract.clone(),
                    signal_type,
                    price: far_price,
                    quantity: self.config.quantity * leverage_multiplier,
                    confidence: 0.85,
                    created_at: event.timestamp,
                });
            }
        }

        None
    }
}

impl Strategy for CalendarSpreadStrategy {
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
        self.state = CalendarSpreadState::new();
    }
}
