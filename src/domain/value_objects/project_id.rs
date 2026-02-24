use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated project identifier
///
/// ProjectId must:
/// - Be lowercase
/// - Contain only alphanumeric characters and hyphens
/// - Not start or end with a hyphen
/// - Not contain consecutive hyphens
/// - Be between 1 and 255 characters
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectId(String);

impl ProjectId {
    /// Create a new ProjectId with validation
    pub fn new(id: impl Into<String>) -> DomainResult<Self> {
        let id = id.into();

        // Check length
        if id.is_empty() {
            return Err(DomainError::InvalidProjectId(
                "Project ID cannot be empty".to_string(),
            ));
        }

        if id.len() > 255 {
            return Err(DomainError::InvalidProjectId(format!(
                "Project ID too long: {} characters",
                id.len()
            )));
        }

        // Check format: only lowercase, alphanumeric, and hyphens
        if !id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(DomainError::InvalidProjectId(
                "Project ID must contain only lowercase letters, numbers, and hyphens".to_string(),
            ));
        }

        // Check doesn't start/end with hyphen
        if id.starts_with('-') || id.ends_with('-') {
            return Err(DomainError::InvalidProjectId(
                "Project ID cannot start or end with a hyphen".to_string(),
            ));
        }

        // Check no consecutive hyphens
        if id.contains("--") {
            return Err(DomainError::InvalidProjectId(
                "Project ID cannot contain consecutive hyphens".to_string(),
            ));
        }

        Ok(Self(id))
    }

    /// Create without validation (use only when certain the ID is valid)
    ///
    /// # Safety
    /// This bypasses validation. Only use with trusted input.
    pub fn new_unchecked(id: impl Into<String>) -> Self {
        Self(id.into())
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

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ProjectId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for ProjectId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_project_ids() {
        assert!(ProjectId::new("my-project").is_ok());
        assert!(ProjectId::new("project123").is_ok());
        assert!(ProjectId::new("a").is_ok());
        assert!(ProjectId::new("my-awesome-project-2024").is_ok());
    }

    #[test]
    fn invalid_project_ids() {
        assert!(ProjectId::new("").is_err());
        assert!(ProjectId::new("My-Project").is_err()); // uppercase
        assert!(ProjectId::new("-project").is_err()); // starts with hyphen
        assert!(ProjectId::new("project-").is_err()); // ends with hyphen
        assert!(ProjectId::new("my--project").is_err()); // consecutive hyphens
        assert!(ProjectId::new("my_project").is_err()); // underscore
        assert!(ProjectId::new("my project").is_err()); // space
    }
}
