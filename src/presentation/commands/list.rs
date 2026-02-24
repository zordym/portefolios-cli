use crate::application::PortfolioService;
use crate::application::dto::ListProjectsQuery;
use crate::infrastructure::PortfolioRepository;
use crate::presentation::formatters::FormatterFactory;
use anyhow::Result;
use colored::Colorize;

pub fn execute<R: PortfolioRepository>(
    service: &PortfolioService<R>,
    language: Option<String>,
    architecture: Option<String>,
    status: Option<String>,
    format: &str,
    show_stats: bool,
) -> Result<()> {
    let query = ListProjectsQuery {
        language,
        architecture,
        status,
    };

    let projects = service.list_projects(query)?;

    // Show statistics if requested
    if show_stats {
        let stats = service.get_statistics()?;
        println!("\n{}", "Portfolio Statistics".bold().blue());
        println!("━━━━━━━━━━━━━━━━━━━━");
        println!("Total Projects: {}", stats.total_projects);
        println!(
            "Completed: {} ({:.1}%)",
            stats.completed_count,
            stats.completion_percentage()
        );
        println!("In Progress: {}", stats.in_progress_count);
        println!("Planned: {}", stats.planned_count);
        if stats.archived_count > 0 {
            println!("Archived: {}", stats.archived_count);
        }
        println!();
    }

    // Format and display projects
    let formatter = FormatterFactory::create(format);
    formatter.print(&projects);

    Ok(())
}
