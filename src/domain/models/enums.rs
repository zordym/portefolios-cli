use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Programming language for a project
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Kotlin,
    Java,
    Rust,
}

impl Language {
    /// Get all available languages
    pub fn all() -> &'static [Language] {
        &[Language::Kotlin, Language::Java, Language::Rust]
    }

    /// Get the default file extension for this language
    pub fn file_extension(&self) -> &'static str {
        match self {
            Language::Kotlin => "kt",
            Language::Java => "java",
            Language::Rust => "rs",
        }
    }

    /// Get the typical source directory structure
    pub fn source_directory(&self) -> &'static str {
        match self {
            Language::Kotlin => "src/main/kotlin",
            Language::Java => "src/main/java",
            Language::Rust => "src",
        }
    }

    /// Get the build file name for this language
    pub fn build_file(&self) -> &'static str {
        match self {
            Language::Kotlin | Language::Java => "build.gradle.kts",
            Language::Rust => "Cargo.toml",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Java => write!(f, "Java"),
            Language::Rust => write!(f, "Rust"),
        }
    }
}

impl FromStr for Language {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kotlin" | "kt" => Ok(Language::Kotlin),
            "java" => Ok(Language::Java),
            "rust" | "rs" => Ok(Language::Rust),
            _ => Err(DomainError::InvalidLanguage(s.to_string())),
        }
    }
}

/// Software architecture pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    Hexagonal,
    Onion,
    Layered,
    Pipeline,
    Microkernel,
}

impl Architecture {
    /// Get all available architectures
    pub fn all() -> &'static [Architecture] {
        &[
            Architecture::Hexagonal,
            Architecture::Onion,
            Architecture::Layered,
            Architecture::Pipeline,
            Architecture::Microkernel,
        ]
    }

    /// Get a brief description of this architecture
    pub fn description(&self) -> &'static str {
        match self {
            Architecture::Hexagonal => "Ports and Adapters pattern with domain at the center",
            Architecture::Onion => "Concentric layers with dependencies pointing inward",
            Architecture::Layered => "Traditional N-tier architecture with clear layer separation",
            Architecture::Pipeline => "Data flows through sequential processing stages",
            Architecture::Microkernel => "Minimal core with plugin-based extensions",
        }
    }

    /// Get typical layer names for this architecture
    pub fn typical_layers(&self) -> &'static [&'static str] {
        match self {
            Architecture::Hexagonal => &[
                "domain/model",
                "domain/ports/input",
                "domain/ports/output",
                "application",
                "infrastructure/adapters",
            ],
            Architecture::Onion => &[
                "domain/core",
                "domain/services",
                "infrastructure",
                "presentation",
            ],
            Architecture::Layered => &["presentation", "application", "domain", "infrastructure"],
            Architecture::Pipeline => &["ingestion", "processing", "output"],
            Architecture::Microkernel => &["core", "plugins"],
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::Hexagonal => write!(f, "Hexagonal"),
            Architecture::Onion => write!(f, "Onion"),
            Architecture::Layered => write!(f, "Layered"),
            Architecture::Pipeline => write!(f, "Pipeline"),
            Architecture::Microkernel => write!(f, "Microkernel"),
        }
    }
}

impl FromStr for Architecture {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hexagonal" | "ports-and-adapters" => Ok(Architecture::Hexagonal),
            "onion" => Ok(Architecture::Onion),
            "layered" | "n-tier" => Ok(Architecture::Layered),
            "pipeline" | "pipes-and-filters" => Ok(Architecture::Pipeline),
            "microkernel" | "plug-in" => Ok(Architecture::Microkernel),
            _ => Err(DomainError::InvalidArchitecture(s.to_string())),
        }
    }
}

/// Project status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Planned,
    InProgress,
    Completed,
    Archived,
}

impl Status {
    /// Get all available statuses
    pub fn all() -> &'static [Status] {
        &[
            Status::Planned,
            Status::InProgress,
            Status::Completed,
            Status::Archived,
        ]
    }

    /// Check if this status can transition to another status
    pub fn can_transition_to(&self, target: Status) -> bool {
        use Status::*;

        matches!(
            (self, target),
            // From Planned
            (Planned, InProgress) | (Planned, Archived) |
            // From InProgress
            (InProgress, Completed) | (InProgress, Planned) | (InProgress, Archived) |
            // From Completed
            (Completed, Archived) | (Completed, InProgress) |
            // From Archived
            (Archived, InProgress) | (Archived, Planned)
        )
    }

    /// Get a display symbol for this status
    pub fn symbol(&self) -> &'static str {
        match self {
            Status::Planned => "○",
            Status::InProgress => "◐",
            Status::Completed => "✓",
            Status::Archived => "📦",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Planned => write!(f, "Planned"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
            Status::Archived => write!(f, "Archived"),
        }
    }
}

impl FromStr for Status {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.to_lowercase().replace(['-', '_', ' '], "");
        match normalized.as_str() {
            "planned" => Ok(Status::Planned),
            "inprogress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "archived" => Ok(Status::Archived),
            _ => Err(DomainError::InvalidStatus(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_from_str() {
        assert_eq!("kotlin".parse::<Language>().unwrap(), Language::Kotlin);
        assert_eq!("java".parse::<Language>().unwrap(), Language::Java);
        assert_eq!("rust".parse::<Language>().unwrap(), Language::Rust);
        assert!("python".parse::<Language>().is_err());
    }

    #[test]
    fn architecture_from_str() {
        assert_eq!(
            "hexagonal".parse::<Architecture>().unwrap(),
            Architecture::Hexagonal
        );
        assert!("unknown".parse::<Architecture>().is_err());
    }

    #[test]
    fn status_transitions() {
        assert!(Status::Planned.can_transition_to(Status::InProgress));
        assert!(Status::InProgress.can_transition_to(Status::Completed));
        assert!(!Status::Planned.can_transition_to(Status::Completed));
    }
}
