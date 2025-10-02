// Core configuration module
// Re-export from pkg for internal use
pub use crate::pkg::config::*;

// Module-specific configuration
#[derive(Debug, Clone)]
pub struct ModulesConfig {
    pub enable_users: bool,
    // Add more module flags as needed
}

impl Default for ModulesConfig {
    fn default() -> Self {
        Self {
            enable_users: true,
        }
    }
}
