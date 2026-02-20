use crate::config::Config;
use crate::utils;
use anyhow::Result;
use std::fs;

pub fn execute(config: &Config, output: &str, format: &str, detailed: bool) -> Result<()> {
    let portfolio = utils::load_portfolio(config)?;

    println!("Generating documentation in {} format...", format);

    let output_dir = std::path::Path::new(output);
    fs::create_dir_all(output_dir)?;

    let mut doc_content = String::from("# Architecture Portfolio\n\n");
    doc_content.push_str("## Projects Overview\n\n");

    for project in &portfolio.projects {
        doc_content.push_str(&format!("### {}\n\n", project.name));
        doc_content.push_str(&format!("- **Language**: {}\n", project.language));
        doc_content.push_str(&format!("- **Architecture**: {}\n", project.architecture));
        doc_content.push_str(&format!("- **Status**: {}\n", project.status));

        if detailed {
            doc_content.push_str(&format!("\n{}\n", project.description));
            if !project.frameworks.is_empty() {
                doc_content.push_str(&format!("\n**Frameworks**: {}\n", project.frameworks.join(", ")));
            }
        }
        doc_content.push_str("\n---\n\n");
    }

    let output_file = output_dir.join("portfolio.md");
    fs::write(&output_file, doc_content)?;

    println!("✓ Documentation generated: {}", output_file.display());

    Ok(())
}