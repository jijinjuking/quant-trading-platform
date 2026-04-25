//! JWT auth adapter.

use std::collections::HashSet;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::port::auth_port::AuthPort;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    #[serde(default)]
    permissions: Vec<String>,
    #[serde(default)]
    role: Option<String>,
}

pub struct JwtAuthAdapter {
    decoding_key: DecodingKey,
    validation: Validation,
    admin_users: HashSet<String>,
    write_permissions: HashSet<String>,
}

impl JwtAuthAdapter {
    pub fn new(secret: String) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let admin_users = std::env::var("GATEWAY_ADMIN_USERS")
            .unwrap_or_default()
            .split(",")
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(ToString::to_string)
            .collect::<HashSet<_>>();

        let write_permissions = std::env::var("GATEWAY_WRITE_PERMISSIONS")
            .unwrap_or_else(|_| "strategies:write,orders:write,positions:write,risk:write,proxy:write".to_string())
            .split(",")
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(ToString::to_string)
            .collect::<HashSet<_>>();

        Self {
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation,
            admin_users,
            write_permissions,
        }
    }

    fn decode_claims(&self, token: &str) -> Option<Claims> {
        decode::<Claims>(token, &self.decoding_key, &self.validation)
            .ok()
            .map(|data| data.claims)
    }
}

impl AuthPort for JwtAuthAdapter {
    fn validate_token(&self, token: &str) -> bool {
        self.decode_claims(token).is_some()
    }

    fn get_user_id(&self, token: &str) -> Option<String> {
        self.decode_claims(token).map(|claims| claims.sub)
    }

    fn check_permission(&self, user_id: &str, resource: &str) -> bool {
        if user_id.trim().is_empty() {
            return false;
        }

        if self.admin_users.contains(user_id) {
            return true;
        }

        if resource.ends_with(":read") {
            return true;
        }

        self.write_permissions.contains(resource)
    }
}
