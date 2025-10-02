use uuid::Uuid;

use pkg::RepositoryResult;
use crate::domain::User;
use crate::delivery::http::dto::{CreateUserDto, UpdateUserDto};

#[async_trait::async_trait]
pub trait IUserService {
    async fn create_user(&self, dto: CreateUserDto) -> RepositoryResult<User>;
    
    async fn get_user(&self, id: Uuid) -> RepositoryResult<User>;
    
    async fn get_all_users(&self) -> RepositoryResult<Vec<User>>;
    
    async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> RepositoryResult<User>;
    
    async fn delete_user(&self, id: Uuid) -> RepositoryResult<bool>;
    
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<User>>;
    
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;
    
    async fn get_users_by_age_range(&self, min_age: i32, max_age: i32) -> RepositoryResult<Vec<User>>;
    
    async fn get_user_count(&self) -> RepositoryResult<usize>;
    
    async fn get_statistics(&self) -> RepositoryResult<UserStatistics>;
}

#[derive(Debug, Clone)]
pub struct UserStatistics {
    pub total_users: usize,
    pub users_with_age: usize,
    pub average_age: Option<f64>,
}
