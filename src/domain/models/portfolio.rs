use crate::domain::models::project::Project;

#[derive(Debug, Default)]
pub struct Portfolio {
    pub projects: Vec<Project>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Project> {
        let search = name.to_lowercase();
        self.projects
            .iter()
            .find(|p| p.name.to_lowercase() == search || p.id.to_lowercase() == search)
    }

    pub fn apply_filters(
        &self,
        language: Option<String>,
        architecture: Option<String>,
        status: Option<String>,
    ) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| {
                if let Some(ref lang) = language {
                    if p.language.to_string().to_lowercase() != lang.to_lowercase() {
                        return false;
                    }
                }
                if let Some(ref arch) = architecture {
                    if p.architecture.to_string().to_lowercase() != arch.to_lowercase() {
                        return false;
                    }
                }
                if let Some(ref stat) = status {
                    if p.status.to_string().to_lowercase().replace(" ", "")
                        != stat.to_lowercase().replace("-", "").replace("_", "")
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
