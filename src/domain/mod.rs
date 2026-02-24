//! Domain layer - Core business logic and models
//!
//! This layer contains:
//! - Domain entities with business behavior
//! - Value objects for validated primitives
//! - Domain services for complex business operations
//! - Domain-specific errors

pub mod errors;
pub mod models;
pub mod services;
pub mod value_objects;

// Re-export commonly used types
pub use errors::DomainError;
pub use models::{Architecture, Language, Portfolio, Project, Status};
pub use value_objects::{ProjectId, ProjectName};
