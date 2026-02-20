use crate::config::Config;
use crate::utils;
use anyhow::Result;
use std::process::Command;

pub fn execute(
    config: &Config,
    project_name: &str,
    editor: Option<&str>,
    terminal: bool,
) -> Result<()> {
    let portfolio = utils::load_portfolio(config)?;
    let project = portfolio.find_by_name(project_name)
        .ok_or_else(|| anyhow::anyhow!("Project not found: {}", project_name))?;

    if terminal {
        open_terminal(&project.path)?;
    } else {
        let editor_cmd = editor.unwrap_or(&config.default_editor);
        open_in_editor(&project.path, editor_cmd)?;
    }

    Ok(())
}

fn open_in_editor(path: &str, editor: &str) -> Result<()> {
    Command::new(editor)
        .arg(path)
        .spawn()?;

    println!("Opening project in {}", editor);
    Ok(())
}

fn open_terminal(path: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    Command::new("open")
        .args(&["-a", "Terminal", path])
        .spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("gnome-terminal")
        .args(&["--working-directory", path])
        .spawn()?;

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(&["/c", "start", path])
        .spawn()?;

    Ok(())
}