use crate::infrastructure::errors::{InfrastructureError, InfrastructureResult};
use std::path::Path;
use std::process::Command;

/// Trait for editor operations
pub trait EditorService: Send + Sync {
    /// Open a path in an editor
    fn open(&self, path: &Path, editor: &str) -> InfrastructureResult<()>;
}

/// System editor service implementation
pub struct SystemEditorService {
    allowed_root: Option<std::path::PathBuf>,
}

impl SystemEditorService {
    pub fn new() -> Self {
        Self { allowed_root: None }
    }

    /// Create with path validation
    pub fn with_allowed_root(root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            allowed_root: Some(root.into()),
        }
    }

    /// Validate that the path is within allowed boundaries
    fn validate_path(&self, path: &Path) -> InfrastructureResult<()> {
        if let Some(ref allowed_root) = self.allowed_root {
            let canonical = path
                .canonicalize()
                .map_err(|e| InfrastructureError::InvalidPath(e.to_string()))?;

            if !canonical.starts_with(allowed_root) {
                return Err(InfrastructureError::InvalidPath(format!(
                    "Path {} is outside allowed root {}",
                    canonical.display(),
                    allowed_root.display()
                )));
            }
        }

        Ok(())
    }
}

impl Default for SystemEditorService {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorService for SystemEditorService {
    fn open(&self, path: &Path, editor: &str) -> InfrastructureResult<()> {
        // Validate a path
        self.validate_path(path)?;

        // Launch editor (non-blocking)
        Command::new(editor)
            .arg(path)
            .spawn()
            .map_err(|e| InfrastructureError::EditorError(e.to_string()))?;

        tracing::info!("Opened {} in {}", path.display(), editor);
        Ok(())
    }
}

/// Terminal service for opening terminals
#[allow(dead_code)]
pub trait TerminalService: Send + Sync {
    /// Open a terminal in a directory
    fn open(&self, path: &Path) -> InfrastructureResult<()>;
}
#[allow(dead_code)]
pub struct SystemTerminalService;

impl SystemTerminalService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemTerminalService {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalService for SystemTerminalService {
    fn open(&self, path: &Path) -> InfrastructureResult<()> {
        Command::new("gnome-terminal")
            .arg("--working-directory")
            .arg(path)
            .spawn()
            .map_err(|e| InfrastructureError::ProcessError(e.to_string()))?;

        tracing::info!("Opened terminal at: {}", path.display());
        Ok(())
    }
}
