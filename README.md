# Portfolio CLI

A CLI tool for managing and documenting architecture portfolio projects, built with Rust.

## Overview

Portfolio CLI helps you organize, track, and manage your software architecture projects. It provides a command-line
interface to create projects, track their progress, and generate documentation.

## Features

- **Project Management**: Create, update, and delete portfolio projects
- **Multi-Architecture Support**: Track projects with different architectural patterns (hexagonal, onion, layered,
  pipeline, microkernel)
- **Multi-Language Support**: Manage projects in Kotlin, Java, and Rust
- **Status Tracking**: Monitor project status (planned, in-progress, completed, archived)
- **Documentation Generation**: Automatically generate portfolio documentation
- **Project Launcher**: Open projects in your configured editor or terminal
- **Statistics Dashboard**: View portfolio statistics and completion rates
- **Filtering & Search**: Filter projects by language, architecture, or status

## Installation

### Prerequisites

- Rust 1.70+ (Edition 2024)

### Build from Source

```bash
git clone <repository-url>
cd portfolio-cli
cargo build --release
```

The binary will be available at `target/release/portfolio-cli`.

## Usage

### Initialize Configuration

Before using the CLI, initialize the configuration file:

```bash
portfolio-cli init
```

This creates a configuration file at `~/.config/portfolio-cli/config.toml`.

### Commands

#### List Projects

```bash
# List all projects
portfolio-cli list

# Filter by language
portfolio-cli list --language rust

# Filter by architecture
portfolio-cli list --architecture hexagonal

# Filter by status
portfolio-cli list --status in-progress

# Show statistics
portfolio-cli list --stats

# Output as JSON
portfolio-cli list --format json
```

#### Create a New Project

```bash
portfolio-cli new my-project \
  --language rust \
  --architecture hexagonal \
  --description "A sample project"
```

#### Update a Project

```bash
portfolio-cli update my-project \
  --status completed \
  --description "Updated description" \
  --add-framework "actix-web" \
  --gitlab-url "https://gitlab.com/user/project"
```

#### Open a Project

```bash
# Open in configured editor
portfolio-cli open my-project

# Open in terminal
portfolio-cli open my-project --terminal

# Use specific editor
portfolio-cli open my-project --editor code
```

#### Run a Project

```bash
# Run with default settings
portfolio-cli run my-project

# Run with custom port
portfolio-cli run my-project --port 8080

# Run with specific profile
portfolio-cli run my-project --profile prod --debug
```

#### Generate Documentation

```bash
# Generate docs in default location (docs/)
portfolio-cli docs

# Generate detailed documentation
portfolio-cli docs --output ./documentation --detailed
```

#### View Statistics

```bash
portfolio-cli stats
```

#### Delete a Project

```bash
# Delete with confirmation
portfolio-cli delete my-project

# Skip confirmation
portfolio-cli delete my-project --yes
```

### Global Options

- `--config <path>`: Use a custom configuration file
- `--verbose` / `-v`: Enable verbose output

## Architecture

The project follows a **layered architecture** with clear separation of concerns:

```
src/
├── application/      # Application services and DTOs
├── domain/          # Domain models, value objects, and business logic
├── infrastructure/  # Repositories, configuration, and external services
└── presentation/    # CLI interface, commands, and formatters
```

## Configuration

Configuration is stored in `~/.config/portfolio-cli/config.toml` (or custom location).

Example configuration:

```toml
[editor]
default = "code"

[projects]
base_path = "~/projects"

[output]
default_format = "table"
```

## Dependencies

- **clap**: Command-line argument parsing
- **serde/serde_json/toml**: Serialization
- **anyhow/thiserror**: Error handling
- **colored/tabled**: Terminal UI
- **chrono**: Date and time handling
- **rayon**: Parallel processing
- **tracing**: Logging and diagnostics
- **keyring**: Secure token storage

## Development

### Running Tests

```bash
cargo test
```

### Running with Debug Output

```bash
cargo run -- list --verbose
```

### Project Structure

- `src/main.rs`: Application entry point
- `src/lib.rs`: Library root
- `src/presentation/cli_application.rs`: CLI orchestration
- `src/domain/`: Core domain models and business logic
- `tests/`: Integration and unit tests

## Author

Mollet Tristan

## License

See LICENSE file for details.
