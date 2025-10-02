use async_trait::async_trait;
use uuid::Uuid;

use crate::pkg::{RepositoryError, RepositoryResult};
use crate::adapters::{BaseRepository, InMemoryBaseRepository};
use crate::internal::modules::users::domain::User;
use crate::internal::modules::users::delivery::http::dto::{CreateUserDto, UpdateUserDto};
use super::interface::UserRepository;

/// In-memory implementation using BaseRepository (composition, not inheritance)
#[derive(Debug, Clone)]
pub struct InMemoryUserRepository {
    base: InMemoryBaseRepository<User, Uuid>,
}

impl InMemoryUserRepository {
    /// Create a new in-memory user repository
    pub fn new() -> Self {
        Self {
            base: InMemoryBaseRepository::new(),
        }
    }

    /// Create with initial data
    pub fn with_data(users: Vec<User>) -> Self {
        let repo = Self::new();
        Self {
            base: InMemoryBaseRepository::new(),
        }
    }

    /// Helper: Check for duplicate username
    async fn check_duplicate_username(&self, username: &str, exclude_id: Option<Uuid>) -> RepositoryResult<()> {
        let users = self.base.get_all().await?;
        for user in users {
            if user.username == username && exclude_id.map_or(true, |id| user.id != id) {
                return Err(RepositoryError::ValidationError(
                    format!("Username '{}' is already taken", username)
                ));
            }
        }
        Ok(())
    }

    /// Helper: Check for duplicate email
    async fn check_duplicate_email(&self, email: &str, exclude_id: Option<Uuid>) -> RepositoryResult<()> {
        let users = self.base.get_all().await?;
        for user in users {
            if user.email == email && exclude_id.map_or(true, |id| user.id != id) {
                return Err(RepositoryError::ValidationError(
                    format!("Email '{}' is already taken", email)
                ));
            }
        }
        Ok(())
    }
}

// Implement BaseRepository trait (common CRUD operations)
#[async_trait]
impl BaseRepository<User, Uuid> for InMemoryUserRepository {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<User>> {
        self.base.get(&id).await
    }

    async fn find_all(&self) -> RepositoryResult<Vec<User>> {
        self.base.get_all().await
    }

    async fn save(&self, entity: User) -> RepositoryResult<User> {
        // Validate before saving
        entity.validate()?;
        
        // Check for duplicates
        self.check_duplicate_username(&entity.username, None).await?;
        self.check_duplicate_email(&entity.email, None).await?;
        
        // Use base repository to save
        self.base.insert(entity.id, entity.clone()).await?;
        Ok(entity)
    }

    async fn update(&self, id: Uuid, entity: User) -> RepositoryResult<User> {
        // Validate before updating
        entity.validate()?;
        
        // Check for duplicates (excluding current user)
        self.check_duplicate_username(&entity.username, Some(id)).await?;
        self.check_duplicate_email(&entity.email, Some(id)).await?;
        
        // Use base repository to update
        self.base.update_entity(id, entity).await
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<bool> {
        self.base.remove(&id).await
    }

    async fn exists(&self, id: Uuid) -> RepositoryResult<bool> {
        self.base.contains(&id).await
    }

    async fn count(&self) -> RepositoryResult<usize> {
        self.base.count_all().await
    }
}

// Implement UserRepository trait (user-specific operations)
#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>> {
        let users = self.base.get_all().await?;
        Ok(users.into_iter().find(|u| u.username == username))
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let users = self.base.get_all().await?;
        Ok(users.into_iter().find(|u| u.email == email))
    }

    async fn find_by_age_range(&self, min_age: i32, max_age: i32) -> RepositoryResult<Vec<User>> {
        let users = self.base.get_all().await?;
        Ok(users
            .into_iter()
            .filter(|u| {
                u.age
                    .map(|age| age >= min_age && age <= max_age)
                    .unwrap_or(false)
            })
            .collect())
    }

    async fn create_user(&self, dto: CreateUserDto) -> RepositoryResult<User> {
        let user = User::new(dto.username, dto.email, dto.full_name, dto.age);
        self.save(user).await
    }

    async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> RepositoryResult<User> {
        let mut user = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(id))?;

        if let Some(username) = dto.username {
            user.username = username;
        }
        if let Some(email) = dto.email {
            user.email = email;
        }
        if let Some(full_name) = dto.full_name {
            user.full_name = full_name;
        }
        if let Some(age) = dto.age {
            user.age = Some(age);
        }

        self.update(id, user).await
    }
}
