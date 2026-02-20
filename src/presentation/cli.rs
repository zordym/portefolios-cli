use crate::application::commands::{docs, list, new, open, run};
use crate::domain::config;
use clap::{Parser, Subcommand};

/// Architecture Portfolio CLI - Manage your portfolio projects
#[derive(Parser)]
#[command(name = "portfolio-cli")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool for managing architecture portfolio projects", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to custom configuration file
    #[arg(long, global = true)]
    pub config: Option<String>,

    /// Enable verbose output
    #[arg(long, short, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all projects in the portfolio
    List {
        /// Filter by language (kotlin, java, rust)
        #[arg(long)]
        language: Option<String>,

        /// Filter by architecture type
        #[arg(long)]
        architecture: Option<String>,

        /// Filter by status (planned, in-progress, completed)
        #[arg(long)]
        status: Option<String>,

        /// Output format (table, json, compact)
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Create a new project
    New {
        /// Project name
        #[arg(long)]
        name: String,

        /// Programming language (kotlin, java, rust)
        #[arg(long)]
        language: String,

        /// Architecture type (hexagonal, onion, layered, pipeline)
        #[arg(long)]
        architecture: String,

        /// Project description
        #[arg(long)]
        description: Option<String>,

        /// Interactive mode for guided setup
        #[arg(long, short)]
        interactive: bool,
    },

    /// Open a project in your editor
    Open {
        /// Project name or ID
        project: String,

        /// Editor to use (overrides config)
        #[arg(long)]
        editor: Option<String>,

        /// Open terminal instead of editor
        #[arg(long)]
        terminal: bool,
    },

    /// Run a project
    Run {
        /// Project name or ID
        project: String,

        /// Port to run on
        #[arg(long)]
        port: Option<u16>,

        /// Profile to use (dev, test, prod)
        #[arg(long)]
        profile: Option<String>,

        /// Enable debug mode
        #[arg(long)]
        debug: bool,
    },

    /// Generate documentation
    Docs {
        /// Output directory
        #[arg(long, default_value = "docs")]
        output: String,

        /// Format (markdown, html)
        #[arg(long, default_value = "markdown")]
        format: String,

        /// Include project details
        #[arg(long)]
        detailed: bool,
    },

    /// Initialize configuration file
    Init {
        /// Force overwrite existing config
        #[arg(long)]
        force: bool,
    },
}

pub fn parse_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Handle init command separately as it creates the config
    if let Commands::Init { force } = cli.command {
        return config::init_config(force);
    }

    // Load configuration
    let config = config::Config::load(cli.config.as_deref())?;

    if cli.verbose {
        println!("Configuration loaded from: {:?}", config.config_path());
    }

    // Execute commands
    match cli.command {
        Commands::List {
            language,
            architecture,
            status,
            format,
        } => {
            list::execute(&config, language, architecture, status, &format)?;
        }
        Commands::New {
            name,
            language,
            architecture,
            description,
            interactive,
        } => {
            new::execute(
                &config,
                &name,
                &language,
                &architecture,
                description.as_deref(),
                interactive,
            )?;
        }
        Commands::Open {
            project,
            editor,
            terminal,
        } => {
            open::execute(&config, &project, editor.as_deref(), terminal)?;
        }
        Commands::Run {
            project,
            port,
            profile,
            debug,
        } => {
            run::execute(&config, &project, port, profile.as_deref(), debug)?;
        }
        Commands::Docs {
            output,
            format,
            detailed,
        } => {
            docs::execute(&config, &output, &format, detailed)?;
        }
        Commands::Init { .. } => unreachable!(),
    }

    Ok(())
}
