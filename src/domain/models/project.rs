use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::models::{Architecture, Language, Status};
use crate::domain::value_objects::{ProjectId, ProjectName};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A project entity representing a portfolio project
///
/// This is a rich domain model with business behavior, not just data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: ProjectId,

    name: ProjectName,

    description: String,

    language: Language,

    architecture: Architecture,

    path: PathBuf,

    status: Status,

    frameworks: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_url: Option<String>,

    created_at: DateTime<Utc>,

    updated_at: DateTime<Utc>,
}

impl Project {
    /// Create a new project with required fields
    ///
    /// This is the factory method that ensures a valid project is created.
    pub fn new(
        name: ProjectName,
        language: Language,
        architecture: Architecture,
        path: PathBuf,
    ) -> Self {
        let now = Utc::now();
        let id = name.to_id();

        Self {
            id,
            name,
            description: String::new(),
            language,
            architecture,
            path,
            status: Status::Planned,
            frameworks: Vec::new(),
            gitlab_url: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a project with all fields (for deserialization)
    #[allow(clippy::too_many_arguments)]
    pub fn with_all_fields(
        id: ProjectId,
        name: ProjectName,
        description: String,
        language: Language,
        architecture: Architecture,
        path: PathBuf,
        status: Status,
        frameworks: Vec<String>,
        gitlab_url: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            language,
            architecture,
            path,
            status,
            frameworks,
            gitlab_url,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ProjectId {
        &self.id
    }

    pub fn name(&self) -> &ProjectName {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn architecture(&self) -> Architecture {
        self.architecture
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn frameworks(&self) -> &[String] {
        &self.frameworks
    }

    pub fn gitlab_url(&self) -> Option<&str> {
        self.gitlab_url.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Update the project description
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
        self.touch();
    }

    /// Update the project status with validation
    pub fn set_status(&mut self, new_status: Status) -> DomainResult<()> {
        if !self.status.can_transition_to(new_status) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: new_status.to_string(),
            });
        }

        self.status = new_status;
        self.touch();
        Ok(())
    }

    /// Force set status without validation (use with caution)
    pub fn force_set_status(&mut self, status: Status) {
        self.status = status;
        self.touch();
    }

    /// Add a framework to the project
    pub fn add_framework(&mut self, framework: impl Into<String>) -> DomainResult<()> {
        let framework = framework.into();

        if framework.trim().is_empty() {
            return Err(DomainError::EmptyFrameworkName);
        }

        if self.frameworks.contains(&framework) {
            return Err(DomainError::DuplicateFramework(framework));
        }

        self.frameworks.push(framework);
        self.touch();
        Ok(())
    }

    /// Remove a framework from the project
    pub fn remove_framework(&mut self, framework: &str) -> bool {
        if let Some(pos) = self.frameworks.iter().position(|f| f == framework) {
            self.frameworks.remove(pos);
            self.touch();
            true
        } else {
            false
        }
    }

    /// Set the GitLab URL
    pub fn set_gitlab_url(&mut self, url: impl Into<String>) {
        self.gitlab_url = Some(url.into());
        self.touch();
    }

    /// Clear the GitLab URL
    pub fn clear_gitlab_url(&mut self) {
        self.gitlab_url = None;
        self.touch();
    }

    /// Update the timestamp
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    // ==================== Query Methods ====================

    /// Check if the project is completed
    pub fn is_completed(&self) -> bool {
        self.status == Status::Completed
    }

    /// Check if the project is in progress
    pub fn is_in_progress(&self) -> bool {
        self.status == Status::InProgress
    }

    /// Check if the project is planned
    pub fn is_planned(&self) -> bool {
        self.status == Status::Planned
    }

    /// Check if the project is archived
    pub fn is_archived(&self) -> bool {
        self.status == Status::Archived
    }

    /// Check if the project uses a specific framework
    pub fn uses_framework(&self, framework: &str) -> bool {
        self.frameworks.iter().any(|f| f == framework)
    }

    /// Check if the project has a GitLab URL
    pub fn has_gitlab_url(&self) -> bool {
        self.gitlab_url.is_some()
    }

    /// Get the project's age in days
    pub fn age_in_days(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.created_at).num_days()
    }

    /// Get the days since last update
    pub fn days_since_update(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.updated_at).num_days()
    }
}

// Implement PartialEq for testing
impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Project {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project() -> Project {
        Project::new(
            ProjectName::new("Test Project").unwrap(),
            Language::Rust,
            Architecture::Hexagonal,
            PathBuf::from("/test"),
        )
    }

    #[test]
    fn create_new_project() {
        let project = sample_project();
        assert_eq!(project.id().as_str(), "test-project");
        assert_eq!(project.name().as_str(), "Test Project");
        assert_eq!(project.language(), Language::Rust);
        assert_eq!(project.status(), Status::Planned);
    }

    #[test]
    fn status_transition_validation() {
        let mut project = sample_project();

        // Valid transition
        assert!(project.set_status(Status::InProgress).is_ok());
        assert_eq!(project.status(), Status::InProgress);

        // Valid transition
        assert!(project.set_status(Status::Completed).is_ok());

        // Invalid transition (Completed -> Planned not allowed)
        assert!(project.set_status(Status::Planned).is_err());
    }

    #[test]
    fn add_frameworks() {
        let mut project = sample_project();

        assert!(project.add_framework("tokio").is_ok());
        assert!(project.add_framework("serde").is_ok());
        assert_eq!(project.frameworks().len(), 2);

        // Duplicate should fail
        assert!(project.add_framework("tokio").is_err());

        // Empty should fail
        assert!(project.add_framework("").is_err());
    }

    #[test]
    fn query_methods() {
        let mut project = sample_project();

        assert!(project.is_planned());
        assert!(!project.is_completed());

        project.force_set_status(Status::InProgress);
        assert!(project.is_in_progress());

        project.add_framework("tokio").unwrap();
        assert!(project.uses_framework("tokio"));
        assert!(!project.uses_framework("async-std"));
    }
}
