//! # 信号策略适配器 (Signal Strategy Adapter)
//!
//! 生成并发布交易信号。

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use shared::event::market_event::MarketEvent;
use tokio::sync::Mutex;
use tracing::info;

use crate::domain::logic::grid::{GridConfig, GridState};
use crate::domain::logic::mean::{MeanReversionConfig, MeanReversionState};
use crate::domain::logic::signal_generator::{generate_signal_from_market_event, StrategyRuntime};
use crate::domain::model::strategy_config::StrategyType;
use crate::domain::port::message_port::SignalMessagePort;
use crate::domain::port::strategy_port::StrategyPort;

/// 信号策略适配器 (Signal Strategy Adapter)
pub struct SignalStrategy<M>
where
    M: SignalMessagePort,
{
    publisher: M,
    strategy_type: StrategyType,
    grid_config: GridConfig,
    mean_reversion_config: MeanReversionConfig,
    grid_state: Mutex<GridState>,
    mean_reversion_state: Mutex<MeanReversionState>,
}

impl<M> SignalStrategy<M>
where
    M: SignalMessagePort,
{
    pub fn new(
        publisher: M,
        strategy_type: StrategyType,
        grid_config: GridConfig,
        mean_reversion_config: MeanReversionConfig,
    ) -> Self {
        Self {
            publisher,
            strategy_type,
            grid_config,
            mean_reversion_config,
            grid_state: Mutex::new(GridState::new()),
            mean_reversion_state: Mutex::new(MeanReversionState::new()),
        }
    }
}

#[async_trait]
impl<M> StrategyPort for SignalStrategy<M>
where
    M: SignalMessagePort + Send + Sync,
{
    async fn on_market_event(&self, event: &MarketEvent) -> Result<()> {
        // 调试：打印价格
        if let shared::event::market_event::MarketEventData::Trade(trade) = &event.data {
            println!(
                "[Grid] price={}, range=[{}, {}], grids={}",
                trade.price,
                self.grid_config.lower_price,
                self.grid_config.upper_price,
                self.grid_config.grid_count
            );
        }

        let signal = match &self.strategy_type {
            StrategyType::Grid => {
                let mut state = self.grid_state.lock().await;
                let result = generate_signal_from_market_event(
                    event,
                    StrategyRuntime::Grid {
                        config: &self.grid_config,
                        state: &mut *state,
                    },
                );
                // 调试：打印网格状态
                println!(
                    "[Grid] last_grid_index={:?}, signal={:?}",
                    state.last_grid_index,
                    result.as_ref().map(|s| &s.signal_type)
                );
                result
            }
            StrategyType::MeanReversion => {
                let mut state = self.mean_reversion_state.lock().await;
                generate_signal_from_market_event(
                    event,
                    StrategyRuntime::MeanReversion {
                        config: &self.mean_reversion_config,
                        state: &mut *state,
                    },
                )
            }
        };

        if let Some(signal) = signal {
            println!(
                "[Grid] ✅ SIGNAL: symbol={}, type={:?}, qty={}",
                signal.symbol, signal.signal_type, signal.quantity
            );
            let ok = self.publisher.publish_signal(&signal);
            if !ok {
                return Err(anyhow!("publish signal failed"));
            }
            info!(
                symbol = %signal.symbol,
                signal_type = ?signal.signal_type,
                "Signal published"
            );
        }

        Ok(())
    }
}
