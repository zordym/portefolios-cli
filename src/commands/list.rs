use crate::config::Config;
use crate::utils;
use anyhow::Result;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct ProjectRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Language")]
    language: String,
    #[tabled(rename = "Architecture")]
    architecture: String,
    #[tabled(rename = "Status")]
    status: String,
}

pub fn execute(
    config: &Config,
    language: Option<String>,
    architecture: Option<String>,
    status: Option<String>,
    format: &str,
) -> Result<()> {
    let portfolio = utils::load_portfolio(config)?;
    let projects = portfolio.apply_filters(language, architecture, status);

    match format {
        "json" => print_json(&projects)?,
        "compact" => print_compact(&projects),
        _ => print_table(&projects),
    }

    Ok(())
}

fn print_table(projects: &[&crate::models::Project]) {
    if projects.is_empty() {
        println!("{}", "No projects found matching the filters.".yellow());
        return;
    }

    let rows: Vec<ProjectRow> = projects.iter().map(|p| ProjectRow {
        name: p.name.clone(),
        language: p.language.to_string(),
        architecture: p.architecture.to_string(),
        status: format_status(&p.status),
    }).collect();

    let table = Table::new(rows).with(Style::modern()).to_string();
    println!("\n{}", table);
    println!("\n{}: {}", "Total".bold(), projects.len());
}

fn print_json(projects: &[&crate::models::Project]) -> Result<()> {
    let json = serde_json::to_string_pretty(projects)?;
    println!("{}", json);
    Ok(())
}

fn print_compact(projects: &[&crate::models::Project]) {
    if projects.is_empty() {
        println!("{}", "No projects found.".yellow());
        return;
    }

    for project in projects {
        println!("{} ({}) - {} [{}]",
                 project.name.bold(),
                 project.language,
                 project.architecture,
                 format_status(&project.status)
        );
    }
    println!("\n{}: {}", "Total".bold(), projects.len());
}

fn format_status(status: &crate::models::Status) -> String {
    match status {
        crate::models::Status::Completed => "✓ Completed".to_string(),
        crate::models::Status::InProgress => "◐ In Progress".to_string(),
        crate::models::Status::Planned => "○ Planned".to_string(),
    }
}
