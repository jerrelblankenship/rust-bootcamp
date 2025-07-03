# Exercise 2 - Level 3 Hint (Near-Complete Solution)

## Complete Error Handling Solution

You've made good progress! Here's a nearly complete solution showing proper error handling patterns for CLI tools.

### 1. Custom Error Type with Context

```rust
use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: PathBuf },
    
    #[error("Invalid configuration: {message}")]
    ConfigInvalid { message: String },
    
    #[error("File operation failed on '{path}': {source}")]
    FileOperation {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
}
```

### 2. Helper Functions for Common Errors

```rust
impl CliError {
    pub fn file_not_found(path: impl Into<PathBuf>) -> Self {
        CliError::ConfigNotFound { path: path.into() }
    }
    
    pub fn invalid_config(msg: impl Into<String>) -> Self {
        CliError::ConfigInvalid { message: msg.into() }
    }
}
```

### 3. Context Extension for Better Error Messages

```rust
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .context(format!("Failed to read config file '{}'", path))?;
    
    let config: Config = toml::from_str(&contents)
        .context("Failed to parse TOML configuration")?;
    
    validate_config(&config)
        .context("Configuration validation failed")?;
    
    Ok(config)
}
```

### 4. User-Friendly Error Display

```rust
fn main() {
    if let Err(e) = run() {
        // Show the main error
        eprintln!("{} {}", "Error:".red().bold(), e);
        
        // Show the error chain for context
        let mut source = e.source();
        while let Some(err) = source {
            eprintln!("  {} {}", "Caused by:".yellow(), err);
            source = err.source();
        }
        
        // Provide helpful suggestions
        if let Some(suggestion) = suggest_fix(&e) {
            eprintln!("\n{} {}", "Suggestion:".green().bold(), suggestion);
        }
        
        std::process::exit(1);
    }
}

fn suggest_fix(error: &anyhow::Error) -> Option<String> {
    // Downcast to check specific error types
    if let Some(cli_err) = error.downcast_ref::<CliError>() {
        match cli_err {
            CliError::ConfigNotFound { path } => {
                Some(format!(
                    "Create a config file at '{}' or specify --config",
                    path.display()
                ))
            }
            CliError::InvalidInput { message } if message.contains("port") => {
                Some("Port must be between 1 and 65535".to_string())
            }
            _ => None,
        }
    } else if error.to_string().contains("Permission denied") {
        Some("Try running with elevated permissions (sudo)".to_string())
    } else {
        None
    }
}
```

### 5. Validation with Helpful Errors

```rust
fn validate_port(port: u16) -> Result<u16, CliError> {
    if port == 0 {
        Err(CliError::InvalidInput {
            message: "Port cannot be 0".to_string(),
        })
    } else if port < 1024 {
        Err(CliError::InvalidInput {
            message: format!(
                "Port {} is privileged. Use a port >= 1024 or run as root",
                port
            ),
        })
    } else {
        Ok(port)
    }
}
```

### 6. Complete Working Example

```rust
use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use thiserror::Error;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    config: Option<String>,
    
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // Validate port with helpful error
    let port = validate_port(cli.port)?;
    
    // Load config with detailed error context
    let config_path = cli.config.unwrap_or_else(|| "config.toml".to_string());
    let config = read_config(&config_path)?;
    
    println!("{} on port {}", "Server starting".green(), port);
    
    // Simulate work that might fail
    process_data(&config)
        .context("Failed to process data")?;
    
    Ok(())
}

fn process_data(config: &Config) -> Result<()> {
    // Simulate potential failures with good error messages
    if config.database_url.is_empty() {
        return Err(CliError::ConfigInvalid {
            message: "database_url cannot be empty".to_string(),
        }.into());
    }
    
    // More processing...
    Ok(())
}
```

### Key Patterns Used:

1. **Custom Error Types**: Use `thiserror` for domain-specific errors
2. **Error Context**: Use `anyhow::Context` to add helpful context
3. **Error Chains**: Show the full chain of errors to users
4. **Suggestions**: Provide actionable fix suggestions
5. **Colored Output**: Use colors to highlight important information
6. **Exit Codes**: Always exit with non-zero on error

### C# Comparison:

```csharp
// C# approach
try {
    var config = LoadConfig(configPath);
    ValidatePort(port);
} catch (FileNotFoundException ex) {
    Console.Error.WriteLine($"Error: Config file not found: {ex.FileName}");
    Console.Error.WriteLine("Suggestion: Create a config file or use --config");
    Environment.Exit(1);
}

// Rust approach - errors as values, not exceptions
let config = read_config(&config_path)
    .context("Failed to load configuration")?;
```

### Testing Your Error Handling:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_port_validation() {
        assert!(validate_port(0).is_err());
        assert!(validate_port(80).is_err()); // Privileged
        assert!(validate_port(8080).is_ok());
    }
    
    #[test]
    fn test_error_display() {
        let err = CliError::ConfigNotFound {
            path: PathBuf::from("/etc/app.conf"),
        };
        assert_eq!(
            err.to_string(),
            "Configuration file not found: /etc/app.conf"
        );
    }
}
```

Remember: Good error handling is what separates professional CLI tools from amateur ones. Users should never see a panic or cryptic error message!