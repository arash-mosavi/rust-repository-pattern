/// Users module database migrations using code-first migration system
/// 
/// This file contains structured migrations for the users module.
/// Each migration tracks its version, has a name, and includes SQL.

use core_db::Migration;

/// Migration 1: Create the users table with indexes and triggers
const MIGRATION_CREATE_USERS_TABLE: &str = r#"
-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    full_name VARCHAR(255) NOT NULL,
    age INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_age ON users(age);

-- Create a trigger to auto-update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
"#;

/// Example: Migration 2 (future)
/// Uncomment and modify when you need to add new features
/*
const MIGRATION_ADD_USER_ROLES: &str = r#"
-- Add role and is_active columns to users
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS role VARCHAR(50) DEFAULT 'user',
ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT true;

CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_is_active ON users(is_active);
"#;
*/

/// All migrations for the users module
/// These will be executed in order by version number
pub const MIGRATIONS: &[Migration] = &[
    Migration::new(
        "users",                         // module name
        1,                               // version
        "create_users_table",            // migration name
        MIGRATION_CREATE_USERS_TABLE,    // SQL to execute
    ),
    // Add future migrations here:
    // Migration::new("users", 2, "add_user_roles", MIGRATION_ADD_USER_ROLES),
    // Migration::new("users", 3, "add_email_verification", MIGRATION_ADD_EMAIL_VERIFICATION),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations_array_not_empty() {
        assert!(!MIGRATIONS.is_empty());
        assert_eq!(MIGRATIONS.len(), 1); // Update this as you add more migrations
    }

    #[test]
    fn test_migration_metadata() {
        let migration = &MIGRATIONS[0];
        assert_eq!(migration.module, "users");
        assert_eq!(migration.version, 1);
        assert_eq!(migration.name, "create_users_table");
        assert!(!migration.sql.is_empty());
    }

    #[test]
    fn test_migrations_are_valid_sql() {
        for migration in MIGRATIONS {
            assert!(!migration.sql.is_empty());
            assert!(migration.sql.contains("CREATE TABLE"));
        }
    }

    #[test]
    fn test_migrations_have_unique_versions() {
        let mut versions = std::collections::HashSet::new();
        for migration in MIGRATIONS {
            assert!(
                versions.insert(migration.version),
                "Duplicate version found: {}",
                migration.version
            );
        }
    }
}
