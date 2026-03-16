use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::value_objects::ProjectId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated project name
///
/// ProjectName must:
/// - Not be empty or whitespace-only
/// - Be between 1 and 255 characters
/// - Not contain path separators (/, \)
/// - Not contain path traversal sequences (...)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectName(String);

impl ProjectName {
    /// Create a new ProjectName with validation
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        let trimmed = name.trim();

        // Check not empty
        if trimmed.is_empty() {
            return Err(DomainError::EmptyProjectName);
        }

        // Check length
        if name.len() > 255 {
            return Err(DomainError::ProjectNameTooLong(name.len()));
        }

        // Security: Check for path separators
        if name.contains('/') || name.contains('\\') {
            return Err(DomainError::InvalidCharactersInName(
                "Project name cannot contain path separators".to_string(),
            ));
        }

        // Security: Check for path traversal
        if name.contains("..") {
            return Err(DomainError::InvalidCharactersInName(
                "Project name cannot contain '..'".to_string(),
            ));
        }

        // Check for null bytes (security)
        if name.contains('\0') {
            return Err(DomainError::InvalidCharactersInName(
                "Project name cannot contain null bytes".to_string(),
            ));
        }

        Ok(Self(name))
    }

    /// Convert the name to a valid project ID
    ///
    /// Transforms the name by:
    /// - Converting to lowercase
    /// - Replacing spaces and underscores with hyphens
    /// - Removing invalid characters
    /// - Collapsing multiple hyphens
    pub fn to_id(&self) -> ProjectId {
        let mut id = self
            .0
            .to_lowercase()
            .replace([' ', '_'], "-")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>();

        // Remove leading/trailing hyphens
        id = id.trim_matches('-').to_string();

        // Collapse multiple hyphens
        while id.contains("--") {
            id = id.replace("--", "-");
        }

        // Fallback if empty
        if id.is_empty() {
            id = "project".to_string();
        }

        ProjectId::new_unchecked(id)
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into the inner string
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ProjectName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for ProjectName {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_project_names() {
        assert!(ProjectName::new("My Project").is_ok());
        assert!(ProjectName::new("Project 2024").is_ok());
        assert!(ProjectName::new("A").is_ok());
    }

    #[test]
    fn invalid_project_names() {
        assert!(ProjectName::new("").is_err());
        assert!(ProjectName::new("   ").is_err());
        assert!(ProjectName::new("path/to/project").is_err());
        assert!(ProjectName::new("../project").is_err());
        assert!(ProjectName::new("project\\file").is_err());
    }

    #[test]
    fn name_to_id_conversion() {
        let name = ProjectName::new("My Awesome Project").unwrap();
        assert_eq!(name.to_id().as_str(), "my-awesome-project");

        let name = ProjectName::new("Project_With_Underscores").unwrap();
        assert_eq!(name.to_id().as_str(), "project-with-underscores");

        let name = ProjectName::new("Project   With   Spaces").unwrap();
        assert_eq!(name.to_id().as_str(), "project-with-spaces");
    }
}
