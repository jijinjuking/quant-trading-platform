//! # 信号生成入口 (Signal Generator)
//!
//! 领域层策略信号生成入口。

use shared::event::market_event::MarketEvent;

use crate::domain::logic::grid::{calculate_grid_signal, GridConfig, GridState};
use crate::domain::logic::mean::{
    calculate_mean_reversion_signal, MeanReversionConfig, MeanReversionState,
};
use crate::domain::model::signal::Signal;

/// 策略运行上下文 (Strategy Runtime)
pub enum StrategyRuntime<'a> {
    /// 网格策略运行上下文 (Grid Runtime)
    Grid {
        /// 网格策略配置 (Grid Config)
        config: &'a GridConfig,
        /// 网格策略状态 (Grid State)
        state: &'a mut GridState,
    },
    /// 均值回归策略运行上下文 (Mean Reversion Runtime)
    MeanReversion {
        /// 均值回归策略配置 (Mean Reversion Config)
        config: &'a MeanReversionConfig,
        /// 均值回归策略状态 (Mean Reversion State)
        state: &'a mut MeanReversionState,
    },
}

/// 根据运行上下文生成交易信号 (Generate Trading Signal)
pub fn generate_signal_from_market_event(
    event: &MarketEvent,
    runtime: StrategyRuntime<'_>,
) -> Option<Signal> {
    match runtime {
        StrategyRuntime::Grid { config, state } => calculate_grid_signal(event, config, state),
        StrategyRuntime::MeanReversion { config, state } => {
            calculate_mean_reversion_signal(event, config, state)
        }
    }
}
