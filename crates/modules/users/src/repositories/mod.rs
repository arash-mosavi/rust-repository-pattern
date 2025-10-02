pub mod interface;
pub mod migration;
pub mod repository;

pub use interface::UserRepository;
pub use migration::MIGRATIONS as USER_MIGRATIONS;
pub use repository::InMemoryUserRepository;
