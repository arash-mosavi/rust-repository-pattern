use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;

use pkg::{RepositoryError, RepositoryResult};
use crate::domain::User;
use crate::delivery::http::dto::{CreateUserDto, UpdateUserDto};
use crate::repositories::UserRepository;
use super::interface::{IUserService, UserStatistics};

/// Service layer that contains business logic and uses the repository
/// This demonstrates dependency injection with the repository pattern
pub struct UserService<R: UserRepository> {
    repository: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    /// Create a new user service with a repository implementation
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Create a new user with business logic validation
    pub async fn create_user(&self, dto: CreateUserDto) -> RepositoryResult<User> {
        // Business logic: Check if username already exists
        if let Some(_existing) = self.repository.find_by_username(&dto.username).await? {
            return Err(RepositoryError::ValidationError(format!(
                "Username '{}' is already taken",
                dto.username
            )));
        }

        // Business logic: Check if email already exists
        if let Some(_existing) = self.repository.find_by_email(&dto.email).await? {
            return Err(RepositoryError::ValidationError(format!(
                "Email '{}' is already registered",
                dto.email
            )));
        }

        // Create the user through repository
        self.repository.create_user(dto).await
    }

    /// Get a user by ID
    pub async fn get_user(&self, id: Uuid) -> RepositoryResult<User> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or(RepositoryError::NotFound(id))
    }

    /// Get all users
    pub async fn get_all_users(&self) -> RepositoryResult<Vec<User>> {
        self.repository.find_all().await
    }

    /// Update a user with business logic
    pub async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> RepositoryResult<User> {
        // Check if user exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(RepositoryError::NotFound(id))?;

        // Business logic: If username is being changed, check if it's available
        if let Some(ref new_username) = dto.username {
            if new_username != &existing.username {
                if let Some(_) = self.repository.find_by_username(new_username).await? {
                    return Err(RepositoryError::ValidationError(format!(
                        "Username '{}' is already taken",
                        new_username
                    )));
                }
            }
        }

        // Business logic: If email is being changed, check if it's available
        if let Some(ref new_email) = dto.email {
            if new_email != &existing.email {
                if let Some(_) = self.repository.find_by_email(new_email).await? {
                    return Err(RepositoryError::ValidationError(format!(
                        "Email '{}' is already registered",
                        new_email
                    )));
                }
            }
        }

        self.repository.update_user(id, dto).await
    }

    /// Delete a user
    pub async fn delete_user(&self, id: Uuid) -> RepositoryResult<bool> {
        // Business logic: Additional checks before deletion
        let exists = self.repository.exists(id).await?;
        if !exists {
            return Err(RepositoryError::NotFound(id));
        }

        self.repository.delete(id).await
    }

    /// Search users by username
    pub async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>> {
        self.repository.find_by_username(username).await
    }

    /// Search users by email
    pub async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        self.repository.find_by_email(email).await
    }

    /// Get users in an age range
    pub async fn get_users_by_age_range(
        &self,
        min_age: i32,
        max_age: i32,
    ) -> RepositoryResult<Vec<User>> {
        // Business logic: Validate age range
        if min_age > max_age {
            return Err(RepositoryError::ValidationError(
                "Minimum age cannot be greater than maximum age".to_string(),
            ));
        }

        self.repository.find_by_age_range(min_age, max_age).await
    }

    /// Get total user count
    pub async fn get_user_count(&self) -> RepositoryResult<usize> {
        self.repository.count().await
    }

    /// Get statistics about users
    pub async fn get_statistics(&self) -> RepositoryResult<UserStatistics> {
        let all_users = self.repository.find_all().await?;
        let total = all_users.len();

        let ages: Vec<i32> = all_users.iter().filter_map(|u| u.age).collect();

        let average_age = if !ages.is_empty() {
            Some(ages.iter().sum::<i32>() as f64 / ages.len() as f64)
        } else {
            None
        };

        let users_with_age = ages.len();

        Ok(UserStatistics {
            total_users: total,
            users_with_age,
            average_age,
        })
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> IUserService for UserService<R> {
    async fn create_user(&self, dto: CreateUserDto) -> RepositoryResult<User> {
        self.create_user(dto).await
    }
    
    async fn get_user(&self, id: Uuid) -> RepositoryResult<User> {
        self.get_user(id).await
    }
    
    async fn get_all_users(&self) -> RepositoryResult<Vec<User>> {
        self.get_all_users().await
    }
    
    async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> RepositoryResult<User> {
        self.update_user(id, dto).await
    }
    
    async fn delete_user(&self, id: Uuid) -> RepositoryResult<bool> {
        self.delete_user(id).await
    }
    
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>> {
        self.find_by_username(username).await
    }
    
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        self.find_by_email(email).await
    }
    
    async fn get_users_by_age_range(&self, min_age: i32, max_age: i32) -> RepositoryResult<Vec<User>> {
        self.get_users_by_age_range(min_age, max_age).await
    }
    
    async fn get_user_count(&self) -> RepositoryResult<usize> {
        self.get_user_count().await
    }
    
    async fn get_statistics(&self) -> RepositoryResult<UserStatistics> {
        self.get_statistics().await
    }
}
