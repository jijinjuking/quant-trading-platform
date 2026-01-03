//! # 策略评估处理器 (Evaluate Handler)
//!
//! 处理策略评估请求，供 trading-engine 调用。
//!
//! ## API
//! - `POST /api/v1/strategy/evaluate`: 评估策略，返回交易意图

use axum::{extract::State, Json};
use chrono::Utc;
use tracing::{debug, warn};

use crate::domain::logic::grid::{calculate_grid_signal, GridState};
use crate::domain::logic::mean::{calculate_mean_reversion_signal, MeanReversionState};
use crate::domain::model::signal::SignalType;
use crate::domain::port::{GridStateData, MeanReversionStateData};
use crate::interface::http::dto::{
    ApiResponse, EvaluateRequest, EvaluateResponse, OrderIntentDto,
};
use crate::state::AppState;
use shared::event::market_event::{MarketEvent, MarketEventData, MarketEventType, TradeData};

/// POST /api/v1/strategy/evaluate
///
/// 评估策略，根据行情生成交易意图。
/// 这是 trading-engine 调用的核心 API。
pub async fn evaluate_strategy(
    State(state): State<AppState>,
    Json(req): Json<EvaluateRequest>,
) -> Json<ApiResponse<EvaluateResponse>> {
    debug!(
        "收到策略评估请求: strategy_id={}, symbol={}, price={}",
        req.strategy_id, req.symbol, req.price
    );

    // 构造 MarketEvent
    let market_event = MarketEvent {
        event_type: MarketEventType::Trade,
        exchange: "binance".to_string(),
        symbol: req.symbol.clone(),
        timestamp: Utc::now(),
        data: MarketEventData::Trade(TradeData {
            trade_id: req.timestamp.to_string(),
            price: req.price,
            quantity: req.quantity,
            is_buyer_maker: req.is_buyer_maker,
        }),
    };

    // 使用策略 ID 作为状态 Key
    let strategy_id = req.strategy_id.to_string();

    // 根据配置的策略类型评估（从 Redis 读取/写入状态）
    let signal = evaluate_with_redis_state(&state, &market_event, &strategy_id).await;

    // 转换为响应
    let response = match signal {
        Some(sig) => {
            let side = match sig.signal_type {
                SignalType::Buy => "buy",
                SignalType::Sell => "sell",
                SignalType::Hold => "hold",
            };

            // Hold 信号不生成交易意图
            if matches!(sig.signal_type, SignalType::Hold) {
                EvaluateResponse {
                    has_intent: false,
                    intent: None,
                }
            } else {
                EvaluateResponse {
                    has_intent: true,
                    intent: Some(OrderIntentDto {
                        id: sig.id,
                        strategy_id: sig.strategy_id,
                        symbol: sig.symbol,
                        side: side.to_string(),
                        quantity: sig.quantity,
                        price: Some(sig.price),
                        order_type: "limit".to_string(),
                        confidence: sig.confidence,
                        created_at: Utc::now().timestamp_millis(),
                    }),
                }
            }
        }
        None => EvaluateResponse {
            has_intent: false,
            intent: None,
        },
    };

    Json(ApiResponse::ok(response))
}

/// 使用 Redis 存储的状态进行策略评估
async fn evaluate_with_redis_state(
    state: &AppState,
    event: &MarketEvent,
    strategy_id: &str,
) -> Option<crate::domain::model::signal::Signal> {
    use crate::domain::model::strategy_config::StrategyType;

    let config = &state.config;

    match config.strategy_type {
        StrategyType::Grid => evaluate_grid_with_redis(state, event, strategy_id).await,
        StrategyType::MeanReversion => evaluate_mean_with_redis(state, event, strategy_id).await,
    }
}

/// 网格策略评估（使用 Redis 状态）
async fn evaluate_grid_with_redis(
    state: &AppState,
    event: &MarketEvent,
    strategy_id: &str,
) -> Option<crate::domain::model::signal::Signal> {
    // 1. 从 Redis 读取状态
    let state_data = match state.strategy_state.get_grid_state(strategy_id).await {
        Ok(data) => data.unwrap_or_default(),
        Err(e) => {
            warn!("读取网格策略状态失败: {}, 使用默认状态", e);
            GridStateData::default()
        }
    };

    // 2. 转换为 domain 层的 GridState
    let mut grid_state = GridState {
        last_grid_index: state_data.last_grid_index,
        last_price: state_data.last_price,
    };

    // 3. 执行策略计算
    let signal = calculate_grid_signal(event, &state.config.grid_config, &mut grid_state);

    // 4. 保存更新后的状态到 Redis
    let updated_data = GridStateData {
        last_grid_index: grid_state.last_grid_index,
        last_price: grid_state.last_price,
    };
    if let Err(e) = state.strategy_state.save_grid_state(strategy_id, &updated_data).await {
        warn!("保存网格策略状态失败: {}", e);
    }

    signal
}

/// 均值回归策略评估（使用 Redis 状态）
async fn evaluate_mean_with_redis(
    state: &AppState,
    event: &MarketEvent,
    strategy_id: &str,
) -> Option<crate::domain::model::signal::Signal> {
    // 1. 从 Redis 读取状态
    let state_data = match state.strategy_state.get_mean_reversion_state(strategy_id).await {
        Ok(data) => data.unwrap_or_default(),
        Err(e) => {
            warn!("读取均值回归策略状态失败: {}, 使用默认状态", e);
            MeanReversionStateData::default()
        }
    };

    // 2. 转换为 domain 层的 MeanReversionState
    let mut mean_state = MeanReversionState {
        price_history: state_data.price_history,
    };

    // 3. 执行策略计算
    let signal = calculate_mean_reversion_signal(
        event,
        &state.config.mean_reversion_config,
        &mut mean_state,
    );

    // 4. 保存更新后的状态到 Redis
    let updated_data = MeanReversionStateData {
        price_history: mean_state.price_history,
    };
    if let Err(e) = state
        .strategy_state
        .save_mean_reversion_state(strategy_id, &updated_data)
        .await
    {
        warn!("保存均值回归策略状态失败: {}", e);
    }

    signal
}

/// POST /api/v1/strategy/evaluate/batch
///
/// 批量评估多个策略（预留接口）
#[allow(dead_code)]
pub async fn evaluate_batch() -> Json<ApiResponse<Vec<EvaluateResponse>>> {
    warn!("批量评估接口尚未实现");
    Json(ApiResponse::err("批量评估接口尚未实现"))
}
