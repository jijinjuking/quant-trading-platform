//! Infrastructure Errors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
}
