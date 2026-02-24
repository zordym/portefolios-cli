//! Application layer - Use cases and orchestration
//!
//! This layer contains:
//! - Application services that orchestrate domain and infrastructure
//! - DTOs (Data Transfer Objects) for command/query inputs
//! - Application-specific errors

pub mod dto;
pub mod errors;
pub mod services;

// Re-export commonly used types
pub use errors::ApplicationError;
pub use services::{DocumentationService, PortfolioService, ProjectService};
