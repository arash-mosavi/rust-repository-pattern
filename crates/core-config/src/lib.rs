use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;

        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("DATABASE_MAX_CONNECTIONS".to_string()))?;

        Ok(Self {
            database_url,
            max_connections,
        })
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://localhost/repository_pattern".to_string(),
            max_connections: 10,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("SERVER_PORT".to_string()))?;

        Ok(Self { host, port })
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub modules: ModulesConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database: DatabaseConfig::from_env()?,
            server: ServerConfig::from_env()?,
            modules: ModulesConfig::default(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModulesConfig {
    pub users_enabled: bool,
}

impl Default for ModulesConfig {
    fn default() -> Self {
        Self {
            users_enabled: true,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid configuration value for {0}")]
    InvalidValue(String),

    #[error("Configuration error: {0}")]
    Other(String),
}
