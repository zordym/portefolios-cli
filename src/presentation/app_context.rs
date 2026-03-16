use crate::application::DocumentationService;
use crate::infrastructure::FileSystemPortfolioRepository;
use crate::infrastructure::services::{GitService, SystemEditorService, SystemProcessRunner};
use crate::{ConfigStructure, PortfolioService, ProjectService};
use std::sync::Arc;

/// Application context holding all services
pub struct AppContext {
    pub config: ConfigStructure,
    pub project_service: Arc<ProjectService<FileSystemPortfolioRepository, GitService>>,
    pub portfolio_service: Arc<PortfolioService<FileSystemPortfolioRepository>>,
    pub documentation_service: Arc<DocumentationService>,
    pub editor_service: Arc<SystemEditorService>,
    pub process_runner: Arc<SystemProcessRunner>,
}

impl AppContext {
    pub fn new(config: ConfigStructure) -> Self {
        // Create infrastructure services
        let repository = Arc::new(FileSystemPortfolioRepository::new(&config.portfolio_root));
        let vcs = Arc::new(GitService::new());
        let editor_service = Arc::new(SystemEditorService::with_allowed_root(
            &config.portfolio_root,
        ));
        let process_runner = Arc::new(SystemProcessRunner::new());

        // Create application services
        let project_service = Arc::new(ProjectService::new(repository.clone(), vcs));
        let portfolio_service = Arc::new(PortfolioService::new(repository));
        let documentation_service = Arc::new(DocumentationService::new());

        Self {
            config,
            project_service,
            portfolio_service,
            documentation_service,
            editor_service,
            process_runner,
        }
    }
}
