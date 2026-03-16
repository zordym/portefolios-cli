use crate::Config;
use crate::infrastructure::config::ConfigLoader;
use crate::presentation::app_context::AppContext;
use crate::presentation::{Cli, Commands, commands};
use clap::Parser;
use tracing_subscriber::EnvFilter;

/// Main CLI application orchestrator
pub struct CliApplication {
    cli: Cli,
}

impl Default for CliApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl CliApplication {
    /// Creates a new CLI application instance
    pub fn new() -> Self {
        Self { cli: Cli::parse() }
    }

    /// Runs the CLI application
    pub fn run(self) -> anyhow::Result<()> {
        self.init_tracing();

        if let Some(()) = self.handle_init_command()? {
            return Ok(());
        }

        let config = self.load_config()?;
        let ctx = AppContext::new(config);

        self.execute_command(&ctx)
    }

    fn init_tracing(&self) {
        let filter = if self.cli.verbose {
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

    fn handle_init_command(&self) -> anyhow::Result<Option<()>> {
        if let Commands::Init { force } = self.cli.command {
            let config_path = ConfigLoader::init_config(force)?;
            println!("✓ Configuration initialized at: {}", config_path.display());
            println!("\nEdit this file to customize your portfolio settings.");
            return Ok(Some(()));
        }
        Ok(None)
    }

    fn load_config(&self) -> anyhow::Result<Config> {
        let config = ConfigLoader::load(self.cli.config.as_deref())?;

        if self.cli.verbose {
            println!(
                "Configuration loaded from: {:?}",
                ConfigLoader::default_config_path()
            );
        }

        Ok(config)
    }

    fn execute_command(self, ctx: &AppContext) -> anyhow::Result<()> {
        match self.cli.command {
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
                use crate::application::dto::ProjectQuery;

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
                use crate::application::dto::ProjectQuery;
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
                use crate::application::dto::UpdateProjectCommand;

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
}
