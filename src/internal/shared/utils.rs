/// Shared utilities and helpers for validation
pub mod validation {
    use regex::Regex;
    use once_cell::sync::Lazy;

    pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    });

    pub static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[a-zA-Z0-9_]{3,50}$").unwrap()
    });

    /// Validate email format
    pub fn validate_email(email: &str) -> Result<(), String> {
        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if !EMAIL_REGEX.is_match(email) {
            return Err("Invalid email format".to_string());
        }

        Ok(())
    }

    /// Validate username format
    pub fn validate_username(username: &str) -> Result<(), String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        if !USERNAME_REGEX.is_match(username) {
            return Err(
                "Username must be 3-50 characters long and contain only letters, numbers, and underscores"
                    .to_string(),
            );
        }

        Ok(())
    }

    /// Validate string length
    pub fn validate_length(
        value: &str,
        field_name: &str,
        min: usize,
        max: usize,
    ) -> Result<(), String> {
        let len = value.len();
        if len < min || len > max {
            return Err(format!(
                "{} must be between {} and {} characters",
                field_name, min, max
            ));
        }
        Ok(())
    }

    /// Validate non-empty string
    pub fn validate_not_empty(value: &str, field_name: &str) -> Result<(), String> {
        if value.trim().is_empty() {
            return Err(format!("{} cannot be empty", field_name));
        }
        Ok(())
    }

    /// Validate positive number
    pub fn validate_positive(value: i32, field_name: &str) -> Result<(), String> {
        if value <= 0 {
            return Err(format!("{} must be positive", field_name));
        }
        Ok(())
    }

    /// Validate range
    pub fn validate_range(
        value: i32,
        field_name: &str,
        min: i32,
        max: i32,
    ) -> Result<(), String> {
        if value < min || value > max {
            return Err(format!(
                "{} must be between {} and {}",
                field_name, min, max
            ));
        }
        Ok(())
    }
}

/// Shared utilities for string manipulation
pub mod string_utils {
    /// Truncate string to max length
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len])
        }
    }

    /// Sanitize string (remove special characters)
    pub fn sanitize(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '_' || *c == '-')
            .collect()
    }

    /// Convert to slug format
    pub fn to_slug(s: &str) -> String {
        s.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }
}

/// Shared utilities for date/time operations
pub mod datetime_utils {
    use chrono::{DateTime, Utc, Duration};

    /// Get current UTC timestamp
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    /// Check if date is in the past
    pub fn is_past(date: DateTime<Utc>) -> bool {
        date < now()
    }

    /// Check if date is in the future
    pub fn is_future(date: DateTime<Utc>) -> bool {
        date > now()
    }

    /// Add days to a date
    pub fn add_days(date: DateTime<Utc>, days: i64) -> DateTime<Utc> {
        date + Duration::days(days)
    }

    /// Format date for display
    pub fn format_date(date: DateTime<Utc>) -> String {
        date.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

/// Shared utilities for pagination
pub mod pagination_utils {
    /// Calculate total pages
    pub fn calculate_total_pages(total_items: u64, page_size: u32) -> u32 {
        if page_size == 0 {
            return 0;
        }
        ((total_items as f64) / (page_size as f64)).ceil() as u32
    }

    /// Calculate offset from page number
    pub fn calculate_offset(page: u32, page_size: u32) -> u32 {
        if page == 0 {
            return 0;
        }
        (page - 1) * page_size
    }

    /// Validate page number
    pub fn validate_page(page: u32, total_pages: u32) -> Result<(), String> {
        if page == 0 {
            return Err("Page number must be greater than 0".to_string());
        }
        if page > total_pages && total_pages > 0 {
            return Err(format!(
                "Page {} exceeds total pages {}",
                page, total_pages
            ));
        }
        Ok(())
    }
}
