//! Authentication application service.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::model::user::{MembershipTier, User, UserStatus};
use crate::domain::port::user_repository_port::UserRepositoryPort;
use crate::domain::service::user_domain_service::UserDomainService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub email: String,
    pub username: String,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserView {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub membership_tier: MembershipTier,
    pub status: UserStatus,
    pub created_at: chrono::DateTime<Utc>,
}

impl From<User> for UserView {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            membership_tier: value.membership_tier,
            status: value.status,
            created_at: value.created_at,
        }
    }
}

pub struct AuthService {
    repository: Arc<dyn UserRepositoryPort>,
    domain_service: UserDomainService,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(repository: Arc<dyn UserRepositoryPort>, jwt_secret: String) -> Self {
        Self {
            repository,
            domain_service: UserDomainService::new(),
            jwt_secret,
        }
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<UserView> {
        let username = username.trim();
        let email = email.trim().to_ascii_lowercase();

        if username.is_empty() {
            return Err(anyhow!("username cannot be empty"));
        }
        if email.is_empty() || !email.contains('@') {
            return Err(anyhow!("invalid email"));
        }

        if !self.domain_service.validate_password(password)? {
            return Err(anyhow!(
                "password must be 8+ chars and include upper/lower/digit/symbol"
            ));
        }

        if self.repository.find_by_email(&email).is_some() {
            return Err(anyhow!("email already exists"));
        }
        if self.repository.find_by_username(username).is_some() {
            return Err(anyhow!("username already exists"));
        }

        let password_hash = Self::hash_password(password)?;
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email,
            password_hash,
            membership_tier: MembershipTier::Free,
            status: UserStatus::Active,
            created_at: Utc::now(),
        };

        if !self.repository.save(&user) {
            return Err(anyhow!("failed to persist user"));
        }

        Ok(UserView::from(user))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(String, UserView)> {
        let email = email.trim().to_ascii_lowercase();
        let user = self
            .repository
            .find_by_email(&email)
            .ok_or_else(|| anyhow!("invalid credentials"))?;

        if !matches!(user.status, UserStatus::Active) {
            return Err(anyhow!("user is not active"));
        }

        Self::verify_password(password, &user.password_hash)?;

        let claims = AuthClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            username: user.username.clone(),
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .context("failed to encode jwt token")?;

        Ok((token, UserView::from(user)))
    }

    pub fn user_from_token(&self, token: &str) -> Result<UserView> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let data = decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .context("invalid token")?;

        let user_id = Uuid::parse_str(&data.claims.sub).context("invalid subject in token")?;
        let user = self
            .repository
            .find_by_id(user_id)
            .ok_or_else(|| anyhow!("user not found"))?;

        Ok(UserView::from(user))
    }

    fn hash_password(password: &str) -> Result<String> {
        let salt_raw = Uuid::new_v4();
        let salt = SaltString::encode_b64(salt_raw.as_bytes()).map_err(|e| anyhow!("invalid salt: {}", e))?;
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("failed to hash password: {}", e))?
            .to_string();
        Ok(hash)
    }

    fn verify_password(password: &str, password_hash: &str) -> Result<()> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(|e| anyhow!("invalid stored password hash: {}", e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow!("invalid credentials"))
    }
}
