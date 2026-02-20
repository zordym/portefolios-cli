# Portfolio CLI - Deep Architecture Analysis

## Executive Summary

This is a Rust-based CLI tool designed to manage architecture portfolio projects across multiple languages (Kotlin,
Java, Rust) and architectural patterns (Hexagonal, Onion, Layered, Pipeline, Microkernel). The current implementation
follows a **procedural, command-driven architecture** with basic modularity.

**Current State**: Functional but monolithic in structure
**Recommended Architecture**: **Domain-Driven Layered Architecture with Repository Pattern**
**Overall Assessment**: Good foundation, needs architectural refactoring for scalability and testability

---

## Current Architecture Analysis

### 1. Project Structure

```
src/
├── main.rs           # Entry point + CLI argument parsing
├── config.rs         # Configuration management
├── models.rs         # Domain models (Project, Portfolio, enums)
├── utils.rs          # Utility functions (portfolio loading)
└── commands/         # Command implementations
    ├── mod.rs
    ├── list.rs
    ├── new.rs
    ├── open.rs
    ├── run.rs
    └── docs.rs
```

### 2. Current Architecture Pattern

**Classification**: **Procedural Layered Architecture** (partial implementation)

The codebase exhibits:

- ✅ **Presentation Layer**: CLI interface via `clap` (main.rs)
- ⚠️ **Business Logic**: Mixed between commands and utils
- ⚠️ **Data Access**: Tightly coupled file system operations in utils
- ❌ **Domain Layer**: Models exist but lack behavior
- ❌ **Separation of Concerns**: Weak boundaries between layers

### 3. Key Architectural Issues

#### 3.1 Tight Coupling

- **Location**: `utils.rs:7-25`
- **Issue**: Direct file system access in utility function
- **Impact**: Cannot mock or test portfolio loading independently

```rust
pub fn load_portfolio(config: &Config) -> Result<Portfolio> {
    // Direct WalkDir coupling - hard to test
    for entry in WalkDir::new(&config.portfolio_root)
```

#### 3.2 Anemic Domain Model

- **Location**: `models.rs:118-163`
- **Issue**: `Portfolio` struct has minimal behavior, mostly data container
- **Impact**: Business logic scattered across command modules

#### 3.3 Missing Abstraction Layers

- No repository/persistence abstraction
- No service layer for business logic
- Commands directly call utilities and file system operations

#### 3.4 Error Handling Strategy

- **Current**: Uses `anyhow::Result` everywhere
- **Issue**: Generic errors provide poor error context
- **Location**: Throughout codebase (main.rs, commands/*.rs)

#### 3.5 Configuration Management

- **Location**: `config.rs:29-55`
- **Issue**: Config has dual responsibility (loading + data)
- **Missing**: Environment-specific configuration strategies

#### 3.6 Limited Testability

- No dependency injection
- Hard-coded external dependencies (file system, git commands)
- No trait abstractions for swappable implementations

---

## Recommended Architecture: Domain-Driven Layered Architecture

### Overview

```
┌─────────────────────────────────────────────────┐
│          Presentation Layer (CLI)               │
│  • main.rs - CLI parsing & routing              │
│  • commands/*.rs - Command handlers             │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│          Application Layer (Services)           │
│  • ProjectService - Project CRUD operations     │
│  • PortfolioService - Portfolio management      │
│  • DocumentationService - Doc generation        │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│          Domain Layer (Business Logic)          │
│  • Models with behavior (Project, Portfolio)    │
│  • Domain services                              │
│  • Value objects (Language, Architecture)       │
│  • Domain events                                │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│     Infrastructure Layer (Persistence)          │
│  • Repository traits (PortfolioRepository)      │
│  • Repository implementations (FileSystem)      │
│  • External services (Git, Editor)              │
│  • Configuration providers                      │
└─────────────────────────────────────────────────┘
```

### Why This Architecture?

1. **Clear Separation of Concerns**: Each layer has distinct responsibilities
2. **Testability**: Layers can be tested independently via traits
3. **Flexibility**: Easy to swap implementations (file → database)
4. **Maintainability**: Changes in one layer don't cascade
5. **Domain Focus**: Business logic centralized in domain layer
6. **Rust-idiomatic**: Leverages traits, enums, and type safety

---

## Detailed Refactoring Roadmap

### Phase 1: Foundation - Domain Layer Enhancement

#### 1.1 Enrich Domain Models

**Current Problem**: Anemic domain model in `models.rs:118-163`

**Recommended Changes**:

```rust
// src/domain/models/project.rs
impl Project {
    // Factory methods
    pub fn new(
        name: String,
        language: Language,
        architecture: Architecture,
    ) -> Result<Self, DomainError> {
        Self::validate_name(&name)?;
        // Business logic here
    }

    // Business methods
    pub fn update_status(&mut self, status: Status) -> Result<(), DomainError> {
        // Validate state transitions
    }

    pub fn add_framework(&mut self, framework: String) -> Result<(), DomainError> {
        // Validation logic
    }

    // Queries
    pub fn is_completed(&self) -> bool {
        self.status == Status::Completed
    }
}

// src/domain/models/portfolio.rs
impl Portfolio {
    pub fn add_project(&mut self, project: Project) -> Result<(), DomainError> {
        // Validate uniqueness, business rules
    }

    pub fn statistics(&self) -> PortfolioStats {
        // Calculate stats
    }
}
```

#### 1.2 Introduce Value Objects

```rust
// src/domain/value_objects/project_name.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidProjectName("Name cannot be empty"));
        }
        Ok(Self(name))
    }

    pub fn to_id(&self) -> ProjectId {
        ProjectId(self.0.to_lowercase().replace(" ", "-"))
    }
}
```

#### 1.3 Domain Error Types

**Replace**: `anyhow::Result` in domain layer
**With**: Custom error enums

```rust
// src/domain/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),

    #[error("Project already exists: {0}")]
    ProjectAlreadyExists(String),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),
}
```

### Phase 2: Infrastructure Layer - Repository Pattern

#### 2.1 Define Repository Traits

```rust
// src/infrastructure/repositories/portfolio_repository.rs
pub trait PortfolioRepository: Send + Sync {
    fn find_all(&self) -> Result<Portfolio, RepositoryError>;
    fn find_by_id(&self, id: &ProjectId) -> Result<Option<Project>, RepositoryError>;
    fn save(&self, project: &Project) -> Result<(), RepositoryError>;
    fn delete(&self, id: &ProjectId) -> Result<(), RepositoryError>;
}

// Implementation
pub struct FileSystemPortfolioRepository {
    root_path: PathBuf,
}

impl PortfolioRepository for FileSystemPortfolioRepository {
    fn find_all(&self) -> Result<Portfolio, RepositoryError> {
        // Implementation from current utils.rs
    }
}
```

**Benefits**:

- Testability: Mock repository for unit tests
- Flexibility: Can add SQLite, PostgreSQL implementations
- Decoupling: Domain doesn't depend on file system

#### 2.2 External Service Abstractions

```rust
// src/infrastructure/services/version_control.rs
pub trait VersionControlService: Send + Sync {
    fn init_repository(&self, path: &Path) -> Result<(), InfraError>;
    fn commit(&self, path: &Path, message: &str) -> Result<(), InfraError>;
}

pub struct GitService;

impl VersionControlService for GitService {
    // Implementation
}

// src/infrastructure/services/editor.rs
pub trait EditorService: Send + Sync {
    fn open(&self, path: &Path) -> Result<(), InfraError>;
}
```

### Phase 3: Application Layer - Service Orchestration

#### 3.1 Project Service

```rust
// src/application/services/project_service.rs
pub struct ProjectService<R: PortfolioRepository, V: VersionControlService> {
    repository: Arc<R>,
    vcs: Arc<V>,
}

impl<R: PortfolioRepository, V: VersionControlService> ProjectService<R, V> {
    pub fn create_project(
        &self,
        command: CreateProjectCommand,
    ) -> Result<Project, ApplicationError> {
        // 1. Validate command
        let project = Project::new(
            command.name,
            command.language,
            command.architecture,
        )?;

        // 2. Check for duplicates
        if self.repository.find_by_id(&project.id)?.is_some() {
            return Err(ApplicationError::ProjectExists(project.id));
        }

        // 3. Create directory structure
        self.create_directory_structure(&project)?;

        // 4. Initialize VCS
        self.vcs.init_repository(&project.path)?;

        // 5. Save to repository
        self.repository.save(&project)?;

        Ok(project)
    }

    pub fn list_projects(
        &self,
        filters: ProjectFilters,
    ) -> Result<Vec<Project>, ApplicationError> {
        let portfolio = self.repository.find_all()?;
        Ok(portfolio.apply_filters(
            filters.language,
            filters.architecture,
            filters.status,
        ))
    }
}
```

**Benefits**:

- Single responsibility: Orchestrates domain and infrastructure
- Transaction management (if needed)
- Cross-cutting concerns (logging, metrics)

#### 3.2 Command Pattern for CLI

```rust
// src/application/commands/create_project_command.rs
#[derive(Debug)]
pub struct CreateProjectCommand {
    pub name: String,
    pub language: String,
    pub architecture: String,
    pub description: Option<String>,
}

impl CreateProjectCommand {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validation logic
    }
}
```

### Phase 4: Presentation Layer Refactoring

#### 4.1 Dependency Injection

```rust
// src/main.rs
struct AppContext {
    project_service: Arc<ProjectService<FileSystemPortfolioRepository, GitService>>,
    documentation_service: Arc<DocumentationService>,
    config: Config,
}

impl AppContext {
    fn new(config: Config) -> Self {
        let repository = Arc::new(FileSystemPortfolioRepository::new(&config.portfolio_root));
        let vcs = Arc::new(GitService);
        let project_service = Arc::new(ProjectService::new(repository, vcs));
        let documentation_service = Arc::new(DocumentationService::new());

        Self {
            project_service,
            documentation_service,
            config,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Init { force } = cli.command {
        return config::init_config(force);
    }

    let config = Config::load(cli.config.as_deref())?;
    let ctx = AppContext::new(config);

    match cli.command {
        Commands::New { name, language, architecture, description, interactive } => {
            commands::new::execute(ctx.project_service.clone(), name, language, architecture)?;
        }
        // Other commands...
    }

    Ok(())
}
```

#### 4.2 Simplified Command Handlers

```rust
// src/commands/new.rs
pub fn execute(
    service: Arc<ProjectService<impl PortfolioRepository, impl VersionControlService>>,
    name: String,
    language: String,
    architecture: String,
) -> Result<()> {
    let command = CreateProjectCommand {
        name,
        language,
        architecture,
        description: None,
    };

    command.validate()?;

    let project = service.create_project(command)?;

    println!("✓ Project created: {}", project.name);
    Ok(())
}
```

---

## Proposed Directory Structure

```
src/
├── main.rs                          # Entry point, DI setup
│
├── domain/                          # Core business logic
│   ├── mod.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── project.rs              # Project entity with behavior
│   │   ├── portfolio.rs            # Portfolio aggregate
│   │   └── enums.rs                # Language, Architecture, Status
│   ├── value_objects/
│   │   ├── mod.rs
│   │   ├── project_name.rs
│   │   └── project_id.rs
│   ├── services/                   # Domain services
│   │   └── project_validator.rs
│   └── errors.rs                   # Domain errors
│
├── application/                     # Use cases & orchestration
│   ├── mod.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── project_service.rs      # Project operations
│   │   ├── portfolio_service.rs    # Portfolio management
│   │   └── documentation_service.rs
│   ├── commands/                   # Command DTOs
│   │   ├── mod.rs
│   │   ├── create_project_command.rs
│   │   └── list_projects_query.rs
│   └── errors.rs                   # Application errors
│
├── infrastructure/                  # External concerns
│   ├── mod.rs
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── portfolio_repository.rs # Trait definition
│   │   └── filesystem_repository.rs # File implementation
│   ├── services/
│   │   ├── mod.rs
│   │   ├── version_control.rs      # Git abstraction
│   │   ├── editor_service.rs       # Editor launching
│   │   └── process_runner.rs       # Project execution
│   ├── config/
│   │   ├── mod.rs
│   │   └── config.rs               # Configuration loading
│   └── errors.rs                   # Infrastructure errors
│
├── presentation/                    # CLI interface
│   ├── mod.rs
│   ├── cli.rs                      # Clap definitions
│   ├── commands/                   # Thin command handlers
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   ├── new.rs
│   │   ├── open.rs
│   │   ├── run.rs
│   │   └── docs.rs
│   └── formatters/                 # Output formatting
│       ├── mod.rs
│       ├── table_formatter.rs
│       └── json_formatter.rs
│
└── lib.rs                          # Library exports
```

---

## Specific Improvements by File

### main.rs (Refactor Priority: HIGH)

**Current Issues**:

- Mix of CLI parsing and execution logic
- No dependency injection
- Direct coupling to commands

**Recommended Changes**:

1. Extract CLI definitions to `presentation/cli.rs`
2. Implement dependency injection container
3. Use `AppContext` for service management
4. Move command execution to thin handlers

**Code Location**: Lines 128-164

---

### models.rs (Refactor Priority: MEDIUM)

**Current Issues**:

- Anemic domain models (lines 118-163)
- Manual string parsing in enums (lines 22-30, 55-65, 86-94)
- Missing validation logic

**Recommended Changes**:

1. Add rich behavior to `Project` and `Portfolio`
2. Implement `FromStr` trait for enums instead of custom methods
3. Add domain validation methods
4. Extract to separate files per model

**Example Improvement**:

```rust
// Instead of
impl Language {
    pub fn from_str(s: &str) -> Option<Self> { ... }
}

// Use standard trait
impl FromStr for Language {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { ... }
}
```

---

### config.rs (Refactor Priority: LOW)

**Current Issues**:

- Config struct has loading logic (SRP violation)
- Path resolution is complex (lines 74-95)
- No validation of configuration values

**Recommended Changes**:

1. Separate `Config` (data) from `ConfigLoader` (behavior)
2. Add validation method
3. Consider environment-specific configs
4. Move to `infrastructure/config/`

---

### utils.rs (Refactor Priority: HIGH)

**Current Issues**:

- **Critical**: Direct file system coupling
- Impossible to unit test without actual file system
- Single function file (should be integrated elsewhere)

**Recommended Changes**:

1. **Delete this file entirely**
2. Move logic to `FileSystemPortfolioRepository`
3. Implement `PortfolioRepository` trait

**Migration Path**:

```rust
// From: utils::load_portfolio(config)
// To: repository.find_all()
```

---

### commands/*.rs (Refactor Priority: MEDIUM)

**Current Issues**:

- Commands contain business logic
- Direct dependencies on config and utils
- Hard to test
- Inconsistent error handling

**Recommended Changes**:

1. Make commands thin orchestrators
2. Move business logic to application services
3. Accept service dependencies via parameters
4. Return structured result types

**Example - list.rs**:

```rust
// Current: Business logic in command (lines 19-36)
pub fn execute(config: &Config, ...) -> Result<()> {
    let portfolio = utils::load_portfolio(config)?;
    // formatting logic
}

// Proposed: Delegate to service
pub fn execute(
    service: Arc<ProjectService>,
    filters: ProjectFilters,
    format: OutputFormat,
) -> Result<()> {
    let projects = service.list_projects(filters)?;
    let formatter = FormatterFactory::create(format);
    formatter.display(&projects);
    Ok(())
}
```

---

## Testing Strategy

### Current State

- **No tests found** in the codebase
- `tempfile` in dev-dependencies (line 27 in Cargo.toml) suggests testing was considered

### Recommended Test Architecture

```
tests/
├── unit/                           # Unit tests
│   ├── domain/
│   │   ├── project_tests.rs        # Test business logic
│   │   └── portfolio_tests.rs
│   └── application/
│       └── project_service_tests.rs # Test with mock repos
│
├── integration/                    # Integration tests
│   ├── repository_tests.rs         # Test actual file operations
│   └── command_tests.rs            # Test end-to-end commands
│
└── fixtures/                       # Test data
    └── sample_project.toml
```

### Example Unit Test

```rust
// tests/unit/domain/project_tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_valid_project() {
        let result = Project::new(
            "Test Project".to_string(),
            Language::Rust,
            Architecture::Hexagonal,
        );

        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.status, Status::InProgress);
    }

    #[test]
    fn should_reject_empty_name() {
        let result = Project::new(
            "".to_string(),
            Language::Rust,
            Architecture::Hexagonal,
        );

        assert!(matches!(result, Err(DomainError::InvalidProjectName(_))));
    }
}
```

### Example Integration Test with Mock

```rust
// tests/integration/project_service_tests.rs
struct MockRepository {
    projects: Mutex<Vec<Project>>,
}

impl PortfolioRepository for MockRepository {
    fn find_all(&self) -> Result<Portfolio, RepositoryError> {
        Ok(Portfolio {
            projects: self.projects.lock().unwrap().clone(),
        })
    }

    fn save(&self, project: &Project) -> Result<(), RepositoryError> {
        self.projects.lock().unwrap().push(project.clone());
        Ok(())
    }
}

#[test]
fn should_create_project_successfully() {
    let repo = Arc::new(MockRepository::new());
    let vcs = Arc::new(MockVcs);
    let service = ProjectService::new(repo.clone(), vcs);

    let command = CreateProjectCommand {
        name: "test".to_string(),
        language: "rust".to_string(),
        architecture: "hexagonal".to_string(),
        description: None,
    };

    let result = service.create_project(command);
    assert!(result.is_ok());

    let projects = repo.find_all().unwrap();
    assert_eq!(projects.projects.len(), 1);
}
```

---

## Error Handling Strategy

### Current Issues

- `anyhow::Result` used everywhere (loses type safety)
- No error context or categorization
- User-facing vs internal errors not distinguished

### Recommended Error Hierarchy

```rust
// src/domain/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),
}

// src/application/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

// src/infrastructure/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] toml::de::Error),
}

// src/presentation/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Application error: {0}")]
    Application(#[from] ApplicationError),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Application(ApplicationError::Domain(_)) => 1,
            CliError::Application(ApplicationError::Repository(_)) => 2,
            _ => 99,
        }
    }
}
```

**Benefits**:

- Type-safe error handling
- Clear error propagation path
- Easy to add context at each layer
- Can provide user-friendly messages

---

## Performance Considerations

### Current Performance Analysis

1. **Portfolio Loading** (utils.rs:7-24)
    - Uses `WalkDir` with max_depth=3
    - Reads and parses every `project.toml` file
    - **Issue**: No caching, re-scans on every command
    - **Impact**: Slow for large portfolios (>100 projects)

2. **Filtering** (models.rs:136-162)
    - Filters in memory after loading all projects
    - **Issue**: Could filter during loading

### Recommended Optimizations

#### 1. Implement Caching Layer

```rust
// src/infrastructure/repositories/cached_repository.rs
pub struct CachedPortfolioRepository<R: PortfolioRepository> {
    inner: R,
    cache: Arc<Mutex<Option<(Portfolio, SystemTime)>>>,
    ttl: Duration,
}

impl<R: PortfolioRepository> PortfolioRepository for CachedPortfolioRepository<R> {
    fn find_all(&self) -> Result<Portfolio, RepositoryError> {
        let mut cache = self.cache.lock().unwrap();

        if let Some((portfolio, timestamp)) = &*cache {
            if timestamp.elapsed()? < self.ttl {
                return Ok(portfolio.clone());
            }
        }

        let portfolio = self.inner.find_all()?;
        *cache = Some((portfolio.clone(), SystemTime::now()));
        Ok(portfolio)
    }
}
```

#### 2. Lazy Loading with Indexes

```rust
// Build index on first scan
pub struct IndexedRepository {
    index: HashMap<ProjectId, PathBuf>,
}

impl IndexedRepository {
    fn find_by_id(&self, id: &ProjectId) -> Result<Option<Project>, RepositoryError> {
        // Only load single file
        if let Some(path) = self.index.get(id) {
            return self.load_project(path);
        }
        Ok(None)
    }
}
```

#### 3. Parallel Project Loading

```rust
use rayon::prelude::*;

fn load_portfolio_parallel(&self) -> Result<Portfolio> {
    let projects: Vec<Project> = WalkDir::new(&self.root)
        .max_depth(3)
        .into_iter()
        .par_bridge()  // Parallel iteration
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "project.toml")
        .filter_map(|e| self.load_project(e.path()).ok())
        .collect();

    Ok(Portfolio { projects })
}
```

**Add to Cargo.toml**:

```toml
[dependencies]
rayon = "1.8"  # For parallel processing
```

---

## Security Considerations

### Current Security Issues

1. **Path Traversal** (new.rs:25-26)
   ```rust
   let project_path = config.portfolio_root.join(&project_id);
   ```
    - User-controlled `name` becomes `project_id`
    - Could create projects outside portfolio root

2. **Command Injection** (open.rs:27-32, run.rs:32-67)
    - Editor and terminal commands use user input
    - Potential shell injection

3. **Token Storage** (config.rs:15-16)
    - GitLab token in config file (plaintext)
    - Should use OS keychain

### Recommended Security Measures

#### 1. Path Validation

```rust
// src/domain/value_objects/project_name.rs
impl ProjectName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        // Validate no path traversal characters
        if name.contains("..") || name.contains("/") || name.contains("\\") {
            return Err(DomainError::InvalidProjectName(
                "Name cannot contain path separators".to_string()
            ));
        }

        // Validate reasonable length
        if name.len() > 255 {
            return Err(DomainError::InvalidProjectName(
                "Name too long".to_string()
            ));
        }

        Ok(Self(name))
    }
}
```

#### 2. Safe Command Execution

```rust
// src/infrastructure/services/editor_service.rs
impl EditorService for SystemEditorService {
    fn open(&self, path: &Path) -> Result<(), InfraError> {
        // Validate path is within allowed directories
        let canonical = path.canonicalize()?;
        if !canonical.starts_with(&self.allowed_root) {
            return Err(InfraError::InvalidPath);
        }

        // Use Command builder (no shell interpolation)
        Command::new(&self.editor_binary)
            .arg(canonical)
            .spawn()?;

        Ok(())
    }
}
```

#### 3. Secure Token Storage

```rust
// Use keyring crate
// Cargo.toml
[dependencies]
keyring = "2.0"

// src/infrastructure/config/token_provider.rs
pub trait TokenProvider {
    fn get_gitlab_token(&self) -> Result<Option<String>, InfraError>;
}

pub struct KeychainTokenProvider;

impl TokenProvider for KeychainTokenProvider {
    fn get_gitlab_token(&self) -> Result<Option<String>, InfraError> {
        let entry = keyring::Entry::new("portfolio-cli", "gitlab-token")?;
        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(_) => Ok(None),
        }
    }
}
```

---

## Migration Plan

### Phase 1: Foundation (1-2 weeks)

- [ ] Create new directory structure
- [ ] Define repository traits
- [ ] Implement domain errors
- [ ] Add domain validation to models
- [ ] Set up testing infrastructure

### Phase 2: Infrastructure (1 week)

- [ ] Implement `FileSystemPortfolioRepository`
- [ ] Abstract version control service
- [ ] Abstract editor/terminal services
- [ ] Add caching layer
- [ ] Implement secure token storage

### Phase 3: Application Layer (1 week)

- [ ] Create `ProjectService`
- [ ] Create `PortfolioService`
- [ ] Create `DocumentationService`
- [ ] Define command DTOs
- [ ] Implement application error handling

### Phase 4: Presentation Refactoring (3 days)

- [ ] Extract CLI definitions
- [ ] Implement dependency injection
- [ ] Refactor command handlers
- [ ] Add output formatters

### Phase 5: Testing & Documentation (1 week)

- [ ] Write unit tests (target: 80% coverage)
- [ ] Write integration tests
- [ ] Add doc comments
- [ ] Create architecture decision records (ADRs)
- [ ] Update README with architecture diagram

### Phase 6: Optimizations (Optional)

- [ ] Add parallel loading
- [ ] Implement query optimizations
- [ ] Add metrics/telemetry
- [ ] Performance profiling

---

## Key Architectural Decisions

### ADR 001: Repository Pattern for Data Access

**Status**: Recommended

**Context**: Current implementation directly couples file system operations throughout the codebase.

**Decision**: Introduce Repository pattern with trait-based abstraction.

**Consequences**:

- ✅ Testability: Can mock repositories
- ✅ Flexibility: Easy to add database support
- ✅ Consistency: Single point for data access logic
- ⚠️ Complexity: Additional abstraction layer

---

### ADR 002: Layered Architecture over Clean/Hexagonal

**Status**: Recommended

**Context**: Project is a CLI tool, not a long-running service.

**Decision**: Use layered architecture instead of strict hexagonal.

**Rationale**:

- CLI tools have clear "top-to-bottom" execution flow
- No need for multiple adapters (HTTP, gRPC, etc.)
- Simpler for small team/solo developer
- Still provides separation of concerns
- Easier to understand and maintain

**Consequences**:

- ✅ Simpler structure
- ✅ Less boilerplate
- ⚠️ Less flexible for adding web UI later

---

### ADR 003: File-Based Storage for MVP

**Status**: Current

**Context**: Current implementation uses TOML files per project.

**Decision**: Keep file-based storage for now, abstract for future.

**Rationale**:

- Simple for users to understand and debug
- No database setup required
- Git-friendly (can version control)
- Good enough for <1000 projects
- Abstracted via repository trait (easy to swap later)

**Consequences**:

- ✅ Simple deployment
- ✅ User-friendly
- ⚠️ Performance issues with many projects
- ⚠️ No transactions or concurrent access control

---

## Dependency Recommendations

### Add to Cargo.toml

```toml
[dependencies]
# Current dependencies (keep)
clap = { version = "4.4", features = ["derive", "cargo"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
anyhow = "1.0"  # Keep for main.rs only
thiserror = "1.0"
colored = "2.1"
tabled = "0.15"
chrono = { version = "0.4", features = ["serde"] }
walkdir = "2.4"
dirs = "5.0"

# Recommended additions
rayon = "1.8"                  # Parallel processing
keyring = "2.0"                # Secure token storage
tracing = "0.1"                # Structured logging
tracing-subscriber = "0.3"    # Log output
once_cell = "1.19"            # Lazy statics

[dev-dependencies]
tempfile = "3.8"
mockall = "0.12"              # Mocking framework
assert_fs = "1.0"             # File system assertions
predicates = "3.0"            # Test assertions
```

---

## Metrics for Success

### Code Quality Metrics

- [ ] Test coverage > 80%
- [ ] All public APIs have doc comments
- [ ] No `unwrap()` calls in production code
- [ ] Clippy warnings = 0
- [ ] Cyclomatic complexity < 10 per function

### Architecture Metrics

- [ ] Clear layer separation (import rules enforced)
- [ ] Domain layer has zero external dependencies
- [ ] All external interactions abstracted by traits
- [ ] Error types at each layer

### Performance Metrics

- [ ] `list` command < 100ms for 100 projects
- [ ] `new` command < 500ms
- [ ] Memory usage < 50MB

---

## Conclusion

### Summary

The Portfolio CLI project has a **solid foundation** with good dependency choices and clear intent. However, it suffers
from common issues in early-stage projects:

1. **Tight coupling** between layers
2. **Anemic domain models** with business logic scattered
3. **Limited testability** due to hard-coded dependencies
4. **No abstraction** of external services

### Recommended Path Forward

**Adopt Domain-Driven Layered Architecture** because:

- ✅ Clear separation of concerns
- ✅ Testability through trait abstractions
- ✅ Maintainability for future growth
- ✅ Rust-idiomatic patterns
- ✅ Reasonable complexity for CLI tool

### Priority Actions

**High Priority** (Do First):

1. Implement Repository pattern for portfolio access
2. Add domain validation logic
3. Create application service layer
4. Set up testing infrastructure

**Medium Priority**:

1. Refactor command handlers
2. Implement proper error types
3. Add security validations
4. Extract value objects

**Low Priority** (Nice to Have):

1. Add caching layer
2. Implement parallel loading
3. Add metrics/telemetry
4. Create web UI adapter layer

### Final Assessment

**Current Grade**: C+ (Functional but needs architectural improvements)
**Potential Grade**: A- (With recommended refactoring)
**Effort Required**: ~3-4 weeks for complete refactoring
**ROI**: High - Will enable easier feature additions and maintenance

The codebase is at an ideal stage for refactoring: functional enough to understand requirements, but not so large that
refactoring is prohibitive.

---

## Additional Resources

### Recommended Reading

- "Domain-Driven Design" by Eric Evans (Chapters on Layered Architecture)
- "Clean Architecture" by Robert Martin
- "Programming Rust" by Blandy & Orendorff (Chapter on Traits)
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

### Example Rust Projects with Good Architecture

- `cargo` source code (layered architecture)
- `ripgrep` (performance optimizations)
- `bat` (CLI interface patterns)
- `tokio` (trait abstractions)

### Tools

- `cargo-tarpaulin` - Code coverage
- `cargo-criterion` - Benchmarking
- `cargo-deny` - Dependency auditing
- `cargo-udeps` - Unused dependencies
- `cargo-bloat` - Binary size analysis

---

**Document Version**: 1.0
**Date**: 2026-02-20
**Author**: Architecture Analysis
**Status**: Recommendation
