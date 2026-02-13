//! Structured API error handling
//!
//! Provides consistent JSON error responses with HTTP status mapping.

extern crate alloc;

use alloc::format;
use alloc::string::String;

/// API error codes (machine-readable)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiErrorCode {
    BadRequest,
    NotFound,
    MethodNotAllowed,
    Conflict,
    PayloadTooLarge,
    UnprocessableEntity,
    InternalError,
}

impl ApiErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BadRequest => "BAD_REQUEST",
            Self::NotFound => "NOT_FOUND",
            Self::MethodNotAllowed => "METHOD_NOT_ALLOWED",
            Self::Conflict => "CONFLICT",
            Self::PayloadTooLarge => "PAYLOAD_TOO_LARGE",
            Self::UnprocessableEntity => "UNPROCESSABLE_ENTITY",
            Self::InternalError => "INTERNAL_ERROR",
        }
    }

    pub fn http_status(&self) -> &'static str {
        match self {
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
            Self::MethodNotAllowed => "405 Method Not Allowed",
            Self::Conflict => "409 Conflict",
            Self::PayloadTooLarge => "413 Payload Too Large",
            Self::UnprocessableEntity => "422 Unprocessable Entity",
            Self::InternalError => "500 Internal Server Error",
        }
    }
}

/// Structured API error
#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
}

impl ApiError {
    pub fn new(code: ApiErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorCode::BadRequest, msg)
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorCode::NotFound, msg)
    }

    pub fn method_not_allowed(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorCode::MethodNotAllowed, msg)
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorCode::Conflict, msg)
    }

    pub fn payload_too_large(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorCode::PayloadTooLarge, msg)
    }

    pub fn unprocessable(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorCode::UnprocessableEntity, msg)
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(ApiErrorCode::InternalError, msg)
    }

    /// Convert to JSON error response body
    pub fn to_json(&self) -> String {
        // Escape message for JSON
        let escaped_msg = escape_json_string(&self.message);
        format!(
            r#"{{"error":{{"code":"{}","message":"{}"}}}}"#,
            self.code.as_str(),
            escaped_msg
        )
    }

    /// Get HTTP status line
    pub fn http_status(&self) -> &'static str {
        self.code.http_status()
    }
}

/// Simple JSON string escaping
pub fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c < '\x20' => {
                // Control chars as \uXXXX
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_to_json() {
        let err = ApiError::not_found("Thing 123 not found");
        let json = err.to_json();
        assert!(json.contains("NOT_FOUND"));
        assert!(json.contains("Thing 123 not found"));
    }

    #[test]
    fn test_error_escaping() {
        let err = ApiError::bad_request("Invalid \"key\" in\nrequest");
        let json = err.to_json();
        assert!(json.contains("\\\""));
        assert!(json.contains("\\n"));
    }

    #[test]
    fn test_http_status() {
        assert_eq!(ApiError::not_found("x").http_status(), "404 Not Found");
        assert_eq!(ApiError::bad_request("x").http_status(), "400 Bad Request");
    }
}
