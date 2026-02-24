//! Domain-specific errors
//!
//! These errors represent business rule violations and domain-level failures.

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),

    #[error("Project name cannot be empty")]
    EmptyProjectName,

    #[error("Project name too long (max 255 characters): got {0} characters")]
    ProjectNameTooLong(usize),

    #[error("Project name contains invalid characters: {0}")]
    InvalidCharactersInName(String),

    #[error("Invalid project ID: {0}")]
    InvalidProjectId(String),

    #[error("Project already exists: {0}")]
    ProjectAlreadyExists(String),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Invalid language: {0}")]
    InvalidLanguage(String),

    #[error("Invalid architecture: {0}")]
    InvalidArchitecture(String),

    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("Invalid status transition from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Framework name cannot be empty")]
    EmptyFrameworkName,

    #[error("Duplicate framework: {0}")]
    DuplicateFramework(String),

    #[error("Portfolio is empty")]
    EmptyPortfolio,

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
