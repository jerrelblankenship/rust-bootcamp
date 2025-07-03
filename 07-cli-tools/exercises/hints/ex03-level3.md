# Exercise 3 Hints - Level 3: Working Solution 🔴

## ⚠️ Last Resort Guidance

**Use these hints only if Level 1 and Level 2 didn't help!** This contains a nearly complete solution.

**Important**: Try to understand each part rather than just copying. The goal is learning configuration patterns, not just completing the exercise.

## 🛠️ Complete Configuration Management Implementation

Here's a comprehensive, production-ready configuration system that handles all the requirements:

### Complete Working Solution

```rust
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;

// Main CLI structure
#[derive(Parser)]
#[clap(name = "config-demo", version = "1.0", about = "Configuration management demo")]
struct Cli {
    /// Configuration file path
    #[clap(short, long, help = "Path to TOML configuration file")]
    config: Option<PathBuf>,
    
    /// Override host address
    #[clap(long, help = "Server host address")]
    host: Option<String>,
    
    /// Override port number
    #[clap(long, help = "Server port number")]
    port: Option<u16>,
    
    /// Override database URL
    #[clap(long, help = "Database connection URL")]
    database_url: Option<String>,
    
    /// Enable debug mode
    #[clap(long, help = "Enable debug logging")]
    debug: bool,
    
    /// Set log level
    #[clap(long, help = "Log level (error, warn, info, debug, trace)")]
    log_level: Option<String>,
    
    /// Set API key
    #[clap(long, help = "API key for external services")]
    api_key: Option<String>,
    
    /// Show final configuration and exit
    #[clap(long, help = "Display resolved configuration")]
    show_config: bool,
}

// Configuration structure with validation
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    /// Database connection URL
    database_url: String,
    
    /// Maximum database connections
    max_connections: u32,
    
    /// Server host address
    host: String,
    
    /// Server port number
    port: u16,
    
    /// Debug mode flag
    debug_mode: bool,
    
    /// Logging level
    log_level: String,
    
    /// Optional API key
    api_key: Option<String>,
}

// Comprehensive error handling
#[derive(Debug)]
enum ConfigError {
    FileNotFound { path: String },
    ParseError { message: String, file: Option<String> },
    ValidationError { field: String, value: String, reason: String },
    EnvironmentError { variable: String, value: String },
    IoError(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::FileNotFound { path } => {
                writeln!(f, "❌ Configuration file not found: {}", path)?;
                writeln!(f, "💡 Suggestions:")?;
                writeln!(f, "   • Create a config file at {}", path)?;
                writeln!(f, "   • Use --config to specify a different path")?;
                write!(f, "   • Run without --config to use defaults + environment variables")
            },
            ConfigError::ParseError { message, file } => {
                writeln!(f, "❌ Configuration parse error: {}", message)?;
                if let Some(file_path) = file {
                    writeln!(f, "📁 File: {}", file_path)?;
                }
                writeln!(f, "💡 Suggestions:")?;
                writeln!(f, "   • Check TOML syntax (quotes, brackets, etc.)")?;
                write!(f, "   • Verify field names match the expected schema")
            },
            ConfigError::ValidationError { field, value, reason } => {
                writeln!(f, "❌ Configuration validation failed")?;
                writeln!(f, "🔧 Field: {}", field)?;
                writeln!(f, "❓ Value: {}", value)?;
                writeln!(f, "⚠️  Reason: {}", reason)?;
                write!(f, "💡 Suggestion: Check the field requirements and try again")
            },
            ConfigError::EnvironmentError { variable, value } => {
                writeln!(f, "❌ Invalid environment variable: {}", variable)?;
                writeln!(f, "❓ Value: {}", value)?;
                write!(f, "💡 Suggestion: Check the variable format and valid values")
            },
            ConfigError::IoError(err) => {
                writeln!(f, "❌ File system error: {}", err)?;
                write!(f, "💡 Suggestion: Check file permissions and path accessibility")
            },
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

// Default configuration implementation
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "sqlite://./app.db".to_string(),
            max_connections: 10,
            host: "localhost".to_string(),
            port: 8080,
            debug_mode: false,
            log_level: "info".to_string(),
            api_key: None,
        }
    }
}

impl Config {
    /// Load configuration with full precedence chain
    fn load(config_file: Option<&Path>) -> Result<Self, ConfigError> {
        // Step 1: Start with defaults
        let mut config = Config::default();
        
        // Step 2: Load from file if specified
        if let Some(file_path) = config_file {
            config = Self::load_from_file(file_path)?;
        }
        
        // Step 3: Apply environment variable overrides
        config = Self::apply_env_overrides(config)?;
        
        // Step 4: Validate the complete configuration
        config.validate()?;
        
        Ok(config)
    }
    
    /// Load configuration from TOML file
    fn load_from_file(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound { 
                path: path.display().to_string() 
            })?;
        
        toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError { 
                message: e.to_string(),
                file: Some(path.display().to_string()),
            })
    }
    
    /// Apply environment variable overrides
    fn apply_env_overrides(mut self) -> Result<Self, ConfigError> {
        // Standard environment variables (following common conventions)
        if let Ok(database_url) = env::var("DATABASE_URL") {
            self.database_url = database_url;
        }
        
        if let Ok(port_str) = env::var("PORT") {
            self.port = port_str.parse()
                .map_err(|_| ConfigError::EnvironmentError {
                    variable: "PORT".to_string(),
                    value: port_str,
                })?;
        }
        
        // App-specific environment variables with consistent prefix
        if let Ok(host) = env::var("MYAPP_HOST") {
            self.host = host;
        }
        
        if let Ok(max_conn_str) = env::var("MYAPP_MAX_CONNECTIONS") {
            self.max_connections = max_conn_str.parse()
                .map_err(|_| ConfigError::EnvironmentError {
                    variable: "MYAPP_MAX_CONNECTIONS".to_string(),
                    value: max_conn_str,
                })?;
        }
        
        if let Ok(debug_str) = env::var("MYAPP_DEBUG") {
            self.debug_mode = matches!(debug_str.to_lowercase().as_str(), "true" | "1" | "yes" | "on");
        }
        
        if let Ok(log_level) = env::var("MYAPP_LOG_LEVEL") {
            self.log_level = log_level;
        }
        
        if let Ok(api_key) = env::var("MYAPP_API_KEY") {
            self.api_key = Some(api_key);
        }
        
        Ok(self)
    }
    
    /// Apply command-line argument overrides (highest precedence)
    fn apply_cli_overrides(mut self, cli: &Cli) -> Self {
        if let Some(host) = &cli.host {
            self.host = host.clone();
        }
        
        if let Some(port) = cli.port {
            self.port = port;
        }
        
        if let Some(database_url) = &cli.database_url {
            self.database_url = database_url.clone();
        }
        
        if cli.debug {
            self.debug_mode = true;
            // Automatically increase log level when debug is enabled
            if self.log_level == "info" {
                self.log_level = "debug".to_string();
            }
        }
        
        if let Some(log_level) = &cli.log_level {
            self.log_level = log_level.clone();
        }
        
        if let Some(api_key) = &cli.api_key {
            self.api_key = Some(api_key.clone());
        }
        
        self
    }
    
    /// Comprehensive configuration validation
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate port range
        if self.port == 0 {
            return Err(ConfigError::ValidationError {
                field: "port".to_string(),
                value: self.port.to_string(),
                reason: "Port cannot be 0".to_string(),
            });
        }
        
        if self.port < 1024 && !Self::is_running_as_root() {
            return Err(ConfigError::ValidationError {
                field: "port".to_string(),
                value: self.port.to_string(),
                reason: "Ports below 1024 require root privileges".to_string(),
            });
        }
        
        // Validate max_connections range
        if self.max_connections == 0 {
            return Err(ConfigError::ValidationError {
                field: "max_connections".to_string(),
                value: self.max_connections.to_string(),
                reason: "Must be at least 1".to_string(),
            });
        }
        
        if self.max_connections > 1000 {
            return Err(ConfigError::ValidationError {
                field: "max_connections".to_string(),
                value: self.max_connections.to_string(),
                reason: "Cannot exceed 1000 connections".to_string(),
            });
        }
        
        // Validate log level
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            return Err(ConfigError::ValidationError {
                field: "log_level".to_string(),
                value: self.log_level.clone(),
                reason: format!("Must be one of: {}", valid_levels.join(", ")),
            });
        }
        
        // Validate database URL format
        if self.database_url.is_empty() {
            return Err(ConfigError::ValidationError {
                field: "database_url".to_string(),
                value: "".to_string(),
                reason: "Database URL cannot be empty".to_string(),
            });
        }
        
        // Basic database URL format validation
        let valid_schemes = ["sqlite://", "postgresql://", "mysql://"];
        if !valid_schemes.iter().any(|scheme| self.database_url.starts_with(scheme)) {
            return Err(ConfigError::ValidationError {
                field: "database_url".to_string(),
                value: self.database_url.clone(),
                reason: format!("Must start with one of: {}", valid_schemes.join(", ")),
            });
        }
        
        // Validate host format (basic check)
        if self.host.is_empty() {
            return Err(ConfigError::ValidationError {
                field: "host".to_string(),
                value: "".to_string(),
                reason: "Host cannot be empty".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Check if running as root (Unix systems)
    #[cfg(unix)]
    fn is_running_as_root() -> bool {
        unsafe { libc::geteuid() == 0 }
    }
    
    #[cfg(not(unix))]
    fn is_running_as_root() -> bool {
        false // Windows doesn't have the same port restrictions
    }
    
    /// Pretty-print configuration for debugging
    fn display(&self) {
        println!("📋 Current Configuration:");
        println!("┌─────────────────────────────────────┐");
        println!("│ Database Settings                   │");
        println!("├─────────────────────────────────────┤");
        println!("│ URL: {:30} │", self.database_url);
        println!("│ Max Connections: {:18} │", self.max_connections);
        println!("├─────────────────────────────────────┤");
        println!("│ Server Settings                     │");
        println!("├─────────────────────────────────────┤");
        println!("│ Host: {:30} │", self.host);
        println!("│ Port: {:30} │", self.port);
        println!("├─────────────────────────────────────┤");
        println!("│ Runtime Settings                    │");
        println!("├─────────────────────────────────────┤");
        println!("│ Debug Mode: {:24} │", self.debug_mode);
        println!("│ Log Level: {:25} │", self.log_level);
        println!("│ API Key: {:27} │", 
                 self.api_key.as_ref().map_or("None".to_string(), |_| "*****".to_string()));
        println!("└─────────────────────────────────────┘");
    }
}

// Main function with complete error handling
fn main() {
    let cli = Cli::parse();
    
    match load_configuration(&cli) {
        Ok(config) => {
            if cli.show_config {
                config.display();
                return;
            }
            
            if config.debug_mode {
                println!("🐛 Debug mode enabled");
                config.display();
            }
            
            // Use the configuration
            run_application(&config);
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

/// Load configuration with full precedence chain
fn load_configuration(cli: &Cli) -> Result<Config, ConfigError> {
    if !cli.show_config {
        println!("⚙️  Loading configuration...");
    }
    
    // Load base configuration (defaults → file → environment)
    let mut config = Config::load(cli.config.as_deref())?;
    
    // Apply CLI overrides (highest precedence)
    config = config.apply_cli_overrides(cli);
    
    // Final validation after all overrides
    config.validate()?;
    
    if !cli.show_config {
        println!("✅ Configuration loaded successfully");
        
        if config.debug_mode {
            println!("🔍 Configuration sources applied:");
            println!("   1. Default values");
            if cli.config.is_some() {
                println!("   2. Config file: {}", cli.config.as_ref().unwrap().display());
            }
            println!("   3. Environment variables");
            println!("   4. Command line arguments");
        }
    }
    
    Ok(config)
}

/// Simulate running the application with the configuration
fn run_application(config: &Config) {
    println!("🚀 Starting application...");
    println!("📡 Server listening on {}:{}", config.host, config.port);
    println!("🗄️  Database: {}", config.database_url);
    println!("📊 Max connections: {}", config.max_connections);
    println!("📝 Log level: {}", config.log_level);
    
    if config.api_key.is_some() {
        println!("🔑 API key configured");
    }
    
    println!("✨ Application running successfully!");
    
    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("🛑 Application finished");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.log_level, "info");
        assert!(!config.debug_mode);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid port should fail
        config.port = 0;
        assert!(config.validate().is_err());
        
        // Invalid log level should fail
        config.port = 8080;
        config.log_level = "invalid".to_string();
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_env_overrides() {
        env::set_var("MYAPP_HOST", "0.0.0.0");
        env::set_var("MYAPP_DEBUG", "true");
        
        let config = Config::default();
        let config = config.apply_env_overrides().unwrap();
        
        assert_eq!(config.host, "0.0.0.0");
        assert!(config.debug_mode);
        
        env::remove_var("MYAPP_HOST");
        env::remove_var("MYAPP_DEBUG");
    }
    
    #[test]
    fn test_toml_loading() {
        let toml_content = r#"
database_url = "postgresql://test"
max_connections = 20
host = "example.com"
port = 9000
debug_mode = true
log_level = "debug"
"#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        
        let config = Config::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.database_url, "postgresql://test");
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.host, "example.com");
        assert_eq!(config.port, 9000);
        assert!(config.debug_mode);
        assert_eq!(config.log_level, "debug");
    }
}
```

### Example TOML Configuration File

Create a `config.toml` file:

```toml
# Example configuration file for the CLI app
# This demonstrates all supported configuration options

# Database configuration
database_url = "postgresql://user:password@localhost/myapp"
max_connections = 25

# Server configuration
host = "0.0.0.0"  # Listen on all interfaces
port = 3000

# Runtime configuration
debug_mode = false
log_level = "info"  # error, warn, info, debug, trace

# Optional API key for external services
# Uncomment and set your API key
# api_key = "your-secret-key-here"
```

### Testing Your Implementation

```bash
# Test 1: Default configuration
cargo run --bin ex03-config-files

# Test 2: With config file
cargo run --bin ex03-config-files -- --config config.toml

# Test 3: Environment variable overrides
MYAPP_HOST=127.0.0.1 MYAPP_PORT=9000 MYAPP_DEBUG=true cargo run --bin ex03-config-files

# Test 4: CLI argument overrides (highest precedence)
cargo run --bin ex03-config-files -- --host 0.0.0.0 --port 8000 --debug

# Test 5: Show resolved configuration
cargo run --bin ex03-config-files -- --config config.toml --debug --show-config

# Test 6: Error cases
cargo run --bin ex03-config-files -- --port 0      # Should fail validation
cargo run --bin ex03-config-files -- --config nonexistent.toml  # Should show helpful error

# Test 7: Complex precedence chain
MYAPP_HOST=env-host cargo run --bin ex03-config-files -- --config config.toml --host cli-host --show-config
# Should show: cli-host (CLI overrides everything)
```

### Key Implementation Features

1. **🔄 Complete Precedence Chain**: CLI → ENV → File → Defaults
2. **✅ Comprehensive Validation**: Range checking, format validation, privilege checks
3. **🎯 Helpful Error Messages**: Clear explanations and suggestions
4. **🔧 Developer-Friendly**: Debug output, config display, testing utilities
5. **📊 Production-Ready**: Proper error handling, logging, documentation

This implementation provides a robust foundation for CLI configuration management that follows industry best practices!

## 🎓 Ready for Next Challenge

Congratulations! You've mastered configuration management in Rust. You should now understand:

✅ **Configuration Precedence**: CLI → ENV → File → Defaults
✅ **Error Handling**: Helpful messages with suggestions  
✅ **Validation**: Range checking and format validation
✅ **C# Translation**: From IConfiguration to Rust patterns
✅ **Testing**: How to verify configuration behavior

**Your configuration system now has:**
- Sensible defaults that work out of the box
- TOML file loading with error recovery
- Environment variable overrides for deployment
- CLI argument precedence for testing
- Comprehensive validation with helpful errors

## 🚀 Move to Exercise 4

Now that you have solid configuration handling, you're ready to learn about making CLI tools that work well in Unix pipelines. Exercise 4 will teach you how to handle stdin/stdout properly and respect Unix conventions.

Remember: The patterns you learned here (precedence chains, validation, error handling) apply to many areas of Rust development beyond just CLI tools!