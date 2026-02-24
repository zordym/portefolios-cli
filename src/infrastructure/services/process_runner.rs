use crate::domain::Language;
use crate::infrastructure::errors::{InfrastructureError, InfrastructureResult};
use std::path::Path;
use std::process::{Command, Stdio};

/// Trait for running project processes
pub trait ProcessRunner: Send + Sync {
    /// Run a project with the given configuration
    fn run(
        &self,
        project_path: &Path,
        language: Language,
        config: &RunConfig,
    ) -> InfrastructureResult<()>;
}

/// Configuration for running a project
#[derive(Debug, Clone, Default)]
pub struct RunConfig {
    pub port: Option<u16>,
    pub profile: Option<String>,
    pub debug: bool,
}

/// System process runner implementation
pub struct SystemProcessRunner;

impl SystemProcessRunner {
    pub fn new() -> Self {
        Self
    }

    /// Run a JVM-based project (Kotlin/Java)
    fn run_jvm(&self, project_path: &Path, config: &RunConfig) -> InfrastructureResult<()> {
        let mut cmd = if project_path.join("gradlew").exists() {
            let mut c = Command::new("./gradlew");
            c.arg("bootRun");
            c
        } else if project_path.join("mvnw").exists() {
            let mut c = Command::new("./mvnw");
            c.arg("spring-boot:run");
            c
        } else {
            return Err(InfrastructureError::ProcessError(
                "No build tool found (gradlew or mvnw)".to_string(),
            ));
        };

        cmd.current_dir(project_path);

        if let Some(port) = config.port {
            cmd.env("SERVER_PORT", port.to_string());
        }

        if let Some(ref profile) = config.profile {
            cmd.env("SPRING_PROFILES_ACTIVE", profile);
        }

        if config.debug {
            cmd.env(
                "JAVA_OPTS",
                "-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:5005",
            );
            tracing::info!("Debug mode enabled on port 5005");
        }

        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|e| InfrastructureError::ProcessError(e.to_string()))?;

        Ok(())
    }

    /// Run a Rust project
    fn run_rust(&self, project_path: &Path, config: &RunConfig) -> InfrastructureResult<()> {
        let mut cmd = Command::new("cargo");
        cmd.arg("run");

        if !config.debug {
            cmd.arg("--release");
        }

        cmd.current_dir(project_path);

        if let Some(port) = config.port {
            cmd.env("PORT", port.to_string());
        }

        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|e| InfrastructureError::ProcessError(e.to_string()))?;

        Ok(())
    }
}

impl Default for SystemProcessRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessRunner for SystemProcessRunner {
    fn run(
        &self,
        project_path: &Path,
        language: Language,
        config: &RunConfig,
    ) -> InfrastructureResult<()> {
        tracing::info!(
            "Running project at: {} with language: {}",
            project_path.display(),
            language
        );

        match language {
            Language::Kotlin | Language::Java => self.run_jvm(project_path, config),
            Language::Rust => self.run_rust(project_path, config),
        }
    }
}
