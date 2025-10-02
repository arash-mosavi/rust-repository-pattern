use serde::{Deserialize, Serialize};
use validator::Validate;

/// DTO for creating a new user
/// Validation rules similar to class-validator or ozzo-validation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    /// Username must be between 3 and 50 characters, alphanumeric with underscores
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    /// Email must be valid format
    #[validate(email)]
    pub email: String,
    
    /// Full name must be between 2 and 100 characters
    #[validate(length(min = 2, max = 100))]
    pub full_name: String,
    
    /// Age must be between 1 and 150 if provided
    #[validate(range(min = 1, max = 150))]
    pub age: Option<i32>,
}

/// DTO for updating an existing user
/// Optional fields with same validation rules
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    /// Username must be between 3 and 50 characters if provided
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    
    /// Email must be valid format if provided
    #[validate(email)]
    pub email: Option<String>,
    
    /// Full name must be between 2 and 100 characters if provided
    #[validate(length(min = 2, max = 100))]
    pub full_name: Option<String>,
    
    /// Age must be between 1 and 150 if provided
    #[validate(range(min = 1, max = 150))]
    pub age: Option<i32>,
}

/// Response DTOs for HTTP API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub age: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}
