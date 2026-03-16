use crate::application::dto::{CreateProjectCommand, UpdateProjectCommand};
use crate::application::errors::{ApplicationError, ApplicationResult};
use crate::domain::value_objects::{ProjectId, ProjectName};
use crate::domain::{Language, Project, Status};
use crate::infrastructure::InfrastructureError;
use crate::infrastructure::repositories::PortfolioRepository;
use crate::infrastructure::services::VersionControlService;
use std::fs;
use std::sync::Arc;

/// Application service for project operations
pub struct ProjectService<R: PortfolioRepository, V: VersionControlService> {
    repository: Arc<R>,
    vcs: Arc<V>,
}

impl<R: PortfolioRepository, V: VersionControlService> ProjectService<R, V> {
    pub fn new(repository: Arc<R>, vcs: Arc<V>) -> Self {
        Self { repository, vcs }
    }

    /// Create a new project
    pub fn create_project(&self, command: CreateProjectCommand) -> ApplicationResult<Project> {
        tracing::info!("Creating project: {}", command.name);

        // Validate command
        command
            .validate()
            .map_err(ApplicationError::ValidationError)?;

        // Create domain objects
        let name = ProjectName::new(&command.name)?;
        let id = name.to_id();

        // Check for duplicates
        if self.repository.exists(&id)? {
            return Err(ApplicationError::ProjectAlreadyExists(id.to_string()));
        }

        // Create a project path
        let project_path = command.base_path.join(id.as_str());

        // Create a project entity
        let mut project = Project::new(
            name,
            command.language,
            command.architecture,
            project_path.clone(),
        );

        if let Some(desc) = command.description {
            project.set_description(desc);
        }

        // Create directory structure
        self.create_directory_structure(&project)?;

        // Create a build configuration
        self.create_build_config(&project)?;

        // Create documentation
        self.create_documentation(&project)?;

        // Initialize version control
        if !self.vcs.is_repository(&project_path) {
            self.vcs.init(&project_path)?;

            // Create .gitignore
            self.create_gitignore(&project)?;

            // Initial commit
            self.vcs.add(&project_path, &["."])?;
            self.vcs.commit(&project_path, "Initial commit")?;
        }

        // Save to repository
        self.repository.save(&project)?;

        tracing::info!("Project created successfully: {}", project.id());
        Ok(project)
    }

    /// Update a project
    pub fn update_project(&self, command: UpdateProjectCommand) -> ApplicationResult<Project> {
        let id = ProjectId::new(&command.project_id)?;

        let mut project = self
            .repository
            .find_by_id(&id)?
            .ok_or_else(|| ApplicationError::ProjectNotFound(command.project_id.clone()))?;

        // Update fields
        if let Some(description) = command.description {
            project.set_description(description);
        }

        if let Some(status_str) = command.status {
            let status: Status = status_str.parse()?;
            project.set_status(status)?;
        }

        for framework in command.add_frameworks {
            project.add_framework(framework)?;
        }

        for framework in &command.remove_frameworks {
            project.remove_framework(framework);
        }

        if let Some(url) = command.gitlab_url {
            project.set_gitlab_url(url);
        }

        // Save
        self.repository.save(&project)?;

        Ok(project)
    }

    /// Delete a project
    pub fn delete_project(&self, project_id: &str) -> ApplicationResult<()> {
        let id = ProjectId::new(project_id)?;

        if !self.repository.exists(&id)? {
            return Err(ApplicationError::ProjectNotFound(project_id.to_string()));
        }

        self.repository.delete(&id)?;
        tracing::info!("Project deleted: {}", project_id);

        Ok(())
    }

    // ==================== Private Helper Methods ====================

    fn create_directory_structure(&self, project: &Project) -> ApplicationResult<()> {
        let base_path = project.path();
        fs::create_dir_all(base_path).map_err(InfrastructureError::Io)?;

        let src_base = match project.language() {
            Language::Kotlin => base_path.join("src/main/kotlin"),
            Language::Java => base_path.join("src/main/java"),
            Language::Rust => base_path.join("src"),
        };

        // Create architecture-specific directories
        for layer in project.architecture().typical_layers() {
            fs::create_dir_all(src_base.join(layer)).map_err(InfrastructureError::Io)?;
        }

        // Create docs directories
        fs::create_dir_all(base_path.join("docs")).map_err(InfrastructureError::Io)?;

        Ok(())
    }

    fn create_build_config(&self, project: &Project) -> ApplicationResult<()> {
        let path = project.path();
        let project_id = project.id().as_str();

        match project.language() {
            Language::Kotlin => {
                let build_gradle = r#"plugins {
    kotlin("jvm") version "1.9.22"
    kotlin("plugin.spring") version "1.9.22"
    id("org.springframework.boot") version "3.2.1"
}

group = "com.portfolio"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-webflux")
    implementation("org.jetbrains.kotlin:kotlin-reflect")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
}
"#
                .to_string();
                fs::write(path.join("build.gradle.kts"), build_gradle)
                    .map_err(InfrastructureError::Io)?;
                fs::write(
                    path.join("settings.gradle.kts"),
                    format!("rootProject.name = \"{}\"", project_id),
                )
                .map_err(InfrastructureError::Io)?;
            }
            Language::Java => {
                let pom = format!(
                    r#"<?xml version="1.0"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.portfolio</groupId>
    <artifactId>{}</artifactId>
    <version>0.1.0</version>
</project>
"#,
                    project_id
                );
                fs::write(path.join("pom.xml"), pom).map_err(InfrastructureError::Io)?;
            }
            Language::Rust => {
                let cargo = format!(
                    r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
                    project_id
                );
                fs::write(path.join("Cargo.toml"), cargo).map_err(InfrastructureError::Io)?;
            }
        }

        Ok(())
    }

    fn create_documentation(&self, project: &Project) -> ApplicationResult<()> {
        let path = project.path();

        let readme = format!(
            r#"# {}

## Description

{}

## Architecture

This project follows **{}** architecture.

{}

## Technology Stack

- **Language**: {}
- **Architecture**: {}

## Getting Started

### Prerequisites

Install required tools for {} development.

### Running

Instructions for running the project.

## Documentation

See `ARCHITECTURE.md` for detailed architecture documentation.
"#,
            project.name(),
            project.description(),
            project.architecture(),
            project.architecture().description(),
            project.language(),
            project.architecture(),
            project.language()
        );

        fs::write(path.join("README.md"), readme).map_err(InfrastructureError::Io)?;

        let architecture_doc = format!(
            r#"# Architecture Documentation

## Overview

This project implements {} architecture.

{}

## Structure

{}

## Design Decisions

Key architectural decisions and their rationale will be documented here.

## Components

Description of main components and their responsibilities.
"#,
            project.architecture(),
            project.architecture().description(),
            project
                .architecture()
                .typical_layers()
                .iter()
                .map(|layer| format!("- `{}`", layer))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(path.join("ARCHITECTURE.md"), architecture_doc)
            .map_err(InfrastructureError::Io)?;

        Ok(())
    }

    fn create_gitignore(&self, project: &Project) -> ApplicationResult<()> {
        let gitignore = match project.language() {
            Language::Rust => "target/\n**/*.rs.bk\nCargo.lock\n",
            Language::Kotlin | Language::Java => {
                "build/\n.gradle/\ntarget/\n*.class\n.idea/\n*.iml\n"
            }
        };

        fs::write(project.path().join(".gitignore"), gitignore).map_err(InfrastructureError::Io)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Architecture;
    use crate::infrastructure::repositories::FileSystemPortfolioRepository;
    use crate::infrastructure::services::GitService;
    use tempfile::TempDir;

    #[test]
    fn test_create_project() {
        let temp_dir = TempDir::new().unwrap();

        let repo = Arc::new(FileSystemPortfolioRepository::new(temp_dir.path()));
        let vcs = Arc::new(GitService::new());
        let service = ProjectService::new(repo, vcs);

        let command = CreateProjectCommand::new(
            "Test Project".to_string(),
            Language::Rust,
            Architecture::Hexagonal,
            temp_dir.path().to_path_buf(),
        );

        let result = service.create_project(command);
        assert!(result.is_ok());

        let project = result.unwrap();
        assert_eq!(project.name().as_str(), "Test Project");
        assert_eq!(project.language(), Language::Rust);

        // Verify files were created
        assert!(project.path().join("Cargo.toml").exists());
        assert!(project.path().join("README.md").exists());
    }
}
