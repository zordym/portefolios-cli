use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStructure {
    /// Root directory containing portfolio projects
    pub portfolio_root: PathBuf,

    /// GitLab instance URL
    pub gitlab_url: String,

    /// Directory containing documentation templates
    pub templates_dir: PathBuf,

    /// Default editor command
    pub default_editor: String,

    /// Cache directory for generated files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<PathBuf>,
}

impl ConfigStructure {
    /// Create a default configuration
    pub fn default_config() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        Self {
            portfolio_root: current_dir.clone(),
            gitlab_url: "https://gitlab.com".to_string(),
            templates_dir: current_dir.join("templates"),
            default_editor: std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string()),
            cache_dir: Some(current_dir.join(".cache")),
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if !self.portfolio_root.exists() {
            return Err(format!(
                "Portfolio root does not exist: {}",
                self.portfolio_root.display()
            ));
        }

        Ok(())
    }
}

impl Default for ConfigStructure {
    fn default() -> Self {
        Self::default_config()
    }
}
