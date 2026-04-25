//! Strategy management handlers.

use axum::{extract::State, Json};
use uuid::Uuid;

use crate::application::scheduler::StrategyConfig;
use crate::domain::model::lifecycle_state::LifecycleState;
use crate::domain::service::strategy_registry::StrategyQuery;
use crate::interface::http::dto::{
    ApiResponse, CreateStrategyRequest, CreateStrategyResponse, StrategyInfoDto,
    StrategyListResponse,
};
use crate::state::AppState;

/// GET /api/v1/strategies
pub async fn list_strategies(
    State(state): State<AppState>,
) -> Json<ApiResponse<StrategyListResponse>> {
    let Some(registry) = state.strategy_registry.as_ref() else {
        return Json(ApiResponse::err("strategy registry is not initialized"));
    };

    let mut handles = registry.query(&StrategyQuery::all());
    handles.sort_by_key(|h| h.metadata().created_at);

    let strategies = handles
        .into_iter()
        .map(|handle| {
            let metadata = handle.metadata();
            StrategyInfoDto {
                instance_id: metadata.instance_id,
                strategy_type: metadata.kind.to_string(),
                market_type: metadata.market_type.to_string(),
                symbol: metadata.symbol.clone(),
                is_active: handle.lifecycle_state() == LifecycleState::Running,
            }
        })
        .collect::<Vec<_>>();

    Json(ApiResponse::ok(StrategyListResponse {
        total: strategies.len(),
        strategies,
    }))
}

/// POST /api/v1/strategies
pub async fn create_strategy(
    State(state): State<AppState>,
    Json(req): Json<CreateStrategyRequest>,
) -> Json<ApiResponse<CreateStrategyResponse>> {
    let Some(loader) = state.strategy_loader.as_ref() else {
        return Json(ApiResponse::err("strategy loader is not initialized"));
    };

    let owner_id = match req.owner_id {
        Some(v) => v,
        None => return Json(ApiResponse::err("owner_id is required")),
    };

    let strategy_type = req.strategy_type.trim().to_ascii_lowercase();
    if strategy_type != "spot_grid" && strategy_type != "spot_mean_reversion" {
        return Json(ApiResponse::err(
            "unsupported strategy_type, allowed: spot_grid, spot_mean_reversion",
        ));
    }

    let symbol = req.symbol.trim().to_uppercase();
    if symbol.is_empty() {
        return Json(ApiResponse::err("symbol cannot be empty"));
    }

    let market_type = req.market_type.trim().to_ascii_lowercase();
    if market_type != "spot" {
        return Json(ApiResponse::err("only spot market_type is currently supported"));
    }

    let instance_id = Uuid::new_v4();
    let strategy_name = req
        .name
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| format!("{}-{}", strategy_type, symbol));

    let config = StrategyConfig {
        instance_id,
        strategy_type,
        symbol,
        owner_id,
        name: strategy_name,
        params: req.config,
        auto_start: req.auto_start.unwrap_or(true),
    };

    match loader.load_strategies(vec![config]).await {
        Ok(ids) if !ids.is_empty() => Json(ApiResponse::ok(CreateStrategyResponse {
            instance_id: ids[0],
            message: "strategy created".to_string(),
        })),
        Ok(_) => Json(ApiResponse::err("strategy was rejected by config validation")),
        Err(err) => Json(ApiResponse::err(format!("failed to create strategy: {}", err))),
    }
}
