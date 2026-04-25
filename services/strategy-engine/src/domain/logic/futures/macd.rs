//! # 合约 MACD 策略 (Futures MACD Strategy)
//!
//! 基于 MACD 指标的合约趋势跟踪策略。
//! 支持杠杆、双向持仓。

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType};
use crate::domain::model::signal::{Signal, SignalType};

/// 合约 MACD 配置
#[derive(Debug, Clone)]
pub struct FuturesMacdConfig {
    /// 快线周期
    pub fast_period: usize,
    /// 慢线周期
    pub slow_period: usize,
    /// 信号线周期
    pub signal_period: usize,
    /// 交易数量
    pub quantity: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
}

impl Default for FuturesMacdConfig {
    fn default() -> Self {
        Self {
            fast_period: 12,
            slow_period: 26,
            signal_period: 9,
            quantity: Decimal::new(1, 3), // 0.001
            leverage: LeverageConfig {
                leverage: 10,
                margin_type: crate::domain::model::market_type::MarginType::Cross,
            },
        }
    }
}

/// 合约 MACD 状态
#[derive(Debug, Clone)]
pub struct FuturesMacdState {
    /// 价格历史
    pub price_history: Vec<Decimal>,
    /// MACD 历史
    pub macd_history: Vec<Decimal>,
    /// 快线 EMA
    pub fast_ema: Option<Decimal>,
    /// 慢线 EMA
    pub slow_ema: Option<Decimal>,
    /// 信号线
    pub signal_line: Option<Decimal>,
    /// 上一次柱状图值
    pub last_histogram: Option<Decimal>,
}

impl FuturesMacdState {
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

impl Default for FuturesMacdState {
    fn default() -> Self {
        Self::new()
    }
}

/// 合约 MACD 策略
pub struct FuturesMacdStrategy {
    meta: StrategyMeta,
    config: FuturesMacdConfig,
    state: FuturesMacdState,
}

impl FuturesMacdStrategy {
    /// 创建合约 MACD 策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: FuturesMacdConfig,
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
                strategy_type: "futures_macd".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: FuturesMacdState::new(),
        }
    }

    /// 计算 MACD 信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        // 参数校验
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

        // 更新价格历史
        self.state.price_history.push(price);
        if self.state.price_history.len() > max_period {
            let overflow = self.state.price_history.len() - max_period;
            self.state.price_history.drain(0..overflow);
        }

        // 计算快线 EMA
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

        // 计算慢线 EMA
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

        // 计算 MACD 线
        let macd_line = fast_ema - slow_ema;
        self.state.macd_history.push(macd_line);
        if self.state.macd_history.len() > self.config.signal_period {
            let overflow = self.state.macd_history.len() - self.config.signal_period;
            self.state.macd_history.drain(0..overflow);
        }

        // 计算信号线
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

        // 计算柱状图（MACD - Signal）
        let histogram = macd_line - signal_line;
        let last_histogram = self.state.last_histogram;
        self.state.last_histogram = Some(histogram);

        // 判断信号类型（合约支持做空）
        let signal_type = match last_histogram {
            Some(last) if last <= Decimal::ZERO && histogram > Decimal::ZERO => {
                // 金叉：做多
                Some(SignalType::Buy)
            }
            Some(last) if last >= Decimal::ZERO && histogram < Decimal::ZERO => {
                // 死叉：做空
                Some(SignalType::Sell)
            }
            _ => None,
        };

        // 计算带杠杆的数量
        let leverage_multiplier =
            Decimal::from_u32(self.config.leverage.leverage).unwrap_or(Decimal::ONE);
        let leveraged_quantity = self.config.quantity * leverage_multiplier;

        // 生成信号
        signal_type.map(|signal_type| Signal {
            id: Uuid::new_v4(),
            strategy_id: self.meta.instance_id,
            symbol: event.symbol.clone(),
            signal_type,
            price,
            quantity: leveraged_quantity,
            confidence: 0.85,
            created_at: event.timestamp,
        })
    }
}

impl Strategy for FuturesMacdStrategy {
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
        self.state = FuturesMacdState::new();
    }
}

/// 计算平均值
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

/// 更新 EMA
fn update_ema(current: Decimal, previous: Decimal, period: usize) -> Option<Decimal> {
    if period == 0 {
        return None;
    }
    let period_decimal = Decimal::from_usize(period)?;
    let multiplier = Decimal::from_u32(2)? / (period_decimal + Decimal::ONE);
    Some((current - previous) * multiplier + previous)
}
