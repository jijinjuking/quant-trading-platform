//! # 现货均值回归策略 (Spot Mean Reversion Strategy)
//!
//! 现货市场的均值回归策略实现。

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// 现货均值回归策略配置
#[derive(Debug, Clone)]
pub struct SpotMeanReversionConfig {
    /// 移动平均窗口大小
    pub window_size: usize,
    /// 偏离阈值百分比（如 0.02 表示 2%）
    pub threshold_percent: Decimal,
    /// 交易数量
    pub quantity: Decimal,
}

/// 现货均值回归策略状态
#[derive(Debug, Clone)]
pub struct SpotMeanReversionState {
    /// 历史价格队列
    pub price_history: Vec<Decimal>,
}

impl SpotMeanReversionState {
    /// 创建初始状态
    pub fn new() -> Self {
        Self {
            price_history: Vec::new(),
        }
    }
}

impl Default for SpotMeanReversionState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货均值回归策略
pub struct SpotMeanReversionStrategy {
    meta: StrategyMeta,
    config: SpotMeanReversionConfig,
    state: SpotMeanReversionState,
}

impl SpotMeanReversionStrategy {
    /// 创建现货均值回归策略实例
    pub fn new(instance_id: Uuid, symbol: String, config: SpotMeanReversionConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_mean_reversion".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotMeanReversionState::new(),
        }
    }

    /// 计算均值回归信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        // 参数校验
        if self.config.window_size == 0 {
            return None;
        }
        if self.config.quantity <= Decimal::ZERO {
            return None;
        }

        // 更新价格历史
        self.state.price_history.push(trade.price);
        if self.state.price_history.len() > self.config.window_size {
            let overflow = self.state.price_history.len() - self.config.window_size;
            self.state.price_history.drain(0..overflow);
        }

        // 窗口未满，不产生信号
        if self.state.price_history.len() < self.config.window_size {
            return None;
        }

        // 计算移动平均
        let sum = self
            .state
            .price_history
            .iter()
            .fold(Decimal::ZERO, |acc, price| acc + *price);
        let window_size = Decimal::from_usize(self.config.window_size)?;
        if window_size == Decimal::ZERO {
            return None;
        }

        let moving_average = sum / window_size;
        if moving_average == Decimal::ZERO {
            return None;
        }

        // 计算偏离度
        let deviation = (trade.price - moving_average) / moving_average;

        // 判断信号类型
        let signal_type = if deviation > self.config.threshold_percent {
            Some(SignalType::Sell) // 价格高于均值，卖出
        } else if deviation < -self.config.threshold_percent {
            Some(SignalType::Buy) // 价格低于均值，买入
        } else {
            None
        };

        // 生成信号
        signal_type.map(|signal_type| Signal {
            id: Uuid::new_v4(),
            strategy_id: self.meta.instance_id,
            symbol: event.symbol.clone(),
            signal_type,
            price: trade.price,
            quantity: self.config.quantity,
            confidence: 1.0,
            created_at: event.timestamp,
        })
    }
}

impl Strategy for SpotMeanReversionStrategy {
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
        self.state = SpotMeanReversionState::new();
    }
}
