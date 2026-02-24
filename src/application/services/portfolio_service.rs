use crate::application::dto::{ListProjectsQuery, ProjectFilters, ProjectQuery};
use crate::application::errors::{ApplicationError, ApplicationResult};
use crate::domain::models::PortfolioStatistics;
use crate::domain::value_objects::ProjectId;
use crate::domain::{Architecture, Language, Portfolio, Project, Status};
use crate::infrastructure::repositories::PortfolioRepository;
use std::sync::Arc;

/// Application service for portfolio operations
pub struct PortfolioService<R: PortfolioRepository> {
    repository: Arc<R>,
}

impl<R: PortfolioRepository> PortfolioService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Get a single project
    pub fn get_project(&self, query: ProjectQuery) -> ApplicationResult<Project> {
        // Try as ID first
        if let Ok(id) = ProjectId::new(&query.identifier) {
            if let Some(project) = self.repository.find_by_id(&id)? {
                return Ok(project);
            }
        }

        // Search in all projects by name
        let portfolio = self.repository.find_all()?;
        portfolio
            .find_by_name_or_id(&query.identifier)
            .cloned()
            .ok_or_else(|| ApplicationError::ProjectNotFound(query.identifier))
    }

    /// List projects with filters
    pub fn list_projects(&self, query: ListProjectsQuery) -> ApplicationResult<Vec<Project>> {
        let portfolio = self.repository.find_all()?;

        // Parse filters
        let filters = self.parse_filters(query)?;

        // Apply filters
        let filtered =
            portfolio.apply_filters(filters.language, filters.architecture, filters.status);

        Ok(filtered.into_iter().cloned().collect())
    }

    /// Get all projects (no filters)
    pub fn all_projects(&self) -> ApplicationResult<Portfolio> {
        self.repository.find_all().map_err(Into::into)
    }

    /// Get portfolio statistics
    pub fn get_statistics(&self) -> ApplicationResult<PortfolioStatistics> {
        let portfolio = self.repository.find_all()?;
        Ok(portfolio.statistics())
    }

    // Helper to parse query filters
    fn parse_filters(&self, query: ListProjectsQuery) -> ApplicationResult<ProjectFilters> {
        let mut filters = ProjectFilters::new();

        if let Some(lang_str) = query.language {
            let language: Language = lang_str.parse()?;
            filters = filters.with_language(language);
        }

        if let Some(arch_str) = query.architecture {
            let architecture: Architecture = arch_str.parse()?;
            filters = filters.with_architecture(architecture);
        }

        if let Some(status_str) = query.status {
            let status: Status = status_str.parse()?;
            filters = filters.with_status(status);
        }

        Ok(filters)
    }
}
