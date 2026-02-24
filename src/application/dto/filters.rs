use crate::domain::{Architecture, Language, Status};

/// Filters for project queries
#[derive(Debug, Clone, Default)]
pub struct ProjectFilters {
    pub language: Option<Language>,
    pub architecture: Option<Architecture>,
    pub status: Option<Status>,
}

impl ProjectFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn with_architecture(mut self, architecture: Architecture) -> Self {
        self.architecture = Some(architecture);
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }
}
