use crate::infrastructure::errors::{InfrastructureError, InfrastructureResult};
use std::path::Path;
use std::process::Command;

/// Trait for version control operations
pub trait VersionControlService: Send + Sync {
    /// Initialize a new repository
    fn init(&self, path: &Path) -> InfrastructureResult<()>;

    /// Add files to staging
    fn add(&self, path: &Path, files: &[&str]) -> InfrastructureResult<()>;

    /// Create a commit
    fn commit(&self, path: &Path, message: &str) -> InfrastructureResult<()>;

    /// Check if a directory is a git repository
    fn is_repository(&self, path: &Path) -> bool;
}

/// Git implementation of version control service
pub struct GitService;

impl GitService {
    pub fn new() -> Self {
        Self
    }

    /// Execute a git command
    fn execute_git(
        &self,
        path: &Path,
        args: &[&str],
    ) -> InfrastructureResult<std::process::Output> {
        let output = Command::new("git").args(args).current_dir(path).output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(InfrastructureError::GitError(error.to_string()));
        }

        Ok(output)
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionControlService for GitService {
    fn init(&self, path: &Path) -> InfrastructureResult<()> {
        self.execute_git(path, &["init"])?;
        tracing::info!("Initialized git repository at: {}", path.display());
        Ok(())
    }

    fn add(&self, path: &Path, files: &[&str]) -> InfrastructureResult<()> {
        let mut args = vec!["add"];
        args.extend_from_slice(files);
        self.execute_git(path, &args)?;
        Ok(())
    }

    fn commit(&self, path: &Path, message: &str) -> InfrastructureResult<()> {
        self.execute_git(path, &["commit", "-m", message])?;
        tracing::info!("Created commit: {}", message);
        Ok(())
    }

    fn is_repository(&self, path: &Path) -> bool {
        path.join(".git").is_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_git_init() {
        let temp_dir = TempDir::new().unwrap();
        let git = GitService::new();

        assert!(!git.is_repository(temp_dir.path()));

        git.init(temp_dir.path()).unwrap();

        assert!(git.is_repository(temp_dir.path()));
    }
}
