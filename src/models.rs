use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Kotlin,
    Java,
    Rust,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Java => write!(f, "Java"),
            Language::Rust => write!(f, "Rust"),
        }
    }
}

impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kotlin" => Some(Language::Kotlin),
            "java" => Some(Language::Java),
            "rust" => Some(Language::Rust),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    Hexagonal,
    Onion,
    Layered,
    Pipeline,
    Microkernel,
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Architecture::Hexagonal => write!(f, "Hexagonal"),
            Architecture::Onion => write!(f, "Onion"),
            Architecture::Layered => write!(f, "Layered"),
            Architecture::Pipeline => write!(f, "Pipeline"),
            Architecture::Microkernel => write!(f, "Microkernel"),
        }
    }
}

impl Architecture {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hexagonal" => Some(Architecture::Hexagonal),
            "onion" => Some(Architecture::Onion),
            "layered" => Some(Architecture::Layered),
            "pipeline" => Some(Architecture::Pipeline),
            "microkernel" => Some(Architecture::Microkernel),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Planned,
    InProgress,
    Completed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Planned => write!(f, "Planned"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

impl Status {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace("-", "").replace("_", "").as_str() {
            "planned" => Some(Status::Planned),
            "inprogress" => Some(Status::InProgress),
            "completed" => Some(Status::Completed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: Language,
    pub architecture: Architecture,
    pub path: String,
    pub status: Status,
    pub frameworks: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab_url: Option<String>,

    #[serde(default)]
    pub created_at: DateTime<Utc>,

    #[serde(default)]
    pub updated_at: DateTime<Utc>,
}

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
        self.projects.iter().find(|p|
            p.name.to_lowercase() == search ||
                p.id.to_lowercase() == search
        )
    }

    pub fn apply_filters(
        &self,
        language: Option<String>,
        architecture: Option<String>,
        status: Option<String>,
    ) -> Vec<&Project> {
        self.projects.iter()
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
                    if p.status.to_string().to_lowercase().replace(" ", "") != stat.to_lowercase().replace("-", "").replace("_", "") {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
