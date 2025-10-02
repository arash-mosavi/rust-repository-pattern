use async_trait::async_trait;
use pkg::RepositoryResult;

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn begin(&mut self) -> RepositoryResult<()>;

    async fn commit(&mut self) -> RepositoryResult<()>;

    async fn rollback(&mut self) -> RepositoryResult<()>;
}

#[async_trait]
pub trait DatabaseService: Send + Sync {
    async fn health_check(&self) -> RepositoryResult<bool>;

    fn connection_info(&self) -> String;
}
