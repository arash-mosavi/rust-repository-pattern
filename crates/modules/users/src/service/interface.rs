use uuid::Uuid;

use pkg::RepositoryResult;
use crate::domain::User;
use crate::delivery::http::dto::{CreateUserDto, UpdateUserDto};

/// Service interface defining business operations for users
#[async_trait::async_trait]
pub trait IUserService {
    /// Create a new user with business logic validation
    async fn create_user(&self, dto: CreateUserDto) -> RepositoryResult<User>;
    
    /// Get a user by ID
    async fn get_user(&self, id: Uuid) -> RepositoryResult<User>;
    
    /// Get all users
    async fn get_all_users(&self) -> RepositoryResult<Vec<User>>;
    
    /// Update a user with business logic
    async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> RepositoryResult<User>;
    
    /// Delete a user
    async fn delete_user(&self, id: Uuid) -> RepositoryResult<bool>;
    
    /// Search users by username
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>>;
    
    /// Search users by email
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;
    
    /// Get users in an age range
    async fn get_users_by_age_range(&self, min_age: i32, max_age: i32) -> RepositoryResult<Vec<User>>;
    
    /// Get total user count
    async fn get_user_count(&self) -> RepositoryResult<usize>;
    
    /// Get statistics about users
    async fn get_statistics(&self) -> RepositoryResult<UserStatistics>;
}

/// Statistics about users in the system
#[derive(Debug, Clone)]
pub struct UserStatistics {
    pub total_users: usize,
    pub users_with_age: usize,
    pub average_age: Option<f64>,
}
