use async_trait::async_trait;
use uuid::Uuid;

use pkg::RepositoryResult;
use baserepository::BaseRepository;
use crate::domain::User;
use crate::delivery::http::dto::{CreateUserDto, UpdateUserDto};

#[async_trait]
pub trait UserRepository: BaseRepository<User, Uuid> {
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>>;

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;

    async fn find_by_age_range(&self, min_age: i32, max_age: i32) -> RepositoryResult<Vec<User>>;

    async fn create_user(&self, dto: CreateUserDto) -> RepositoryResult<User>;

    async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> RepositoryResult<User>;
}
