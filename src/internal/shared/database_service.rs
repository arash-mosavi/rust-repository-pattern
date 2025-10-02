use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use crate::pkg::{RepositoryError, RepositoryResult};

/// Database service for managing database connections and transactions
#[derive(Debug, Clone)]
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    /// Create a new database service with custom configuration
    pub async fn new(
        database_url: &str,
        max_connections: u32,
        connect_timeout: Duration,
    ) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(connect_timeout)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Create from environment variables
    pub async fn from_env() -> RepositoryResult<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| RepositoryError::DatabaseError("DATABASE_URL not set".to_string()))?;

        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let connect_timeout_secs = std::env::var("DATABASE_CONNECT_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        Self::new(&database_url, max_connections, Duration::from_secs(connect_timeout_secs))
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }

    /// Check if the database connection is healthy
    pub async fn health_check(&self) -> RepositoryResult<bool> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| true)
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Close the database connection pool
    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// Get pool statistics
    pub fn pool_size(&self) -> u32 {
        self.pool.size()
    }

    /// Get number of idle connections
    pub fn idle_connections(&self) -> usize {
        self.pool.num_idle()
    }
}

/// Transaction helper for executing multiple operations atomically
pub struct Transaction<'a> {
    tx: sqlx::Transaction<'a, sqlx::Postgres>,
}

impl<'a> Transaction<'a> {
    /// Begin a new transaction
    pub async fn begin(pool: &'a PgPool) -> RepositoryResult<Self> {
        let tx = pool
            .begin()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(Self { tx })
    }

    /// Commit the transaction
    pub async fn commit(self) -> RepositoryResult<()> {
        self.tx
            .commit()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> RepositoryResult<()> {
        self.tx
            .rollback()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Get a mutable reference to the transaction
    pub fn as_mut(&mut self) -> &mut sqlx::Transaction<'a, sqlx::Postgres> {
        &mut self.tx
    }
}
