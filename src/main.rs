use anyhow::Result;
use clap::Parser;
use portfolio_cli::application::{DocumentationService, PortfolioService, ProjectService};
use portfolio_cli::infrastructure::config::{Config, ConfigLoader};
use portfolio_cli::infrastructure::repositories::FileSystemPortfolioRepository;
use portfolio_cli::infrastructure::services::{
    GitService, SystemEditorService, SystemProcessRunner,
};
use portfolio_cli::presentation::cli::{Cli, Commands};
use portfolio_cli::presentation::commands;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

/// Application context holding all services
struct AppContext {
    config: Config,
    project_service: Arc<ProjectService<FileSystemPortfolioRepository, GitService>>,
    portfolio_service: Arc<PortfolioService<FileSystemPortfolioRepository>>,
    documentation_service: Arc<DocumentationService>,
    editor_service: Arc<SystemEditorService>,
    process_runner: Arc<SystemProcessRunner>,
}

impl AppContext {
    fn new(config: Config) -> Self {
        // Create infrastructure services
        let repository = Arc::new(FileSystemPortfolioRepository::new(&config.portfolio_root));
        let vcs = Arc::new(GitService::new());
        let editor_service = Arc::new(SystemEditorService::with_allowed_root(
            &config.portfolio_root,
        ));
        let process_runner = Arc::new(SystemProcessRunner::new());

        // Create application services
        let project_service = Arc::new(ProjectService::new(repository.clone(), vcs));
        let portfolio_service = Arc::new(PortfolioService::new(repository));
        let documentation_service = Arc::new(DocumentationService::new());

        Self {
            config,
            project_service,
            portfolio_service,
            documentation_service,
            editor_service,
            process_runner,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    init_tracing(cli.verbose);

    // Handle init command separately (creates config)
    if let Commands::Init { force } = cli.command {
        let config_path = ConfigLoader::init_config(force)?;
        println!("✓ Configuration initialized at: {}", config_path.display());
        println!("\nEdit this file to customize your portfolio settings.");
        return Ok(());
    }

    // Load configuration
    let config = ConfigLoader::load(cli.config.as_deref())?;

    if cli.verbose {
        println!(
            "Configuration loaded from: {:?}",
            ConfigLoader::default_config_path()
        );
    }

    // Create application context
    let ctx = AppContext::new(config);

    // Route commands
    match cli.command {
        Commands::List {
            language,
            architecture,
            status,
            format,
            stats,
        } => {
            commands::list::execute(
                &ctx.portfolio_service,
                language,
                architecture,
                status,
                &format,
                stats,
            )?;
        }

        Commands::New {
            name,
            language,
            architecture,
            description,
        } => {
            commands::new::execute(
                &ctx.project_service,
                &ctx.config,
                name,
                language,
                architecture,
                description,
            )?;
        }

        Commands::Open {
            project,
            editor,
            terminal,
        } => {
            use portfolio_cli::application::dto::ProjectQuery;

            let proj = ctx.portfolio_service.get_project(ProjectQuery {
                identifier: project,
            })?;

            println!("✓ Opened project: {}", proj.name());
        }

        Commands::Run {
            project,
            port,
            profile,
            debug,
        } => {
            use portfolio_cli::application::dto::ProjectQuery;
            //use portfolio_cli::infrastructure::services::RunConfig;

            let proj = ctx.portfolio_service.get_project(ProjectQuery {
                identifier: project,
            })?;

            println!("→ Starting project: {}", proj.name());

            //let run_config = RunConfig {
            //    port,
            //    profile,
            //    debug,
            //};
            //
            //ctx.process_runner.run(proj.path(), proj.language(), &run_config)?;
        }

        Commands::Docs { output, detailed } => {
            let portfolio = ctx.portfolio_service.all_projects()?;
            ctx.documentation_service.generate_documentation(
                &portfolio,
                std::path::Path::new(&output),
                detailed,
            )?;

            println!("✓ Documentation generated in: {}", output);
        }

        Commands::Update {
            project,
            description,
            status,
            add_framework,
            remove_framework,
            gitlab_url,
        } => {
            use portfolio_cli::application::dto::UpdateProjectCommand;

            let mut command = UpdateProjectCommand::new(project);
            command.description = description;
            command.status = status;
            command.add_frameworks = add_framework;
            command.remove_frameworks = remove_framework;
            command.gitlab_url = gitlab_url;

            let proj = ctx.project_service.update_project(command)?;
            println!("✓ Updated project: {}", proj.name());
        }

        Commands::Delete { project, yes } => {
            if !yes {
                use std::io::{self, Write};
                print!("Are you sure you want to delete '{}'? [y/N]: ", project);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }

            ctx.project_service.delete_project(&project)?;
            println!("✓ Deleted project: {}", project);
        }

        Commands::Stats => {
            let stats = ctx.portfolio_service.get_statistics()?;

            use colored::Colorize;
            println!("\n{}", "Portfolio Statistics".bold().blue());
            println!("━━━━━━━━━━━━━━━━━━━━");
            println!("Total Projects: {}", stats.total_projects);
            println!();
            println!("By Status:");
            println!(
                "  Completed: {} ({:.1}%)",
                stats.completed_count,
                stats.completion_percentage()
            );
            println!("  In Progress: {}", stats.in_progress_count);
            println!("  Planned: {}", stats.planned_count);
            if stats.archived_count > 0 {
                println!("  Archived: {}", stats.archived_count);
            }

            if let Some(lang) = stats.most_used_language() {
                println!("\nMost Used Language: {}", lang);
            }

            if let Some(arch) = stats.most_used_architecture() {
                println!("Most Used Architecture: {}", arch);
            }
        }

        Commands::Init { .. } => unreachable!(),
    }

    Ok(())
}

fn init_tracing(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("portfolio_cli=debug,info")
    } else {
        EnvFilter::new("portfolio_cli=warn")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}
