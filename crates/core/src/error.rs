use std::fmt;

/// Errors that can occur in the core crate
#[derive(Debug)]
pub enum CoreError {
    /// Invalid UUID format
    InvalidUuid(uuid::Error),
    /// Missing node in engine
    NodeNotFound(crate::model::OutputId),
    /// Invalid node type 
    InvalidNodeType {
        output_id: crate::model::OutputId,
        expected: &'static str,
        actual: &'static str,
    },
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::InvalidUuid(err) => write!(f, "Invalid UUID format: {}", err),
            CoreError::NodeNotFound(id) => write!(f, "Cannot find node with id {}", id),
            CoreError::InvalidNodeType { output_id, expected, actual } => {
                write!(f, "Node with id [{}] is {}, not {}", output_id, actual, expected)
            }
        }
    }
}

impl std::error::Error for CoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CoreError::InvalidUuid(err) => Some(err),
            _ => None,
        }
    }
}

pub type CoreResult<T> = Result<T, CoreError>;