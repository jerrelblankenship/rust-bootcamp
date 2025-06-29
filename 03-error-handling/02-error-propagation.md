# Lesson 02: Error Propagation

Master the art of flowing errors through your application with Rust's ? operator. Learn how to build robust error handling chains that are both safe and elegant.

## 🎯 Learning Objectives

- Master the `?` operator for concise error propagation
- Understand error type conversion and the `From` trait
- Learn to chain operations safely without nested error handling
- Use the `anyhow` crate for flexible error handling in applications
- Build error handling chains that are both safe and performant
- Compare Rust's explicit propagation with C#'s exception bubbling

## 📚 Introduction

In the previous lesson, we learned about `Result<T, E>` and `Option<T>`. Now let's master how errors flow through your application. This is where Rust's approach really shines compared to C#'s exception handling - errors are explicit, visible, and efficient.

## 🌊 The Problem: Nested Error Handling

Without proper tools, error handling becomes verbose and error-prone:

```rust
use std::fs;

// Verbose nested error handling - don't do this!
fn load_user_config_verbose(path: &str) -> Result<UserConfig, ConfigError> {
    let file_contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => return Err(ConfigError::FileError(e)),
    };
    
    let parsed_json = match serde_json::from_str(&file_contents) {
        Ok(json) => json,
        Err(e) => return Err(ConfigError::ParseError(e)),
    };
    
    let config = match validate_config(parsed_json) {
        Ok(config) => config,
        Err(e) => return Err(ConfigError::ValidationError(e)),
    };
    
    Ok(config)
}

#[derive(Debug)]
enum ConfigError {
    FileError(std::io::Error),
    ParseError(serde_json::Error),
    ValidationError(String),
}

#[derive(serde::Deserialize, Debug)]
struct UserConfig {
    username: String,
    theme: String,
    auto_save: bool,
}

fn validate_config(_value: serde_json::Value) -> Result<UserConfig, String> {
    // Validation logic here
    todo!()
}
```

This repetitive pattern is exactly what the `?` operator solves!

## ⚡ The ? Operator: Your Error Handling Superpower

The `?` operator transforms error handling from verbose to elegant:

```rust
use anyhow::Result;

// Elegant error propagation with ? - much better!
fn load_user_config(path: &str) -> Result<UserConfig> {
    let file_contents = fs::read_to_string(path)?;
    let parsed_json: serde_json::Value = serde_json::from_str(&file_contents)?;
    let config = validate_config(parsed_json)?;
    Ok(config)
}

fn validate_config(value: serde_json::Value) -> Result<UserConfig> {
    let config: UserConfig = serde_json::from_value(value)?;
    
    if config.username.is_empty() {
        anyhow::bail!("Username cannot be empty");
    }
    
    if !["light", "dark", "auto"].contains(&config.theme.as_str()) {
        anyhow::bail!("Invalid theme: {}", config.theme);
    }
    
    Ok(config)
}
```

**What ? Does:**
1. **Success case**: Unwraps the `Ok(value)` and continues execution
2. **Error case**: Returns `Err(e)` from the current function immediately
3. **Type conversion**: Automatically converts error types when possible

## 🔄 Comparing with C# Exception Handling

### C# Exception Propagation
```csharp
public class ConfigLoader
{
    public UserConfig LoadConfig(string path)
    {
        try
        {
            // Each operation can throw - exceptions bubble up automatically
            var contents = File.ReadAllText(path);
            var json = JsonSerializer.Deserialize<JsonElement>(contents);
            return ValidateConfig(json);
        }
        catch (FileNotFoundException ex)
        {
            throw new ConfigException("Config file not found", ex);
        }
        catch (JsonException ex)
        {
            throw new ConfigException("Invalid JSON format", ex);
        }
        // Other exceptions bubble up uncaught - potential crashes!
    }
    
    private UserConfig ValidateConfig(JsonElement json)
    {
        var config = json.Deserialize<UserConfig>();
        
        if (string.IsNullOrEmpty(config.Username))
            throw new ValidationException("Username required");
            
        return config;
    }
}
```

### Rust Error Propagation
```rust
use anyhow::{Context, Result};

struct ConfigLoader;

impl ConfigLoader {
    fn load_config(&self, path: &str) -> Result<UserConfig> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {}", path))?;
            
        let json: serde_json::Value = serde_json::from_str(&contents)
            .context("Invalid JSON format in config file")?;
            
        self.validate_config(json)
    }
    
    fn validate_config(&self, json: serde_json::Value) -> Result<UserConfig> {
        let config: UserConfig = serde_json::from_value(json)
            .context("Failed to deserialize config")?;
            
        if config.username.is_empty() {
            anyhow::bail!("Username cannot be empty");
        }
        
        Ok(config)
    }
}
```

**Key Differences:**
- **Explicit**: Error types are visible in function signatures
- **Opt-in**: You choose which errors to handle vs. propagate
- **Composable**: Easy to add context at each level
- **Performance**: No stack unwinding overhead

## 🔗 Error Type Conversion with From Trait

The `?` operator automatically converts between compatible error types:

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;

// Custom error type that can be created from other error types
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

// Implement From trait for automatic conversions
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Now ? operator automatically converts io::Error and ParseIntError to AppError
fn process_config_file(path: &str) -> Result<i32, AppError> {
    let contents = fs::read_to_string(path)?; // io::Error -> AppError
    let number = contents.trim().parse()?;     // ParseIntError -> AppError
    
    if number < 0 {
        return Err(AppError::Custom("Number must be positive".to_string()));
    }
    
    Ok(number)
}
```

## 🧰 Using anyhow for Flexible Error Handling

The `anyhow` crate provides a flexible error type perfect for applications:

```rust
use anyhow::{anyhow, bail, Context, Result};
use std::fs;

// anyhow::Result<T> is shorthand for Result<T, anyhow::Error>
fn complex_operation(config_path: &str, data_path: &str) -> Result<ProcessingResult> {
    // Load configuration with context
    let config_content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to load config from {}", config_path))?;
    
    let config: Config = serde_json::from_str(&config_content)
        .context("Failed to parse configuration JSON")?;
    
    // Validate configuration
    if config.batch_size == 0 {
        bail!("Batch size cannot be zero");
    }
    
    if config.output_dir.is_empty() {
        return Err(anyhow!("Output directory not specified"));
    }
    
    // Load and process data
    let data = fs::read_to_string(data_path)
        .with_context(|| format!("Failed to load data from {}", data_path))?;
    
    let processed = process_data(&data, &config)
        .context("Data processing failed")?;
    
    Ok(processed)
}

#[derive(serde::Deserialize)]
struct Config {
    batch_size: usize,
    output_dir: String,
    max_retries: u32,
}

struct ProcessingResult {
    processed_count: usize,
    output_path: String,
}

fn process_data(data: &str, config: &Config) -> Result<ProcessingResult> {
    if data.is_empty() {
        bail!("Cannot process empty data");
    }
    
    let lines: Vec<&str> = data.lines().collect();
    let batch_count = (lines.len() + config.batch_size - 1) / config.batch_size;
    
    if batch_count > 1000 {
        return Err(anyhow!(
            "Too many batches ({}) - consider increasing batch size", 
            batch_count
        ));
    }
    
    Ok(ProcessingResult {
        processed_count: lines.len(),
        output_path: format!("{}/processed.json", config.output_dir),
    })
}
```

### anyhow Benefits

1. **Flexibility**: Can wrap any error type
2. **Context**: Easy to add contextual information with `.context()`
3. **Ergonomics**: Convenient macros (`anyhow!`, `bail!`)
4. **Debugging**: Excellent error message formatting with error chains

## 🔧 Advanced Error Propagation Patterns

### Result Chaining with Combinators

```rust
use anyhow::Result;

fn process_user_data(user_id: u32) -> Result<ProcessedUser> {
    // Chain multiple operations together elegantly
    let user = fetch_user(user_id)?;
    let validated_user = user.validate()?;
    let enhanced_user = enhance_profile(validated_user)?;
    let scored_user = calculate_score(enhanced_user)?;
    Ok(ProcessedUser::from(scored_user))
}

#[derive(Debug)]
struct User {
    id: u32,
    email: String,
    age: u32,
    profile_score: f64,
    final_score: f64,
}

impl User {
    fn validate(self) -> Result<User> {
        if self.email.is_empty() {
            anyhow::bail!("User email is required");
        }
        if self.age < 13 {
            anyhow::bail!("User must be at least 13 years old");
        }
        Ok(self)
    }
    
    fn activity_multiplier(&self) -> f64 {
        // Simulate complex calculation
        1.5
    }
}

fn enhance_profile(mut user: User) -> Result<User> {
    user.profile_score = fetch_profile_score(&user.email)
        .context("Failed to fetch profile score")?;
    Ok(user)
}

fn calculate_score(mut user: User) -> Result<User> {
    user.final_score = user.profile_score * user.activity_multiplier();
    Ok(user)
}

struct ProcessedUser {
    id: u32,
    score: f64,
}

impl From<User> for ProcessedUser {
    fn from(user: User) -> Self {
        ProcessedUser {
            id: user.id,
            score: user.final_score,
        }
    }
}

fn fetch_user(id: u32) -> Result<User> {
    if id == 0 {
        anyhow::bail!("Invalid user ID: 0");
    }
    
    Ok(User {
        id,
        email: format!("user{}@example.com", id),
        age: 25,
        profile_score: 0.0,
        final_score: 0.0,
    })
}

fn fetch_profile_score(email: &str) -> Result<f64> {
    if email.contains("@") {
        Ok(85.5)
    } else {
        anyhow::bail!("Invalid email format");
    }
}
```

### Collecting Multiple Results

```rust
// Process multiple items and collect results
fn process_multiple_users(user_ids: &[u32]) -> Result<Vec<ProcessedUser>> {
    user_ids
        .iter()
        .map(|&id| process_user_data(id))
        .collect() // Stops at first error
}

// Process multiple items, collecting successes and failures separately
fn process_multiple_users_lenient(user_ids: &[u32]) -> (Vec<ProcessedUser>, Vec<anyhow::Error>) {
    let (successes, failures): (Vec<_>, Vec<_>) = user_ids
        .iter()
        .map(|&id| process_user_data(id))
        .partition_map(|result| match result {
            Ok(user) => itertools::Either::Left(user),
            Err(e) => itertools::Either::Right(e),
        });
    
    (successes, failures)
}

// Note: This requires the itertools crate for partition_map
// Add to Cargo.toml: itertools = "0.11"
```

## 🔄 Comparison with C#

| C# Error Handling | Rust Error Propagation | Key Difference |
|------------------|------------------------|----------------|
| `try-catch` blocks | `match` expressions | Explicit in signatures |
| `throw` statement | `return Err(e)` | Errors are return values |
| Exception bubbling | `?` operator | Visible and controlled |
| `finally` blocks | Drop trait | Automatic cleanup |
| Stack unwinding | Direct returns | No performance overhead |
| Hidden exceptions | Explicit Result types | Cannot ignore errors |

## 💻 Practice Exercises

### Exercise 1: Implement Error Propagation

```rust
use anyhow::Result;

// Complete this function using the ? operator
fn calculate_user_score(user_data: &str) -> Result<f64> {
    // Parse JSON - add proper error handling
    let value: serde_json::Value = todo!(); // Use ?
    
    // Extract age - handle missing field
    let age = value["age"].as_u64()
        .ok_or_else(|| anyhow::anyhow!("Missing age field"))?;
    
    // Extract scores array - handle missing field
    let scores = value["scores"].as_array()
        .ok_or_else(|| anyhow::anyhow!("Missing scores field"))?;
    
    // Calculate average score - handle parse errors with ?
    let total: f64 = todo!(); // Sum all scores, handle conversion errors
    let average = total / scores.len() as f64;
    
    // Apply age multiplier
    let age_multiplier = if age < 18 { 0.8 } else { 1.0 };
    
    Ok(average * age_multiplier)
}

fn main() {
    let test_data = r#"
    {
        "age": 25,
        "scores": [85, 92, 78, 90]
    }
    "#;
    
    match calculate_user_score(test_data) {
        Ok(score) => println!("User score: {:.2}", score),
        Err(e) => eprintln!("Error calculating score: {}", e),
    }
}
```

### Exercise 2: Create Custom Error with From Implementations

```rust
use std::fs;

// Define a custom error type and implement From traits
#[derive(Debug)]
enum ProcessingError {
    // TODO: Add variants for:
    // - IO errors
    // - JSON parsing errors  
    // - Validation errors (with custom message)
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement display formatting
        todo!()
    }
}

impl std::error::Error for ProcessingError {}

// TODO: Implement From<std::io::Error>
// TODO: Implement From<serde_json::Error>

fn process_file(path: &str) -> Result<ProcessedData, ProcessingError> {
    // TODO: Read file, parse JSON, validate, and return ProcessedData
    // Use ? operator throughout
    todo!()
}

struct ProcessedData {
    count: usize,
    summary: String,
}

fn main() {
    match process_file("data.json") {
        Ok(data) => println!("Processed {} items: {}", data.count, data.summary),
        Err(e) => eprintln!("Processing failed: {}", e),
    }
}
```

### Exercise 3: Error Context Practice

```rust
use anyhow::{Context, Result};

// Add appropriate context to each operation
fn backup_user_data(user_id: u32, backup_dir: &str) -> Result<String> {
    // TODO: Add context to each operation
    let user_data = fetch_user_data(user_id)?; // Add context about which user
    let serialized = serialize_data(&user_data)?; // Add context about serialization
    let backup_path = create_backup_file(backup_dir, user_id, &serialized)?; // Add context about backup
    
    Ok(backup_path)
}

// Stub functions for the exercise
fn fetch_user_data(id: u32) -> Result<UserData> { 
    if id == 0 {
        anyhow::bail!("Invalid user ID");
    }
    Ok(UserData { id, name: format!("User{}", id) })
}

fn serialize_data(data: &UserData) -> Result<Vec<u8>> { 
    serde_json::to_vec(data).map_err(Into::into)
}

fn create_backup_file(dir: &str, user_id: u32, data: &[u8]) -> Result<String> { 
    let path = format!("{}/user_{}_backup.json", dir, user_id);
    std::fs::write(&path, data)?;
    Ok(path)
}

#[derive(serde::Serialize)]
struct UserData {
    id: u32,
    name: String,
}

fn main() {
    match backup_user_data(42, "/tmp/backups") {
        Ok(path) => println!("Backup created: {}", path),
        Err(e) => {
            eprintln!("Backup failed: {}", e);
            // Print the error chain
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {}", err);
                source = err.source();
            }
        }
    }
}
```

## 🚀 Mini-Project: File Processing Pipeline

Build a robust file processing pipeline that demonstrates error propagation:

```rust
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

struct FileProcessor {
    input_dir: String,
    output_dir: String,
}

impl FileProcessor {
    fn new(input_dir: String, output_dir: String) -> Result<Self> {
        // Validate directories exist
        if !Path::new(&input_dir).exists() {
            anyhow::bail!("Input directory does not exist: {}", input_dir);
        }
        
        // Create output directory if it doesn't exist
        fs::create_dir_all(&output_dir)
            .with_context(|| format!("Failed to create output directory: {}", output_dir))?;
        
        Ok(FileProcessor {
            input_dir,
            output_dir,
        })
    }
    
    fn process_all_files(&self) -> Result<ProcessingSummary> {
        let entries = fs::read_dir(&self.input_dir)
            .with_context(|| format!("Failed to read input directory: {}", self.input_dir))?;
        
        let mut processed = 0;
        let mut errors = 0;
        
        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() {
                match self.process_single_file(&path) {
                    Ok(_) => processed += 1,
                    Err(e) => {
                        eprintln!("Error processing {}: {}", path.display(), e);
                        errors += 1;
                    }
                }
            }
        }
        
        Ok(ProcessingSummary { processed, errors })
    }
    
    fn process_single_file(&self, input_path: &Path) -> Result<()> {
        let content = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read file: {}", input_path.display()))?;
        
        let processed_content = self.transform_content(&content)
            .with_context(|| format!("Failed to process content from: {}", input_path.display()))?;
        
        let output_filename = input_path.file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid filename: {}", input_path.display()))?;
        
        let output_path = Path::new(&self.output_dir).join(output_filename);
        
        fs::write(&output_path, processed_content)
            .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;
        
        println!("Processed: {} -> {}", input_path.display(), output_path.display());
        Ok(())
    }
    
    fn transform_content(&self, content: &str) -> Result<String> {
        if content.is_empty() {
            anyhow::bail!("Cannot process empty file");
        }
        
        // Simple transformation: add line numbers
        let lines: Vec<String> = content
            .lines()
            .enumerate()
            .map(|(i, line)| format!("{:3}: {}", i + 1, line))
            .collect();
        
        Ok(lines.join("\n"))
    }
}

struct ProcessingSummary {
    processed: usize,
    errors: usize,
}

fn main() -> Result<()> {
    let processor = FileProcessor::new("./input".to_string(), "./output".to_string())
        .context("Failed to initialize file processor")?;
    
    let summary = processor.process_all_files()
        .context("File processing failed")?;
    
    println!("Processing complete: {} files processed, {} errors", 
             summary.processed, summary.errors);
    
    Ok(())
}
```

## 🔑 Key Takeaways

1. **? operator**: Makes error propagation concise and explicit
2. **From trait**: Enables automatic error type conversion
3. **anyhow crate**: Perfect for applications needing flexible error handling
4. **Context**: Add meaningful information at each level of the call stack
5. **Error chains**: Preserve error history for better debugging
6. **Type safety**: Compiler ensures all error paths are handled

## 📚 Additional Resources

- [anyhow crate documentation](https://docs.rs/anyhow/)
- [thiserror crate](https://docs.rs/thiserror/) - For deriving error types
- [Rust Book - Recoverable Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Use the ? operator for error propagation
- [ ] Implement From traits for error type conversion
- [ ] Add context to errors using anyhow
- [ ] Chain multiple fallible operations safely
- [ ] Handle collections of Results appropriately
- [ ] Debug error chains effectively

---

Next: [Custom Error Types](03-custom-errors.md) - Design domain-specific errors that guide users to solutions →
