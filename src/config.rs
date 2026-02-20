use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Root directory containing portfolio projects
    pub portfolio_root: PathBuf,

    /// GitLab instance URL
    pub gitlab_url: String,

    /// GitLab personal access token (optional, can use env var)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab_token: Option<String>,

    /// Directory containing documentation templates
    pub templates_dir: PathBuf,

    /// Default editor command
    pub default_editor: String,

    /// Cache directory for generated files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<PathBuf>,
}

impl Config {
    /// Load configuration from file or create default
    pub fn load(custom_path: Option<&str>) -> Result<Self> {
        let config_path = Self::resolve_config_path(custom_path);

        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            Ok(Self::default())
        }
    }

    /// Load configuration from a specific file
    fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read configuration file")?;

        let mut config: Config = toml::from_str(&content)
            .context("Failed to parse configuration file")?;

        // Override with environment variables if present
        if let Ok(token) = std::env::var("GITLAB_TOKEN") {
            config.gitlab_token = Some(token);
        }

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, content)
            .context("Failed to write configuration file")?;

        Ok(())
    }

    /// Resolve the configuration file path
    fn resolve_config_path(custom_path: Option<&str>) -> PathBuf {
        if let Some(path) = custom_path {
            return PathBuf::from(path);
        }

        // Try current directory first
        let current_dir = std::env::current_dir().ok();
        if let Some(dir) = current_dir {
            let local_config = dir.join("portfolio.toml");
            if local_config.exists() {
                return local_config;
            }
        }

        // Fall back to user config directory
        if let Some(config_dir) = dirs::config_dir() {
            return config_dir.join("portfolio-cli").join("portfolio.toml");
        }

        // Last resort: current directory
        PathBuf::from("portfolio.toml")
    }

    /// Get the path where config was loaded from
    pub fn config_path(&self) -> PathBuf {
        Self::resolve_config_path(None)
    }
}

impl Default for Config {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        Self {
            portfolio_root: current_dir.clone(),
            gitlab_url: "https://gitlab.com".to_string(),
            gitlab_token: std::env::var("GITLAB_TOKEN").ok(),
            templates_dir: current_dir.join("templates"),
            default_editor: std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string()),
            cache_dir: Some(current_dir.join(".cache")),
        }
    }
}

/// Initialize a new configuration file
pub fn init_config(force: bool) -> Result<()> {
    let config_path = Config::resolve_config_path(None);

    if config_path.exists() && !force {
        anyhow::bail!(
            "Configuration file already exists at: {}\nUse --force to overwrite",
            config_path.display()
        );
    }

    let config = Config::default();
    config.save(&config_path)?;

    println!("✓ Configuration initialized at: {}", config_path.display());
    println!("\nEdit this file to customize your portfolio settings.");

    Ok(())
}