//! Presentation layer - CLI interface
//!
//! This layer contains:
//! - CLI argument parsing definitions
//! - Thin command handlers
//! - Output formatters

pub mod app_context;
pub mod cli;
pub mod cli_application;
pub mod commands;
pub mod formatters;

// Re-export commonly used types
pub use cli::{Cli, Commands};
