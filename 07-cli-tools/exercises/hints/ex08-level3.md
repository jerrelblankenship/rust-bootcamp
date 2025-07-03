# Exercise 8 - Level 3 Hint (Near-Complete Solution)

## Complete Professional Subcommand System

Here's a comprehensive implementation of a git-style CLI tool with advanced features.

```rust
use clap::{Parser, Subcommand, Args, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;
use colored::Colorize;
use anyhow::{Context, Result};

// Main CLI structure with global options
#[derive(Parser)]
#[clap(
    name = "dev-tools",
    version = env!("CARGO_PKG_VERSION"),
    about = "Professional developer productivity suite",
    long_about = "A comprehensive toolkit for developers with file operations, 
project management, and development server capabilities."
)]
struct Cli {
    /// Enable verbose output
    #[clap(short, long, global = true, help = "Show detailed operation information")]
    verbose: bool,
    
    /// Quiet mode (suppress non-essential output)
    #[clap(short, long, global = true, conflicts_with = "verbose")]
    quiet: bool,
    
    /// Configuration file path
    #[clap(
        short, 
        long, 
        global = true, 
        help = "Path to configuration file",
        env = "DEV_TOOLS_CONFIG"
    )]
    config: Option<PathBuf>,
    
    /// Output format for structured data
    #[clap(long, global = true, value_enum, default_value = "human")]
    output: OutputMode,
    
    /// Dry run mode (don't make actual changes)
    #[clap(long, global = true)]
    dry_run: bool,
    
    #[clap(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputMode {
    Human,
    Json,
    Yaml,
    Table,
}

#[derive(Subcommand)]
enum Commands {
    /// File operations (convert, validate, watch)
    #[clap(alias = "f")]
    File {
        #[clap(subcommand)]
        action: FileCommands,
    },
    
    /// Configuration management
    #[clap(alias = "cfg")]
    Config {
        #[clap(subcommand)]
        action: ConfigCommands,
    },
    
    /// Project management and scaffolding
    #[clap(alias = "proj")]
    Project {
        #[clap(subcommand)]
        action: ProjectCommands,
    },
    
    /// Development server with hot reload
    #[clap(alias = "s")]
    Serve(ServeArgs),
    
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },
    
    /// Show detailed tool information
    Info,
}

#[derive(Subcommand)]
enum FileCommands {
    /// Convert between file formats
    #[clap(alias = "conv")]
    Convert {
        /// Input file path
        #[clap(short, long, help = "Source file to convert")]
        input: PathBuf,
        
        /// Output format
        #[clap(short, long, value_enum, help = "Target format for conversion")]
        format: FileFormat,
        
        /// Output file path (stdout if not specified)
        #[clap(short, long)]
        output: Option<PathBuf>,
        
        /// Preserve original file metadata
        #[clap(long)]
        preserve_metadata: bool,
        
        /// Pretty-print output
        #[clap(long, default_value = "true")]
        pretty: bool,
    },
    
    /// Validate file syntax and structure
    #[clap(alias = "val")]
    Validate {
        /// Files to validate
        #[clap(required = true, help = "Files or directories to validate")]
        paths: Vec<PathBuf>,
        
        /// File types to include (if validating directories)
        #[clap(long, value_delimiter = ',')]
        include: Vec<String>,
        
        /// File patterns to exclude
        #[clap(long, value_delimiter = ',')]
        exclude: Vec<String>,
        
        /// Stop on first error
        #[clap(long)]
        fail_fast: bool,
        
        /// Validate file content, not just syntax
        #[clap(long)]
        strict: bool,
    },
    
    /// Watch files and directories for changes
    Watch {
        /// Directory to watch
        #[clap(default_value = ".", help = "Root directory to monitor")]
        directory: PathBuf,
        
        /// File patterns to watch
        #[clap(
            short, 
            long, 
            action = clap::ArgAction::Append,
            help = "Glob patterns for files to watch"
        )]
        pattern: Vec<String>,
        
        /// Command to run on changes
        #[clap(short, long)]
        command: Option<String>,
        
        /// Ignore hidden files and directories
        #[clap(long, default_value = "true")]
        ignore_hidden: bool,
        
        /// Debounce delay in milliseconds
        #[clap(long, default_value = "500")]
        debounce: u64,
    },
    
    /// Find and replace text in files
    Replace {
        /// Search pattern (supports regex with --regex flag)
        pattern: String,
        
        /// Replacement text
        replacement: String,
        
        /// Files to process
        files: Vec<PathBuf>,
        
        /// Use regular expressions
        #[clap(long)]
        regex: bool,
        
        /// Case insensitive matching
        #[clap(short, long)]
        ignore_case: bool,
        
        /// Create backup files
        #[clap(short, long)]
        backup: bool,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Display current configuration
    Show {
        /// Show specific configuration section
        #[clap(help = "Configuration key or section to display")]
        key: Option<String>,
    },
    
    /// Set configuration value
    Set {
        /// Configuration key (use dot notation for nested keys)
        #[clap(help = "Key to set (e.g., 'editor.command')")]
        key: String,
        
        /// Configuration value
        #[clap(help = "Value to set")]
        value: String,
        
        /// Set value globally (for all projects)
        #[clap(long)]
        global: bool,
    },
    
    /// Get configuration value
    Get {
        /// Configuration key to retrieve
        key: String,
        
        /// Show default value if key is not set
        #[clap(long)]
        show_default: bool,
    },
    
    /// Remove configuration key
    Unset {
        /// Configuration key to remove
        key: String,
        
        /// Remove from global configuration
        #[clap(long)]
        global: bool,
    },
    
    /// List all configuration keys
    List {
        /// Show configuration sources
        #[clap(long)]
        show_sources: bool,
    },
    
    /// Reset configuration to defaults
    Reset {
        /// Confirm destructive operation
        #[clap(long, help = "Confirm that you want to reset all configuration")]
        confirm: bool,
        
        /// Reset global configuration instead of local
        #[clap(long)]
        global: bool,
    },
    
    /// Edit configuration file in default editor
    Edit {
        /// Edit global configuration
        #[clap(long)]
        global: bool,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// Initialize new project from template
    #[clap(alias = "new")]
    Init {
        /// Project name
        #[clap(help = "Name of the new project")]
        name: String,
        
        /// Project template
        #[clap(
            short, 
            long, 
            default_value = "default",
            help = "Template to use for project initialization"
        )]
        template: String,
        
        /// Target directory (defaults to project name)
        #[clap(short, long)]
        directory: Option<PathBuf>,
        
        /// Don't initialize git repository
        #[clap(long)]
        no_git: bool,
        
        /// Additional template variables
        #[clap(long, value_parser = parse_key_val)]
        var: Vec<(String, String)>,
    },
    
    /// Build project
    Build {
        /// Build profile
        #[clap(long, default_value = "debug", value_enum)]
        profile: BuildProfile,
        
        /// Target platform
        #[clap(long)]
        target: Option<String>,
        
        /// Number of parallel jobs
        #[clap(short, long)]
        jobs: Option<usize>,
        
        /// Enable all optimizations
        #[clap(long)]
        optimize: bool,
        
        /// Build specific packages only
        #[clap(short, long, action = clap::ArgAction::Append)]
        package: Vec<String>,
    },
    
    /// Clean build artifacts
    Clean {
        /// Remove all build artifacts
        #[clap(long)]
        all: bool,
        
        /// Remove specific target directory
        #[clap(long)]
        target: Option<String>,
        
        /// Packages to clean
        #[clap(short, long, action = clap::ArgAction::Append)]
        package: Vec<String>,
    },
    
    /// Test project
    Test {
        /// Test name pattern
        #[clap(help = "Run tests matching this pattern")]
        pattern: Option<String>,
        
        /// Run ignored tests
        #[clap(long)]
        ignored: bool,
        
        /// Test specific packages
        #[clap(short, long, action = clap::ArgAction::Append)]
        package: Vec<String>,
        
        /// Number of test threads
        #[clap(long)]
        test_threads: Option<usize>,
    },
    
    /// List available project templates
    Templates,
    
    /// Show project information
    Info,
}

#[derive(Args)]
struct ServeArgs {
    /// Port to listen on
    #[clap(short, long, default_value = "8080", env = "DEV_TOOLS_PORT")]
    port: u16,
    
    /// Host to bind to
    #[clap(long, default_value = "localhost")]
    host: String,
    
    /// Document root directory
    #[clap(short, long, default_value = ".")]
    root: PathBuf,
    
    /// Enable hot reload
    #[clap(long, help = "Automatically reload on file changes")]
    hot_reload: bool,
    
    /// CORS allowed origins
    #[clap(long, action = clap::ArgAction::Append)]
    cors_origin: Vec<String>,
    
    /// Enable HTTPS
    #[clap(long)]
    https: bool,
    
    /// SSL certificate file (required for HTTPS)
    #[clap(long, requires = "https")]
    cert: Option<PathBuf>,
    
    /// SSL private key file (required for HTTPS)
    #[clap(long, requires = "https")]
    key: Option<PathBuf>,
    
    /// Open browser after starting
    #[clap(long)]
    open: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum FileFormat {
    Json,
    Yaml,
    Toml,
    Xml,
    Csv,
    Markdown,
}

#[derive(ValueEnum, Clone, Debug)]
enum BuildProfile {
    Debug,
    Release,
    Test,
    Bench,
}

// Configuration structure
#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    editor: EditorConfig,
    project: ProjectConfig,
    server: ServerConfig,
    file: FileConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct EditorConfig {
    command: String,
    args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectConfig {
    default_template: String,
    auto_git_init: bool,
    default_license: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    default_port: u16,
    default_host: String,
    hot_reload: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileConfig {
    backup_extension: String,
    preserve_permissions: bool,
    line_ending: String,
}

// Implement Default for config structs
impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            command: detect_default_editor(),
            args: vec![],
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            default_template: "default".to_string(),
            auto_git_init: true,
            default_license: "MIT".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            default_port: 8080,
            default_host: "localhost".to_string(),
            hot_reload: true,
        }
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            backup_extension: ".bak".to_string(),
            preserve_permissions: true,
            line_ending: if cfg!(windows) { "crlf" } else { "lf" }.to_string(),
        }
    }
}

// Main execution logic
fn main() {
    let cli = Cli::parse();
    
    // Initialize logging
    setup_logging(cli.verbose, cli.quiet);
    
    // Load configuration
    let config = match load_config(cli.config.as_deref()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} Failed to load configuration: {}", "Error:".red(), e);
            process::exit(1);
        }
    };
    
    // Execute command
    let result = execute_command(cli.command, &config, &cli);
    
    match result {
        Ok(()) => {},
        Err(e) => {
            eprintln!("{} {}", "Error:".red(), e);
            
            // Show error chain for better debugging
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("{} {}", "Caused by:".yellow(), err);
                source = err.source();
            }
            
            process::exit(1);
        }
    }
}

// Helper functions
fn setup_logging(verbose: bool, quiet: bool) {
    let level = if quiet {
        log::LevelFilter::Error
    } else if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    
    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp(None)
        .format_module_path(false)
        .init();
}

fn load_config(config_path: Option<&std::path::Path>) -> Result<Config> {
    // Implementation for loading configuration from file or defaults
    let config = Config::default();
    Ok(config)
}

fn execute_command(command: Commands, config: &Config, cli: &Cli) -> Result<()> {
    match command {
        Commands::File { action } => execute_file_command(action, config, cli),
        Commands::Config { action } => execute_config_command(action, config, cli),
        Commands::Project { action } => execute_project_command(action, config, cli),
        Commands::Serve(args) => execute_serve_command(args, config, cli),
        Commands::Completions { shell } => generate_completions(shell),
        Commands::Info => show_info(),
    }
}

fn execute_file_command(action: FileCommands, config: &Config, cli: &Cli) -> Result<()> {
    // Implementation for file commands
    Ok(())
}

fn execute_config_command(action: ConfigCommands, config: &Config, cli: &Cli) -> Result<()> {
    // Implementation for config commands
    Ok(())
}

fn execute_project_command(action: ProjectCommands, config: &Config, cli: &Cli) -> Result<()> {
    // Implementation for project commands
    Ok(())
}

fn execute_serve_command(args: ServeArgs, config: &Config, cli: &Cli) -> Result<()> {
    // Implementation for serve command
    Ok(())
}

fn generate_completions(shell: clap_complete::Shell) -> Result<()> {
    use clap::CommandFactory;
    use clap_complete::generate;
    
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    
    generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
    Ok(())
}

fn show_info() -> Result<()> {
    println!("{} {}", "dev-tools".bold(), env!("CARGO_PKG_VERSION"));
    println!("A professional developer productivity suite");
    println!();
    println!("{}", "Build Information:".bold());
    println!("  Rust version: {}", env!("RUSTC_VERSION"));
    println!("  Target: {}", env!("TARGET"));
    println!("  Profile: {}", env!("PROFILE"));
    Ok(())
}

fn detect_default_editor() -> String {
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            if cfg!(windows) {
                "notepad".to_string()
            } else {
                "vi".to_string()
            }
        })
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() == 2 {
        Ok((parts[0].to_string(), parts[1].to_string()))
    } else {
        Err(format!("Invalid key=value pair: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    
    #[test]
    fn test_cli_structure() {
        // Verify CLI can be built without panics
        let _cmd = Cli::command();
    }
    
    #[test]
    fn test_all_subcommands() {
        let cmd = Cli::command();
        
        // Test each major command can be parsed
        let test_cases = vec![
            vec!["dev-tools", "file", "convert", "-i", "test.json", "-f", "yaml"],
            vec!["dev-tools", "config", "set", "editor.command", "vim"],
            vec!["dev-tools", "project", "init", "my-project"],
            vec!["dev-tools", "serve", "--port", "3000"],
            vec!["dev-tools", "completions", "bash"],
            vec!["dev-tools", "info"],
        ];
        
        for args in test_cases {
            let result = cmd.clone().try_get_matches_from(args.clone());
            assert!(result.is_ok(), "Failed to parse: {:?}", args);
        }
    }
    
    #[test]
    fn test_global_flags() {
        let cmd = Cli::command();
        
        // Test global flags work with subcommands
        let args = vec![
            "dev-tools", "--verbose", "--dry-run", 
            "file", "convert", "-i", "test.json", "-f", "yaml"
        ];
        
        let result = cmd.try_get_matches_from(args);
        assert!(result.is_ok());
    }
}
```

### Usage Examples

```bash
# File operations with various options
dev-tools file convert -i data.json -f yaml --pretty
dev-tools file validate src/ --include "*.rs,*.toml" --strict
dev-tools file watch . -p "*.rs" -p "*.toml" --command "cargo check"
dev-tools file replace "old_text" "new_text" src/*.rs --regex --backup

# Configuration management
dev-tools config show
dev-tools config set editor.command "code --wait"
dev-tools config list --show-sources
dev-tools config edit --global

# Project management
dev-tools project init my-app --template rust --var author="John Doe"
dev-tools project build --profile release --optimize
dev-tools project test --pattern integration

# Development server
dev-tools serve --port 3000 --hot-reload --open
dev-tools serve --https --cert cert.pem --key key.pem

# Utility commands
dev-tools completions bash > /etc/bash_completion.d/dev-tools
dev-tools info

# Global flags work everywhere
dev-tools --verbose --dry-run project build
dev-tools --quiet --output json config show
```

This implementation provides a comprehensive, professional CLI tool with excellent user experience and extensive functionality!