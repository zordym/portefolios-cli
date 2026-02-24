use crate::domain::DomainError;
use crate::infrastructure::InfrastructureError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfrastructureError),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Project already exists: {0}")]
    ProjectAlreadyExists(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
