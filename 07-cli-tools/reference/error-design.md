# Error Design Guidelines

How to create helpful, actionable error messages that guide users to success.

## 🎯 Error Message Philosophy

Great error messages follow three principles:
1. **What went wrong?** - Clear description of the problem
2. **Why did it happen?** - Context about the cause
3. **How to fix it?** - Actionable suggestions for resolution

## ❌ Bad vs ✅ Good Error Examples

### File Not Found

**❌ Bad:**
```
Error: No such file or directory (os error 2)
```

**✅ Good:**
```
❌ File 'config.toml' not found

💡 Suggestions:
  • Check the filename for typos
  • Verify the file exists in the current directory
  • Use absolute path: /full/path/to/config.toml
  • Create a default config: my-tool init --config
```

### Invalid Arguments

**❌ Bad:**
```
error: The following required arguments were not provided:
    --input <INPUT>
```

**✅ Good:**
```
❌ Missing required argument: --input

💡 Example usage:
  my-tool process --input data.csv
  my-tool process --input "*.txt"

📋 Use --help for complete usage information
```

### Configuration Errors

**❌ Bad:**
```
Error: invalid type: string "invalid", expected enum OutputFormat
```

**✅ Good:**
```
❌ Invalid output format: "invalid"

💡 Valid formats:
  • json    - JavaScript Object Notation
  • yaml    - YAML Ain't Markup Language  
  • csv     - Comma Separated Values
  • toml    - Tom's Obvious Minimal Language

📝 Example: --format json
```

## 🏗️ Error Type Design

### Structured Error Enum

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("File '{path}' not found")]
    FileNotFound { path: String },
    
    #[error("Permission denied: '{path}'")]
    PermissionDenied { path: String },
    
    #[error("Invalid format '{format}'. Valid options: {valid_options}")]
    InvalidFormat { format: String, valid_options: String },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Network error: {source}")]
    Network { #[from] source: reqwest::Error },
    
    #[error("IO error: {source}")]
    Io { #[from] source: std::io::Error },
}
```

### Error Context and Suggestions

```rust
impl CliError {
    /// Get contextual suggestions for this error
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            CliError::FileNotFound { path } => vec![
                format!("Check if '{}' exists", path),
                "Verify the current working directory".to_string(),
                "Use absolute path if file is in different directory".to_string(),
                "Check file permissions".to_string(),
            ],
            CliError::PermissionDenied { path } => vec![
                format!("Check permissions for '{}'", path),
                "Try running with appropriate privileges".to_string(),
                "Ensure the file is not locked by another process".to_string(),
            ],
            CliError::InvalidFormat { valid_options, .. } => vec![
                format!("Use one of: {}", valid_options),
                "Check documentation for format specifications".to_string(),
            ],
            CliError::Config { .. } => vec![
                "Verify configuration file syntax".to_string(),
                "Check for missing required fields".to_string(),
                "Use 'my-tool config validate' to check configuration".to_string(),
            ],
            _ => vec!["Check the documentation for more information".to_string()],
        }
    }
    
    /// Get related documentation or help commands
    pub fn help_command(&self) -> Option<String> {
        match self {
            CliError::InvalidFormat { .. } => Some("my-tool --help".to_string()),
            CliError::Config { .. } => Some("my-tool config --help".to_string()),
            _ => None,
        }
    }
}
```

## 🎨 Error Presentation

### Colored Error Output

```rust
use colored::*;

pub fn print_error(error: &CliError) {
    // Main error message
    eprintln!("{} {}", "❌".red(), error.to_string().red().bold());
    eprintln!();
    
    // Suggestions
    let suggestions = error.suggestions();
    if !suggestions.is_empty() {
        eprintln!("{}", "💡 Suggestions:".yellow().bold());
        for suggestion in suggestions {
            eprintln!("  • {}", suggestion);
        }
        eprintln!();
    }
    
    // Help command
    if let Some(help_cmd) = error.help_command() {
        eprintln!("{} {}", "📋".blue(), format!("For more help: {}", help_cmd).blue());
        eprintln!();
    }
    
    // Debug information (only in verbose mode)
    if std::env::var("RUST_LOG").is_ok() {
        eprintln!("{}", "🔍 Debug information:".dim());
        eprintln!("  {:#?}", error);
    }
}
```

### Error Formatting Functions

```rust
/// Format validation errors for configuration
pub fn format_validation_errors(errors: &[String]) -> String {
    let mut output = String::new();
    output.push_str("❌ Configuration validation failed:\n\n");
    
    for (i, error) in errors.iter().enumerate() {
        output.push_str(&format!("  {}. {}\n", i + 1, error));
    }
    
    output.push_str("\n💡 Fix these issues and try again.\n");
    output.push_str("📋 Use 'my-tool config validate' to check your configuration.\n");
    
    output
}

/// Format file operation errors with context
pub fn format_file_error(operation: &str, path: &Path, error: &std::io::Error) -> String {
    let path_str = path.display();
    
    match error.kind() {
        std::io::ErrorKind::NotFound => {
            format!(
                "❌ Cannot {}: '{}' not found\n\n\
                💡 Suggestions:\n\
                  • Check the path is correct: {}\n\
                  • Verify the file exists\n\
                  • Check for typos in the filename",
                operation, path_str, path_str
            )
        },
        std::io::ErrorKind::PermissionDenied => {
            format!(
                "❌ Cannot {}: Permission denied for '{}'\n\n\
                💡 Suggestions:\n\
                  • Check file permissions\n\
                  • Run with appropriate privileges\n\
                  • Ensure file is not locked by another process",
                operation, path_str
            )
        },
        std::io::ErrorKind::AlreadyExists => {
            format!(
                "❌ Cannot {}: '{}' already exists\n\n\
                💡 Suggestions:\n\
                  • Use --force to overwrite\n\
                  • Choose a different filename\n\
                  • Remove the existing file first",
                operation, path_str
            )
        },
        _ => {
            format!(
                "❌ Cannot {}: {} ({})\n\n\
                💡 Suggestion:\n\
                  • Check system logs for more details",
                operation, error, path_str
            )
        }
    }
}
```

## 🛡️ Error Recovery Patterns

### Graceful Degradation

```rust
pub fn load_config_with_fallback() -> Config {
    match Config::load() {
        Ok(config) => {
            println!("✅ Configuration loaded successfully");
            config
        },
        Err(e) => {
            eprintln!("⚠️  Failed to load configuration: {}", e);
            eprintln!("💡 Using default configuration");
            eprintln!("📋 Create config file: my-tool init --config");
            Config::default()
        }
    }
}
```

### Partial Success Reporting

```rust
pub struct OperationResults<T, E> {
    pub successes: Vec<T>,
    pub failures: Vec<E>,
}

impl<T, E> OperationResults<T, E> {
    pub fn report_summary(&self) 
    where 
        T: std::fmt::Display,
        E: std::fmt::Display,
    {
        let total = self.successes.len() + self.failures.len();
        
        if self.failures.is_empty() {
            println!("✅ All {} operations completed successfully", total);
        } else if self.successes.is_empty() {
            println!("❌ All {} operations failed", total);
        } else {
            println!(
                "⚠️  Partial success: {}/{} operations completed", 
                self.successes.len(), 
                total
            );
        }
        
        if !self.failures.is_empty() {
            eprintln!("\n❌ Failures:");
            for failure in &self.failures {
                eprintln!("  • {}", failure);
            }
        }
    }
}
```

### Error Aggregation

```rust
pub fn process_files_with_error_collection(files: Vec<PathBuf>) -> Result<(), Vec<CliError>> {
    let mut errors = Vec::new();
    let mut successful_count = 0;
    
    for file in &files {
        match process_file(file) {
            Ok(()) => successful_count += 1,
            Err(e) => errors.push(e),
        }
    }
    
    if errors.is_empty() {
        println!("✅ Successfully processed {} files", successful_count);
        Ok(())
    } else {
        eprintln!("❌ {} files failed to process:", errors.len());
        for error in &errors {
            eprintln!("  • {}", error);
        }
        
        if successful_count > 0 {
            eprintln!("✅ {} files processed successfully", successful_count);
        }
        
        Err(errors)
    }
}
```

## 🔍 Error Investigation Aids

### Debug Information

```rust
pub fn print_debug_info(error: &CliError) {
    if std::env::var("DEBUG").is_ok() || std::env::var("RUST_LOG").is_ok() {
        eprintln!("🔍 Debug Information:");
        eprintln!("  Error: {:#?}", error);
        eprintln!("  Backtrace: {}", std::backtrace::Backtrace::capture());
        eprintln!("  Environment:");
        eprintln!("    PWD: {:?}", std::env::current_dir());
        eprintln!("    USER: {:?}", std::env::var("USER"));
        eprintln!("    PATH: {:?}", std::env::var("PATH"));
    }
}
```

### Error Reporting

```rust
pub fn offer_error_reporting(error: &CliError) {
    if std::env::var("MY_TOOL_AUTO_REPORT").is_ok() {
        return; // Skip if auto-reporting is disabled
    }
    
    eprintln!("🐛 This error might be a bug in my-tool.");
    eprintln!("💬 Consider reporting it at: https://github.com/user/my-tool/issues");
    eprintln!("📝 Include this information:");
    eprintln!("  • Command run: {}", std::env::args().collect::<Vec<_>>().join(" "));
    eprintln!("  • Error: {}", error);
    eprintln!("  • Version: {}", env!("CARGO_PKG_VERSION"));
    eprintln!("  • Platform: {}", std::env::consts::OS);
}
```

## 📋 Error Message Checklist

### Content Requirements
- [ ] Clear description of what went wrong
- [ ] Context about why it happened
- [ ] Specific, actionable suggestions
- [ ] Examples of correct usage when appropriate
- [ ] Reference to help or documentation

### Formatting Requirements
- [ ] Uses appropriate colors (red for errors, yellow for warnings)
- [ ] Includes emoji or symbols for visual scanning
- [ ] Maintains consistent formatting across all errors
- [ ] Respects NO_COLOR environment variable
- [ ] Works well in both terminals and logs

### User Experience
- [ ] Suggests the most common solution first
- [ ] Avoids technical jargon when possible
- [ ] Provides copy-pasteable examples
- [ ] Includes relevant help commands
- [ ] Offers ways to get more help or report bugs

### Technical Quality
- [ ] Preserves error chain context
- [ ] Includes debug information when appropriate
- [ ] Handles Unicode and special characters properly
- [ ] Exits with appropriate error codes
- [ ] Logs errors appropriately for debugging

## 🎓 Key Principles

1. **Empathy First** - Remember the user is frustrated when they see your error
2. **Be Specific** - "File not found" is better than "Error occurred"
3. **Suggest Solutions** - Don't just describe problems, suggest fixes
4. **Show Examples** - Demonstrate correct usage when relevant
5. **Stay Calm** - Error messages should be helpful, not alarming
6. **Test Thoroughly** - Trigger every error condition and verify the message quality
7. **Iterate Based on Feedback** - Real users will reveal unclear error messages

Remember: Error messages are often a user's first impression of your tool's quality. Make them count!