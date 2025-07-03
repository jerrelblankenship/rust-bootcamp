# Configuration Hierarchies

Best practices for configuration management with proper precedence and validation.

## 🎯 Configuration Philosophy

Good configuration systems follow these principles:
1. **Sensible Defaults** - Tool works without any configuration
2. **Multiple Sources** - Support files, environment variables, and CLI arguments
3. **Clear Precedence** - Later sources override earlier ones
4. **Validation** - Catch configuration errors early with helpful messages
5. **Discoverability** - Users can understand where settings come from

## 📊 Standard Precedence Order

From highest to lowest priority:

1. **Command Line Arguments** - Immediate overrides for testing
2. **Environment Variables** - Deployment-specific settings
3. **Configuration Files** - Project/user preferences
4. **Built-in Defaults** - Fallback values that always work

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}
```

## 🏗️ Configuration Loading Pattern

### Layered Configuration Builder

```rust
use config::{Config as ConfigBuilder, ConfigError, Environment, File};

pub fn load_config() -> Result<Config, ConfigError> {
    let mut builder = ConfigBuilder::builder()
        // 1. Start with defaults
        .add_source(Config::default())
        
        // 2. Add system-wide config
        .add_source(File::with_name("/etc/myapp/config").required(false))
        
        // 3. Add user config
        .add_source(File::with_name(&format!("{}/.config/myapp/config", 
            dirs::home_dir().unwrap().display())).required(false))
        
        // 4. Add local project config
        .add_source(File::with_name("./myapp.toml").required(false))
        .add_source(File::with_name("./config/myapp.toml").required(false))
        
        // 5. Add environment variables (with prefix)
        .add_source(Environment::with_prefix("MYAPP").separator("_"))
        
        // 6. Add command line overrides (handled separately)
        ;
    
    // Build and validate
    let config: Config = builder.build()?.try_deserialize()?;
    config.validate()?;
    Ok(config)
}
```

### Manual Implementation

```rust
impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // 1. Start with defaults
        let mut config = Config::default();
        
        // 2. Layer on file configs (multiple locations)
        for path in Self::config_file_locations() {
            if path.exists() {
                let file_config = Self::from_file(&path)?;
                config = config.merge(file_config);
                break; // Use first found config file
            }
        }
        
        // 3. Layer on environment variables
        config = config.merge_env()?;
        
        // 4. Validate final configuration
        config.validate()?;
        
        Ok(config)
    }
    
    fn config_file_locations() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // Command line specified
        if let Some(config_path) = std::env::args()
            .find(|arg| arg.starts_with("--config="))
            .map(|arg| PathBuf::from(arg.strip_prefix("--config=").unwrap()))
        {
            paths.push(config_path);
        }
        
        // Environment variable
        if let Ok(config_path) = std::env::var("MYAPP_CONFIG_FILE") {
            paths.push(PathBuf::from(config_path));
        }
        
        // Standard locations
        paths.extend([
            PathBuf::from("./myapp.toml"),
            PathBuf::from("./config/myapp.toml"),
        ]);
        
        // User config directory
        if let Some(config_dir) = dirs::config_dir() {
            paths.push(config_dir.join("myapp").join("config.toml"));
        }
        
        // System config
        #[cfg(unix)]
        paths.push(PathBuf::from("/etc/myapp/config.toml"));
        
        paths
    }
}
```

## 🔧 Environment Variable Mapping

### Nested Configuration

```rust
// Environment variables with nested structure
// MYAPP_SERVER_HOST=localhost
// MYAPP_SERVER_PORT=8080
// MYAPP_DATABASE_URL=postgres://...

use std::env;

impl Config {
    fn merge_env(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        // Server configuration
        if let Ok(host) = env::var("MYAPP_SERVER_HOST") {
            self.server.host = host;
        }
        if let Ok(port_str) = env::var("MYAPP_SERVER_PORT") {
            self.server.port = port_str.parse()
                .map_err(|_| format!("Invalid port in MYAPP_SERVER_PORT: {}", port_str))?;
        }
        
        // Database configuration
        if let Ok(db_url) = env::var("MYAPP_DATABASE_URL") {
            self.database.url = db_url;
        }
        if let Ok(pool_size_str) = env::var("MYAPP_DATABASE_POOL_SIZE") {
            self.database.pool_size = pool_size_str.parse()
                .map_err(|_| format!("Invalid pool size in MYAPP_DATABASE_POOL_SIZE: {}", pool_size_str))?;
        }
        
        // Logging configuration
        if let Ok(log_level) = env::var("MYAPP_LOG_LEVEL") {
            self.logging.level = log_level;
        }
        
        Ok(self)
    }
}
```

### Standard Environment Variables

```rust
impl Config {
    fn apply_standard_env_vars(mut self) -> Self {
        // Standard environment variables that many tools recognize
        if let Ok(port) = env::var("PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                self.server.port = port_num;
            }
        }
        
        if let Ok(host) = env::var("HOST") {
            self.server.host = host;
        }
        
        if let Ok(database_url) = env::var("DATABASE_URL") {
            self.database.url = database_url;
        }
        
        if let Ok(log_level) = env::var("RUST_LOG") {
            // Parse RUST_LOG format: "debug", "myapp=info", etc.
            if let Some(level) = parse_rust_log_level(&log_level) {
                self.logging.level = level;
            }
        }
        
        self
    }
}
```

## ✅ Configuration Validation

### Comprehensive Validation

```rust
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl Config {
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // Validate server configuration
        if self.server.port == 0 {
            errors.push(ValidationError {
                field: "server.port".to_string(),
                message: "Port cannot be 0".to_string(),
                suggestion: Some("Use a port between 1024-65535 for user applications".to_string()),
            });
        }
        
        if self.server.port < 1024 && !is_privileged_user() {
            errors.push(ValidationError {
                field: "server.port".to_string(),
                message: format!("Port {} requires elevated privileges", self.server.port),
                suggestion: Some("Use a port >= 1024 or run with appropriate privileges".to_string()),
            });
        }
        
        if self.server.host.is_empty() {
            errors.push(ValidationError {
                field: "server.host".to_string(),
                message: "Host cannot be empty".to_string(),
                suggestion: Some("Use 'localhost' for local development or '0.0.0.0' for all interfaces".to_string()),
            });
        }
        
        // Validate database configuration
        if self.database.url.is_empty() {
            errors.push(ValidationError {
                field: "database.url".to_string(),
                message: "Database URL is required".to_string(),
                suggestion: Some("Set DATABASE_URL environment variable or add to config file".to_string()),
            });
        } else if !is_valid_database_url(&self.database.url) {
            errors.push(ValidationError {
                field: "database.url".to_string(),
                message: "Invalid database URL format".to_string(),
                suggestion: Some("Use format: postgres://user:pass@host:port/dbname".to_string()),
            });
        }
        
        // Validate logging configuration
        match self.logging.level.as_str() {
            "debug" | "info" | "warn" | "error" => {},
            _ => errors.push(ValidationError {
                field: "logging.level".to_string(),
                message: format!("Invalid log level: '{}'", self.logging.level),
                suggestion: Some("Valid levels: debug, info, warn, error".to_string()),
            }),
        }
        
        // Validate file paths exist
        if !self.logging.file.as_os_str().is_empty() && !self.logging.file.parent().map_or(true, |p| p.exists()) {
            errors.push(ValidationError {
                field: "logging.file".to_string(),
                message: format!("Log file directory does not exist: {:?}", self.logging.file.parent()),
                suggestion: Some("Create the directory or specify a different log file path".to_string()),
            });
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn format_validation_errors(errors: &[ValidationError]) -> String {
    let mut output = String::new();
    output.push_str("❌ Configuration validation failed:\n\n");
    
    for (i, error) in errors.iter().enumerate() {
        output.push_str(&format!("{}. {} ({})\n", i + 1, error.message, error.field));
        if let Some(suggestion) = &error.suggestion {
            output.push_str(&format!("   💡 {}\n", suggestion));
        }
        output.push('\n');
    }
    
    output.push_str("📋 Fix these issues and try again.\n");
    output
}
```

## 🔍 Configuration Debugging

### Configuration Source Tracking

```rust
#[derive(Debug)]
pub struct ConfigSource {
    pub value: String,
    pub source: String,
    pub precedence: u8,
}

#[derive(Debug)]
pub struct TrackedConfig {
    pub config: Config,
    pub sources: std::collections::HashMap<String, ConfigSource>,
}

impl TrackedConfig {
    pub fn show_sources(&self) {
        println!("📋 Configuration Sources:");
        println!("{:<30} {:<20} {:<10} {}", "Setting", "Value", "Source", "Precedence");
        println!("{}", "-".repeat(80));
        
        let mut sources: Vec<_> = self.sources.iter().collect();
        sources.sort_by_key(|(_, source)| source.precedence);
        
        for (key, source) in sources {
            println!("{:<30} {:<20} {:<10} {}", 
                key, 
                source.value, 
                source.source, 
                source.precedence
            );
        }
    }
    
    pub fn explain_setting(&self, key: &str) {
        if let Some(source) = self.sources.get(key) {
            println!("Setting: {}", key);
            println!("Value: {}", source.value);
            println!("Source: {}", source.source);
            println!("Precedence: {} (higher numbers override lower)", source.precedence);
        } else {
            println!("Setting '{}' not found", key);
        }
    }
}
```

### Configuration Diff

```rust
pub fn show_config_diff(default: &Config, current: &Config) {
    println!("🔍 Configuration Changes from Defaults:");
    
    if default.server.port != current.server.port {
        println!("  server.port: {} → {}", default.server.port, current.server.port);
    }
    
    if default.server.host != current.server.host {
        println!("  server.host: {} → {}", default.server.host, current.server.host);
    }
    
    if default.database.url != current.database.url {
        println!("  database.url: {} → {}", 
            if default.database.url.is_empty() { "<empty>" } else { &default.database.url },
            if current.database.url.is_empty() { "<empty>" } else { &current.database.url }
        );
    }
    
    // Add more fields as needed
}
```

## 🛠️ Configuration Management Commands

### Configuration Subcommands

```rust
#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show {
        /// Show configuration sources
        #[clap(long)]
        sources: bool,
        
        /// Show only non-default values
        #[clap(long)]
        diff: bool,
    },
    
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., server.port)
        key: String,
        /// Configuration value
        value: String,
    },
    
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    
    /// Validate current configuration
    Validate,
    
    /// Reset configuration to defaults
    Reset {
        /// Skip confirmation prompt
        #[clap(short, long)]
        yes: bool,
    },
    
    /// Generate example configuration file
    Init {
        /// Output file path
        #[clap(short, long, default_value = "config.toml")]
        output: PathBuf,
        
        /// Overwrite existing file
        #[clap(long)]
        force: bool,
    },
}

pub fn handle_config_command(cmd: ConfigCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ConfigCommands::Show { sources, diff } => {
            let config = Config::load()?;
            
            if sources {
                let tracked = TrackedConfig::load()?;
                tracked.show_sources();
            } else if diff {
                let default = Config::default();
                show_config_diff(&default, &config);
            } else {
                println!("{}", toml::to_string_pretty(&config)?);
            }
        },
        
        ConfigCommands::Set { key, value } => {
            set_config_value(&key, &value)?;
            println!("✅ Set {}: {}", key, value);
        },
        
        ConfigCommands::Get { key } => {
            let config = Config::load()?;
            if let Some(value) = get_config_value(&config, &key) {
                println!("{}", value);
            } else {
                return Err(format!("Configuration key '{}' not found", key).into());
            }
        },
        
        ConfigCommands::Validate => {
            let config = Config::load()?;
            match config.validate() {
                Ok(()) => println!("✅ Configuration is valid"),
                Err(errors) => {
                    eprintln!("{}", format_validation_errors(&errors));
                    std::process::exit(1);
                }
            }
        },
        
        ConfigCommands::Reset { yes } => {
            if !yes {
                if !confirm_reset()? {
                    println!("Reset cancelled");
                    return Ok(());
                }
            }
            
            reset_config()?;
            println!("✅ Configuration reset to defaults");
        },
        
        ConfigCommands::Init { output, force } => {
            if output.exists() && !force {
                return Err(format!("File '{}' already exists. Use --force to overwrite", output.display()).into());
            }
            
            let default_config = Config::default();
            let toml_content = toml::to_string_pretty(&default_config)?;
            std::fs::write(&output, toml_content)?;
            
            println!("✅ Created configuration file: {}", output.display());
            println!("📝 Edit the file and restart the application to apply changes");
        },
    }
    
    Ok(())
}
```

## 📋 Configuration Best Practices

### Security Considerations
- [ ] Never log sensitive configuration values
- [ ] Use environment variables for secrets in production
- [ ] Validate file permissions on config files
- [ ] Support encrypted configuration for sensitive data
- [ ] Provide ways to test configuration without applying it

### User Experience
- [ ] Provide clear error messages for invalid configurations
- [ ] Show where each configuration value comes from
- [ ] Support configuration validation without running the application
- [ ] Offer configuration templates for common scenarios
- [ ] Document all configuration options with examples

### Maintainability
- [ ] Use strong typing for configuration structures
- [ ] Validate configuration at startup, not during operation
- [ ] Support configuration hot-reloading when appropriate
- [ ] Version configuration file formats for backwards compatibility
- [ ] Provide migration tools for configuration format changes

## 🎓 Key Takeaways

1. **Start Simple** - Provide good defaults so the tool works without configuration
2. **Layer Gradually** - Build configuration from multiple sources with clear precedence
3. **Validate Early** - Catch configuration errors at startup with helpful messages
4. **Make it Discoverable** - Users should understand where settings come from
5. **Plan for Change** - Configuration needs will evolve over time
6. **Test Thoroughly** - Verify configuration loading from all sources and error conditions

Remember: Good configuration makes your tool adaptable to different environments while remaining simple to use!