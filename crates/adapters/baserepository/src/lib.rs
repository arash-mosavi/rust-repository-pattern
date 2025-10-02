use async_trait::async_trait;
use pkg::{RepositoryError, RepositoryResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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

#[derive(Debug)]
pub struct InMemoryBaseRepository<T, ID>
where
    T: Clone + Send + Sync,
    ID: Clone + Eq + std::hash::Hash + Send + Sync,
{
    storage: Arc<RwLock<HashMap<ID, T>>>,
}

impl<T, ID> InMemoryBaseRepository<T, ID>
where
    T: Clone + Send + Sync,
    ID: Clone + Eq + std::hash::Hash + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, id: ID, entity: T) -> RepositoryResult<()> {
        let mut storage = self.storage.write().await;
        if storage.contains_key(&id) {
            return Err(RepositoryError::AlreadyExists(
                pkg::EntityId::nil(), // Placeholder, will be fixed with proper ID
            ));
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
            return Err(RepositoryError::NotFound(pkg::EntityId::nil()));
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

impl<T, ID> Default for InMemoryBaseRepository<T, ID>
where
    T: Clone + Send + Sync,
    ID: Clone + Eq + std::hash::Hash + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}
