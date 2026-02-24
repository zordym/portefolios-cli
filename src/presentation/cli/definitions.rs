use clap::{Parser, Subcommand};

/// Architecture Portfolio CLI - Manage your portfolio projects
#[derive(Parser, Debug)]
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

#[derive(Subcommand, Debug)]
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

        /// Show statistics
        #[arg(long)]
        stats: bool,
    },

    /// Create a new project
    New {
        /// Project name
        name: String,

        /// Programming language (kotlin, java, rust)
        #[arg(long)]
        language: String,

        /// Architecture type (hexagonal, onion, layered, pipeline, microkernel)
        #[arg(long)]
        architecture: String,

        /// Project description
        #[arg(long)]
        description: Option<String>,
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

        /// Include detailed project information
        #[arg(long)]
        detailed: bool,
    },

    /// Initialize configuration file
    Init {
        /// Force overwrite existing config
        #[arg(long)]
        force: bool,
    },

    /// Update a project
    Update {
        /// Project name or ID
        project: String,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New status
        #[arg(long)]
        status: Option<String>,

        /// Add framework
        #[arg(long)]
        add_framework: Vec<String>,

        /// Remove framework
        #[arg(long)]
        remove_framework: Vec<String>,

        /// GitLab URL
        #[arg(long)]
        gitlab_url: Option<String>,
    },

    /// Delete a project
    Delete {
        /// Project name or ID
        project: String,

        /// Skip confirmation
        #[arg(long)]
        yes: bool,
    },

    /// Show portfolio statistics
    Stats,
}
