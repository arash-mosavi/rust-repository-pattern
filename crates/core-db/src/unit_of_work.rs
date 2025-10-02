use async_trait::async_trait;
use pkg::RepositoryResult;

/// Unit of Work pattern for managing transactions across multiple repositories
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    /// Begin a new transaction
    async fn begin(&mut self) -> RepositoryResult<()>;

    /// Commit the current transaction
    async fn commit(&mut self) -> RepositoryResult<()>;

    /// Rollback the current transaction
    async fn rollback(&mut self) -> RepositoryResult<()>;
}

/// Database service trait for common database operations
#[async_trait]
pub trait DatabaseService: Send + Sync {
    /// Check if the database connection is healthy
    async fn health_check(&self) -> RepositoryResult<bool>;

    /// Get the database connection info (for debugging)
    fn connection_info(&self) -> String;
}
