use crate::domain::models::architecture::Architecture;
use crate::domain::models::language::Language;
use crate::domain::models::status::Status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
