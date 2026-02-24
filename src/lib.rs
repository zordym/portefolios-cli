//! Portfolio CLI Library
//!
//! This library provides core functionality for managing architecture portfolios.
//! It follows a layered architecture pattern:
//!
//! - **Domain**: Core business logic
//! - **Application**: Use case orchestration
//! - **Infrastructure**: External integrations
//! - **Presentation**: CLI interface

// Re-export layers
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub use application::{PortfolioService, ProjectService};
// Re-export commonly used types for convenience
pub use domain::{Architecture, Language, Portfolio, Project, Status};
pub use infrastructure::{Config, PortfolioRepository};
