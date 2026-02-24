use crate::application::errors::{ApplicationError, ApplicationResult};
use crate::domain::Portfolio;
use crate::infrastructure::InfrastructureError;
use serde::de::Unexpected::Str;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Service for generating documentation
pub struct DocumentationService;

impl DocumentationService {
    pub fn new() -> Self {
        Self
    }

    /// Generate portfolio documentation
    pub fn generate_documentation(
        &self,
        portfolio: &Portfolio,
        output_dir: &Path,
        detailed: bool,
    ) -> ApplicationResult<()> {
        fs::create_dir_all(output_dir)
            .map_err(|err| ApplicationError::ProjectNotFound(String::from(err.description())))?;
        let mut content = String::from("# Architecture Portfolio\n\n");
        content.push_str("## Projects Overview\n\n");

        // Add statistics
        let stats = portfolio.statistics();
        content.push_str(&format!("**Total Projects**: {}\n", stats.total_projects));
        content.push_str(&format!(
            "**Completion Rate**: {:.1}%\n\n",
            stats.completion_percentage()
        ));

        content.push_str("### Projects by Status\n\n");
        content.push_str(&format!("- Planned: {}\n", stats.planned_count));
        content.push_str(&format!("- In Progress: {}\n", stats.in_progress_count));
        content.push_str(&format!("- Completed: {}\n", stats.completed_count));
        content.push_str(&format!("- Archived: {}\n\n", stats.archived_count));

        content.push_str("---\n\n");

        // Add project details
        for project in portfolio.projects() {
            content.push_str(&format!("### {}\n\n", project.name()));
            content.push_str(&format!("- **ID**: `{}`\n", project.id()));
            content.push_str(&format!("- **Language**: {}\n", project.language()));
            content.push_str(&format!("- **Architecture**: {}\n", project.architecture()));
            content.push_str(&format!(
                "- **Status**: {} {}\n",
                project.status().symbol(),
                project.status()
            ));

            if detailed {
                if !project.description().is_empty() {
                    content.push_str(&format!("\n{}\n", project.description()));
                }

                if !project.frameworks().is_empty() {
                    content.push_str(&format!(
                        "\n**Frameworks**: {}\n",
                        project.frameworks().join(", ")
                    ));
                }

                if let Some(url) = project.gitlab_url() {
                    content.push_str(&format!("\n**Repository**: {}\n", url));
                }

                content.push_str(&format!(
                    "\n**Created**: {}\n",
                    project.created_at().format("%Y-%m-%d")
                ));
                content.push_str(&format!(
                    "**Last Updated**: {}\n",
                    project.updated_at().format("%Y-%m-%d")
                ));
            }

            content.push_str("\n---\n\n");
        }

        let output_file = output_dir.join("portfolio.md");
        fs::write(&output_file, content)
            .map_err(|err| ApplicationError::Infrastructure(InfrastructureError::Io(err)))?;

        tracing::info!("Documentation generated: {}", output_file.display());
        Ok(())
    }
}

impl Default for DocumentationService {
    fn default() -> Self {
        Self::new()
    }
}
