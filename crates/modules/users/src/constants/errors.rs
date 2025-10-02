use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User with ID {0} not found")]
    NotFound(Uuid),

    #[error("User with ID {0} already exists")]
    AlreadyExists(Uuid),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type UserResult<T> = Result<T, UserError>;

impl From<String> for UserError {
    fn from(s: String) -> Self {
        UserError::ValidationError(s)
    }
}
