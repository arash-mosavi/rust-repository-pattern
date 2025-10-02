// Adapters layer - Infrastructure implementations
pub mod base_repository;

// Re-export commonly used items
pub use base_repository::{BaseRepository, InMemoryBaseRepository, PostgresBaseRepository};
