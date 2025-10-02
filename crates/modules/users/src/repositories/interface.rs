use async_trait::async_trait;
use uuid::Uuid;

use pkg::RepositoryResult;
use baserepository::BaseRepository;
use crate::domain::User;
use crate::delivery::http::dto::{CreateUserDto, UpdateUserDto};

/// User-specific repository trait with additional query methods
/// Extends BaseRepository from shared module for common CRUD operations
#[async_trait]
pub trait UserRepository: BaseRepository<User, Uuid> {
    /// Find a user by username
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>>;

    /// Find a user by email
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;

    /// Find users by age range
    async fn find_by_age_range(&self, min_age: i32, max_age: i32) -> RepositoryResult<Vec<User>>;

    /// Create a new user from DTO
    async fn create_user(&self, dto: CreateUserDto) -> RepositoryResult<User>;

    /// Update a user from DTO
    async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> RepositoryResult<User>;
}
