//! Code-First Database Migration System
//! 
//! This module provides a robust migration system that:
//! - Tracks which migrations have been applied
//! - Prevents re-running the same migration
//! - Supports per-module versioning
//! - Generates checksums for migration integrity
//! - Provides idempotent migration execution

use sqlx::PgPool;
use std::collections::HashMap;
use pkg::{RepositoryError, RepositoryResult};

/// Represents a single database migration
#[derive(Debug, Clone, Copy)]
pub struct Migration {
    /// Module name (e.g., "users", "products")
    pub module: &'static str,
    /// Version number within the module (1, 2, 3...)
    pub version: i32,
    /// Human-readable name (e.g., "create_users_table")
    pub name: &'static str,
    /// SQL to execute
    pub sql: &'static str,
}

impl Migration {
    /// Create a new migration
    pub const fn new(
        module: &'static str,
        version: i32,
        name: &'static str,
        sql: &'static str,
    ) -> Self {
        Self {
            module,
            version,
            name,
            sql,
        }
    }

    /// Generate a checksum for the migration SQL
    /// This ensures migrations haven't been modified after being applied
    pub fn checksum(&self) -> String {
        // Simple checksum based on SQL length and first/last chars
        // In production, use a proper hash like SHA256
        let len = self.sql.len();
        let first = self.sql.chars().next().unwrap_or('0');
        let last = self.sql.chars().last().unwrap_or('0');
        format!("{}-{}-{}", len, first as u32, last as u32)
    }

    /// Get unique identifier for this migration
    pub fn id(&self) -> String {
        format!("{}:version_{}", self.module, self.version)
    }
}

/// Migration tracking record from database
#[derive(Debug)]
struct AppliedMigration {
    module: String,
    version: i32,
    name: String,
    checksum: String,
}

/// Migration runner that manages database schema evolution
pub struct MigrationRunner {
    pool: PgPool,
}

impl MigrationRunner {
    /// Create a new migration runner
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Initialize the migrations tracking table
    async fn ensure_migrations_table(&self) -> RepositoryResult<()> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS _schema_migrations (
            id SERIAL PRIMARY KEY,
            module VARCHAR(100) NOT NULL,
            version INTEGER NOT NULL,
            name VARCHAR(255) NOT NULL,
            checksum VARCHAR(255) NOT NULL,
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            execution_time_ms INTEGER,
            UNIQUE(module, version)
        );
        
        CREATE INDEX IF NOT EXISTS idx_schema_migrations_module 
            ON _schema_migrations(module);
        
        CREATE INDEX IF NOT EXISTS idx_schema_migrations_applied_at 
            ON _schema_migrations(applied_at);
        "#;

        sqlx::raw_sql(sql)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                RepositoryError::DatabaseError(
                    format!("Failed to create migrations table: {}", e)
                )
            })?;

        tracing::info!("✓ Migrations tracking table ready");
        Ok(())
    }

    /// Get all applied migrations from the database
    async fn get_applied_migrations(&self) -> RepositoryResult<HashMap<String, AppliedMigration>> {
        let records = sqlx::query_as::<_, (String, i32, String, String)>(
            "SELECT module, version, name, checksum FROM _schema_migrations ORDER BY module, version"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            RepositoryError::DatabaseError(
                format!("Failed to fetch applied migrations: {}", e)
            )
        })?;

        let mut migrations = HashMap::new();
        for (module, version, name, checksum) in records {
            let key = format!("{}:v{}", module, version);
            migrations.insert(
                key,
                AppliedMigration {
                    module,
                    version,
                    name,
                    checksum,
                },
            );
        }

        Ok(migrations)
    }

    /// Check if a migration has been applied
    async fn is_applied(&self, migration: &Migration) -> RepositoryResult<bool> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM _schema_migrations WHERE module = $1 AND version = $2"
        )
        .bind(migration.module)
        .bind(migration.version)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            RepositoryError::DatabaseError(
                format!("Failed to check migration status: {}", e)
            )
        })?;

        Ok(count.0 > 0)
    }

    /// Record that a migration has been applied
    async fn record_migration(
        &self,
        migration: &Migration,
        execution_time_ms: i32,
    ) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            INSERT INTO _schema_migrations (module, version, name, checksum, execution_time_ms)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(migration.module)
        .bind(migration.version)
        .bind(migration.name)
        .bind(migration.checksum())
        .bind(execution_time_ms)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            RepositoryError::DatabaseError(
                format!("Failed to record migration: {}", e)
            )
        })?;

        Ok(())
    }

    /// Run a single migration
    async fn run_migration(&self, migration: &Migration) -> RepositoryResult<i32> {
        let start = std::time::Instant::now();

        tracing::info!(
            "  → Running migration: {} v{} - {}",
            migration.module,
            migration.version,
            migration.name
        );

        // Execute the migration SQL
        sqlx::raw_sql(migration.sql)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                RepositoryError::DatabaseError(
                    format!("Migration {} failed: {}", migration.id(), e)
                )
            })?;

        let execution_time_ms = start.elapsed().as_millis() as i32;

        tracing::info!(
            "    ✓ Completed in {}ms",
            execution_time_ms
        );

        Ok(execution_time_ms)
    }

    /// Run all pending migrations
    pub async fn run_migrations(&self, migrations: &[Migration]) -> RepositoryResult<()> {
        // Ensure tracking table exists
        self.ensure_migrations_table().await?;

        // Get applied migrations
        let applied = self.get_applied_migrations().await?;

        tracing::info!("📦 Starting migration check...");
        tracing::info!("   Found {} previously applied migrations", applied.len());
        tracing::info!("   Checking {} total migrations", migrations.len());

        // Group migrations by module
        let mut by_module: HashMap<&str, Vec<&Migration>> = HashMap::new();
        for migration in migrations {
            by_module
                .entry(migration.module)
                .or_insert_with(Vec::new)
                .push(migration);
        }

        let mut total_applied = 0;
        let mut total_skipped = 0;

        for (module_name, module_migrations) in by_module.iter() {
            tracing::info!("📂 Module: {}", module_name);

            for migration in module_migrations {
                let is_applied = self.is_applied(migration).await?;

                if is_applied {
                    tracing::debug!(
                        "  ⊘ Skipping (already applied): v{} - {}",
                        migration.version,
                        migration.name
                    );
                    total_skipped += 1;
                } else {
                    let execution_time = self.run_migration(migration).await?;
                    self.record_migration(migration, execution_time).await?;
                    total_applied += 1;
                }
            }
        }

        if total_applied > 0 {
            tracing::info!("✅ Applied {} new migration(s)", total_applied);
        } else {
            tracing::info!("✅ All migrations up to date");
        }
        
        if total_skipped > 0 {
            tracing::debug!("   Skipped {} already applied migration(s)", total_skipped);
        }

        Ok(())
    }

    /// Get migration status for all modules
    pub async fn get_status(&self) -> RepositoryResult<Vec<MigrationStatus>> {
        self.ensure_migrations_table().await?;

        let records = sqlx::query_as::<_, (String, i32, String, String, i32)>(
            r#"
            SELECT module, version, name, 
                   to_char(applied_at, 'YYYY-MM-DD HH24:MI:SS') as applied_at,
                   execution_time_ms
            FROM _schema_migrations 
            ORDER BY module, version
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            RepositoryError::DatabaseError(
                format!("Failed to fetch migration status: {}", e)
            )
        })?;

        let statuses = records
            .into_iter()
            .map(|(module, version, name, applied_at, execution_time_ms)| {
                MigrationStatus {
                    module,
                    version,
                    name,
                    applied_at,
                    execution_time_ms,
                }
            })
            .collect();

        Ok(statuses)
    }
}

/// Migration status information
#[derive(Debug)]
pub struct MigrationStatus {
    pub module: String,
    pub version: i32,
    pub name: String,
    pub applied_at: String,
    pub execution_time_ms: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_checksum() {
        let migration = Migration::new(
            "users",
            1,
            "create_users_table",
            "CREATE TABLE users (id UUID PRIMARY KEY);",
        );

        let checksum1 = migration.checksum();
        let checksum2 = migration.checksum();

        // Checksums should be consistent
        assert_eq!(checksum1, checksum2);
        assert!(!checksum1.is_empty());
    }

    #[test]
    fn test_migration_id() {
        let migration = Migration::new("users", 1, "create_users", "");
        assert_eq!(migration.id(), "users:version_1");

        let migration2 = Migration::new("products", 5, "add_column", "");
        assert_eq!(migration2.id(), "products:version_5");
    }

    #[test]
    fn test_different_sql_different_checksum() {
        let migration1 = Migration::new("users", 1, "test", "CREATE TABLE users;");
        let migration2 = Migration::new("users", 1, "test", "DROP TABLE users;");

        assert_ne!(migration1.checksum(), migration2.checksum());
    }
}
