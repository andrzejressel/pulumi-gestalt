use std::fmt;

/// Errors that can occur in the rust_integration crate
#[derive(Debug)]
pub enum IntegrationError {
    /// Environment variable not set
    MissingEnvironmentVariable(String),
    /// Invalid JSON format
    InvalidJson(serde_json::Error),
    /// Function not found
    FunctionNotFound(String),
    /// Core engine error
    CoreError(pulumi_gestalt_core::CoreError),
    /// Configuration error
    ConfigError(anyhow::Error),
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrationError::MissingEnvironmentVariable(var) => {
                write!(f, "Environment variable {} must be set", var)
            }
            IntegrationError::InvalidJson(err) => {
                write!(f, "Invalid JSON format: {}", err)
            }
            IntegrationError::FunctionNotFound(name) => {
                write!(f, "Function {} not found", name)
            }
            IntegrationError::CoreError(err) => {
                write!(f, "Core engine error: {}", err)
            }
            IntegrationError::ConfigError(err) => {
                write!(f, "Configuration error: {}", err)
            }
        }
    }
}

impl std::error::Error for IntegrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IntegrationError::InvalidJson(err) => Some(err),
            IntegrationError::CoreError(err) => Some(err),
            IntegrationError::ConfigError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for IntegrationError {
    fn from(err: serde_json::Error) -> Self {
        IntegrationError::InvalidJson(err)
    }
}

impl From<pulumi_gestalt_core::CoreError> for IntegrationError {
    fn from(err: pulumi_gestalt_core::CoreError) -> Self {
        IntegrationError::CoreError(err)
    }
}

impl From<anyhow::Error> for IntegrationError {
    fn from(err: anyhow::Error) -> Self {
        IntegrationError::ConfigError(err)
    }
}

pub type IntegrationResult<T> = Result<T, IntegrationError>;