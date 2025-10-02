use async_trait::async_trait;
use sqlx::{PgPool, Postgres, FromRow};
use pkg::{RepositoryError, RepositoryResult};

/// PostgreSQL base repository implementation
/// Modules can use this for common database operations
#[derive(Debug, Clone)]
pub struct PostgresBaseRepository<T>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Sync + Unpin,
{
    pool: PgPool,
    table_name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PostgresBaseRepository<T>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Sync + Unpin,
{
    pub fn new(pool: PgPool, table_name: impl Into<String>) -> Self {
        Self {
            pool,
            table_name: table_name.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// Execute a query and return a single optional result
    pub async fn query_one<'q, Q>(
        &self,
        query: Q,
    ) -> RepositoryResult<Option<T>>
    where
        Q: sqlx::Execute<'q, Postgres>,
    {
        sqlx::query_as::<_, T>(query.sql())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Execute a query and return all results
    pub async fn query_all<'q, Q>(
        &self,
        query: Q,
    ) -> RepositoryResult<Vec<T>>
    where
        Q: sqlx::Execute<'q, Postgres>,
    {
        sqlx::query_as::<_, T>(query.sql())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Execute a query and return the number of affected rows
    pub async fn execute<'q, Q>(
        &self,
        query: Q,
    ) -> RepositoryResult<u64>
    where
        Q: sqlx::Execute<'q, Postgres>,
    {
        sqlx::query(query.sql())
            .execute(&self.pool)
            .await
            .map(|result| result.rows_affected())
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Execute a raw SQL query and return one result
    pub async fn query_one_raw(&self, sql: &str) -> RepositoryResult<Option<T>> {
        sqlx::query_as::<_, T>(sql)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Execute a raw SQL query and return all results
    pub async fn query_all_raw(&self, sql: &str) -> RepositoryResult<Vec<T>> {
        sqlx::query_as::<_, T>(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Execute a raw SQL command
    pub async fn execute_raw(&self, sql: &str) -> RepositoryResult<u64> {
        sqlx::query(sql)
            .execute(&self.pool)
            .await
            .map(|result| result.rows_affected())
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }
}
