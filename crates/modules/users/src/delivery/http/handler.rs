use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use pkg::RepositoryError;
use crate::domain::User;
use crate::delivery::http::dto::{CreateUserDto, UpdateUserDto, UserResponse, ApiResponse, UserListResponse};
use crate::repositories::UserRepository;
use crate::service::UserService;

pub struct HttpUserHandler<R: UserRepository> {
    service: Arc<UserService<R>>,
}

impl<R: UserRepository> HttpUserHandler<R> {
    pub fn new(service: Arc<UserService<R>>) -> Self {
        Self { service }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AgeRangeQuery {
    #[validate(range(min = 1, max = 150))]
    pub min_age: i32,
    #[validate(range(min = 1, max = 150))]
    pub max_age: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UsernameQuery {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            age: user.age,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<Vec<String>>,
}

pub struct AppError(pub RepositoryError);

impl From<RepositoryError> for AppError {
    fn from(err: RepositoryError) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self.0 {
            RepositoryError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Resource not found: {}", id),
                None,
            ),
            RepositoryError::AlreadyExists(id) => (
                StatusCode::CONFLICT,
                format!("Resource already exists with id: {}", id),
                None,
            ),
            RepositoryError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "Validation failed".to_string(),
                Some(vec![msg]),
            ),
            RepositoryError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
                Some(vec![msg]),
            ),
            RepositoryError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
                Some(vec![msg]),
            ),
            RepositoryError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
                Some(vec![msg]),
            ),
            RepositoryError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                "Forbidden".to_string(),
                Some(vec![msg]),
            ),
            RepositoryError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "Bad request".to_string(),
                Some(vec![msg]),
            ),
        };

        let body = Json(ErrorResponse {
            error: message,
            details,
        });

        (status, body).into_response()
    }
}

pub async fn create_user<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    dto.validate()
        .map_err(|e| AppError(RepositoryError::ValidationError(format!("{}", e))))?;

    let user = handler.service.create_user(dto).await?;
    let response = ApiResponse::success(UserResponse::from(user));
    
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_all_users<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
) -> Result<impl IntoResponse, AppError> {
    let users = handler.service.get_all_users().await?;
    let total = users.len();
    
    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(UserResponse::from)
        .collect();
    
    let response = ApiResponse::success(UserListResponse {
        users: user_responses,
        total,
    });
    
    Ok(Json(response))
}

pub async fn get_user<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = handler.service.get_user(id).await?;
    let response = ApiResponse::success(UserResponse::from(user));
    
    Ok(Json(response))
}

pub async fn update_user<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    dto.validate()
        .map_err(|e| AppError(RepositoryError::ValidationError(format!("{}", e))))?;

    let user = handler.service.update_user(id, dto).await?;
    let response = ApiResponse::success(UserResponse::from(user));
    
    Ok(Json(response))
}

pub async fn delete_user<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let deleted = handler.service.delete_user(id).await?;
    
    let response = ApiResponse::success(serde_json::json!({
        "deleted": deleted,
        "id": id.to_string(),
    }));
    
    Ok(Json(response))
}

pub async fn find_by_username<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
    Query(query): Query<UsernameQuery>,
) -> Result<impl IntoResponse, AppError> {
    query.validate()
        .map_err(|e| AppError(RepositoryError::ValidationError(format!("{}", e))))?;

    let user = handler.service.find_by_username(&query.username).await?;
    
    match user {
        Some(u) => {
            let response = ApiResponse::success(UserResponse::from(u));
            Ok(Json(response))
        }
        None => Err(AppError(RepositoryError::ValidationError(
            format!("User not found with username: {}", query.username)
        ))),
    }
}

pub async fn filter_by_age_range<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
    Query(query): Query<AgeRangeQuery>,
) -> Result<impl IntoResponse, AppError> {
    query.validate()
        .map_err(|e| AppError(RepositoryError::ValidationError(format!("{}", e))))?;

    if query.min_age > query.max_age {
        return Err(AppError(RepositoryError::ValidationError(
            "min_age must be less than or equal to max_age".to_string()
        )));
    }

    let users = handler.service.get_users_by_age_range(query.min_age, query.max_age).await?;
    let total = users.len();
    
    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(UserResponse::from)
        .collect();
    
    let response = ApiResponse::success(UserListResponse {
        users: user_responses,
        total,
    });
    
    Ok(Json(response))
}

pub async fn get_statistics<R: UserRepository>(
    State(handler): State<Arc<HttpUserHandler<R>>>,
) -> Result<impl IntoResponse, AppError> {
    let stats = handler.service.get_statistics().await?;
    
    let response = ApiResponse::success(serde_json::json!({
        "total_users": stats.total_users,
        "users_with_age": stats.users_with_age,
        "average_age": stats.average_age,
    }));
    
    Ok(Json(response))
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}
