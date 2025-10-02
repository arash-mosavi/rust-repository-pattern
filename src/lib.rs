// Module declarations for library following Go-style structure
pub mod pkg;        // Public reusable libraries
pub mod adapters;   // Infrastructure adapters
pub mod internal;   // Application-specific code

// Re-export commonly used types from pkg
pub use pkg::{RepositoryError, RepositoryResult, DatabaseConfig};

// Re-export adapters
pub use adapters::{BaseRepository, InMemoryBaseRepository, PostgresBaseRepository};

// Re-export internal modules for library use
pub use internal::modules;
pub use internal::composition;

// Legacy compatibility - keep old structure accessible
pub mod users {
    pub use crate::internal::modules::users::*;
}
