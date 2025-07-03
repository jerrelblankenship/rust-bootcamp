# Exercise 8 - Level 2 Hint (Specific Solutions)

## Building Complex Subcommand Structures

Let's create a professional subcommand system like git or cargo.

### 1. Multi-Level Command Structure

```rust
use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "dev-tools", version = "1.0", about = "Developer productivity tools")]
struct Cli {
    /// Enable verbose output
    #[clap(short, long, global = true)]
    verbose: bool,
    
    /// Configuration file path
    #[clap(short, long, global = true)]
    config: Option<PathBuf>,
    
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// File operations
    File {
        #[clap(subcommand)]
        action: FileCommands,
    },
    /// Configuration management
    Config {
        #[clap(subcommand)]
        action: ConfigCommands,
    },
    /// Project management
    Project {
        #[clap(subcommand)]
        action: ProjectCommands,
    },
    /// Development server
    Serve(ServeArgs),
}

#[derive(Subcommand)]
enum FileCommands {
    /// Convert file formats
    Convert {
        /// Input file path
        #[clap(short, long)]
        input: PathBuf,
        
        /// Output format (json, yaml, toml)
        #[clap(short, long, value_enum)]
        format: OutputFormat,
        
        /// Output file (optional)
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    /// Validate file syntax
    Validate {
        /// Files to validate
        files: Vec<PathBuf>,
        
        /// Stop on first error
        #[clap(long)]
        fail_fast: bool,
    },
    /// Watch files for changes
    Watch {
        /// Directory to watch
        #[clap(default_value = ".")]
        directory: PathBuf,
        
        /// File patterns to watch
        #[clap(short, long, action = clap::ArgAction::Append)]
        pattern: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Reset configuration to defaults
    Reset {
        /// Confirm reset
        #[clap(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// Initialize new project
    Init {
        /// Project name
        name: String,
        
        /// Project template
        #[clap(short, long, default_value = "default")]
        template: String,
    },
    /// Build project
    Build {
        /// Build profile (debug, release)
        #[clap(long, default_value = "debug")]
        profile: String,
        
        /// Enable optimization
        #[clap(long)]
        optimize: bool,
    },
    /// Clean build artifacts
    Clean {
        /// Remove all artifacts
        #[clap(long)]
        all: bool,
    },
}

#[derive(Args)]
struct ServeArgs {
    /// Port to listen on
    #[clap(short, long, default_value = "8080")]
    port: u16,
    
    /// Host to bind to
    #[clap(long, default_value = "localhost")]
    host: String,
    
    /// Enable hot reload
    #[clap(long)]
    hot_reload: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Json,
    Yaml,
    Toml,
}
```

### 2. Command Execution Logic

```rust
fn main() {
    let cli = Cli::parse();
    
    // Set up logging based on verbose flag
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    }
    
    // Load configuration
    let config = load_config(cli.config.as_deref());
    
    if let Err(e) = execute_command(cli.command, &config, cli.verbose) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn execute_command(
    command: Commands, 
    config: &Config, 
    verbose: bool
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::File { action } => execute_file_command(action, config, verbose),
        Commands::Config { action } => execute_config_command(action, config, verbose),
        Commands::Project { action } => execute_project_command(action, config, verbose),
        Commands::Serve(args) => execute_serve_command(args, config, verbose),
    }
}

fn execute_file_command(
    action: FileCommands,
    config: &Config,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        FileCommands::Convert { input, format, output } => {
            if verbose {
                println!("Converting {} to {:?}", input.display(), format);
            }
            
            let converted = convert_file(&input, &format)?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, converted)?;
            } else {
                println!("{}", converted);
            }
        },
        
        FileCommands::Validate { files, fail_fast } => {
            let mut errors = 0;
            
            for file in files {
                match validate_file(&file) {
                    Ok(()) => {
                        if verbose {
                            println!("✓ {}", file.display());
                        }
                    },
                    Err(e) => {
                        eprintln!("✗ {}: {}", file.display(), e);
                        errors += 1;
                        
                        if fail_fast {
                            return Err(e);
                        }
                    }
                }
            }
            
            if errors > 0 {
                return Err(format!("{} file(s) failed validation", errors).into());
            }
        },
        
        FileCommands::Watch { directory, pattern } => {
            if verbose {
                println!("Watching {} for changes", directory.display());
                println!("Patterns: {:?}", pattern);
            }
            
            watch_directory(&directory, &pattern)?;
        },
    }
    
    Ok(())
}

fn execute_config_command(
    action: ConfigCommands,
    config: &Config,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigCommands::Show => {
            println!("{:#?}", config);
        },
        
        ConfigCommands::Set { key, value } => {
            if verbose {
                println!("Setting {} = {}", key, value);
            }
            set_config_value(&key, &value)?;
        },
        
        ConfigCommands::Get { key } => {
            if let Some(value) = get_config_value(&key)? {
                println!("{}", value);
            } else {
                eprintln!("Configuration key '{}' not found", key);
                std::process::exit(1);
            }
        },
        
        ConfigCommands::Reset { confirm } => {
            if !confirm {
                eprintln!("Use --confirm to reset configuration");
                std::process::exit(1);
            }
            
            reset_config()?;
            println!("Configuration reset to defaults");
        },
    }
    
    Ok(())
}
```

### 3. Help System Integration

```rust
// Custom help for complex commands
fn print_usage_examples() {
    println!("Examples:");
    println!("  # Convert JSON to YAML");
    println!("  dev-tools file convert -i data.json -f yaml");
    println!();
    println!("  # Watch TypeScript files");
    println!("  dev-tools file watch --pattern '*.ts' --pattern '*.tsx'");
    println!();
    println!("  # Start development server");
    println!("  dev-tools serve --port 3000 --hot-reload");
    println!();
    println!("  # Set configuration");
    println!("  dev-tools config set editor.command 'code'");
}
```

### 4. Testing Subcommands

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    
    #[test]
    fn test_cli_parsing() {
        // Test that all subcommands parse correctly
        let cmd = Cli::command();
        
        // Test file convert command
        let args = vec![
            "dev-tools", "file", "convert", 
            "--input", "test.json", "--format", "yaml"
        ];
        
        let matches = cmd.clone().try_get_matches_from(args);
        assert!(matches.is_ok());
        
        // Test config set command
        let args = vec![
            "dev-tools", "config", "set", "editor", "vim"
        ];
        
        let matches = cmd.clone().try_get_matches_from(args);
        assert!(matches.is_ok());
    }
    
    #[test]
    fn test_help_generation() {
        // Ensure help can be generated for all subcommands
        let cmd = Cli::command();
        let help = cmd.render_help();
        assert!(help.to_string().contains("file"));
        assert!(help.to_string().contains("config"));
    }
}
```

### Usage Examples

```bash
# File operations
dev-tools file convert -i data.json -f yaml -o output.yaml
dev-tools file validate *.toml --fail-fast
dev-tools file watch . --pattern "*.rs" --pattern "*.toml"

# Configuration
dev-tools config show
dev-tools config set editor.command "code"
dev-tools config get editor.command

# Project management
dev-tools project init my-app --template rust
dev-tools project build --profile release --optimize

# Development server
dev-tools serve --port 3000 --hot-reload

# Global flags work with any command
dev-tools --verbose file convert -i test.json -f yaml
dev-tools --config ./custom.toml project build
```

This structure provides intuitive command grouping while maintaining the flexibility to add new commands easily!