//! Value objects - Validated primitive types
//!
//! Value objects ensure that invalid data cannot be constructed.
//! They enforce business rules at the type level.

mod project_id;
mod project_name;

pub use project_id::ProjectId;
pub use project_name::ProjectName;
