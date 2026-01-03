//! # 资金费率套利策略 (Funding Rate Arbitrage Strategy)
//!
//! 合约专属策略：利用资金费率进行套利。
//! 当资金费率为正时做空，为负时做多。

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType};
use crate::domain::model::signal::{Signal, SignalType};

/// 资金费率套利策略配置
#[derive(Debug, Clone)]
pub struct FundingArbConfig {
    /// 资金费率阈值（绝对值，如 0.001 表示 0.1%）
    pub funding_rate_threshold: Decimal,
    /// 交易数量
    pub quantity: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
    /// 最大持仓时间（小时）
    pub max_hold_hours: u32,
}

/// 资金费率套利策略状态
#[derive(Debug, Clone)]
pub struct FundingArbState {
    /// 当前资金费率
    pub current_funding_rate: Option<Decimal>,
    /// 上次资金费率
    pub last_funding_rate: Option<Decimal>,
    /// 是否有持仓
    pub has_position: bool,
    /// 持仓方向（true=多，false=空）
    pub is_long: bool,
    /// 入场时间戳
    pub entry_timestamp: Option<i64>,
}

impl FundingArbState {
    /// 创建初始状态
    pub fn new() -> Self {
        Self {
            current_funding_rate: None,
            last_funding_rate: None,
            has_position: false,
            is_long: false,
            entry_timestamp: None,
        }
    }
}

impl Default for FundingArbState {
    fn default() -> Self {
        Self::new()
    }
}

/// 资金费率套利策略
pub struct FundingArbStrategy {
    meta: StrategyMeta,
    config: FundingArbConfig,
    state: FundingArbState,
}

impl FundingArbStrategy {
    /// 创建资金费率套利策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: FundingArbConfig,
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
                strategy_type: "funding_arb".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: FundingArbState::new(),
        }
    }

    /// 更新资金费率
    ///
    /// 外部调用此方法更新资金费率数据
    pub fn update_funding_rate(&mut self, rate: Decimal) {
        self.state.last_funding_rate = self.state.current_funding_rate;
        self.state.current_funding_rate = Some(rate);
    }

    /// 计算套利信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        // 需要资金费率数据
        let funding_rate = self.state.current_funding_rate?;

        // 参数校验
        if self.config.quantity <= Decimal::ZERO {
            return None;
        }

        let threshold = self.config.funding_rate_threshold;
        let current_ts = event.timestamp.timestamp();

        // 检查是否需要平仓（超时）
        if self.state.has_position {
            if let Some(entry_ts) = self.state.entry_timestamp {
                let hold_hours = (current_ts - entry_ts) / 3600;
                if hold_hours >= self.config.max_hold_hours as i64 {
                    // 超时平仓
                    let signal_type = if self.state.is_long {
                        SignalType::Sell
                    } else {
                        SignalType::Buy
                    };

                    self.state.has_position = false;
                    self.state.entry_timestamp = None;

                    let leverage_multiplier = Decimal::from_u32(self.config.leverage.leverage)
                        .unwrap_or(Decimal::ONE);

                    return Some(Signal {
                        id: Uuid::new_v4(),
                        strategy_id: self.meta.instance_id,
                        symbol: event.symbol.clone(),
                        signal_type,
                        price: trade.price,
                        quantity: self.config.quantity * leverage_multiplier,
                        confidence: 0.8, // 超时平仓置信度较低
                        created_at: event.timestamp,
                    });
                }
            }
        }

        // 无持仓时，根据资金费率开仓
        if !self.state.has_position {
            let signal_type = if funding_rate > threshold {
                // 资金费率为正且超过阈值，做空（收取资金费）
                self.state.is_long = false;
                Some(SignalType::Sell)
            } else if funding_rate < -threshold {
                // 资金费率为负且超过阈值，做多（收取资金费）
                self.state.is_long = true;
                Some(SignalType::Buy)
            } else {
                None
            };

            if signal_type.is_some() {
                self.state.has_position = true;
                self.state.entry_timestamp = Some(current_ts);
            }

            let leverage_multiplier = Decimal::from_u32(self.config.leverage.leverage)
                .unwrap_or(Decimal::ONE);

            return signal_type.map(|st| Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type: st,
                price: trade.price,
                quantity: self.config.quantity * leverage_multiplier,
                confidence: 1.0,
                created_at: event.timestamp,
            });
        }

        // 有持仓时，检查是否需要反向平仓
        if self.state.has_position {
            let should_close = if self.state.is_long {
                // 持有多仓，资金费率转正时平仓
                funding_rate > threshold
            } else {
                // 持有空仓，资金费率转负时平仓
                funding_rate < -threshold
            };

            if should_close {
                let signal_type = if self.state.is_long {
                    SignalType::Sell
                } else {
                    SignalType::Buy
                };

                self.state.has_position = false;
                self.state.entry_timestamp = None;

                let leverage_multiplier = Decimal::from_u32(self.config.leverage.leverage)
                    .unwrap_or(Decimal::ONE);

                return Some(Signal {
                    id: Uuid::new_v4(),
                    strategy_id: self.meta.instance_id,
                    symbol: event.symbol.clone(),
                    signal_type,
                    price: trade.price,
                    quantity: self.config.quantity * leverage_multiplier,
                    confidence: 0.9,
                    created_at: event.timestamp,
                });
            }
        }

        None
    }
}

impl Strategy for FundingArbStrategy {
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
        self.state = FundingArbState::new();
    }
}
