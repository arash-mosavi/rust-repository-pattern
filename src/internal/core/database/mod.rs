// Database factory for creating database connections
use sqlx::PgPool;
use crate::pkg::{DatabaseConfig, RepositoryResult, RepositoryError};

pub struct DatabaseFactory;

impl DatabaseFactory {
    /// Create a PostgreSQL connection pool from environment variables
    pub async fn create_postgres_pool_from_env() -> RepositoryResult<PgPool> {
        let config = DatabaseConfig::from_env()
            .map_err(|e| RepositoryError::DatabaseError(e))?;
        
        Self::create_postgres_pool(&config).await
    }

    /// Create a PostgreSQL connection pool from configuration
    pub async fn create_postgres_pool(config: &DatabaseConfig) -> RepositoryResult<PgPool> {
        config.create_pool()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Run migrations on the database
    pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(pool).await
    }
}
