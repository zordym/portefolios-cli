use super::ProjectFormatter;
use crate::domain::Project;
use colored::Colorize;

pub struct CompactFormatter;

impl ProjectFormatter for CompactFormatter {
    fn format(&self, projects: &[Project]) -> String {
        if projects.is_empty() {
            return "No projects found.".yellow().to_string();
        }

        let mut output = String::new();

        for project in projects {
            output.push_str(&format!(
                "{} ({}) - {} [{}]\n",
                project.name().to_string().bold(),
                project.language(),
                project.architecture(),
                format!("{} {}", project.status().symbol(), project.status())
            ));
        }

        output.push_str(&format!("\n{}: {}", "Total".bold(), projects.len()));
        output
    }
}
