pub mod constants;
pub mod domain;
pub mod delivery;
pub mod repositories;
pub mod service;
pub mod types;

// Re-export commonly used types for convenience
pub use domain::*;
pub use delivery::*;
pub use repositories::{UserRepository, InMemoryUserRepository};
pub use service::{UserService, IUserService, UserStatistics};
pub use constants::*;
