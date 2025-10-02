use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;

        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| "Invalid DATABASE_MAX_CONNECTIONS value".to_string())?;

        Ok(Self {
            database_url,
            max_connections,
        })
    }

    /// Create a database connection pool
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&self.database_url)
            .await
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://localhost/repository_pattern".to_string(),
            max_connections: 10,
        }
    }
}
