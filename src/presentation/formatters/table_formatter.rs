use super::ProjectFormatter;
use crate::domain::Project;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct ProjectRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Language")]
    language: String,
    #[tabled(rename = "Architecture")]
    architecture: String,
    #[tabled(rename = "Status")]
    status: String,
}

pub struct TableFormatter;

impl ProjectFormatter for TableFormatter {
    fn format(&self, projects: &[Project]) -> String {
        if projects.is_empty() {
            return "No projects found.".yellow().to_string();
        }

        let rows: Vec<ProjectRow> = projects
            .iter()
            .map(|p| ProjectRow {
                id: p.id().to_string(),
                name: p.name().to_string(),
                language: p.language().to_string(),
                architecture: p.architecture().to_string(),
                status: format!("{} {}", p.status().symbol(), p.status()),
            })
            .collect();

        let mut output = String::new();
        output.push_str(&Table::new(rows).with(Style::modern()).to_string());
        output.push_str(&format!("\n\n{}: {}", "Total".bold(), projects.len()));
        output
    }
}
