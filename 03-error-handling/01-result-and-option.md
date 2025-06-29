# Lesson 01: Result and Option Types

Welcome to Rust's approach to error handling! As a C# developer, you're used to exceptions flying around your call stack. Rust takes a radically different approach: errors are values, not control flow interruptions.

## 🎯 Learning Objectives

- Master the `Result<T, E>` type for fallible operations
- Understand `Option<T>` for handling null-like scenarios
- Learn pattern matching for exhaustive error handling
- Apply the `?` operator for elegant error propagation
- Compare Rust's approach with C#'s exception model
- Build safe programs that can't crash from null references

## 📚 Introduction

In C#, errors come at you from two directions: null references (causing `NullReferenceException`) and exceptions (disrupting control flow). These can crash your program at runtime, often in production when you least expect it.

Rust eliminates both problems by making errors explicit in the type system. No more surprise crashes, no more forgotten null checks - the compiler ensures you handle every possible failure case.

## 🚫 The Problems with Exceptions and Nulls

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
    // What if we forget other exception types?
}
```

**Problems:**
- Exceptions are "invisible" in method signatures
- Easy to forget to handle specific error types
- Performance overhead of stack unwinding
- Can interrupt control flow unexpectedly

## ✅ The Rust Way: Errors as Values

### Option<T> - No More Null References

`Option<T>` replaces nullable types and eliminates `NullReferenceException`:

```rust
// Rust Option approach - null safety by design
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

#[derive(Debug)]
struct User {
    id: u32,
    email: String,
}

// Compiler forces you to handle both cases
fn main() {
    match get_user_email(42) {
        Some(email) => println!("Email: {}", email),
        None => println!("User not found"),
    }
    
    // No surprise crashes - all cases handled!
}
```

### Result<T, E> - Explicit Error Handling

`Result<T, E>` replaces exceptions with explicit error values:

```rust
use std::fs;
use std::io;

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    EmptyFile,
    InvalidFormat(String),
}

// Rust file reading with Result - explicit error handling
fn read_config(path: &str) -> Result<String, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(ConfigError::IoError)?;
    
    if content.is_empty() {
        return Err(ConfigError::EmptyFile);
    }
    
    if !content.contains("version") {
        return Err(ConfigError::InvalidFormat(
            "Missing version field".to_string()
        ));
    }
    
    Ok(content)
}

// All error cases are visible and must be handled
fn main() {
    match read_config("app.conf") {
        Ok(content) => println!("Config loaded: {} chars", content.len()),
        Err(ConfigError::IoError(e)) => eprintln!("File error: {}", e),
        Err(ConfigError::EmptyFile) => eprintln!("Config file is empty!"),
        Err(ConfigError::InvalidFormat(msg)) => eprintln!("Invalid format: {}", msg),
    }
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
    // Compiler guarantees all cases are handled!
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
        Err(ConfigError::InvalidFormat(msg)) => {
            eprintln!("Invalid config format: {}", msg);
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

// With ? operator (concise and clear)
fn load_user_data(path: &str) -> Result<UserData, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let user_data = serde_json::from_str(&content)?;
    Ok(user_data)
}

#[derive(serde::Deserialize, Debug)]
struct UserData {
    name: String,
    email: String,
}
```

**How ? Works:**
1. If `Result` is `Ok(value)`, extracts `value`
2. If `Result` is `Err(e)`, returns `Err(e)` from the function
3. Error type must be convertible to the function's error type

## 🔄 Comparison with C#

| C# Pattern | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| `null` | `None` | Compiler forces handling |
| `try-catch` | `match` | Explicit in function signature |
| `throw` | `return Err(e)` | Errors are return values |
| `finally` | Drop trait | Automatic cleanup |
| `using` | RAII | Deterministic resource cleanup |
| Exception propagation | `?` operator | Explicit in code |

## 💻 Practice Exercises

### Exercise 1: Basic Option Handling

```rust
fn main() {
    // Fix this function to handle the Option properly
    let user_name = get_user_name(42);
    // TODO: Print the user's name or "Unknown" if not found
    // Hint: Use pattern matching or unwrap_or
}

fn get_user_name(user_id: u32) -> Option<String> {
    if user_id > 0 && user_id <= 100 {
        Some(format!("User{}", user_id))
    } else {
        None
    }
}
```

### Exercise 2: Result Error Handling

```rust
fn main() {
    // Fix this to handle all possible errors
    let result = divide_numbers("10", "2");
    // TODO: Handle both success and error cases
    // Print the result or an appropriate error message
}

fn divide_numbers(a_str: &str, b_str: &str) -> Result<f64, String> {
    let a = a_str.parse::<f64>().map_err(|_| "Invalid first number")?;
    let b = b_str.parse::<f64>().map_err(|_| "Invalid second number")?;
    
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    
    Ok(a / b)
}
```

### Exercise 3: Option Chaining

```rust
#[derive(Debug)]
struct User {
    id: u32,
    profile: Option<Profile>,
}

#[derive(Debug)]
struct Profile {
    email: Option<String>,
}

fn main() {
    let user = User {
        id: 1,
        profile: Some(Profile {
            email: Some("user@example.com".to_string()),
        }),
    };
    
    // TODO: Extract the email using Option chaining
    // If any step fails, print "No email found"
    // Hint: Use .and_then() or pattern matching
}
```

## 🚀 Mini-Project: Configuration Validator

Build a configuration validator that demonstrates comprehensive error handling:

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Config {
    database_url: String,
    port: u16,
    debug_mode: bool,
}

#[derive(Debug)]
enum ConfigError {
    MissingField(String),
    InvalidPort(String),
    InvalidBool(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ConfigError::InvalidPort(value) => write!(f, "Invalid port number: {}", value),
            ConfigError::InvalidBool(value) => write!(f, "Invalid boolean value: {}", value),
        }
    }
}

impl std::error::Error for ConfigError {}

fn parse_config(input: &str) -> Result<Config, ConfigError> {
    let mut fields = HashMap::new();
    
    // Parse key=value pairs
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            fields.insert(key.trim(), value.trim());
        }
    }
    
    // Extract required fields
    let database_url = fields.get("database_url")
        .ok_or_else(|| ConfigError::MissingField("database_url".to_string()))?
        .to_string();
    
    let port_str = fields.get("port")
        .ok_or_else(|| ConfigError::MissingField("port".to_string()))?;
    
    let port = port_str.parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort(port_str.to_string()))?;
    
    let debug_str = fields.get("debug_mode")
        .ok_or_else(|| ConfigError::MissingField("debug_mode".to_string()))?;
    
    let debug_mode = match debug_str.to_lowercase().as_str() {
        "true" | "yes" | "1" => true,
        "false" | "no" | "0" => false,
        _ => return Err(ConfigError::InvalidBool(debug_str.to_string())),
    };
    
    Ok(Config {
        database_url,
        port,
        debug_mode,
    })
}

fn main() {
    let config_text = r#"
        database_url=postgresql://localhost/myapp
        port=8080
        debug_mode=true
    "#;
    
    match parse_config(config_text) {
        Ok(config) => {
            println!("Configuration loaded successfully:");
            println!("  Database: {}", config.database_url);
            println!("  Port: {}", config.port);
            println!("  Debug: {}", config.debug_mode);
        }
        Err(e) => {
            eprintln!("Configuration error: {}", e);
        }
    }
}
```

## 🔑 Key Takeaways

1. **Explicit Error Handling**: Rust makes errors visible in function signatures
2. **Compiler Assistance**: The compiler ensures you handle all error cases
3. **No Runtime Surprises**: Null pointer exceptions are impossible
4. **Zero Overhead**: Error handling is just data flow, no exceptions
5. **Pattern Matching**: Exhaustive handling prevents forgotten error cases
6. **Composability**: `?` operator makes error handling ergonomic

## 📚 Additional Resources

- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [Option and Result API Documentation](https://doc.rust-lang.org/std/option/)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Use Option<T> to handle potentially missing values
- [ ] Create and handle Result<T, E> for operations that can fail
- [ ] Apply pattern matching to handle all error cases
- [ ] Use the ? operator for error propagation
- [ ] Explain why Rust's approach prevents runtime crashes
- [ ] Compare Rust's error handling with C#'s exception model

---

Next: [Error Propagation](02-error-propagation.md) - Learn elegant error flow with the ? operator →
