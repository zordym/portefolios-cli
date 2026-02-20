use crate::config::Config;
use crate::utils;
use anyhow::Result;
use colored::Colorize;
use std::process::{Command, Stdio};

pub fn execute(
    config: &Config,
    project_name: &str,
    port: Option<u16>,
    profile: Option<&str>,
    debug: bool,
) -> Result<()> {
    let portfolio = utils::load_portfolio(config)?;
    let project = portfolio.find_by_name(project_name)
        .ok_or_else(|| anyhow::anyhow!("Project not found: {}", project_name))?;

    println!("\n{} Starting project: {}", "→".blue(), project.name.bold());

    match project.language {
        crate::models::Language::Kotlin | crate::models::Language::Java => {
            run_jvm_project(&project.path, port, profile, debug)?;
        },
        crate::models::Language::Rust => {
            run_rust_project(&project.path, port, debug)?;
        },
    }

    Ok(())
}

fn run_jvm_project(path: &str, port: Option<u16>, profile: Option<&str>, debug: bool) -> Result<()> {
    let project_path = std::path::Path::new(path);

    let mut cmd = if project_path.join("gradlew").exists() {
        let mut c = Command::new("./gradlew");
        c.arg("bootRun");
        c
    } else if project_path.join("mvnw").exists() {
        let mut c = Command::new("./mvnw");
        c.arg("spring-boot:run");
        c
    } else {
        anyhow::bail!("No build tool found (gradlew or mvnw)");
    };

    cmd.current_dir(project_path);

    if let Some(p) = port {
        cmd.env("SERVER_PORT", p.to_string());
    }

    if let Some(prof) = profile {
        cmd.env("SPRING_PROFILES_ACTIVE", prof);
    }

    if debug {
        cmd.env("JAVA_OPTS", "-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:5005");
        println!("{} Debug mode enabled on port 5005", "→".yellow());
    }

    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}

fn run_rust_project(path: &str, port: Option<u16>, debug: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run");

    if !debug {
        cmd.arg("--release");
    }

    cmd.current_dir(path);

    if let Some(p) = port {
        cmd.env("PORT", p.to_string());
    }

    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}