//! Domain models
//!
//! These represent core business entities with behavior.

mod enums;
mod portfolio;
mod project;

pub use enums::{Architecture, Language, Status};
pub use portfolio::Portfolio;
pub use portfolio::PortfolioStatistics;
pub use project::Project;
