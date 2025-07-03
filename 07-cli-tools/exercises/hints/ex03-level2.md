# Exercise 3 Hints - Level 2: Specific Guidance 🟡

## 🎯 Specific Solutions

You've tried Level 1 hints but need more guidance. Here are specific implementation patterns for configuration management:

### 1. Implementing Default Configuration

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    database_url: String,
    max_connections: u32,
    host: String,
    port: u16,
    debug_mode: bool,
    log_level: String,
    api_key: Option<String>,
}

// Implement Default trait for sensible fallbacks
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "sqlite://app.db".to_string(),
            max_connections: 10,
            host: "localhost".to_string(),
            port: 8080,
            debug_mode: false,
            log_level: "info".to_string(),
            api_key: None,
        }
    }
}
```

### 2. Configuration Loading Pattern

```rust
use std::fs;
use std::path::Path;
use std::env;

#[derive(Debug)]
enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    ValidationError(String),
}

impl Config {
    // Load configuration with precedence: CLI > ENV > File > Defaults
    fn load(config_file: Option<&Path>) -> Result<Self, ConfigError> {
        // 1. Start with defaults
        let mut config = Config::default();
        
        // 2. Load from file if provided
        if let Some(file_path) = config_file {
            config = Self::load_from_file(file_path)?;
        }
        
        // 3. Override with environment variables
        config = Self::apply_env_overrides(config)?;
        
        // 4. Validate final configuration
        config.validate()?;
        
        Ok(config)
    }
    
    fn load_from_file(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound(path.display().to_string()))?;
        
        toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(format!("TOML parse error: {}", e)))
    }
}
```

### 3. Environment Variable Overrides

```rust
impl Config {
    fn apply_env_overrides(mut self) -> Result<Self, ConfigError> {
        // Standard environment variables
        if let Ok(database_url) = env::var("DATABASE_URL") {
            self.database_url = database_url;
        }
        
        if let Ok(port) = env::var("PORT") {
            self.port = port.parse()
                .map_err(|_| ConfigError::ValidationError("Invalid PORT value".to_string()))?;
        }
        
        // App-specific environment variables with prefix
        if let Ok(host) = env::var("MYAPP_HOST") {
            self.host = host;
        }
        
        if let Ok(max_conn) = env::var("MYAPP_MAX_CONNECTIONS") {
            self.max_connections = max_conn.parse()
                .map_err(|_| ConfigError::ValidationError("Invalid MAX_CONNECTIONS".to_string()))?;
        }
        
        if let Ok(debug) = env::var("MYAPP_DEBUG") {
            self.debug_mode = debug.to_lowercase() == "true" || debug == "1";
        }
        
        if let Ok(log_level) = env::var("MYAPP_LOG_LEVEL") {
            self.log_level = log_level;
        }
        
        if let Ok(api_key) = env::var("MYAPP_API_KEY") {
            self.api_key = Some(api_key);
        }
        
        Ok(self)
    }
}
```

### 4. Configuration Validation

```rust
impl Config {
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate port range
        if self.port == 0 || self.port < 1024 {
            return Err(ConfigError::ValidationError(
                format!("Invalid port {}: must be >= 1024 for non-root users", self.port)
            ));
        }
        
        // Validate max_connections
        if self.max_connections == 0 || self.max_connections > 1000 {
            return Err(ConfigError::ValidationError(
                format!("Invalid max_connections {}: must be 1-1000", self.max_connections)
            ));
        }
        
        // Validate log level
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            return Err(ConfigError::ValidationError(
                format!("Invalid log_level '{}': must be one of {:?}", 
                       self.log_level, valid_levels)
            ));
        }
        
        // Validate database URL format (basic check)
        if self.database_url.is_empty() {
            return Err(ConfigError::ValidationError(
                "database_url cannot be empty".to_string()
            ));
        }
        
        Ok(())
    }
}
```

### 5. Integration with CLI Arguments

```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Configuration file path
    #[clap(short, long)]
    config: Option<PathBuf>,
    
    /// Override host (highest precedence)
    #[clap(long)]
    host: Option<String>,
    
    /// Override port (highest precedence)
    #[clap(long)]
    port: Option<u16>,
    
    /// Enable debug mode
    #[clap(long)]
    debug: bool,
}

impl Config {
    fn apply_cli_overrides(mut self, cli: &Cli) -> Self {
        // CLI arguments have highest precedence
        if let Some(host) = &cli.host {
            self.host = host.clone();
        }
        
        if let Some(port) = cli.port {
            self.port = port;
        }
        
        if cli.debug {
            self.debug_mode = true;
            self.log_level = "debug".to_string();
        }
        
        self
    }
}
```

### 6. Error Display and User Guidance

```rust
use std::fmt;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {}\n", path)?;
                write!(f, "Suggestion: Create a config file or check the path")
            },
            ConfigError::ParseError(msg) => {
                write!(f, "Configuration parse error: {}\n", msg)?;
                write!(f, "Suggestion: Check TOML syntax and field names")
            },
            ConfigError::ValidationError(msg) => {
                write!(f, "Configuration validation failed: {}\n", msg)?;
                write!(f, "Suggestion: Check the configuration values are within valid ranges")
            },
        }
    }
}

impl std::error::Error for ConfigError {}
```

### 7. Complete Configuration Loading Function

```rust
fn load_configuration(cli: &Cli) -> Result<Config, ConfigError> {
    println!("Loading configuration...");
    
    // Load base configuration (defaults + file + env)
    let mut config = Config::load(cli.config.as_deref())?;
    
    // Apply CLI overrides (highest precedence)
    config = config.apply_cli_overrides(cli);
    
    // Final validation after all overrides
    config.validate()?;
    
    if config.debug_mode {
        println!("Final configuration: {:#?}", config);
    }
    
    Ok(config)
}
```

### 8. Example TOML Configuration File

Create a sample `config.toml` file:

```toml
# Database configuration
database_url = "postgresql://user:pass@localhost/mydb"
max_connections = 25

# Server configuration
host = "0.0.0.0"
port = 3000

# Logging
debug_mode = false
log_level = "info"

# Optional API key
# api_key = "your-secret-key-here"
```

### C# Comparison

```csharp
// C# with IConfiguration
var builder = new ConfigurationBuilder()
    .SetBasePath(Directory.GetCurrentDirectory())
    .AddJsonFile("appsettings.json", optional: true)
    .AddJsonFile($"appsettings.{environment}.json", optional: true)
    .AddEnvironmentVariables()
    .AddCommandLine(args);

var config = builder.Build();
```

In Rust, we implement this pattern manually but with more type safety and better error handling.

### Testing Your Configuration

```bash
# Test with default values
cargo run --bin ex03-config-files

# Test with config file
cargo run --bin ex03-config-files -- --config config.toml

# Test with environment variables
MYAPP_HOST=0.0.0.0 MYAPP_PORT=9000 cargo run --bin ex03-config-files

# Test with CLI overrides
cargo run --bin ex03-config-files -- --host 127.0.0.1 --port 8000 --debug

# Test error cases
cargo run --bin ex03-config-files -- --port 80  # Should fail validation
```

The key insight is building a **configuration hierarchy** where each layer can override the previous one, just like ASP.NET Core's configuration system!

## ⏱️ Still Stuck?

If you've been working for **20+ minutes** and configuration loading still isn't working, check Level 3 hints for a complete implementation.

## 🎯 What Should Work Now

After implementing these patterns, you should have:
- ✅ Default configuration that works out of the box
- ✅ TOML file loading with proper error handling
- ✅ Environment variable overrides
- ✅ CLI argument precedence
- ✅ Helpful error messages with suggestions

## 🚀 Next Steps

1. Test each configuration source independently
2. Verify the precedence chain works correctly
3. Add comprehensive validation
4. Test error scenarios with invalid values
5. Move on to exercise 4 when everything compiles and runs!