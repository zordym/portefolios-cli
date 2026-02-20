use serde::{Deserialize, Serialize};

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
