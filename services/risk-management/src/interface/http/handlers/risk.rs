//! # 风险检查处理器 (Risk Handlers)
//!
//! 本模块提供风险检查相关的 HTTP 端点。
//!
//! ## 端点
//! - `POST /api/v1/risk/check`: 执行订单前风险检查

use axum::{extract::State, Json};
use tracing::{debug, info};

use crate::domain::model::risk_decision::RiskDecision;
use crate::domain::service::risk_checker::{RiskChecker, RiskCheckerConfig};
use crate::interface::http::dto::risk_check::{
    RiskCheckDetail, RiskCheckRequest, RiskCheckResponse,
};
use crate::state::AppState;

/// 风险检查处理器
///
/// 执行订单前的风险检查，验证订单是否符合风控要求。
/// 返回结构化的 RiskDecision（Pass / Reject），不是简单的 bool。
pub async fn check_risk(
    State(state): State<AppState>,
    Json(req): Json<RiskCheckRequest>,
) -> Json<RiskCheckResponse> {
    debug!(
        symbol = %req.symbol,
        side = %req.side,
        quantity = %req.quantity,
        price = %req.price,
        "收到风控检查请求"
    );

    // 创建风控检查器
    let checker = RiskChecker::new(RiskCheckerConfig {
        allowed_symbols: state.config.allowed_symbols.clone(),
        min_quantity: state.config.min_quantity,
        max_quantity: state.config.max_quantity,
        max_notional: state.config.max_notional,
    });

    // 执行风控检查
    let decision = checker.check(&req.symbol, &req.side, req.quantity, req.price);

    // 构建响应
    let response = match decision {
        RiskDecision::Pass => {
            info!(
                symbol = %req.symbol,
                side = %req.side,
                quantity = %req.quantity,
                "风控检查通过"
            );
            RiskCheckResponse::approved(vec![
                RiskCheckDetail::passed("symbol_check"),
                RiskCheckDetail::passed("side_check"),
                RiskCheckDetail::passed("quantity_range"),
                RiskCheckDetail::passed("notional_limit"),
            ])
        }
        RiskDecision::Reject { code, reason } => {
            info!(
                symbol = %req.symbol,
                side = %req.side,
                quantity = %req.quantity,
                code = %code,
                reason = %reason,
                "风控检查拒绝"
            );
            RiskCheckResponse::rejected(
                reason.clone(),
                vec![RiskCheckDetail::failed(&code, reason)],
            )
        }
    };

    Json(response)
}
