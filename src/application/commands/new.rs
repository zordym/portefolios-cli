use crate::domain::config::Config;
use crate::domain::models::architecture::Architecture;
use crate::domain::models::language::Language;
use crate::domain::models::project::Project;
use crate::domain::models::status::Status;
use anyhow::{Context, Result};
use chrono::Utc;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn execute(
    config: &Config,
    name: &str,
    language: &str,
    architecture: &str,
    description: Option<&str>,
    interactive: bool,
) -> Result<()> {
    println!("\n{}", "Creating new project...".bold().blue());

    let lang = Language::from_str(language)
        .ok_or_else(|| anyhow::anyhow!("Invalid language: {}", language))?;

    let arch = Architecture::from_str(architecture)
        .ok_or_else(|| anyhow::anyhow!("Invalid architecture: {}", architecture))?;

    let project_id = name.to_lowercase().replace(" ", "-");
    let project_path = config.portfolio_root.join(&project_id);

    if project_path.exists() {
        anyhow::bail!(
            "Project directory already exists: {}",
            project_path.display()
        );
    }

    println!("→ Creating directory structure...");
    create_structure(&project_path, &arch, &lang)?;

    println!("→ Generating build configuration...");
    create_build_config(&project_path, &lang, &project_id)?;

    println!("→ Creating documentation...");
    create_documentation(&project_path, name, &arch, &lang, description)?;

    println!("→ Initializing Git repository...");
    init_git(&project_path)?;

    println!("→ Saving project metadata...");
    save_metadata(&project_path, name, &project_id, &lang, &arch, description)?;

    println!("\n{} Project created successfully!", "✓".green().bold());
    println!("\n  {}: {}", "Path".bold(), project_path.display());
    println!("  {}: cd {}", "Navigate".bold(), project_id);
    println!("  {}: arch-portfolio open {}", "Open".bold(), project_id);

    Ok(())
}

fn create_structure(path: &Path, arch: &Architecture, lang: &Language) -> Result<()> {
    fs::create_dir_all(path)?;

    let src_base = match lang {
        Language::Kotlin => path.join("src/main/kotlin"),
        Language::Java => path.join("src/main/java"),
        Language::Rust => path.join("../.."),
    };

    match arch {
        Architecture::Hexagonal => {
            fs::create_dir_all(src_base.join("domain/model"))?;
            fs::create_dir_all(src_base.join("domain/ports/input"))?;
            fs::create_dir_all(src_base.join("domain/ports/output"))?;
            fs::create_dir_all(src_base.join("application"))?;
            fs::create_dir_all(src_base.join("infrastructure/adapters"))?;
        }
        Architecture::Onion => {
            fs::create_dir_all(src_base.join("domain/core"))?;
            fs::create_dir_all(src_base.join("domain/services"))?;
            fs::create_dir_all(src_base.join("infrastructure"))?;
            fs::create_dir_all(src_base.join("presentation"))?;
        }
        Architecture::Layered => {
            fs::create_dir_all(src_base.join("presentation"))?;
            fs::create_dir_all(src_base.join("application"))?;
            fs::create_dir_all(src_base.join("domain"))?;
            fs::create_dir_all(src_base.join("infrastructure"))?;
        }
        Architecture::Pipeline => {
            fs::create_dir_all(src_base.join("ingestion"))?;
            fs::create_dir_all(src_base.join("processing"))?;
            fs::create_dir_all(src_base.join("output"))?;
        }
        Architecture::Microkernel => {
            fs::create_dir_all(src_base.join("core"))?;
            fs::create_dir_all(src_base.join("plugins"))?;
        }
    }

    fs::create_dir_all(path.join("../../../docs"))?;

    Ok(())
}

fn create_build_config(path: &Path, lang: &Language, project_id: &str) -> Result<()> {
    match lang {
        Language::Kotlin => {
            let build_gradle = format!(
                r#"plugins {{
    kotlin("jvm") version "1.9.22"
    kotlin("plugin.spring") version "1.9.22"
    id("org.springframework.boot") version "3.2.1"
}}

group = "com.portfolio"
version = "0.1.0"

repositories {{
    mavenCentral()
}}

dependencies {{
    implementation("org.springframework.boot:spring-boot-starter-webflux")
    implementation("org.jetbrains.kotlin:kotlin-reflect")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
}}
"#
            );
            fs::write(path.join("build.gradle.kts"), build_gradle)?;
            fs::write(
                path.join("settings.gradle.kts"),
                format!("rootProject.name = \"{}\"", project_id),
            )?;
        }
        Language::Java => {
            let pom = format!(
                r#"<?xml version="1.0"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.portfolio</groupId>
    <artifactId>{}</artifactId>
    <version>0.1.0</version>
</project>
"#,
                project_id
            );
            fs::write(path.join("pom.xml"), pom)?;
        }
        Language::Rust => {
            let cargo = format!(
                r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
                project_id
            );
            fs::write(path.join("../../../Cargo.toml"), cargo)?;
        }
    }
    Ok(())
}

fn create_documentation(
    path: &Path,
    name: &str,
    arch: &Architecture,
    lang: &Language,
    description: Option<&str>,
) -> Result<()> {
    let desc = description.unwrap_or("Project description");

    let readme = format!(
        r#"# {}

## Description

{}

## Architecture

This project follows **{}** architecture.

## Technology Stack

- **Language**: {}
- **Architecture**: {}

## Getting Started

### Prerequisites

Install required tools for {} development.

### Running

Instructions for running the project.

## Documentation

See `ARCHITECTURE.md` for detailed architecture documentation.
"#,
        name, desc, arch, lang, arch, lang
    );

    fs::write(path.join("README.md"), readme)?;

    let architecture_doc = format!(
        r#"# Architecture Documentation

## Overview

This project implements {} architecture.

## Design Decisions

Key architectural decisions and their rationale.

## Components

Description of main components and their responsibilities.
"#,
        arch
    );

    fs::write(path.join("ARCHITECTURE.md"), architecture_doc)?;

    Ok(())
}

fn init_git(path: &Path) -> Result<()> {
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(path)
        .output()
        .context("Failed to initialize git")?;

    let gitignore = match std::fs::read_to_string(path.join("../../../Cargo.toml")) {
        Ok(_) => "target/\n*.lock\n",
        Err(_) => "build/\n.gradle/\ntarget/\n*.class\n",
    };

    fs::write(path.join("../../../.gitignore"), gitignore)?;

    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(path)
        .output()?;

    std::process::Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(path)
        .output()?;

    Ok(())
}

fn save_metadata(
    path: &Path,
    name: &str,
    id: &str,
    lang: &Language,
    arch: &Architecture,
    description: Option<&str>,
) -> Result<()> {
    let now = Utc::now();

    let project = Project {
        id: id.to_string(),
        name: name.to_string(),
        description: description.unwrap_or("").to_string(),
        language: lang.clone(),
        architecture: arch.clone(),
        path: path.display().to_string(),
        status: Status::InProgress,
        frameworks: vec![],
        gitlab_url: None,
        created_at: now,
        updated_at: now,
    };

    let toml = toml::to_string_pretty(&project)?;
    fs::write(path.join("../../../project.toml"), toml)?;

    Ok(())
}
