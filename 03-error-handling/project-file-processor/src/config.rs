// Configuration module - Fix the Broken Code!
//
// This module handles loading and saving configuration for the file processor.
// Students need to implement configuration management with proper error handling,
// environment variable support, and file I/O.

use crate::error::{FileProcessorError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::path::Path;

// Exercise: Define the main configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // TODO: Add configuration fields
    // HINT: Think about what options a file processor might need:
    // - max_file_size: usize (maximum file size to process)
    // - validate_data: bool (whether to validate file contents)
    // - continue_on_error: bool (whether to continue processing after errors)
    // - output_format: String (default output format)
    // - output_directory: String (default output directory)
    // - processing: ProcessingConfig (processing-specific settings)
    // - logging: LoggingConfig (logging configuration)
    
    // FIXME: Add fields here
}

// Exercise: Define sub-configuration structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    // TODO: Add processing-specific configuration
    // HINT: max_concurrent_files: usize, chunk_size: usize, retry_attempts: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    // TODO: Add logging configuration
    // HINT: level: String, verbose: bool, log_to_file: bool
}

// FIXME: Implement Default for Config
impl Default for Config {
    fn default() -> Self {
        // TODO: Create sensible default configuration
        // HINT: Use reasonable defaults like 10MB max file size, enable validation, etc.
        todo!("Implement default configuration")
    }
}

impl Config {
    // Exercise: Load configuration from file
    pub fn from_file(path: &str) -> Result<Self> {
        // TODO: Load configuration from JSON file
        // HINT: Read file with fs::read_to_string(), then parse with serde_json
        // HINT: Use ? operator for error propagation
        // HINT: Handle file not found vs parse errors differently
        todo!("Load configuration from file")
    }
    
    // Exercise: Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // TODO: Load configuration from environment variables
        // HINT: Use env::var() to read environment variables
        // HINT: Handle missing variables gracefully (use defaults)
        // HINT: Parse string values to appropriate types
        // Environment variables to support:
        // - FILE_PROCESSOR_MAX_SIZE
        // - FILE_PROCESSOR_VALIDATE
        // - FILE_PROCESSOR_OUTPUT_FORMAT
        // - FILE_PROCESSOR_OUTPUT_DIR
        todo!("Load configuration from environment")
    }
    
    // Exercise: Save configuration to file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        // TODO: Save configuration to JSON file
        // HINT: Use serde_json::to_string_pretty() for formatting
        // HINT: Write to file with fs::write()
        // HINT: Handle directory creation if needed
        todo!("Save configuration to file")
    }
    
    // Exercise: Merge configurations (CLI overrides config file)
    pub fn merge_with_cli(&mut self, cli_overrides: CliOverrides) {
        // TODO: Apply CLI overrides to configuration
        // HINT: Only override fields that are explicitly set in CLI
        // HINT: Use Option types for CLI overrides to distinguish "not set" from "set to default"
        todo!("Merge CLI overrides with configuration")
    }
    
    // Exercise: Validate configuration values
    pub fn validate(&self) -> Result<()> {
        // TODO: Validate that configuration values are reasonable
        // HINT: Check max_file_size > 0, valid output format, etc.
        // HINT: Return appropriate validation errors
        todo!("Validate configuration values")
    }
    
    // Exercise: Get configuration with environment override
    pub fn get_max_file_size(&self) -> usize {
        // TODO: Get max file size with environment variable override
        // HINT: Check FILE_PROCESSOR_MAX_SIZE environment variable first
        todo!("Get max file size with environment override")
    }
    
    // TODO: Add more getter methods with environment overrides
    // HINT: get_validate_data(), get_output_format(), etc.
}

// Exercise: CLI override structure
#[derive(Debug, Default)]
pub struct CliOverrides {
    // TODO: Define optional CLI overrides
    // HINT: Use Option<T> for each field that can be overridden
    // pub max_size: Option<usize>,
    // pub validate: Option<bool>,
    // pub output_format: Option<String>,
    // pub output_directory: Option<String>,
}

impl CliOverrides {
    // TODO: Create CliOverrides from CLI arguments
    pub fn from_cli(/* TODO: add CLI parameters */) -> Self {
        // TODO: Extract overrides from CLI arguments
        // HINT: Convert CLI arguments to Option values
        todo!("Create CliOverrides from CLI arguments")
    }
}

// Exercise: Configuration discovery - find config file in multiple locations
pub fn discover_config_file() -> Option<String> {
    // TODO: Look for configuration file in standard locations
    // HINT: Check these locations in order:
    // 1. ./file-processor.json
    // 2. ~/.config/file-processor/config.json
    // 3. /etc/file-processor/config.json (Unix)
    // HINT: Return the first file that exists
    todo!("Discover configuration file location")
}

// Exercise: Environment variable helpers
fn parse_env_var<T>(var_name: &str) -> Option<T> 
where 
    T: std::str::FromStr,
{
    // TODO: Parse environment variable to type T
    // HINT: Use env::var() and str::parse()
    // HINT: Return None for missing variables or parse errors
    todo!("Parse environment variable")
}

fn parse_env_bool(var_name: &str) -> Option<bool> {
    // TODO: Parse boolean environment variable
    // HINT: Accept "true", "false", "1", "0", "yes", "no"
    // HINT: Case insensitive parsing
    todo!("Parse boolean environment variable")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{TempDir, NamedTempFile};
    use std::io::Write;
    
    #[test]
    fn test_default_config() {
        // TODO: Test that default configuration is reasonable
        // HINT: Create Config::default() and verify field values
    }
    
    #[test]
    fn test_config_from_file() {
        // TODO: Test loading configuration from file
        // HINT: Create temporary JSON file with test config
        // HINT: Test both valid and invalid JSON
    }
    
    #[test]
    fn test_config_save_to_file() {
        // TODO: Test saving configuration to file
        // HINT: Save config, then reload and verify it matches
    }
    
    #[test]
    fn test_config_from_env() {
        // TODO: Test loading configuration from environment
        // HINT: Set test environment variables, then load config
        // HINT: Clean up environment variables after test
    }
    
    #[test]
    fn test_cli_overrides() {
        // TODO: Test CLI override functionality
        // HINT: Create config, apply overrides, verify changes
    }
    
    #[test]
    fn test_config_validation() {
        // TODO: Test configuration validation
        // HINT: Test both valid and invalid configurations
    }
    
    #[test]
    fn test_config_discovery() {
        // TODO: Test configuration file discovery
        // HINT: Create test files in different locations
    }
    
    #[test]
    fn test_env_var_parsing() {
        // TODO: Test environment variable parsing helpers
        // HINT: Test different value types and edge cases
    }
}

// COMPILATION CHALLENGES:
// 1. Define comprehensive configuration structures
// 2. Implement serialization/deserialization with serde
// 3. Handle file I/O errors properly with error conversion
// 4. Parse environment variables with proper type conversion
// 5. Implement configuration merging and override logic
// 6. Add configuration validation with meaningful errors
// 7. Implement configuration discovery across multiple locations
// 8. Handle edge cases (missing files, invalid JSON, etc.)
//
// CONFIGURATION DESIGN PRINCIPLES:
// - Provide sensible defaults for all settings
// - Support multiple configuration sources (file, env, CLI)
// - Make CLI arguments override config file settings
// - Validate configuration early and provide clear error messages
// - Use structured configuration rather than flat key-value pairs
// - Support both human-readable files and environment variables
//
// REAL-WORLD PATTERNS DEMONSTRATED:
// - 12-factor app configuration (environment variables)
// - Configuration file discovery (XDG Base Directory spec)
// - Layered configuration (defaults < file < environment < CLI)
// - Configuration validation and error handling
// - Structured configuration with nested objects
//
// C# COMPARISON:
// C#: IConfiguration with appsettings.json, environment variables, command line
// Rust: Custom Config struct with serde, env vars, and CLI parsing
//
// Both approaches support layered configuration, but Rust makes the structure
// explicit at compile time rather than using runtime string keys.
