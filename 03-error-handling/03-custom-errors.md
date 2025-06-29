# Lesson 03: Custom Error Types

Now you'll learn to create robust, domain-specific error types that make your APIs clear and your error handling precise. This is where Rust's type system really shines for building maintainable applications.

## 🎯 Learning Objectives

- Design custom error types for your domain
- Use the `thiserror` crate for cleaner error definitions
- Implement error hierarchies and error chains
- Create actionable error types that guide users to solutions
- Compare with C# custom exception design

## 🏗️ Why Custom Error Types Matter

### The Problem with Generic Errors

```rust
// BAD: Generic, unhelpful error handling
fn process_user_data(data: &str) -> Result<User, String> {
    if data.is_empty() {
        return Err("Data is empty".to_string());
    }
    
    if !data.contains('@') {
        return Err("Invalid email".to_string());
    }
    
    // Caller can't handle different errors differently!
    Ok(User { /* ... */ })
}

// Caller can't distinguish between different error types
match process_user_data(input) {
    Ok(user) => println!("Success: {:?}", user),
    Err(e) => {
        // What kind of error? How should we respond?
        eprintln!("Something went wrong: {}", e);
    }
}
```

### The Solution: Specific Error Types

```rust
// GOOD: Specific, actionable errors
#[derive(Debug)]
enum UserProcessingError {
    EmptyData,
    InvalidEmail { provided: String },
    MissingRequiredField { field: String },
    InvalidAge { provided: i32, min: i32, max: i32 },
}

fn process_user_data(data: &str) -> Result<User, UserProcessingError> {
    if data.is_empty() {
        return Err(UserProcessingError::EmptyData);
    }
    
    // Now errors carry specific, actionable information
    let email = extract_email(data)?;
    let age = extract_age(data)?;
    
    Ok(User { email, age })
}

// Caller can handle each error type appropriately
match process_user_data(input) {
    Ok(user) => println!("Success: {:?}", user),
    Err(UserProcessingError::EmptyData) => {
        eprintln!("Please provide user data");
    }
    Err(UserProcessingError::InvalidEmail { provided }) => {
        eprintln!("Invalid email '{}'. Please use format: user@domain.com", provided);
    }
    Err(UserProcessingError::InvalidAge { provided, min, max }) => {
        eprintln!("Age {} is invalid. Must be between {} and {}", provided, min, max);
    }
    Err(UserProcessingError::MissingRequiredField { field }) => {
        eprintln!("Missing required field: {}. Please add it to your data", field);
    }
}
```

## 🔧 Building Custom Error Types

### Basic Custom Error with Manual Implementation

```rust
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed { host: String, port: u16 },
    QueryFailed { query: String, reason: String },
    TransactionTimeout { duration_ms: u64 },
    PermissionDenied { user: String, operation: String },
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed { host, port } => {
                write!(f, "Failed to connect to database at {}:{}", host, port)
            }
            DatabaseError::QueryFailed { query, reason } => {
                write!(f, "Query failed: '{}' - {}", query, reason)
            }
            DatabaseError::TransactionTimeout { duration_ms } => {
                write!(f, "Transaction timed out after {}ms", duration_ms)
            }
            DatabaseError::PermissionDenied { user, operation } => {
                write!(f, "User '{}' is not permitted to perform '{}'", user, operation)
            }
        }
    }
}

impl std::error::Error for DatabaseError {}

// Usage example
fn execute_query(query: &str, user: &str) -> Result<QueryResult, DatabaseError> {
    if query.trim().is_empty() {
        return Err(DatabaseError::QueryFailed {
            query: query.to_string(),
            reason: "Query cannot be empty".to_string(),
        });
    }
    
    if query.to_uppercase().contains("DROP") && user != "admin" {
        return Err(DatabaseError::PermissionDenied {
            user: user.to_string(),
            operation: "DROP operation".to_string(),
        });
    }
    
    // Simulate successful query
    Ok(QueryResult { rows_affected: 5 })
}

struct QueryResult {
    rows_affected: usize,
}
```

## 🎨 Using thiserror for Cleaner Error Types

The `thiserror` crate eliminates boilerplate and makes error types more maintainable:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileProcessingError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied accessing file: {path}")]
    PermissionDenied { path: String },
    
    #[error("File is empty: {path}")]
    EmptyFile { path: String },
    
    #[error("Invalid file format. Expected {expected}, found {found}")]
    InvalidFormat { expected: String, found: String },
    
    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },
    
    #[error("Encoding error in file {path}: {details}")]
    EncodingError { path: String, details: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

// thiserror automatically implements Display and Error traits!

fn process_config_file(path: &str) -> Result<Config, FileProcessingError> {
    // Check file size first
    let metadata = std::fs::metadata(path)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => FileProcessingError::FileNotFound {
                path: path.to_string(),
            },
            std::io::ErrorKind::PermissionDenied => FileProcessingError::PermissionDenied {
                path: path.to_string(),
            },
            _ => FileProcessingError::Io(e),
        })?;
    
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
    if metadata.len() > MAX_FILE_SIZE {
        return Err(FileProcessingError::FileTooLarge {
            size: metadata.len(),
            max: MAX_FILE_SIZE,
        });
    }
    
    // Read and parse file
    let content = std::fs::read_to_string(path)?; // Automatically converts io::Error
    
    if content.is_empty() {
        return Err(FileProcessingError::EmptyFile {
            path: path.to_string(),
        });
    }
    
    // Try to parse as JSON
    let config: Config = serde_json::from_str(&content)?; // Automatically converts serde_json::Error
    
    Ok(config)
}

#[derive(serde::Deserialize)]
struct Config {
    name: String,
    version: String,
    features: Vec<String>,
}
```

## 🔗 Error Chaining and Hierarchies

### Nested Errors for Complex Operations

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),
    
    #[error("Business logic error: {0}")]
    Business(#[from] BusinessError),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid configuration: {field} is required")]
    MissingField { field: String },
    
    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials for user: {username}")]
    InvalidCredentials { username: String },
    
    #[error("User account is locked: {username}")]
    AccountLocked { username: String },
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Insufficient permissions for operation: {operation}")]
    InsufficientPermissions { operation: String },
}

#[derive(Error, Debug)]
pub enum BusinessError {
    #[error("User not found: {user_id}")]
    UserNotFound { user_id: u32 },
    
    #[error("Insufficient balance: need {required}, have {available}")]
    InsufficientBalance { required: f64, available: f64 },
    
    #[error("Order already processed: {order_id}")]
    OrderAlreadyProcessed { order_id: String },
    
    #[error("Inventory unavailable: {item} (requested: {requested}, available: {available})")]
    InventoryUnavailable { 
        item: String, 
        requested: u32, 
        available: u32 
    },
}

// High-level operation that can fail in multiple ways
fn process_order(order_id: &str, user_id: u32) -> Result<OrderResult, ApplicationError> {
    // Load configuration - can fail with ConfigError
    let config = load_config("app.toml")?;
    
    // Authenticate user - can fail with AuthError
    let user = authenticate_user(user_id, &config.auth_token)?;
    
    // Load order from database - can fail with DatabaseError
    let order = load_order(order_id)?;
    
    // Validate business rules - can fail with BusinessError
    validate_order(&order, &user)?;
    
    // Process payment - can fail with various errors
    let payment_result = process_payment(&order)?;
    
    Ok(OrderResult {
        order_id: order_id.to_string(),
        amount: payment_result.amount,
        status: "completed".to_string(),
    })
}

// Error handling can be specific to error type
fn handle_order_processing() {
    match process_order("ORDER-123", 456) {
        Ok(result) => {
            println!("Order processed successfully: {:?}", result);
        }
        Err(ApplicationError::Config(ConfigError::FileNotFound { path })) => {
            eprintln!("Setup issue: Configuration file missing at {}", path);
            eprintln!("Please create the configuration file and try again.");
        }
        Err(ApplicationError::Auth(AuthError::InvalidCredentials { username })) => {
            eprintln!("Authentication failed for user: {}", username);
            eprintln!("Please check your credentials and try again.");
        }
        Err(ApplicationError::Business(BusinessError::InsufficientBalance { required, available })) => {
            eprintln!("Insufficient funds: ${:.2} required, ${:.2} available", required, available);
            eprintln!("Please add funds to your account and try again.");
        }
        Err(e) => {
            eprintln!("Order processing failed: {}", e);
            
            // Print the full error chain
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {}", err);
                source = err.source();
            }
        }
    }
}

// Supporting types and functions
struct AppConfig {
    auth_token: String,
}

struct User {
    id: u32,
    balance: f64,
}

struct Order {
    id: String,
    amount: f64,
    user_id: u32,
}

struct OrderResult {
    order_id: String,
    amount: f64,
    status: String,
}

struct PaymentResult {
    amount: f64,
}

fn load_config(path: &str) -> Result<AppConfig, ConfigError> {
    // Simulate config loading
    Ok(AppConfig {
        auth_token: "secret-token".to_string(),
    })
}

fn authenticate_user(user_id: u32, token: &str) -> Result<User, AuthError> {
    if user_id == 0 {
        return Err(AuthError::InvalidCredentials {
            username: format!("user-{}", user_id),
        });
    }
    
    Ok(User {
        id: user_id,
        balance: 100.0,
    })
}

fn load_order(order_id: &str) -> Result<Order, DatabaseError> {
    if order_id.is_empty() {
        return Err(DatabaseError::QueryFailed {
            query: "SELECT * FROM orders WHERE id = ?".to_string(),
            reason: "Invalid order ID".to_string(),
        });
    }
    
    Ok(Order {
        id: order_id.to_string(),
        amount: 25.99,
        user_id: 456,
    })
}

fn validate_order(order: &Order, user: &User) -> Result<(), BusinessError> {
    if user.balance < order.amount {
        return Err(BusinessError::InsufficientBalance {
            required: order.amount,
            available: user.balance,
        });
    }
    
    Ok(())
}

fn process_payment(order: &Order) -> Result<PaymentResult, BusinessError> {
    Ok(PaymentResult {
        amount: order.amount,
    })
}
```

## 💡 Error Design Best Practices

### 1. Make Errors Actionable

```rust
use thiserror::Error;

// BAD: Vague error
#[derive(Error, Debug)]
#[error("Validation failed")]
struct ValidationError;

// GOOD: Specific, actionable error
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Email is required")]
    MissingEmail,
    
    #[error("Email '{email}' is invalid. Expected format: user@domain.com")]
    InvalidEmailFormat { email: String },
    
    #[error("Password must be at least {min_length} characters (got {actual_length})")]
    PasswordTooShort { min_length: usize, actual_length: usize },
    
    #[error("Age {age} is invalid. Must be between {min} and {max}")]
    InvalidAge { age: i32, min: i32, max: i32 },
}
```

### 2. Include Context in Error Types

```rust
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {method} {url} - {status}")]
    HttpError {
        method: String,
        url: String,
        status: u16,
        #[source]
        source: reqwest::Error,
    },
    
    #[error("Rate limit exceeded: {requests}/{window_seconds}s (retry after {retry_after}s)")]
    RateLimited {
        requests: u32,
        window_seconds: u32,
        retry_after: u32,
    },
    
    #[error("API response invalid: expected {expected_type}, got {actual_type}")]
    InvalidResponse {
        expected_type: String,
        actual_type: String,
        raw_response: String,
    },
}
```

### 3. Create Error Hierarchies for Different Abstraction Levels

```rust
// Low-level errors (for library code)
#[derive(Error, Debug)]
pub enum HttpClientError {
    #[error("Connection timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("DNS resolution failed for {hostname}")]
    DnsError { hostname: String },
    
    #[error("TLS handshake failed")]
    TlsError(#[from] rustls::Error),
}

// Mid-level errors (for service layer)
#[derive(Error, Debug)]
pub enum ApiServiceError {
    #[error("Network error: {0}")]
    Network(#[from] HttpClientError),
    
    #[error("Authentication failed")]
    Unauthorized,
    
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Server error: {message}")]
    ServerError { message: String },
}

// High-level errors (for application layer)
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Service unavailable: {0}")]
    Service(#[from] ApiServiceError),
    
    #[error("User error: {message}")]
    User { message: String },
    
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
}
```

## 🔄 Comparing with C# Custom Exceptions

### C# Exception Hierarchy
```csharp
public class DataProcessingException : Exception
{
    public string FileName { get; }
    
    public DataProcessingException(string fileName, string message) 
        : base(message)
    {
        FileName = fileName;
    }
    
    public DataProcessingException(string fileName, string message, Exception innerException) 
        : base(message, innerException)
    {
        FileName = fileName;
    }
}

public class ValidationException : DataProcessingException
{
    public string Field { get; }
    public object Value { get; }
    
    public ValidationException(string fileName, string field, object value, string message) 
        : base(fileName, message)
    {
        Field = field;
        Value = value;
    }
}

// Usage
try
{
    ProcessDataFile("data.csv");
}
catch (ValidationException ex)
{
    Console.WriteLine($"Validation error in {ex.FileName}: {ex.Field} = {ex.Value}");
}
catch (DataProcessingException ex)
{
    Console.WriteLine($"Processing error: {ex.Message}");
}
```

### Rust Error Hierarchy
```rust
#[derive(Error, Debug)]
pub enum DataProcessingError {
    #[error("Validation error in {file_name}: {field} = {value:?} - {message}")]
    Validation {
        file_name: String,
        field: String,
        value: serde_json::Value,
        message: String,
    },
    
    #[error("Parsing error in {file_name}: {message}")]
    Parsing {
        file_name: String,
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("IO error processing {file_name}: {0}")]
    Io {
        file_name: String,
        #[source]
        source: std::io::Error,
    },
}

// Usage
match process_data_file("data.csv") {
    Ok(data) => println!("Success: {:?}", data),
    Err(DataProcessingError::Validation { file_name, field, value, message }) => {
        eprintln!("Validation error in {}: {} = {:?} - {}", file_name, field, value, message);
    }
    Err(DataProcessingError::Parsing { file_name, message, .. }) => {
        eprintln!("Parsing error in {}: {}", file_name, message);
    }
    Err(e) => {
        eprintln!("Processing error: {}", e);
    }
}
```

**Key Differences:**
- **Rust**: Errors are data, exhaustive matching required
- **C#**: Exceptions are objects, can be ignored (until runtime)
- **Rust**: Error types are part of the function signature
- **C#**: Exception types are not visible in method signatures

## 🎯 Key Takeaways

1. **Custom error types** make APIs clearer and error handling more precise
2. **thiserror crate** eliminates boilerplate for error type definitions
3. **Error hierarchies** provide appropriate abstraction levels
4. **Actionable errors** include context and guidance for resolution
5. **Pattern matching** enables specific handling for specific error types

## 💻 Exercises

### Exercise 1: Design a File Processing Error Type
```rust
use thiserror::Error;

// TODO: Create a comprehensive error type for file processing
#[derive(Error, Debug)]
pub enum FileProcessorError {
    // TODO: Add variants for:
    // - File not found (include path)
    // - Permission denied (include path and operation)
    // - File too large (include size and max allowed)
    // - Invalid file extension (include expected and actual)
    // - Empty file (include path)
    // - Corrupt file (include path and details)
    // - Network error (when reading from URL)
    // - Conversion from io::Error
    // - Conversion from serde_json::Error
}

fn process_file(path: &str) -> Result<ProcessedFile, FileProcessorError> {
    // TODO: Implement file processing with appropriate error handling
    todo!()
}

struct ProcessedFile {
    path: String,
    size: u64,
    content_type: String,
}
```

### Exercise 2: Create an Error Hierarchy
```rust
// TODO: Create a three-level error hierarchy:
// 1. Low-level: NetworkError, DatabaseError, FileSystemError
// 2. Mid-level: ServiceError (wraps low-level errors)
// 3. High-level: ApplicationError (wraps service errors)

// Each level should add appropriate context and abstraction

#[derive(Error, Debug)]
pub enum NetworkError {
    // TODO: Define network-specific errors
}

#[derive(Error, Debug)]
pub enum ServiceError {
    // TODO: Define service-level errors that wrap NetworkError
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    // TODO: Define application-level errors
}

fn perform_complex_operation() -> Result<String, ApplicationError> {
    // TODO: Implement an operation that can fail at multiple levels
    todo!()
}
```

### Exercise 3: Error Recovery Strategies
```rust
// TODO: Implement retry logic with custom error handling
#[derive(Error, Debug)]
pub enum RetryableError {
    #[error("Temporary failure: {message} (attempt {attempt}/{max_attempts})")]
    Temporary {
        message: String,
        attempt: u32,
        max_attempts: u32,
    },
    
    #[error("Permanent failure: {message}")]
    Permanent { message: String },
    
    #[error("Rate limited: retry after {seconds}s")]
    RateLimited { seconds: u32 },
}

async fn retry_operation<F, T, E>(
    operation: F,
    max_attempts: u32,
) -> Result<T, RetryableError>
where
    F: Fn() -> Result<T, E>,
    E: Into<RetryableError>,
{
    // TODO: Implement retry logic with exponential backoff
    // Handle different error types appropriately
    todo!()
}
```

## 📚 Additional Resources

- [thiserror crate documentation](https://docs.rs/thiserror/)
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [Rust API Guidelines - Error Design](https://rust-lang.github.io/api-guidelines/type-safety.html#error-types-are-meaningful-and-well-behaved-c-good-err)

---

Next: [File Processor Project](../project-file-processor/README.md) →
