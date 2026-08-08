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

