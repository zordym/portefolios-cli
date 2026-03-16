use crate::application::ProjectService;
use crate::application::dto::CreateProjectCommand;
use crate::domain::{Architecture, Language};
use crate::infrastructure::config::ConfigStructure;
use crate::infrastructure::repositories::PortfolioRepository;
use crate::infrastructure::services::VersionControlService;
use anyhow::Result;
use colored::Colorize;

pub fn execute<R: PortfolioRepository, V: VersionControlService>(
    service: &ProjectService<R, V>,
    config: &ConfigStructure,
    name: String,
    language: String,
    architecture: String,
    description: Option<String>,
) -> Result<()> {
    println!("\n{}", "Creating new project...".bold().blue());

    // Parse language and architecture
    let lang: Language = language.parse()?;
    let arch: Architecture = architecture.parse()?;

    // Create command
    let mut command = CreateProjectCommand::new(name, lang, arch, config.portfolio_root.clone());

    if let Some(desc) = description {
        command = command.with_description(desc);
    }

    // Execute
    let project = service.create_project(command)?;

    // Success message
    println!("\n{} Project created successfully!", "✓".green().bold());
    println!("\n  {}: {}", "Name".bold(), project.name());
    println!("  {}: {}", "ID".bold(), project.id());
    println!("  {}: {}", "Path".bold(), project.path().display());
    println!("  {}: {}", "Language".bold(), project.language());
    println!("  {}: {}", "Architecture".bold(), project.architecture());
    println!("\n  {}: cd {}", "Navigate".bold(), project.id());
    println!("  {}: portfolio-cli open {}", "Open".bold(), project.id());

    Ok(())
}
