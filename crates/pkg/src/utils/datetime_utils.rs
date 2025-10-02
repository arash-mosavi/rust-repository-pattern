use chrono::{DateTime, Utc, Duration};

/// Get current UTC timestamp
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Add days to a datetime
pub fn add_days(dt: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    dt + Duration::days(days)
}

/// Check if a datetime is in the past
pub fn is_past(dt: DateTime<Utc>) -> bool {
    dt < now()
}

/// Check if a datetime is in the future
pub fn is_future(dt: DateTime<Utc>) -> bool {
    dt > now()
}

/// Format datetime to ISO 8601 string
pub fn to_iso8601(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let current = now();
        assert!(current <= Utc::now());
    }

    #[test]
    fn test_add_days() {
        let dt = now();
        let future = add_days(dt, 1);
        assert!(future > dt);
    }
}
