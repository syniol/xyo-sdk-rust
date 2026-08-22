use serde::{Deserialize, Serialize};

/// Detailed rate limit information extracted from HTTP 429 response headers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RateLimitError {
    /// Recommended retry wait duration in seconds (from `Retry-After`).
    pub retry_after: Option<u64>,
    /// Request limit quota per window (from `RateLimit-Limit`).
    pub rate_limit: Option<u64>,
    /// Remaining request quota in current window (from `RateLimit-Remaining`).
    pub rate_remaining: Option<u64>,
    /// Window reset time or duration in seconds (from `RateLimit-Reset`).
    pub rate_reset: Option<u64>,
}

/// Error type returned by the XYO SDK client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientError {
    pub message: String,
    pub code: u16,
    pub rate_limit: Option<RateLimitError>,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientError (code {}): {}", self.code, self.message)
    }
}

impl std::error::Error for ClientError {}

impl ClientError {
    /// Construct a new `ClientError` without rate limit details.
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            rate_limit: None,
        }
    }

    /// Construct a new `ClientError` with rate limit details.
    pub fn with_rate_limit(
        code: u16,
        message: impl Into<String>,
        rate_limit: RateLimitError,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            rate_limit: Some(rate_limit),
        }
    }

    /// Returns true if this error represents an authentication or authorization failure (HTTP 401 or 403).
    pub fn is_auth(&self) -> bool {
        self.code == 401 || self.code == 403
    }

    /// Returns true if this error represents a rate limit or throttle (HTTP 429).
    pub fn is_rate_limited(&self) -> bool {
        self.code == 429
    }

    /// Returns true if this error represents a resource not found (HTTP 404).
    pub fn is_not_found(&self) -> bool {
        self.code == 404
    }

    /// Returns true if this error represents an internal server error (HTTP 5xx).
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.code)
    }

    /// Returns true if the operation is transient and safe to retry.
    pub fn is_retryable(&self) -> bool {
        self.is_rate_limited()
            || self.is_server_error()
            || (self.code == 0
                && (self.message.to_ascii_lowercase().contains("timed out")
                    || self.message.to_ascii_lowercase().contains("timeout")
                    || self.message.to_ascii_lowercase().contains("connection reset")
                    || self.message.to_ascii_lowercase().contains("network stream error")))
    }
}

/// Helper to extract RateLimit header values from an HTTP response HeaderMap into `RateLimitError`.
pub fn extract_rate_limit_headers(headers: &reqwest::header::HeaderMap) -> Option<RateLimitError> {
    let get_header_str = |keys: &[&str]| -> Option<String> {
        for &k in keys {
            if let Some(val) = headers.get(k) {
                if let Ok(s) = val.to_str() {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }
        }
        None
    };

    let parse_u64 = |keys: &[&str]| -> Option<u64> {
        get_header_str(keys).and_then(|s| s.parse::<u64>().ok())
    };

    let retry_after = parse_u64(&["retry-after", "x-retry-after"]);
    let rate_limit = parse_u64(&["ratelimit-limit", "x-ratelimit-limit", "x-rate-limit-limit"]);
    let rate_remaining = parse_u64(&["ratelimit-remaining", "x-ratelimit-remaining", "x-rate-limit-remaining"]);
    let rate_reset = parse_u64(&["ratelimit-reset", "x-ratelimit-reset", "x-rate-limit-reset"]);

    if retry_after.is_some() || rate_limit.is_some() || rate_remaining.is_some() || rate_reset.is_some() {
        Some(RateLimitError {
            retry_after,
            rate_limit,
            rate_remaining,
            rate_reset,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_error_display() {
        let err = ClientError::new(404, "Not Found");
        assert_eq!(format!("{}", err), "ClientError (code 404): Not Found");
    }

    #[test]
    fn test_client_error_debug() {
        let err = ClientError::new(500, "Internal Error");
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("500"));
        assert!(debug_str.contains("Internal Error"));
    }

    #[test]
    fn test_client_error_classification_methods() {
        let auth_err = ClientError::new(401, "Unauthorized");
        assert!(auth_err.is_auth());
        assert!(!auth_err.is_server_error());
        assert!(!auth_err.is_retryable());

        let forbidden_err = ClientError::new(403, "Forbidden");
        assert!(forbidden_err.is_auth());

        let rate_err = ClientError::new(429, "Too Many Requests");
        assert!(rate_err.is_rate_limited());
        assert!(rate_err.is_retryable());

        let server_err = ClientError::new(503, "Service Unavailable");
        assert!(server_err.is_server_error());
        assert!(server_err.is_retryable());

        let timeout_err = ClientError::new(0, "operation timed out after 30s");
        assert!(timeout_err.is_retryable());
    }

    #[test]
    fn test_client_error_clone_and_eq() {
        let err1 = ClientError::new(400, "Bad Request");
        let err2 = err1.clone();
        assert_eq!(err1, err2);
        assert_eq!(err1.code, 400);
        assert_eq!(err1.message, "Bad Request");

        let err3 = ClientError::new(401, "Unauthorized");
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_client_error_implements_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(ClientError::new(403, "Forbidden"));
        assert_eq!(format!("{}", err), "ClientError (code 403): Forbidden");
        assert!(err.source().is_none());
    }

    #[test]
    fn test_extract_rate_limit_headers() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Retry-After", "60".parse().unwrap());
        headers.insert("RateLimit-Limit", "1000".parse().unwrap());
        headers.insert("RateLimit-Remaining", "5".parse().unwrap());
        headers.insert("RateLimit-Reset", "1700000000".parse().unwrap());

        let rl = extract_rate_limit_headers(&headers).expect("should extract rate limit headers");
        assert_eq!(rl.retry_after, Some(60));
        assert_eq!(rl.rate_limit, Some(1000));
        assert_eq!(rl.rate_remaining, Some(5));
        assert_eq!(rl.rate_reset, Some(1700000000));

        let err_with_rl = ClientError::with_rate_limit(429, "Rate limit exceeded", rl);
        assert!(err_with_rl.is_rate_limited());
        assert!(err_with_rl.rate_limit.is_some());
        assert_eq!(err_with_rl.rate_limit.unwrap().retry_after, Some(60));
    }
}

