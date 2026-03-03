use anyhow::Result;
use portfolio_cli::presentation::cli_application::CliApplication;

fn main() -> Result<()> {
    CliApplication::new().run()
}
