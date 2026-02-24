//! Presentation layer - CLI interface
//!
//! This layer contains:
//! - CLI argument parsing definitions
//! - Thin command handlers
//! - Output formatters

pub mod cli;
pub mod commands;
pub mod formatters;

// Re-export commonly used types
pub use cli::{Cli, Commands};
