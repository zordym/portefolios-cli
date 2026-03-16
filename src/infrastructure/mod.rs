//! Infrastructure layer - External concerns
//!
//! This layer contains:
//! - Repository implementations for data persistence
//! - External service integrations (Git, editor, etc.)
//! - Configuration loading
//! - Infrastructure-specific errors

pub mod config;
pub mod errors;
pub mod repositories;
pub mod services;

pub use config::ConfigStructure;
// Re-export commonly used types
pub use errors::InfrastructureError;
pub use repositories::{FileSystemPortfolioRepository, PortfolioRepository};
