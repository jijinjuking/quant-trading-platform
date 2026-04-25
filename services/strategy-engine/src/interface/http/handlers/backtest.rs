//! Backtest handlers.

use axum::Json;
use chrono::{Duration, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::logic::spot::{SpotGridStrategy, SpotMeanReversionStrategy};
use crate::domain::logic::spot::grid::SpotGridConfig;
use crate::domain::logic::spot::mean::SpotMeanReversionConfig;
use crate::domain::logic::strategy_trait::Strategy;
use crate::domain::model::signal::SignalType;
use crate::interface::http::dto::ApiResponse;
use shared::event::market_event::{MarketEvent, MarketEventData, MarketEventType, TradeData};

#[derive(Debug, Deserialize)]
pub struct BacktestRequest {
    pub strategy_type: String,
    pub symbol: String,
    pub prices: Vec<Decimal>,
    #[serde(default)]
    pub quantity: Option<Decimal>,
    #[serde(default)]
    pub initial_capital: Option<Decimal>,
    #[serde(default)]
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct BacktestResponse {
    pub strategy_type: String,
    pub symbol: String,
    pub total_ticks: usize,
    pub buy_signals: usize,
    pub sell_signals: usize,
    pub final_position: Decimal,
    pub final_cash: Decimal,
    pub final_equity: Decimal,
    pub total_return: Decimal,
}

/// POST /api/v1/backtest
pub async fn run_backtest(Json(req): Json<BacktestRequest>) -> Json<ApiResponse<BacktestResponse>> {
    if req.prices.len() < 2 {
        return Json(ApiResponse::err("prices must contain at least 2 points"));
    }

    let symbol = req.symbol.trim().to_uppercase();
    if symbol.is_empty() {
        return Json(ApiResponse::err("symbol cannot be empty"));
    }

    let strategy_type = req.strategy_type.trim().to_ascii_lowercase();
    let default_qty = req.quantity.unwrap_or(Decimal::new(1, 3));
    if default_qty <= Decimal::ZERO {
        return Json(ApiResponse::err("quantity must be positive"));
    }

    let mut strategy: Box<dyn Strategy> = match strategy_type.as_str() {
        "spot_grid" => {
            let cfg = parse_spot_grid_config(&req.config, default_qty);
            Box::new(SpotGridStrategy::new(Uuid::new_v4(), symbol.clone(), cfg))
        }
        "spot_mean_reversion" => {
            let cfg = parse_spot_mean_config(&req.config, default_qty);
            Box::new(SpotMeanReversionStrategy::new(Uuid::new_v4(), symbol.clone(), cfg))
        }
        _ => {
            return Json(ApiResponse::err(
                "unsupported strategy_type, allowed: spot_grid, spot_mean_reversion",
            ));
        }
    };

    strategy.activate();

    let initial_capital = req.initial_capital.unwrap_or(Decimal::new(10_000, 0));
    let mut cash = initial_capital;
    let mut position = Decimal::ZERO;
    let mut buy_signals = 0usize;
    let mut sell_signals = 0usize;

    let now = Utc::now();
    for (idx, price) in req.prices.iter().copied().enumerate() {
        if price <= Decimal::ZERO {
            return Json(ApiResponse::err("all prices must be positive"));
        }

        let event = MarketEvent {
            event_type: MarketEventType::Trade,
            exchange: "backtest".to_string(),
            symbol: symbol.clone(),
            timestamp: now + Duration::seconds(idx as i64),
            data: MarketEventData::Trade(TradeData {
                trade_id: idx.to_string(),
                price,
                quantity: default_qty,
                is_buyer_maker: false,
            }),
        };

        if let Some(signal) = strategy.on_market_event(&event) {
            let notional = signal.price * signal.quantity;
            match signal.signal_type {
                SignalType::Buy => {
                    buy_signals += 1;
                    cash -= notional;
                    position += signal.quantity;
                }
                SignalType::Sell => {
                    sell_signals += 1;
                    cash += notional;
                    position -= signal.quantity;
                }
                SignalType::Hold => {}
            }
        }
    }

    let last_price = *req.prices.last().unwrap_or(&Decimal::ZERO);
    let final_equity = cash + position * last_price;
    let total_return = if initial_capital.is_zero() {
        Decimal::ZERO
    } else {
        (final_equity - initial_capital) / initial_capital
    };

    Json(ApiResponse::ok(BacktestResponse {
        strategy_type,
        symbol,
        total_ticks: req.prices.len(),
        buy_signals,
        sell_signals,
        final_position: position,
        final_cash: cash,
        final_equity,
        total_return,
    }))
}

fn parse_spot_grid_config(config: &serde_json::Value, quantity: Decimal) -> SpotGridConfig {
    SpotGridConfig {
        upper_price: read_decimal(config.get("upper_price")).unwrap_or(Decimal::new(50_000, 0)),
        lower_price: read_decimal(config.get("lower_price")).unwrap_or(Decimal::new(40_000, 0)),
        grid_count: config
            .get("grid_count")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32)
            .unwrap_or(10),
        quantity_per_grid: read_decimal(config.get("quantity_per_grid")).unwrap_or(quantity),
    }
}

fn parse_spot_mean_config(config: &serde_json::Value, quantity: Decimal) -> SpotMeanReversionConfig {
    SpotMeanReversionConfig {
        window_size: config
            .get("window_size")
            .or_else(|| config.get("period"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(20),
        threshold_percent: read_decimal(config.get("threshold_percent"))
            .or_else(|| read_decimal(config.get("std_dev_multiplier")))
            .unwrap_or(Decimal::new(2, 2)),
        quantity: read_decimal(config.get("quantity")).unwrap_or(quantity),
    }
}

fn read_decimal(value: Option<&serde_json::Value>) -> Option<Decimal> {
    value.and_then(|v| {
        v.as_str()
            .and_then(|s| s.parse::<Decimal>().ok())
            .or_else(|| v.as_f64().and_then(Decimal::from_f64))
    })
}
