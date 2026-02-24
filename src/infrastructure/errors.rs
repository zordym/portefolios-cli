use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration not found at: {0}")]
    ConfigNotFound(String),

    #[error("Project metadata not found at: {0}")]
    ProjectMetadataNotFound(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Git command failed: {0}")]
    GitError(String),

    #[error("Editor command failed: {0}")]
    EditorError(String),

    #[error("Process execution failed: {0}")]
    ProcessError(String),

    #[error("Keyring error: {0}")]
    KeyringError(String),
}

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;
