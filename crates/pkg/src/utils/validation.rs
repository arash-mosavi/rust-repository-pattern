use regex::Regex;
use once_cell::sync::Lazy;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

/// Validate email format
pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

/// Validate username (alphanumeric and underscores, 3-20 chars)
pub fn is_valid_username(username: &str) -> bool {
    let len = username.len();
    len >= 3 && len <= 20 && username.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Validate password strength (at least 8 chars, with uppercase, lowercase, and digit)
pub fn is_strong_password(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    has_uppercase && has_lowercase && has_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@example.co.uk"));
        assert!(!is_valid_email("invalid.email"));
        assert!(!is_valid_email("@example.com"));
    }

    #[test]
    fn test_username_validation() {
        assert!(is_valid_username("user123"));
        assert!(is_valid_username("test_user"));
        assert!(!is_valid_username("ab"));
        assert!(!is_valid_username("user@name"));
    }

    #[test]
    fn test_password_strength() {
        assert!(is_strong_password("Password123"));
        assert!(!is_strong_password("password"));
        assert!(!is_strong_password("PASSWORD"));
        assert!(!is_strong_password("Pass12"));
    }
}
