use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GsheetsError {
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parsing error: {0}")]
    Parsing(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Other error: {0}")]
    Other(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl GsheetsError {
    pub fn remediation_hint(&self) -> &'static str {
        match self {
            GsheetsError::Auth(_) => "Verify authentication method and credentials.",
            GsheetsError::Io(_) => "Check file paths, permissions, and disk space.",
            GsheetsError::Network(_) => "Check network connectivity and proxy settings.",
            GsheetsError::Parsing(_) => "Check input data format and schema.",
            GsheetsError::Validation(_) => "Check input values and required fields.",
            GsheetsError::MissingField(_) => "Ensure all required configuration and request fields are set.",
            GsheetsError::Other(_) => "See error details for more information.",
            GsheetsError::ApiError(_) => "Check API request format and parameters.",
        }
    }
}

pub type McpError = GsheetsError;
