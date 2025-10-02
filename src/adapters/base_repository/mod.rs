use async_trait::async_trait;
use sqlx::{PgPool, Postgres, FromRow};

use crate::pkg::{RepositoryError, RepositoryResult};

/// Base repository trait that all repositories can use
/// This provides common CRUD operations
#[async_trait]
pub trait BaseRepository<T, ID> 
where
    T: Send + Sync,
    ID: Send + Sync,
{
    async fn find_by_id(&self, id: ID) -> RepositoryResult<Option<T>>;
    async fn find_all(&self) -> RepositoryResult<Vec<T>>;
    async fn save(&self, entity: T) -> RepositoryResult<T>;
    async fn update(&self, id: ID, entity: T) -> RepositoryResult<T>;
    async fn delete(&self, id: ID) -> RepositoryResult<bool>;
    async fn exists(&self, id: ID) -> RepositoryResult<bool>;
    async fn count(&self) -> RepositoryResult<usize>;
}

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
}

/// In-memory base repository for testing
#[derive(Debug)]
pub struct InMemoryBaseRepository<T, ID>
where
    T: Clone + Send + Sync,
    ID: Clone + Eq + std::hash::Hash + Send + Sync,
{
    storage: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<ID, T>>>,
}

impl<T, ID> InMemoryBaseRepository<T, ID>
where
    T: Clone + Send + Sync,
    ID: Clone + Eq + std::hash::Hash + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            storage: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn insert(&self, id: ID, entity: T) -> RepositoryResult<()> {
        let mut storage = self.storage.write().await;
        if storage.contains_key(&id) {
            return Err(RepositoryError::ValidationError("Entity with this ID already exists".to_string()));
        }
        storage.insert(id, entity);
        Ok(())
    }

    pub async fn get(&self, id: &ID) -> RepositoryResult<Option<T>> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    pub async fn get_all(&self) -> RepositoryResult<Vec<T>> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    pub async fn update_entity(&self, id: ID, entity: T) -> RepositoryResult<T> {
        let mut storage = self.storage.write().await;
        if !storage.contains_key(&id) {
            return Err(RepositoryError::ValidationError("Entity not found".to_string()));
        }
        storage.insert(id, entity.clone());
        Ok(entity)
    }

    pub async fn remove(&self, id: &ID) -> RepositoryResult<bool> {
        let mut storage = self.storage.write().await;
        Ok(storage.remove(id).is_some())
    }

    pub async fn contains(&self, id: &ID) -> RepositoryResult<bool> {
        let storage = self.storage.read().await;
        Ok(storage.contains_key(id))
    }

    pub async fn count_all(&self) -> RepositoryResult<usize> {
        let storage = self.storage.read().await;
        Ok(storage.len())
    }

    pub async fn clear(&self) {
        let mut storage = self.storage.write().await;
        storage.clear();
    }
}

impl<T, ID> Clone for InMemoryBaseRepository<T, ID>
where
    T: Clone + Send + Sync,
    ID: Clone + Eq + std::hash::Hash + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
}
