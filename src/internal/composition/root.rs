// Composition root - Application setup and dependency wiring
use std::sync::Arc;
use super::ModuleRegistry;
use crate::internal::core::DatabaseFactory;
use crate::internal::modules::users::{
    repositories::InMemoryUserRepository,
    service::UserService,
};
use crate::pkg::RepositoryResult;

/// Application composition root
pub struct CompositionRoot {
    pub module_registry: ModuleRegistry,
    // Add more dependencies as needed
}

impl CompositionRoot {
    /// Create a new composition root with in-memory repositories
    pub fn new_with_in_memory() -> Self {
        let registry = ModuleRegistry::new();
        
        // Register modules here
        // Example: registry.register(Arc::new(UserModule::new()));
        
        Self {
            module_registry: registry,
        }
    }

    /// Create a new composition root with PostgreSQL
    pub async fn new_with_postgres() -> RepositoryResult<Self> {
        let pool = DatabaseFactory::create_postgres_pool_from_env().await?;
        DatabaseFactory::run_migrations(&pool).await
            .map_err(|e| crate::pkg::RepositoryError::DatabaseError(e.to_string()))?;

        let registry = ModuleRegistry::new();
        
        // Register PostgreSQL-backed modules here
        
        Self::with_registry(registry)
    }

    /// Create composition root with custom registry
    pub fn with_registry(registry: ModuleRegistry) -> RepositoryResult<Self> {
        Ok(Self {
            module_registry: registry,
        })
    }

    /// Get the module registry
    pub fn registry(&self) -> &ModuleRegistry {
        &self.module_registry
    }
}

// Helper functions for creating services
impl CompositionRoot {
    /// Create a user service with in-memory repository
    pub fn create_user_service_in_memory() -> Arc<UserService<InMemoryUserRepository>> {
        let repository = Arc::new(InMemoryUserRepository::new());
        Arc::new(UserService::new(repository))
    }
}
