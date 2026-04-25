//! User domain service.

use anyhow::Result;

pub struct UserDomainService;

impl UserDomainService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_password(&self, password: &str) -> Result<bool> {
        let has_min_len = password.chars().count() >= 8;
        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = password
            .chars()
            .any(|c| !c.is_ascii_alphanumeric() && !c.is_ascii_whitespace());

        Ok(has_min_len && has_upper && has_lower && has_digit && has_symbol)
    }
}
