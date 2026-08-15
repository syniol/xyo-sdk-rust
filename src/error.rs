/// Error type returned by the XYO SDK client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientError {
    pub message: String,
    pub code: u16,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientError (code {}): {}", self.code, self.message)
    }
}

impl std::error::Error for ClientError {}

impl ClientError {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_error_display() {
        let err = ClientError {
            code: 404,
            message: "Not Found".to_string(),
        };
        assert_eq!(format!("{}", err), "ClientError (code 404): Not Found");
    }

    #[test]
    fn test_client_error_debug() {
        let err = ClientError {
            code: 500,
            message: "Internal Error".to_string(),
        };
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("500"));
        assert!(debug_str.contains("Internal Error"));
    }

    #[test]
    fn test_client_error_classification_methods() {
        let auth_err = ClientError {
            code: 401,
            message: "Unauthorized".to_string(),
        };
        assert!(auth_err.is_auth());
        assert!(!auth_err.is_server_error());
        assert!(!auth_err.is_retryable());

        let forbidden_err = ClientError {
            code: 403,
            message: "Forbidden".to_string(),
        };
        assert!(forbidden_err.is_auth());

        let rate_err = ClientError {
            code: 429,
            message: "Too Many Requests".to_string(),
        };
        assert!(rate_err.is_rate_limited());
        assert!(rate_err.is_retryable());

        let server_err = ClientError {
            code: 503,
            message: "Service Unavailable".to_string(),
        };
        assert!(server_err.is_server_error());
        assert!(server_err.is_retryable());

        let timeout_err = ClientError {
            code: 0,
            message: "operation timed out after 30s".to_string(),
        };
        assert!(timeout_err.is_retryable());
    }

    #[test]
    fn test_client_error_clone_and_eq() {
        let err1 = ClientError {
            code: 400,
            message: "Bad Request".to_string(),
        };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
        assert_eq!(err1.code, 400);
        assert_eq!(err1.message, "Bad Request");

        let err3 = ClientError {
            code: 401,
            message: "Unauthorized".to_string(),
        };
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_client_error_implements_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(ClientError {
            code: 403,
            message: "Forbidden".to_string(),
        });
        assert_eq!(format!("{}", err), "ClientError (code 403): Forbidden");
        assert!(err.source().is_none());
    }
}

