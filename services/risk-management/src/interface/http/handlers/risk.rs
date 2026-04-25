//! Risk check HTTP handler.

use axum::{extract::State, Json};
use tracing::{debug, info};

use crate::domain::model::risk_decision::RiskDecision;
use crate::domain::service::risk_checker::{RiskChecker, RiskCheckerConfig};
use crate::domain::service::risk_evaluator::{RiskDecision as EvalDecision, RiskEvaluator};
use crate::interface::http::dto::risk_check::{
    RiskCheckDetail, RiskCheckRequest, RiskCheckResponse,
};
use crate::state::AppState;

pub async fn check_risk(
    State(state): State<AppState>,
    Json(req): Json<RiskCheckRequest>,
) -> Json<RiskCheckResponse> {
    debug!(
        symbol = %req.symbol,
        side = %req.side,
        quantity = %req.quantity,
        price = %req.price,
        "received risk check request"
    );

    let checker = RiskChecker::new(RiskCheckerConfig {
        allowed_symbols: state.config.allowed_symbols.clone(),
        min_quantity: state.config.min_quantity,
        max_quantity: state.config.max_quantity,
        max_notional: state.config.max_notional,
    });

    let decision = checker.check(&req.symbol, &req.side, req.quantity, req.price);
    if let RiskDecision::Reject { code, reason } = decision {
        info!(symbol = %req.symbol, code = %code, reason = %reason, "risk rejected by checker");
        return Json(RiskCheckResponse::rejected(
            reason.clone(),
            vec![RiskCheckDetail::failed(code, reason)],
        ));
    }

    let evaluator = RiskEvaluator::new();
    let profile = match state.risk_repository.get_profile(req.strategy_id) {
        Some(p) => p,
        None => {
            return Json(RiskCheckResponse::rejected(
                "missing risk profile",
                vec![RiskCheckDetail::failed("profile", "risk profile not found")],
            ));
        }
    };

    match evaluator.evaluate_order(&profile, req.quantity, req.price) {
        Ok(EvalDecision::Approved) => Json(RiskCheckResponse::approved(vec![
            RiskCheckDetail::passed("symbol_check"),
            RiskCheckDetail::passed("side_check"),
            RiskCheckDetail::passed("quantity_range"),
            RiskCheckDetail::passed("notional_limit"),
            RiskCheckDetail::passed("profile_evaluation"),
        ])),
        Ok(EvalDecision::Rejected(reason)) => Json(RiskCheckResponse::rejected(
            reason.clone(),
            vec![RiskCheckDetail::failed("profile_evaluation", reason)],
        )),
        Ok(EvalDecision::RequiresReview) => Json(RiskCheckResponse::rejected(
            "risk profile requires manual review",
            vec![RiskCheckDetail::failed(
                "profile_review",
                "risk profile needs review before trading",
            )],
        )),
        Err(err) => Json(RiskCheckResponse::rejected(
            "risk evaluator failed",
            vec![RiskCheckDetail::failed("evaluator_error", err.to_string())],
        )),
    }
}
