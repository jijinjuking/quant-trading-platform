//! # 合约网格策略 (Futures Grid Strategy)
//!
//! 合约市场的网格交易策略实现。
//! 支持杠杆、双向持仓。

use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::{LeverageConfig, MarketType, PositionSide};
use crate::domain::model::signal::{Signal, SignalType};

/// 合约网格策略配置
#[derive(Debug, Clone)]
pub struct FuturesGridConfig {
    /// 价格上界
    pub upper_price: Decimal,
    /// 价格下界
    pub lower_price: Decimal,
    /// 网格数量
    pub grid_count: u32,
    /// 每格交易数量
    pub quantity_per_grid: Decimal,
    /// 杠杆配置
    pub leverage: LeverageConfig,
    /// 持仓方向（单向/双向）
    pub position_side: PositionSide,
}

/// 合约网格策略状态
#[derive(Debug, Clone)]
pub struct FuturesGridState {
    /// 上一次价格所在网格索引
    pub last_grid_index: Option<i32>,
    /// 上一次价格
    pub last_price: Option<Decimal>,
    /// 当前多头仓位
    pub long_position: Decimal,
    /// 当前空头仓位
    pub short_position: Decimal,
}

impl FuturesGridState {
    /// 创建初始状态
    pub fn new() -> Self {
        Self {
            last_grid_index: None,
            last_price: None,
            long_position: Decimal::ZERO,
            short_position: Decimal::ZERO,
        }
    }
}

impl Default for FuturesGridState {
    fn default() -> Self {
        Self::new()
    }
}

/// 合约网格策略
pub struct FuturesGridStrategy {
    meta: StrategyMeta,
    config: FuturesGridConfig,
    state: FuturesGridState,
}

impl FuturesGridStrategy {
    /// 创建合约网格策略实例
    pub fn new(
        instance_id: Uuid,
        symbol: String,
        config: FuturesGridConfig,
        market_type: MarketType,
    ) -> Self {
        let market = if market_type.is_futures() {
            market_type
        } else {
            MarketType::UsdtFutures // 默认 U 本位
        };

        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "futures_grid".to_string(),
                market_type: market,
                symbol,
                is_active: false,
            },
            config,
            state: FuturesGridState::new(),
        }
    }

    /// 计算网格信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        // 参数校验
        if self.config.grid_count == 0 {
            return None;
        }
        if self.config.upper_price <= self.config.lower_price {
            return None;
        }
        if self.config.quantity_per_grid <= Decimal::ZERO {
            return None;
        }

        let price = trade.price;

        // 价格超出范围
        if price < self.config.lower_price || price > self.config.upper_price {
            self.state.last_price = Some(price);
            self.state.last_grid_index = None;
            return None;
        }

        // 计算当前网格索引
        let grid_count = Decimal::from_u32(self.config.grid_count)?;
        let grid_size = (self.config.upper_price - self.config.lower_price) / grid_count;
        if grid_size <= Decimal::ZERO {
            return None;
        }

        let position = (price - self.config.lower_price) / grid_size;
        let current_index = position.to_i32()?;

        // 判断信号类型（合约支持双向）
        let signal_type = match self.state.last_grid_index {
            Some(last_index) if current_index < last_index => {
                // 价格下跌，根据持仓方向决定
                match self.config.position_side {
                    PositionSide::Long => Some(SignalType::Buy),   // 做多加仓
                    PositionSide::Short => Some(SignalType::Sell), // 做空平仓
                    PositionSide::Both => Some(SignalType::Buy),   // 双向模式做多
                }
            }
            Some(last_index) if current_index > last_index => {
                // 价格上涨，根据持仓方向决定
                match self.config.position_side {
                    PositionSide::Long => Some(SignalType::Sell),  // 做多平仓
                    PositionSide::Short => Some(SignalType::Buy),  // 做空加仓（反向）
                    PositionSide::Both => Some(SignalType::Sell),  // 双向模式做空
                }
            }
            _ => None,
        };

        // 更新状态
        self.state.last_grid_index = Some(current_index);
        self.state.last_price = Some(price);

        // 计算带杠杆的数量
        let leverage_multiplier = Decimal::from_u32(self.config.leverage.leverage)
            .unwrap_or(Decimal::ONE);
        let leveraged_quantity = self.config.quantity_per_grid * leverage_multiplier;

        // 生成信号
        signal_type.map(|signal_type| Signal {
            id: Uuid::new_v4(),
            strategy_id: self.meta.instance_id,
            symbol: event.symbol.clone(),
            signal_type,
            price,
            quantity: leveraged_quantity,
            confidence: 1.0,
            created_at: event.timestamp,
        })
    }
}

impl Strategy for FuturesGridStrategy {
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
        self.state = FuturesGridState::new();
    }
}
