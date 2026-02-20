mod application;
mod domain;

mod presentation;

use anyhow::Result;

use crate::presentation::cli::parse_cli;

fn main() -> Result<()> {
    parse_cli()
}
