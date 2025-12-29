//! Domain Errors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Risk limit exceeded: {0}")]
    RiskLimitExceeded(String),
}
