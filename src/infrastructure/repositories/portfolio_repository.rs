use crate::domain::value_objects::ProjectId;
use crate::domain::{Portfolio, Project};
use crate::infrastructure::errors::InfrastructureResult;

/// Repository trait for portfolio operations
///
/// This abstraction allows for different storage implementations
/// (file system, database, in-memory for testing, etc.)
pub trait PortfolioRepository: Send + Sync {
    /// Load all projects from storage
    fn find_all(&self) -> InfrastructureResult<Portfolio>;

    /// Find a project by ID
    fn find_by_id(&self, id: &ProjectId) -> InfrastructureResult<Option<Project>>;

    /// Save a project (create or update)
    fn save(&self, project: &Project) -> InfrastructureResult<()>;

    /// Delete a project
    fn delete(&self, id: &ProjectId) -> InfrastructureResult<()>;

    /// Check if a project exists
    fn exists(&self, id: &ProjectId) -> InfrastructureResult<bool> {
        Ok(self.find_by_id(id)?.is_some())
    }
}
