use super::ProjectFormatter;
use crate::domain::Project;

pub struct JsonFormatter;

impl ProjectFormatter for JsonFormatter {
    fn format(&self, projects: &[Project]) -> String {
        serde_json::to_string_pretty(projects)
            .unwrap_or_else(|e| format!("Error formatting JSON: {}", e))
    }
}
