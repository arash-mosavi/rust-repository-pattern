use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// User entity representing a user in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Create a new user with a generated UUID
    pub fn new(username: String, email: String, full_name: String, age: Option<i32>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            full_name,
            age,
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate user data
    pub fn validate(&self) -> Result<(), String> {
        if self.username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if self.email.is_empty() || !self.email.contains('@') {
            return Err("Invalid email address".to_string());
        }
        if self.full_name.is_empty() {
            return Err("Full name cannot be empty".to_string());
        }
        if let Some(age) = self.age {
            if age > 150 {
                return Err("Invalid age".to_string());
            }
        }
        Ok(())
    }
}
