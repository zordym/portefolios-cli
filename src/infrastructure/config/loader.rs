use super::Config;
use crate::infrastructure::errors::{InfrastructureError, InfrastructureResult};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration loader
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from a file or use default
    pub fn load(custom_path: Option<&str>) -> InfrastructureResult<Config> {
        let config_path = Self::resolve_config_path(custom_path);

        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            Ok(Config::default())
        }
    }

    /// Load configuration from a specific file
    fn load_from_file(path: &Path) -> InfrastructureResult<Config> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        // Override with environment variables if present
        if let Ok(_) = std::env::var("GITLAB_TOKEN") {
            // Store token handling will be added later
            tracing::debug!("GitLab token loaded from environment");
        }

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(config: &Config, path: &Path) -> InfrastructureResult<()> {
        let content = toml::to_string_pretty(config)?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| InfrastructureError::Io(err))?;
        }

        fs::write(path, content).map_err(|err| InfrastructureError::Io(err))?;
        Ok(())
    }

    /// Resolve the configuration file path
    fn resolve_config_path(custom_path: Option<&str>) -> PathBuf {
        if let Some(path) = custom_path {
            return PathBuf::from(path);
        }

        // Try current directory first
        if let Ok(dir) = std::env::current_dir() {
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

    /// Get the default config path
    pub fn default_config_path() -> PathBuf {
        Self::resolve_config_path(None)
    }

    /// Initialize a new configuration file
    pub fn init_config(force: bool) -> InfrastructureResult<PathBuf> {
        let config_path = Self::default_config_path();

        if config_path.exists() && !force {
            return Err(InfrastructureError::ConfigNotFound(format!(
                "Configuration file already exists at: {}",
                config_path.display()
            )));
        }

        let config = Config::default();
        Self::save(&config, &config_path)?;

        Ok(config_path)
    }
}
