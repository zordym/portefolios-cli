use crate::domain::config::Config;
use crate::domain::models::portfolio::Portfolio;
use crate::domain::models::project::Project;
use anyhow::Result;
use std::fs;
use walkdir::WalkDir;

pub fn load_portfolio(config: &Config) -> Result<Portfolio> {
    let mut portfolio = Portfolio::new();

    for entry in WalkDir::new(&config.portfolio_root)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "project.toml" {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(project) = toml::from_str::<Project>(&content) {
                    portfolio.projects.push(project);
                }
            }
        }
    }

    Ok(portfolio)
}
