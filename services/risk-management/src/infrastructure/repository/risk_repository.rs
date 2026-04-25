//! Risk repository implementation.

use std::collections::HashMap;
use std::sync::RwLock;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::model::risk_profile::RiskProfile;
use crate::domain::port::risk_repository_port::RiskRepositoryPort;

pub struct RiskRepository {
    profiles: RwLock<HashMap<Uuid, RiskProfile>>,
    default_profile: RiskProfile,
}

impl RiskRepository {
    pub fn new(default_profile: RiskProfile) -> Self {
        Self {
            profiles: RwLock::new(HashMap::new()),
            default_profile,
        }
    }

    pub fn with_defaults(
        max_leverage: Decimal,
        max_drawdown: Decimal,
        max_position_size: Decimal,
        daily_loss_limit: Decimal,
    ) -> Self {
        Self::new(RiskProfile {
            user_id: Uuid::nil(),
            max_leverage,
            max_drawdown,
            max_position_size,
            daily_loss_limit,
        })
    }
}

impl RiskRepositoryPort for RiskRepository {
    fn get_profile(&self, user_id: Uuid) -> Option<RiskProfile> {
        if let Ok(profiles) = self.profiles.read() {
            if let Some(profile) = profiles.get(&user_id) {
                return Some(profile.clone());
            }
        }

        if user_id.is_nil() {
            return None;
        }

        let mut profile = self.default_profile.clone();
        profile.user_id = user_id;
        Some(profile)
    }

    fn save_profile(&self, profile: &RiskProfile) -> bool {
        let mut profiles = match self.profiles.write() {
            Ok(v) => v,
            Err(_) => return false,
        };
        profiles.insert(profile.user_id, profile.clone());
        true
    }
}
