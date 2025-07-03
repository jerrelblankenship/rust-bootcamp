// Exercise 3: Configuration Files - Fix the Broken Config Loading!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 config features to fix)
//
// Your task: Fix broken configuration management to support multiple config sources
// This demonstrates proper CLI configuration patterns and precedence handling
//
// INSTRUCTIONS:
// 1. Run program to see it crash when loading config
// 2. Fix config loading from files, environment, and command line
// 3. Test: `cargo run --bin ex03-config-files -- --config myconfig.toml`
// 4. Implement proper config precedence (CLI > ENV > File > Defaults)
//
// LEARNING STRATEGY:
// - Start with basic file loading
// - Add environment variable support
// - Implement precedence rules (like ASP.NET Core configuration)
// - Add validation and helpful error messages
//
// CONFIGURATION FEATURES TO FIX:
// [] Load config from TOML files
// [] Read environment variables  
// [] Handle command-line overrides
// [] Implement proper precedence order
// [] Add config validation
// [] Create default configurations

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// FIXME: This config struct has issues
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    // Database settings
    database_url: String,  // FIXME: Should have default value
    max_connections: u32,  // FIXME: Should validate range
    
    // Server settings  
    host: String,          // FIXME: Should default to localhost
    port: u16,             // FIXME: Should validate port range
    
    // Feature flags
    debug_mode: bool,      // FIXME: Should default to false
    log_level: String,     // FIXME: Should validate valid levels
    
    // Optional settings
    api_key: Option<String>,  // This is correct - optional
}

#[derive(Debug)]
enum ConfigError {
    // TODO: Define configuration error types
    // HINT: Similar to configuration exceptions in C#
}

fn main() {
    // TERRIBLE: This will crash if any config source fails
    let config = load_config().unwrap();  // FIXME: No error handling!
    
    println!("Configuration loaded:");
    println!("{:#?}", config);
    
    // TODO: Use the configuration to actually configure the application
    run_with_config(config);
}

// FIXME: This function panics on missing files
fn load_config() -> Result<Config, ConfigError> {
    // Step 1: Load defaults
    let mut config = Config::default();  // COMPILE ERROR: No Default implementation!
    
    // Step 2: Override with config file
    if let Some(config_path) = find_config_file() {
        // TERRIBLE: Just panics if file is malformed
        let file_config: Config = toml::from_str(&std::fs::read_to_string(config_path).unwrap()).unwrap();
        config = merge_configs(config, file_config);  // COMPILE ERROR: merge_configs doesn't exist!
    }
    
    // Step 3: Override with environment variables
    config = apply_env_vars(config);  // COMPILE ERROR: Function doesn't exist!
    
    // Step 4: Override with command line arguments
    config = apply_cli_args(config);  // COMPILE ERROR: Function doesn't exist!
    
    // Step 5: Validate final configuration
    validate_config(&config)?;  // COMPILE ERROR: Function doesn't exist!
    
    Ok(config)
}

// TODO: Implement Default for Config
impl Default for Config {
    fn default() -> Self {
        // TODO: Provide sensible defaults
        // HINT: Like DefaultValueAttribute in C# or appsettings.json defaults
        todo!("Implement default configuration")
    }
}

// TODO: Find configuration files in standard locations
fn find_config_file() -> Option<PathBuf> {
    // TODO: Look for config files in order:
    // 1. Command line specified config
    // 2. ./config.toml  
    // 3. ~/.config/myapp/config.toml
    // 4. /etc/myapp/config.toml (Unix)
    
    // HINT: Similar to how .NET Core looks for appsettings.json
    
    let possible_paths = vec![
        PathBuf::from("config.toml"),
        // TODO: Add more standard locations
    ];
    
    // FIXME: Actually implement the search logic
    todo!("Search for config files in standard locations")
}

// TODO: Merge configurations with proper precedence
fn merge_configs(base: Config, override_config: Config) -> Config {
    // TODO: Override base config with non-default values from override_config
    // HINT: Similar to IConfiguration in .NET Core - later sources win
    todo!("Implement config merging")
}

// TODO: Apply environment variable overrides
fn apply_env_vars(mut config: Config) -> Config {
    // TODO: Read environment variables with proper naming convention
    // Examples:
    // MYAPP_DATABASE_URL -> config.database_url  
    // MYAPP_PORT -> config.port
    // MYAPP_DEBUG_MODE -> config.debug_mode
    
    // HINT: Similar to Environment.GetEnvironmentVariable() in C#
    
    if let Ok(db_url) = std::env::var("MYAPP_DATABASE_URL") {
        config.database_url = db_url;
    }
    
    // FIXME: Handle port parsing safely
    if let Ok(port_str) = std::env::var("MYAPP_PORT") {
        config.port = port_str.parse().unwrap();  // TERRIBLE: Will panic on bad input!
    }
    
    // TODO: Add remaining environment variable mappings
    todo!("Complete environment variable mapping")
}

// TODO: Apply command line argument overrides  
fn apply_cli_args(config: Config) -> Config {
    // TODO: Parse command line arguments and override config
    // Should support:
    // --database-url <URL>
    // --port <PORT>
    // --debug
    // --log-level <LEVEL>
    
    // HINT: Use clap or manual parsing, similar to command line providers in .NET
    todo!("Implement CLI argument overrides")
}

// TODO: Validate the final configuration
fn validate_config(config: &Config) -> Result<(), ConfigError> {
    // TODO: Validate all configuration values
    // Examples:
    // - Port should be 1-65535
    // - Database URL should be valid
    // - Log level should be valid (debug, info, warn, error)
    // - Max connections should be reasonable (1-1000)
    
    // Port validation
    if config.port == 0 {
        // FIXME: Return proper error
        return Err(ConfigError::InvalidPort);  // COMPILE ERROR: Variant doesn't exist!
    }
    
    // Log level validation  
    match config.log_level.as_str() {
        "debug" | "info" | "warn" | "error" => {},
        _ => {
            // FIXME: Return proper error with valid options
            todo!("Return log level validation error");
        }
    }
    
    // TODO: Add remaining validations
    todo!("Complete configuration validation")
}

// TODO: Actually use the configuration
fn run_with_config(config: Config) {
    println!("🚀 Starting application with configuration:");
    println!("  Database: {}", config.database_url);
    println!("  Server: {}:{}", config.host, config.port);
    println!("  Debug: {}", config.debug_mode);
    println!("  Log Level: {}", config.log_level);
    
    if let Some(api_key) = &config.api_key {
        println!("  API Key: {}***", &api_key[..4.min(api_key.len())]);
    }
    
    // TODO: Actually configure and start services based on config
    // HINT: Similar to configuring services in .NET Core Startup.cs
}

// TODO: Add configuration management utilities
mod config_utils {
    use super::*;
    
    // TODO: Save current config to file
    pub fn save_config(config: &Config, path: &PathBuf) -> Result<(), ConfigError> {
        // TODO: Serialize config to TOML and save
        // HINT: Like saving user.config in .NET
        todo!("Implement config saving")
    }
    
    // TODO: Show current configuration sources and values
    pub fn show_config_sources() {
        // TODO: Display where each config value came from
        // Example:
        // database_url: "postgres://..." (from environment variable MYAPP_DATABASE_URL)
        // port: 8080 (from config.toml)  
        // debug_mode: true (from command line --debug)
        // host: "localhost" (default value)
        
        println!("Configuration Sources:");
        todo!("Show configuration source tracking");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        // TODO: Test that defaults are sensible
        assert_eq!(config.host, "localhost");
        assert_eq!(config.debug_mode, false);
        // Add more default tests
    }
    
    #[test]
    fn test_config_merging() {
        // TODO: Test that config merging respects precedence
        todo!("Test configuration precedence");
    }
    
    #[test]
    fn test_env_var_parsing() {
        // TODO: Test environment variable parsing
        todo!("Test environment variable overrides");
    }
    
    #[test]
    fn test_config_validation() {
        // TODO: Test that invalid configs are rejected
        todo!("Test configuration validation");
    }
}

// CONFIGURATION PRECEDENCE (highest to lowest):
// 1. Command line arguments  (--port 3000)
// 2. Environment variables   (MYAPP_PORT=3000)  
// 3. Configuration files     (config.toml: port = 3000)
// 4. Default values         (port = 8080)
//
// CONFIGURATION FILE LOCATIONS (in order):
// 1. Specified via --config flag
// 2. ./config.toml (current directory)
// 3. ~/.config/myapp/config.toml (user config)
// 4. /etc/myapp/config.toml (system config, Unix only)
//
// C# COMPARISON:
// C#: IConfiguration with ConfigurationBuilder, appsettings.json, environment variables
// Rust: Manual config struct with serde, toml, and env vars
//
// C#: services.Configure<MySettings>(config.GetSection("MySettings"))
// Rust: let config = Config::load_from_sources()?
//
// 📊 PROGRESS TRACKER:
// Feature 1 (File loading): [ ] Working
// Feature 2 (Environment vars): [ ] Working
// Feature 3 (CLI overrides): [ ] Working
// Feature 4 (Precedence): [ ] Working  
// Feature 5 (Validation): [ ] Working
// Feature 6 (Defaults): [ ] Working
//
// 🎯 SUCCESS CRITERIA:
// ✅ Config loads from multiple sources
// ✅ Precedence order is respected
// ✅ Invalid configs are rejected with helpful messages
// ✅ Environment variables override files
// ✅ CLI arguments override everything
// ✅ Sensible defaults for all settings
//
// 🚀 NEXT CHALLENGE:
// Move to ex04-pipe-friendly.rs for Unix pipeline integration!