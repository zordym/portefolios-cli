use crate::domain::{Architecture, Language};

/// Command to create a new project
#[derive(Debug, Clone)]
pub struct CreateProjectCommand {
    pub name: String,
    pub language: Language,
    pub architecture: Architecture,
    pub description: Option<String>,
    pub base_path: std::path::PathBuf,
}

impl CreateProjectCommand {
    pub fn new(
        name: String,
        language: Language,
        architecture: Architecture,
        base_path: std::path::PathBuf,
    ) -> Self {
        Self {
            name,
            language,
            architecture,
            description: None,
            base_path,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }

        if !self.base_path.exists() {
            return Err(format!(
                "Base path does not exist: {}",
                self.base_path.display()
            ));
        }

        Ok(())
    }
}

/// Command to update a project
#[derive(Debug, Clone, Default)]
pub struct UpdateProjectCommand {
    pub project_id: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub add_frameworks: Vec<String>,
    pub remove_frameworks: Vec<String>,
    pub gitlab_url: Option<String>,
}

impl UpdateProjectCommand {
    pub fn new(project_id: String) -> Self {
        Self {
            project_id,
            ..Default::default()
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}
