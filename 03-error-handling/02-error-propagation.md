# Lesson 02: Error Propagation and the ? Operator

Now that you understand `Result<T, E>` and `Option<T>`, let's master error propagation - how errors flow through your application. This is where Rust's approach really shines compared to C#'s exception handling.

## 🎯 Learning Objectives

- Master the `?` operator for concise error propagation
- Understand error type conversion and the `From` trait
- Learn to chain operations safely
- Use the `anyhow` crate for flexible error handling
- Compare with C#'s exception bubbling

## 🌊 Error Propagation Patterns

### The Problem: Nested Error Handling

Without proper tools, error handling becomes verbose quickly:

```rust
use std::fs;
use serde_json;

// Verbose nested error handling
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
```

This is repetitive and error-prone. Let's see how Rust makes this elegant.

## ⚡ The ? Operator: Your Error Handling Superpower

The `?` operator transforms error handling from verbose to elegant:

```rust
use anyhow::Result;

// Elegant error propagation with ?
fn load_user_config(path: &str) -> Result<UserConfig> {
    let file_contents = fs::read_to_string(path)?;
    let parsed_json: serde_json::Value = serde_json::from_str(&file_contents)?;
    let config = validate_config(parsed_json)?;
    Ok(config)
}

#[derive(serde::Deserialize, Debug)]
struct UserConfig {
    username: String,
    theme: String,
    auto_save: bool,
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
1. **Success case**: Unwraps the `Ok(value)` and continues
2. **Error case**: Returns `Err(e)` from the current function
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
        // Other exceptions bubble up uncaught
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

The `anyhow` crate provides a flexible error type that can wrap any error:

```rust
use anyhow::{anyhow, bail, Context, Result};
use std::fs;

// anyhow::Result<T> is shorthand for Result<T, anyhow::Error>
fn complex_operation(config_path: &str, data_path: &str) -> Result<ProcessingResult> {
    // Load configuration
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
    // Simulate complex processing
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
2. **Context**: Easy to add contextual information
3. **Ergonomics**: Convenient macros (`anyhow!`, `bail!`)
4. **Debugging**: Excellent error message formatting

## 🔧 Advanced Error Propagation Patterns

### Result Chaining with Combinators

```rust
use anyhow::Result;

fn process_user_data(user_id: u32) -> Result<ProcessedUser> {
    // Chain multiple operations together
    fetch_user(user_id)?
        .validate()
        .and_then(|user| enhance_profile(user))
        .and_then(|user| calculate_score(user))
        .map(|user| ProcessedUser::from(user))
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

#[derive(Debug)]
struct User {
    id: u32,
    email: String,
    age: u32,
    profile_score: f64,
    final_score: f64,
}

impl User {
    fn activity_multiplier(&self) -> f64 {
        // Simulate complex calculation
        1.5
    }
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
```

## 💡 Best Practices for Error Propagation

### 1. Use Specific Error Types for Libraries

```rust
// For libraries, create specific error types
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Query failed: {query}")]
    QueryFailed { query: String },
    
    #[error("Transaction was rolled back")]
    TransactionRolledBack,
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub fn execute_query(query: &str) -> Result<QueryResult, DatabaseError> {
    if query.trim().is_empty() {
        return Err(DatabaseError::ValidationError("Query cannot be empty".to_string()));
    }
    
    // Simulate database operation
    if query.contains("DROP") {
        return Err(DatabaseError::QueryFailed { 
            query: query.to_string() 
        });
    }
    
    Ok(QueryResult { rows_affected: 1 })
}

struct QueryResult {
    rows_affected: usize,
}
```

### 2. Use anyhow for Applications

```rust
// For applications, anyhow provides flexibility
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = load_configuration("app.toml")
        .context("Failed to load application configuration")?;
    
    let database = connect_to_database(&config.db_url)
        .context("Failed to establish database connection")?;
    
    run_application(config, database)
        .context("Application runtime error")?;
    
    println!("Application completed successfully");
    Ok(())
}

fn load_configuration(path: &str) -> Result<AppConfig> {
    let content = std::fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

#[derive(serde::Deserialize)]
struct AppConfig {
    db_url: String,
    port: u16,
}

fn connect_to_database(url: &str) -> Result<Database> {
    if url.is_empty() {
        anyhow::bail!("Database URL cannot be empty");
    }
    Ok(Database { url: url.to_string() })
}

struct Database {
    url: String,
}

fn run_application(config: AppConfig, database: Database) -> Result<()> {
    println!("Running application with config: port={}", config.port);
    println!("Connected to database: {}", database.url);
    Ok(())
}
```

### 3. Add Context at the Right Level

```rust
use anyhow::{Context, Result};

fn save_user_profile(user_id: u32, profile: &UserProfile) -> Result<()> {
    let db = get_database_connection()
        .context("Failed to connect to database")?;
    
    let serialized = serialize_profile(profile)
        .with_context(|| format!("Failed to serialize profile for user {}", user_id))?;
    
    db.save(&format!("user:{}", user_id), &serialized)
        .with_context(|| format!("Failed to save profile for user {}", user_id))?;
    
    Ok(())
}

struct UserProfile {
    name: String,
    email: String,
}

struct Database;

impl Database {
    fn save(&self, key: &str, data: &[u8]) -> Result<()> {
        // Simulate database save
        println!("Saving {} bytes to key: {}", data.len(), key);
        Ok(())
    }
}

fn get_database_connection() -> Result<Database> {
    Ok(Database)
}

fn serialize_profile(profile: &UserProfile) -> Result<Vec<u8>> {
    serde_json::to_vec(profile).map_err(Into::into)
}

impl serde::Serialize for UserProfile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("UserProfile", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.end()
    }
}
```

## 🎯 Key Takeaways

1. **? operator**: Makes error propagation concise and explicit
2. **From trait**: Enables automatic error type conversion
3. **anyhow crate**: Perfect for applications needing flexible error handling
4. **Context**: Add meaningful information at each level
5. **Type safety**: Compiler ensures all error paths are handled

## 💻 Exercises

### Exercise 1: Implement Error Propagation
```rust
// Complete this function using the ? operator
fn calculate_user_score(user_data: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // Parse JSON
    let value: serde_json::Value = todo!(); // Use ?
    
    // Extract age
    let age = value["age"].as_u64().ok_or("Missing age field")?;
    
    // Extract scores array
    let scores = value["scores"].as_array().ok_or("Missing scores field")?;
    
    // Calculate average score
    let total: f64 = todo!(); // Sum all scores, handle parse errors with ?
    let average = total / scores.len() as f64;
    
    // Apply age multiplier
    let age_multiplier = if age < 18 { 0.8 } else { 1.0 };
    
    Ok(average * age_multiplier)
}
```

### Exercise 2: Create Custom Error with From Implementations
```rust
// Define a custom error type and implement From traits
#[derive(Debug)]
enum ProcessingError {
    // TODO: Add variants for:
    // - IO errors
    // - JSON parsing errors  
    // - Validation errors (with custom message)
}

// TODO: Implement From<std::io::Error>
// TODO: Implement From<serde_json::Error>
// TODO: Implement Display and Error traits

fn process_file(path: &str) -> Result<ProcessedData, ProcessingError> {
    // TODO: Read file, parse JSON, validate, and return ProcessedData
    // Use ? operator throughout
    todo!()
}

struct ProcessedData {
    count: usize,
    summary: String,
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
fn fetch_user_data(id: u32) -> Result<UserData> { todo!() }
fn serialize_data(data: &UserData) -> Result<Vec<u8>> { todo!() }
fn create_backup_file(dir: &str, user_id: u32, data: &[u8]) -> Result<String> { todo!() }

struct UserData {
    id: u32,
    name: String,
}
```

## 📚 Additional Resources

- [anyhow crate documentation](https://docs.rs/anyhow/)
- [thiserror crate](https://docs.rs/thiserror/) - For deriving error types
- [Rust Book - Recoverable Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

---

Next: [Custom Error Types](03-custom-errors.md) →
