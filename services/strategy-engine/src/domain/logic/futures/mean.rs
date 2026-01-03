//! # 合约均值回归策略 (Futures Mean Reversion Strategy)
//!
//! 合约市场的均值回归策略实现。
//! 支持杠杆、双向持仓。

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType, PositionSide};
use crate::domain::model::signal::{Signal, SignalType};

/// 合约均值回归策略配置
#[derive(Debug, Clone)]
pub struct FuturesMeanReversionConfig {
    /// 移动平均窗口大小
    pub window_size: usize,
    /// 偏离阈值百分比
    pub threshold_percent: Decimal,
    /// 交易数量
    pub quantity: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
    /// 持仓方向
    pub position_side: PositionSide,
}

/// 合约均值回归策略状态
#[derive(Debug, Clone)]
pub struct FuturesMeanReversionState {
    /// 历史价格队列
    pub price_history: Vec<Decimal>,
    /// 当前持仓方向
    pub current_side: Option<PositionSide>,
    /// 当前持仓数量
    pub current_position: Decimal,
}

impl FuturesMeanReversionState {
    /// 创建初始状态
    pub fn new() -> Self {
        Self {
            price_history: Vec::new(),
            current_side: None,
            current_position: Decimal::ZERO,
        }
    }
}

impl Default for FuturesMeanReversionState {
    fn default() -> Self {
        Self::new()
    }
}

/// 合约均值回归策略
pub struct FuturesMeanReversionStrategy {
    meta: StrategyMeta,
    config: FuturesMeanReversionConfig,
    state: FuturesMeanReversionState,
}

impl FuturesMeanReversionStrategy {
    /// 创建合约均值回归策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: FuturesMeanReversionConfig,
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
                strategy_type: "futures_mean_reversion".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: FuturesMeanReversionState::new(),
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

        // 判断信号类型（合约支持做空）
        let signal_type = if deviation > self.config.threshold_percent {
            // 价格高于均值，做空
            Some(SignalType::Sell)
        } else if deviation < -self.config.threshold_percent {
            // 价格低于均值，做多
            Some(SignalType::Buy)
        } else {
            None
        };

        // 计算带杠杆的数量
        let leverage_multiplier = Decimal::from_u32(self.config.leverage.leverage)
            .unwrap_or(Decimal::ONE);
        let leveraged_quantity = self.config.quantity * leverage_multiplier;

        // 生成信号
        signal_type.map(|signal_type| Signal {
            id: Uuid::new_v4(),
            strategy_id: self.meta.instance_id,
            symbol: event.symbol.clone(),
            signal_type,
            price: trade.price,
            quantity: leveraged_quantity,
            confidence: 1.0,
            created_at: event.timestamp,
        })
    }
}

impl Strategy for FuturesMeanReversionStrategy {
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
        self.state = FuturesMeanReversionState::new();
    }
}
