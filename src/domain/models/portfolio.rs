use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::models::{Architecture, Language, Project, Status};
use crate::domain::value_objects::ProjectId;
use std::collections::HashMap;

/// Portfolio aggregate - manages a collection of projects
///
/// This is an aggregate root that ensures consistency across projects.
#[derive(Debug, Clone, Default)]
pub struct Portfolio {
    projects: Vec<Project>,
    // Index for fast lookups
    id_index: HashMap<ProjectId, usize>,
}

impl Portfolio {
    /// Create a new empty portfolio
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a portfolio from a vector of projects
    pub fn from_projects(projects: Vec<Project>) -> Self {
        let mut portfolio = Self::new();
        for project in projects {
            let _ = portfolio.add_project(project);
        }
        portfolio
    }

    /// Add a project to the portfolio
    pub fn add_project(&mut self, project: Project) -> DomainResult<()> {
        if self.id_index.contains_key(project.id()) {
            return Err(DomainError::ProjectAlreadyExists(project.id().to_string()));
        }

        let id = project.id().clone();
        let index = self.projects.len();

        self.projects.push(project);
        self.id_index.insert(id, index);

        Ok(())
    }

    /// Remove a project by ID
    pub fn remove_project(&mut self, id: &ProjectId) -> DomainResult<Project> {
        let index = self
            .id_index
            .remove(id)
            .ok_or_else(|| DomainError::ProjectNotFound(id.to_string()))?;

        let project = self.projects.remove(index);

        self.rebuild_index();

        Ok(project)
    }

    /// Find a project by ID
    pub fn find_by_id(&self, id: &ProjectId) -> Option<&Project> {
        self.id_index
            .get(id)
            .and_then(|&index| self.projects.get(index))
    }

    /// Find a mutable project by ID
    pub fn find_by_id_mut(&mut self, id: &ProjectId) -> Option<&mut Project> {
        self.id_index
            .get(id)
            .and_then(|&index| self.projects.get_mut(index))
    }

    /// Find a project by name or ID
    pub fn find_by_name_or_id(&self, search: &str) -> Option<&Project> {
        let search_lower = search.to_lowercase();

        self.projects.iter().find(|p| {
            p.name().as_str().to_lowercase() == search_lower || p.id().as_str() == search_lower
        })
    }

    /// Get all projects
    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    /// Get the number of projects
    pub fn len(&self) -> usize {
        self.projects.len()
    }

    /// Check if the portfolio is empty
    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }

    /// Rebuild the ID index (call after modifying the projects vector)
    fn rebuild_index(&mut self) {
        self.id_index.clear();
        for (index, project) in self.projects.iter().enumerate() {
            self.id_index.insert(project.id().clone(), index);
        }
    }

    // ==================== Filtering Methods ====================

    /// Filter projects by language
    pub fn filter_by_language(&self, language: Language) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| p.language() == language)
            .collect()
    }

    /// Filter projects by architecture
    pub fn filter_by_architecture(&self, architecture: Architecture) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| p.architecture() == architecture)
            .collect()
    }

    /// Filter projects by status
    pub fn filter_by_status(&self, status: Status) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| p.status() == status)
            .collect()
    }

    /// Apply multiple filters
    pub fn apply_filters(
        &self,
        language: Option<Language>,
        architecture: Option<Architecture>,
        status: Option<Status>,
    ) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| {
                if let Some(lang) = language
                    && p.language() != lang
                {
                    return false;
                }
                if let Some(arch) = architecture
                    && p.architecture() != arch
                {
                    return false;
                }
                if let Some(stat) = status
                    && p.status() != stat
                {
                    return false;
                }
                true
            })
            .collect()
    }

    // ==================== Statistics Methods ====================

    /// Get portfolio statistics
    pub fn statistics(&self) -> PortfolioStatistics {
        let mut stats = PortfolioStatistics {
            total_projects: self.projects.len(),
            ..Default::default()
        };

        for project in &self.projects {
            // Count by status
            match project.status() {
                Status::Planned => stats.planned_count += 1,
                Status::InProgress => stats.in_progress_count += 1,
                Status::Completed => stats.completed_count += 1,
                Status::Archived => stats.archived_count += 1,
            }

            // Count by language
            *stats.language_counts.entry(project.language()).or_insert(0) += 1;

            // Count by architecture
            *stats
                .architecture_counts
                .entry(project.architecture())
                .or_insert(0) += 1;
        }

        stats
    }
}

/// Portfolio statistics
#[derive(Debug, Clone, Default)]
pub struct PortfolioStatistics {
    pub total_projects: usize,
    pub planned_count: usize,
    pub in_progress_count: usize,
    pub completed_count: usize,
    pub archived_count: usize,
    pub language_counts: HashMap<Language, usize>,
    pub architecture_counts: HashMap<Architecture, usize>,
}

impl PortfolioStatistics {
    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.total_projects == 0 {
            return 0.0;
        }
        (self.completed_count as f64 / self.total_projects as f64) * 100.0
    }

    /// Get the most used language
    pub fn most_used_language(&self) -> Option<Language> {
        self.language_counts
            .iter()
            .max_by_key(|&(_, &count)| count)
            .map(|(&lang, _)| lang)
    }

    /// Get the most used architecture
    pub fn most_used_architecture(&self) -> Option<Architecture> {
        self.architecture_counts
            .iter()
            .max_by_key(|&(_, &count)| count)
            .map(|(&arch, _)| arch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::ProjectName;
    use std::path::PathBuf;

    fn sample_project(name: &str, lang: Language, status: Status) -> Project {
        let mut project = Project::new(
            ProjectName::new(name).unwrap(),
            lang,
            Architecture::Hexagonal,
            PathBuf::from("/test"),
        );
        project.force_set_status(status);
        project
    }

    #[test]
    fn add_and_find_projects() {
        let mut portfolio = Portfolio::new();

        let project1 = sample_project("Project 1", Language::Rust, Status::InProgress);
        let id1 = project1.id().clone();

        assert!(portfolio.add_project(project1).is_ok());
        assert_eq!(portfolio.len(), 1);

        let found = portfolio.find_by_id(&id1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name().as_str(), "Project 1");
    }

    #[test]
    fn duplicate_project_rejected() {
        let mut portfolio = Portfolio::new();

        let project1 = sample_project("Project", Language::Rust, Status::Planned);
        let project2 = sample_project("Project", Language::Rust, Status::Planned);

        assert!(portfolio.add_project(project1).is_ok());
        assert!(portfolio.add_project(project2).is_err());
    }

    #[test]
    fn filter_projects() {
        let mut portfolio = Portfolio::new();

        portfolio
            .add_project(sample_project("Rust 1", Language::Rust, Status::Completed))
            .unwrap();
        portfolio
            .add_project(sample_project("Java 1", Language::Java, Status::Completed))
            .unwrap();
        portfolio
            .add_project(sample_project("Rust 2", Language::Rust, Status::InProgress))
            .unwrap();

        let rust_projects = portfolio.filter_by_language(Language::Rust);
        assert_eq!(rust_projects.len(), 2);

        let completed = portfolio.filter_by_status(Status::Completed);
        assert_eq!(completed.len(), 2);
    }

    #[test]
    fn portfolio_statistics() {
        let mut portfolio = Portfolio::new();

        portfolio
            .add_project(sample_project("Rust 1", Language::Rust, Status::Completed))
            .unwrap();
        portfolio
            .add_project(sample_project("Java 1", Language::Java, Status::Completed))
            .unwrap();
        portfolio
            .add_project(sample_project("Rust 2", Language::Rust, Status::InProgress))
            .unwrap();

        let stats = portfolio.statistics();
        assert_eq!(stats.total_projects, 3);
        assert_eq!(stats.completed_count, 2);
        assert_eq!(stats.in_progress_count, 1);
        assert_eq!(stats.most_used_language(), Some(Language::Rust));
        assert!((stats.completion_percentage() - 66.67).abs() < 0.1);
    }
}
