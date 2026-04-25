//! Risk evaluator domain service.

use anyhow::Result;
use rust_decimal::Decimal;

use crate::domain::model::risk_profile::RiskProfile;

pub struct RiskEvaluator;

impl RiskEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, profile: &RiskProfile) -> Result<RiskDecision> {
        if profile.max_leverage <= Decimal::ZERO {
            return Ok(RiskDecision::Rejected("invalid max_leverage".to_string()));
        }
        if profile.max_drawdown < Decimal::ZERO || profile.max_drawdown > Decimal::ONE {
            return Ok(RiskDecision::Rejected("max_drawdown must be between 0 and 1".to_string()));
        }
        if profile.max_position_size <= Decimal::ZERO {
            return Ok(RiskDecision::Rejected("max_position_size must be positive".to_string()));
        }
        if profile.daily_loss_limit <= Decimal::ZERO {
            return Ok(RiskDecision::RequiresReview);
        }
        Ok(RiskDecision::Approved)
    }

    pub fn evaluate_order(
        &self,
        profile: &RiskProfile,
        quantity: Decimal,
        price: Decimal,
    ) -> Result<RiskDecision> {
        if quantity <= Decimal::ZERO {
            return Ok(RiskDecision::Rejected("quantity must be positive".to_string()));
        }
        if price <= Decimal::ZERO {
            return Ok(RiskDecision::Rejected("price must be positive".to_string()));
        }

        if quantity > profile.max_position_size {
            return Ok(RiskDecision::Rejected(format!(
                "position size {} exceeds limit {}",
                quantity, profile.max_position_size
            )));
        }

        let notional = quantity * price;
        let max_notional_by_leverage = profile.max_leverage * profile.daily_loss_limit;
        if max_notional_by_leverage > Decimal::ZERO && notional > max_notional_by_leverage {
            return Ok(RiskDecision::Rejected(format!(
                "notional {} exceeds leverage-adjusted limit {}",
                notional, max_notional_by_leverage
            )));
        }

        self.evaluate(profile)
    }
}

#[derive(Debug, Clone)]
pub enum RiskDecision {
    Approved,
    Rejected(String),
    RequiresReview,
}
