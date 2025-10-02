use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use crate::internal::modules::users::repositories::UserRepository;
use crate::internal::modules::users::service::UserService;
use super::handler::{
    HttpUserHandler,
    create_user,
    get_all_users,
    get_user,
    update_user,
    delete_user,
    find_by_username,
    filter_by_age_range,
    get_statistics,
    health_check,
};

/// Create HTTP router with all user endpoints
/// Similar to Echo's router setup:
/// ```go
/// e := echo.New()
/// e.POST("/api/users", createUser)
/// e.GET("/api/users", getAllUsers)
/// // ...
/// ```
pub fn create_user_router<R: UserRepository + Send + Sync + 'static>(
    service: Arc<UserService<R>>,
) -> Router {
    let handler = Arc::new(HttpUserHandler::new(service));

    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // User CRUD operations
        .route("/api/users", post(create_user::<R>))
        .route("/api/users", get(get_all_users::<R>))
        .route("/api/users/:id", get(get_user::<R>))
        .route("/api/users/:id", put(update_user::<R>))
        .route("/api/users/:id", delete(delete_user::<R>))
        
        // Search and filter endpoints
        .route("/api/users/search/username", get(find_by_username::<R>))
        .route("/api/users/filter/age", get(filter_by_age_range::<R>))
        
        // Statistics endpoint
        .route("/api/users/statistics", get(get_statistics::<R>))
        
        // Add state (handler instance)
        .with_state(handler)
        
        // Add CORS middleware (allow all origins for development)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        
        // Add tracing/logging middleware
        .layer(TraceLayer::new_for_http())
}
