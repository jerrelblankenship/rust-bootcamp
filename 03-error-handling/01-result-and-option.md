# Lesson 01: Result and Option Types

Welcome to Rust's approach to error handling! As a C# developer, you're used to exceptions flying around your call stack. Rust takes a radically different approach: errors are values, not control flow interruptions.

## 🎯 Learning Objectives

- Master the `Result<T, E>` type for fallible operations
- Understand `Option<T>` for handling null-like scenarios
- Learn pattern matching for error handling
- Compare Rust's approach with C#'s exception model
- Use the `?` operator for error propagation

## 🚫 The Problem with Exceptions

### C# Exception Model
```csharp
public class UserService
{
    public User GetUser(int id)
    {
        if (id <= 0)
            throw new ArgumentException("Invalid ID");
            
        var user = database.FindUser(id);
        if (user == null)
            throw new UserNotFoundException($"User {id} not found");
            
        return user;
    }
    
    // Caller must remember to catch exceptions
    try 
    {
        var user = userService.GetUser(userId);
        Console.WriteLine($"Found: {user.Name}");
    }
    catch (ArgumentException ex)
    {
        Console.WriteLine($"Bad input: {ex.Message}");
    }
    catch (UserNotFoundException ex)
    {
        Console.WriteLine($"Not found: {ex.Message}");
    }
}
```

**Problems:**
- Exceptions are "invisible" in method signatures
- Easy to forget to handle errors
- Performance overhead of unwinding the stack
- Can interrupt control flow unexpectedly

## ✅ The Rust Way: Errors as Values

### Option<T> - No More Null References

`Option<T>` replaces nullable types and eliminates `NullReferenceException`:

```rust
// C# nullable approach
public string GetUserEmail(int userId)
{
    var user = GetUser(userId); // Could return null
    return user?.Email; // Null-conditional operator
}

// Rust Option approach
fn get_user_email(user_id: u32) -> Option<String> {
    let user = get_user(user_id)?; // Returns None if user not found
    Some(user.email)
}

fn get_user(user_id: u32) -> Option<User> {
    if user_id == 0 {
        None // Explicit "no value"
    } else {
        Some(User { 
            id: user_id, 
            email: "user@example.com".to_string() 
        })
    }
}
```

**Key Benefits:**
- Compiler forces you to handle the `None` case
- No surprise null pointer exceptions
- Self-documenting API - the return type tells you it might fail

### Result<T, E> - Explicit Error Handling

`Result<T, E>` replaces exceptions with explicit error values:

```rust
use std::fs;
use std::io;

// C# file reading with exceptions
/*
public string ReadConfig(string path)
{
    try 
    {
        return File.ReadAllText(path);
    }
    catch (FileNotFoundException)
    {
        throw new ConfigException("Config file missing");
    }
    catch (UnauthorizedAccessException)
    {
        throw new ConfigException("Cannot access config file");
    }
}
*/

// Rust file reading with Result
fn read_config(path: &str) -> Result<String, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(ConfigError::IoError)?;
    
    if content.is_empty() {
        return Err(ConfigError::EmptyFile);
    }
    
    Ok(content)
}

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    EmptyFile,
}
```

## 🔍 Pattern Matching: Exhaustive Error Handling

### The match Expression

```rust
fn process_user_request(user_id: u32) {
    match get_user(user_id) {
        Some(user) => {
            println!("Processing request for {}", user.email);
            // Handle success case
        }
        None => {
            println!("User {} not found", user_id);
            // Handle missing user case
        }
    }
}

fn handle_file_operation() {
    match read_config("app.conf") {
        Ok(content) => {
            println!("Config loaded: {} characters", content.len());
        }
        Err(ConfigError::IoError(io_err)) => {
            eprintln!("Failed to read file: {}", io_err);
        }
        Err(ConfigError::EmptyFile) => {
            eprintln!("Config file is empty!");
        }
    }
}
```

**Compiler Guarantees:**
- All cases must be handled (exhaustive matching)
- Cannot accidentally ignore errors
- Refactoring is safe - compiler catches missing cases

### if let for Simple Cases

```rust
// When you only care about the success case
if let Some(user) = get_user(user_id) {
    println!("Found user: {}", user.email);
}

// When you only care about one error case
if let Err(ConfigError::EmptyFile) = read_config("app.conf") {
    println!("Please provide a non-empty config file");
}
```

## ⚡ The ? Operator: Early Return Magic

The `?` operator is Rust's equivalent to exception bubbling, but explicit:

```rust
use std::fs;
use std::io;

// Without ? operator (verbose)
fn load_user_data_verbose(path: &str) -> Result<UserData, Box<dyn std::error::Error>> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => return Err(Box::new(e)),
    };
    
    let user_data = match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e)),
    };
    
    Ok(user_data)
}

// With ? operator (concise)
fn load_user_data(path: &str) -> Result<UserData, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let user_data = serde_json::from_str(&content)?;
    Ok(user_data)
}

#[derive(serde::Deserialize)]
struct UserData {
    name: String,
    email: String,
}
```

**How ? Works:**
1. If `Result` is `Ok(value)`, extracts `value`
2. If `Result` is `Err(e)`, returns `Err(e)` from the function
3. Error type must be convertible to the function's error type

## 🔄 C# to Rust Error Handling Comparison

| C# Pattern | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| `null` | `None` | Compiler forces handling |
| `try-catch` | `match` | Explicit in function signature |
| `throw` | `return Err(e)` | Errors are return values |
| `finally` | Drop trait | Automatic cleanup |
| `using` | RAII | Deterministic resource cleanup |
| Exception propagation | `?` operator | Explicit in code |

## 💻 Practical Examples

### Example 1: API Response Handler

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    data: Option<User>,
    error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn parse_api_response(json: &str) -> Result<User, ApiError> {
    let response: ApiResponse = serde_json::from_str(json)
        .map_err(ApiError::ParseError)?;
    
    if response.status != "success" {
        return Err(ApiError::ApiFailure(
            response.error.unwrap_or_else(|| "Unknown error".to_string())
        ));
    }
    
    response.data.ok_or(ApiError::MissingData)
}

#[derive(Debug)]
enum ApiError {
    ParseError(serde_json::Error),
    ApiFailure(String),
    MissingData,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ParseError(e) => write!(f, "Failed to parse JSON: {}", e),
            ApiError::ApiFailure(msg) => write!(f, "API error: {}", msg),
            ApiError::MissingData => write!(f, "Response missing user data"),
        }
    }
}

impl std::error::Error for ApiError {}

// Usage
fn main() {
    let good_response = r#"
    {
        "status": "success",
        "data": {
            "id": 1,
            "name": "Alice",
            "email": "alice@example.com"
        }
    }"#;
    
    let bad_response = r#"
    {
        "status": "error",
        "error": "User not found"
    }"#;
    
    match parse_api_response(good_response) {
        Ok(user) => println!("Success: {:?}", user),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    match parse_api_response(bad_response) {
        Ok(user) => println!("Success: {:?}", user),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Example 2: Configuration Loader with Multiple Error Types

```rust
use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout_ms: u32,
}

#[derive(Debug)]
enum ConfigError {
    FileNotFound(std::io::Error),
    InvalidFormat(String),
    InvalidPort(ParseIntError),
    InvalidTimeout(ParseIntError),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(e) => {
                write!(f, "Configuration file not found: {}", e)
            }
            ConfigError::InvalidFormat(line) => {
                write!(f, "Invalid configuration format: {}", line)
            }
            ConfigError::InvalidPort(e) => {
                write!(f, "Invalid port number: {}", e)
            }
            ConfigError::InvalidTimeout(e) => {
                write!(f, "Invalid timeout value: {}", e)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(ConfigError::FileNotFound)?;
    
    let mut host = None;
    let mut port = None;
    let mut timeout_ms = None;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(ConfigError::InvalidFormat(line.to_string()));
        }
        
        let key = parts[0].trim();
        let value = parts[1].trim();
        
        match key {
            "host" => host = Some(value.to_string()),
            "port" => {
                port = Some(value.parse()
                    .map_err(ConfigError::InvalidPort)?);
            }
            "timeout_ms" => {
                timeout_ms = Some(value.parse()
                    .map_err(ConfigError::InvalidTimeout)?);
            }
            _ => return Err(ConfigError::InvalidFormat(
                format!("Unknown configuration key: {}", key)
            )),
        }
    }
    
    Ok(Config {
        host: host.ok_or_else(|| {
            ConfigError::InvalidFormat("Missing 'host' configuration".to_string())
        })?,
        port: port.ok_or_else(|| {
            ConfigError::InvalidFormat("Missing 'port' configuration".to_string())
        })?,
        timeout_ms: timeout_ms.ok_or_else(|| {
            ConfigError::InvalidFormat("Missing 'timeout_ms' configuration".to_string())
        })?,
    })
}

fn main() -> Result<(), ConfigError> {
    let config = load_config("app.conf")?;
    println!("Loaded config: {:?}", config);
    Ok(())
}
```

## 💡 Working with Option<T>

### Common Option Methods

```rust
fn demonstrate_option_methods() {
    let maybe_user = get_user(42);
    
    // Transform the value if present
    let maybe_email = maybe_user.map(|user| user.email);
    
    // Provide a default value
    let user_or_default = get_user(0).unwrap_or_else(|| User {
        id: 0,
        email: "guest@example.com".to_string(),
    });
    
    // Chain operations
    let email_length = get_user(42)
        .map(|user| user.email)
        .map(|email| email.len())
        .unwrap_or(0);
    
    // Convert Option to Result
    let user_result: Result<User, &str> = get_user(42)
        .ok_or("User not found");
    
    println!("Email length: {}", email_length);
}
```

### Option Chaining vs C# Null-Conditional

```csharp
// C# null-conditional chaining
var result = user?.Profile?.Email?.Length ?? 0;
```

```rust
// Rust Option chaining
let result = user
    .and_then(|u| u.profile)
    .and_then(|p| p.email)
    .map(|e| e.len())
    .unwrap_or(0);

// Or with a more functional approach
let result = user
    .as_ref()
    .and_then(|u| u.profile.as_ref())
    .and_then(|p| p.email.as_ref())
    .map(|e| e.len())
    .unwrap_or(0);
```

## 🎯 Key Takeaways

1. **Explicit Error Handling**: Rust makes errors visible in function signatures
2. **Compiler Assistance**: The compiler ensures you handle all error cases
3. **Performance**: No exception overhead - errors are just data
4. **Composability**: `?` operator makes error handling ergonomic
5. **Type Safety**: Different error types are distinct and explicit

## 🔨 Exercises

### Exercise 1: Basic Option Handling
```rust
// Fix this function to handle the Option properly
fn get_user_name(user_id: u32) -> String {
    let user = find_user(user_id); // Returns Option<User>
    // TODO: Return the user's name or "Unknown" if not found
    todo!()
}

fn find_user(id: u32) -> Option<User> {
    if id > 0 && id <= 100 {
        Some(User {
            id,
            email: format!("user{}@example.com", id),
        })
    } else {
        None
    }
}
```

### Exercise 2: Result Propagation
```rust
// Complete this function using the ? operator
fn calculate_total_score(scores: &[&str]) -> Result<u32, std::num::ParseIntError> {
    let mut total = 0;
    
    for score_str in scores {
        // TODO: Parse each score and add to total
        // Use ? operator for error propagation
    }
    
    Ok(total)
}
```

### Exercise 3: Custom Error Types
```rust
// Create a custom error type for a simple calculator
#[derive(Debug)]
enum CalcError {
    // TODO: Define error variants for:
    // - Division by zero
    // - Invalid operation
    // - Number parsing error
}

// TODO: Implement Display and Error traits for CalcError

fn safe_divide(a: f64, b: f64) -> Result<f64, CalcError> {
    // TODO: Implement safe division
    todo!()
}
```

## 📚 Additional Resources

- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [The anyhow crate](https://docs.rs/anyhow/) - Flexible error handling

---

Next: [Error Propagation](02-error-propagation.md) →
