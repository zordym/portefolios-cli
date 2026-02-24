use crate::domain::value_objects::{ProjectId, ProjectName};
use crate::domain::{Architecture, Language, Portfolio, Project, Status};
use crate::infrastructure::errors::{InfrastructureError, InfrastructureResult};
use crate::infrastructure::repositories::PortfolioRepository;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// File system based portfolio repository
///
/// Stores projects as individual project.toml files in directories
pub struct FileSystemPortfolioRepository {
    root_path: PathBuf,
}

impl FileSystemPortfolioRepository {
    /// Create a new file system repository
    pub fn new(root_path: impl Into<PathBuf>) -> Self {
        Self {
            root_path: root_path.into(),
        }
    }

    /// Get the metadata file path for a project
    fn metadata_path(&self, project_id: &ProjectId) -> PathBuf {
        self.root_path
            .join(project_id.as_str())
            .join("project.toml")
    }

    /// Load a single project from a file
    fn load_project(&self, path: &Path) -> InfrastructureResult<Project> {
        let content = fs::read_to_string(path)?;
        let dto: ProjectDto = toml::from_str(&content)?;
        dto.into_domain()
    }

    /// Save a project to a file
    fn save_project(&self, project: &Project, path: &Path) -> InfrastructureResult<()> {
        let dto = ProjectDto::from_domain(project);
        let content = toml::to_string_pretty(&dto)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, content)?;
        Ok(())
    }
}

impl PortfolioRepository for FileSystemPortfolioRepository {
    fn find_all(&self) -> InfrastructureResult<Portfolio> {
        let mut projects = Vec::new();

        for entry in WalkDir::new(&self.root_path)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == "project.toml" {
                match self.load_project(entry.path()) {
                    Ok(project) => projects.push(project),
                    Err(e) => {
                        tracing::warn!(
                            "Failed to load project from {}: {}",
                            entry.path().display(),
                            e
                        );
                    }
                }
            }
        }

        Ok(Portfolio::from_projects(projects))
    }

    fn find_by_id(&self, id: &ProjectId) -> InfrastructureResult<Option<Project>> {
        let path = self.metadata_path(id);

        if !path.exists() {
            return Ok(None);
        }

        match self.load_project(&path) {
            Ok(project) => Ok(Some(project)),
            Err(e) => Err(e),
        }
    }

    fn save(&self, project: &Project) -> InfrastructureResult<()> {
        let path = self.metadata_path(project.id());
        self.save_project(project, &path)
    }

    fn delete(&self, id: &ProjectId) -> InfrastructureResult<()> {
        let project_dir = self.root_path.join(id.as_str());

        if project_dir.exists() {
            fs::remove_dir_all(project_dir)?;
        }

        Ok(())
    }
}

/// DTO for serializing/deserializing projects
///
/// This separates the domain model from the persistence format
#[derive(Debug, Serialize, Deserialize)]
struct ProjectDto {
    id: String,
    name: String,
    description: String,
    language: String,
    architecture: String,
    path: String,
    status: String,
    frameworks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_url: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    created_at: DateTime<Utc>,
    #[serde(default = "chrono::Utc::now")]
    updated_at: DateTime<Utc>,
}

impl ProjectDto {
    fn from_domain(project: &Project) -> Self {
        Self {
            id: project.id().to_string(),
            name: project.name().to_string(),
            description: project.description().to_string(),
            language: project.language().to_string().to_lowercase(),
            architecture: project.architecture().to_string().to_lowercase(),
            path: project.path().display().to_string(),
            status: format!("{:?}", project.status()).to_lowercase(),
            frameworks: project.frameworks().to_vec(),
            gitlab_url: project.gitlab_url().map(String::from),
            created_at: project.created_at(),
            updated_at: project.updated_at(),
        }
    }

    fn into_domain(self) -> InfrastructureResult<Project> {
        let id =
            ProjectId::new(self.id).map_err(|e| InfrastructureError::InvalidPath(e.to_string()))?;

        let name = ProjectName::new(self.name)
            .map_err(|e| InfrastructureError::InvalidPath(e.to_string()))?;

        let language: Language =
            self.language
                .parse()
                .map_err(|e: crate::domain::DomainError| {
                    InfrastructureError::InvalidPath(e.to_string())
                })?;

        let architecture: Architecture =
            self.architecture
                .parse()
                .map_err(|e: crate::domain::DomainError| {
                    InfrastructureError::InvalidPath(e.to_string())
                })?;

        let status: Status = self
            .status
            .parse()
            .map_err(|e: crate::domain::DomainError| {
                InfrastructureError::InvalidPath(e.to_string())
            })?;

        Ok(Project::with_all_fields(
            id,
            name,
            self.description,
            language,
            architecture,
            PathBuf::from(self.path),
            status,
            self.frameworks,
            self.gitlab_url,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn save_and_load_project() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileSystemPortfolioRepository::new(temp_dir.path());

        let project = Project::new(
            ProjectName::new("Test Project").unwrap(),
            Language::Rust,
            Architecture::Hexagonal,
            temp_dir.path().join("test-project"),
        );

        // Save
        repo.save(&project).unwrap();

        // Load
        let loaded = repo.find_by_id(project.id()).unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name().as_str(), "Test Project");
    }
}
