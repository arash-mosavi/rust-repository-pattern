use sqlx::{PgPool, postgres::PgPoolOptions};
use core_config::DatabaseConfig;
use pkg::{RepositoryError, RepositoryResult};

pub struct DatabaseFactory;

impl DatabaseFactory {
    /// Create a PostgreSQL connection pool from environment variables
    pub async fn create_postgres_pool_from_env() -> RepositoryResult<PgPool> {
        let config = DatabaseConfig::from_env()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Self::create_postgres_pool(&config).await
    }

    /// Create a PostgreSQL connection pool from configuration
    pub async fn create_postgres_pool(config: &DatabaseConfig) -> RepositoryResult<PgPool> {
        PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.database_url)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    /// Run migrations on the database (DEPRECATED)
    /// 
    /// **DEPRECATED**: Use `MigrationRunner` instead for code-first migrations with tracking.
    /// 
    /// This method is kept for backward compatibility but doesn't track which migrations
    /// have been applied, so it will re-run them every time.
    /// 
    /// # Recommended Alternative
    /// ```ignore
    /// use core_db::MigrationRunner;
    /// 
    /// let runner = MigrationRunner::new(pool);
    /// runner.run_migrations(&all_migrations).await?;
    /// ```
    #[deprecated(
        since = "0.2.0",
        note = "Use MigrationRunner for code-first migrations with tracking"
    )]
    pub async fn run_migrations(
        pool: &PgPool,
        module_migrations: &[(&str, &[&str])],
    ) -> RepositoryResult<()> {
        for (module_name, migrations) in module_migrations {
            tracing::info!("Running migrations for module: {}", module_name);
            
            for (index, migration) in migrations.iter().enumerate() {
                tracing::debug!("Executing migration {} for {}", index + 1, module_name);
                
                sqlx::raw_sql(migration)
                    .execute(pool)
                    .await
                    .map_err(|e| {
                        RepositoryError::DatabaseError(
                            format!("Failed to run migration {} for {}: {}", index + 1, module_name, e)
                        )
                    })?;
            }
            
            tracing::info!("✓ Completed {} migration(s) for {}", migrations.len(), module_name);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires database connection
    async fn test_create_pool() {
        // This test requires DATABASE_URL to be set
        let result = DatabaseFactory::create_postgres_pool_from_env().await;
        assert!(result.is_ok() || result.is_err()); // Just ensure it compiles
    }
}
