use thiserror::Error;
use uuid::Uuid;

/// Custom error types for repository operations
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Entity with ID {0} not found")]
    NotFound(Uuid),

    #[error("Entity with ID {0} already exists")]
    AlreadyExists(Uuid),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

/// Result type alias for repository operations
pub type RepositoryResult<T> = Result<T, RepositoryError>;

impl From<String> for RepositoryError {
    fn from(s: String) -> Self {
        RepositoryError::ValidationError(s)
    }
}

impl From<validator::ValidationErrors> for RepositoryError {
    fn from(errors: validator::ValidationErrors) -> Self {
        RepositoryError::ValidationError(errors.to_string())
    }
}
