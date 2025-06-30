# Exercise 3 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working custom error type implementations. Here's the full solution with all exercises implemented.

## 📝 Complete ex03-error-types.rs Implementation

```rust
// Exercise 3: Custom Error Types - Complete Solutions
//
// This file demonstrates comprehensive custom error type design

use std::fmt;
use std::error::Error;
use std::num::ParseIntError;
use std::io;

fn main() {
    println!("=== Exercise 3: Custom Error Types (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    test_validation_errors();
    test_network_errors();
    test_database_errors();
    test_application_errors();
    test_error_conversion();
    test_error_chains();
    
    println!("\n🎉 All error handling exercises completed!");
}

fn test_validation_errors() {
    println!("Testing validation errors...");
    
    // Exercise 3.1: User validation
    let test_cases = [
        ("", "valid@email.com", 25),
        ("John", "", 25),
        ("John", "invalid-email", 25),
        ("John", "valid@email.com", 200),
        ("John", "valid@email.com", 25),
    ];
    
    for (name, email, age) in test_cases {
        match validate_user(name, email, age) {
            Ok(user) => println!("✅ Valid user: {:?}", user),
            Err(e) => println!("❌ Validation error: {}", e),
        }
    }
    
    println!("✅ Validation errors test complete\n");
}

fn test_network_errors() {
    println!("Testing network errors...");
    
    // Simulate different network scenarios
    let urls = [
        "https://api.example.com/users",
        "https://timeout.example.com/data",
        "https://broken-api.example.com/404",
        "invalid-url",
    ];
    
    for url in urls {
        match simulate_http_request(url) {
            Ok(data) => println!("✅ Request successful: {}", data),
            Err(e) => {
                println!("❌ Request failed: {}", e);
                handle_network_error(&e);
            }
        }
    }
    
    println!("✅ Network errors test complete\n");
}

fn test_database_errors() {
    println!("Testing database errors...");
    
    // Simulate database operations
    let user_ids = [1, 2, 999, 42, 100, 200];
    
    for id in user_ids {
        match get_user_from_database(id) {
            Ok(user) => println!("✅ Found user: {}", user),
            Err(e) => {
                println!("❌ Database error: {}", e);
                handle_database_error(&e);
            }
        }
    }
    
    println!("✅ Database errors test complete\n");
}

fn test_application_errors() {
    println!("Testing application errors...");
    
    // Complex operation that can fail in multiple ways
    let file_operations = [
        "read_config.json",
        "process_data.csv",
        "nonexistent_file.txt",
        "corrupted_data.json",
    ];
    
    for filename in file_operations {
        match process_file(filename) {
            Ok(result) => println!("✅ File processed: {}", result),
            Err(e) => {
                println!("❌ Processing failed: {}", e);
                print_error_chain(&e);
            }
        }
    }
    
    println!("✅ Application errors test complete\n");
}

fn test_error_conversion() {
    println!("Testing error conversion...");
    
    // Test automatic error conversion with ?
    let inputs = ["42", "not_a_number", "123", "456"];
    
    for input in inputs {
        match parse_and_validate(input) {
            Ok(value) => println!("✅ Parsed and validated: {}", value),
            Err(e) => println!("❌ Error: {}", e),
        }
    }
    
    println!("✅ Error conversion test complete\n");
}

fn test_error_chains() {
    println!("Testing error chains...");
    
    // Test complex operations with multiple failure points
    match complex_operation() {
        Ok(result) => println!("✅ Complex operation succeeded: {}", result),
        Err(e) => {
            println!("❌ Complex operation failed: {}", e);
            
            // Walk the error chain
            let mut source = e.source();
            let mut level = 1;
            while let Some(err) = source {
                println!("{}└─ Caused by: {}", "  ".repeat(level), err);
                source = err.source();
                level += 1;
            }
        }
    }
    
    println!("✅ Error chains test complete\n");
}

// ============================================================================
// Exercise 3.1: Validation error type - SOLVED
// ============================================================================

#[derive(Debug)]
enum ValidationError {
    EmptyName,
    InvalidEmail(String),
    InvalidAge(u32),
    TooYoung,
    TooOld,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "Name cannot be empty"),
            ValidationError::InvalidEmail(email) => write!(f, "Invalid email format: {}", email),
            ValidationError::InvalidAge(age) => write!(f, "Invalid age: {}", age),
            ValidationError::TooYoung => write!(f, "Age must be at least 13"),
            ValidationError::TooOld => write!(f, "Age must be less than 120"),
        }
    }
}

impl Error for ValidationError {}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

fn validate_user(name: &str, email: &str, age: u32) -> Result<User, ValidationError> {
    // Validate name
    if name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    
    // Validate email
    if email.is_empty() || !is_valid_email(email) {
        return Err(ValidationError::InvalidEmail(email.to_string()));
    }
    
    // Validate age
    match age {
        0 => Err(ValidationError::InvalidAge(age)),
        1..=12 => Err(ValidationError::TooYoung),
        120.. => Err(ValidationError::TooOld),
        _ => Ok(User {
            name: name.to_string(),
            email: email.to_string(),
            age,
        }),
    }
}

// ============================================================================
// Exercise 3.2: Network error type - SOLVED
// ============================================================================

#[derive(Debug)]
enum NetworkError {
    Timeout,
    ConnectionFailed(String),
    HttpError(u16),
    InvalidUrl(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::Timeout => write!(f, "Request timed out"),
            NetworkError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            NetworkError::HttpError(status) => {
                match *status {
                    400 => write!(f, "Bad Request (400): The request was invalid"),
                    401 => write!(f, "Unauthorized (401): Authentication required"),
                    403 => write!(f, "Forbidden (403): Access denied"),
                    404 => write!(f, "Not Found (404): Resource not found"),
                    500 => write!(f, "Internal Server Error (500): Server error"),
                    502 => write!(f, "Bad Gateway (502): Invalid response from upstream"),
                    503 => write!(f, "Service Unavailable (503): Service temporarily unavailable"),
                    _ => write!(f, "HTTP Error: {} - Unexpected status code", status),
                }
            }
            NetworkError::InvalidUrl(url) => write!(f, "Invalid URL format: {}", url),
        }
    }
}

impl Error for NetworkError {}

impl NetworkError {
    fn is_retryable(&self) -> bool {
        match self {
            NetworkError::Timeout => true,
            NetworkError::ConnectionFailed(_) => true,
            NetworkError::HttpError(status) => matches!(status, 429 | 502 | 503 | 504),
            NetworkError::InvalidUrl(_) => false,
        }
    }
    
    fn retry_delay(&self) -> std::time::Duration {
        match self {
            NetworkError::Timeout => std::time::Duration::from_secs(1),
            NetworkError::ConnectionFailed(_) => std::time::Duration::from_secs(2),
            NetworkError::HttpError(429) => std::time::Duration::from_secs(60), // Rate limited
            NetworkError::HttpError(_) => std::time::Duration::from_secs(5),
            NetworkError::InvalidUrl(_) => std::time::Duration::from_secs(0),
        }
    }
}

fn simulate_http_request(url: &str) -> Result<String, NetworkError> {
    // Simulate different failure scenarios based on URL content
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(NetworkError::InvalidUrl(url.to_string()));
    }
    
    if url.contains("timeout") {
        return Err(NetworkError::Timeout);
    }
    
    if url.contains("broken") {
        return Err(NetworkError::ConnectionFailed("DNS resolution failed".to_string()));
    }
    
    if url.contains("404") {
        return Err(NetworkError::HttpError(404));
    }
    
    if url.contains("500") {
        return Err(NetworkError::HttpError(500));
    }
    
    // Simulate success
    Ok(format!("HTTP 200 OK: Response from {}", url))
}

fn handle_network_error(error: &NetworkError) {
    match error {
        NetworkError::Timeout => {
            println!("💡 Suggestion: Check network connection and retry with longer timeout");
        }
        NetworkError::ConnectionFailed(msg) => {
            println!("💡 Suggestion: Check network connectivity: {}", msg);
        }
        NetworkError::HttpError(status) => {
            match status {
                404 => println!("💡 Suggestion: Verify the URL is correct"),
                500..=599 => println!("💡 Suggestion: Server issue, try again later"),
                400..=499 => println!("💡 Suggestion: Check request parameters"),
                _ => println!("💡 Suggestion: Check HTTP status code documentation"),
            }
        }
        NetworkError::InvalidUrl(url) => {
            println!("💡 Suggestion: Fix URL format: {}", url);
        }
    }
    
    if error.is_retryable() {
        println!("🔄 This error might succeed if retried after {:?}", error.retry_delay());
    } else {
        println!("❌ This error will not succeed on retry");
    }
}

// ============================================================================
// Exercise 3.3: Database error type - SOLVED
// ============================================================================

#[derive(Debug)]
enum DatabaseError {
    ConnectionFailed,
    UserNotFound(u32),
    QueryFailed(String),
    PermissionDenied,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed => write!(f, "Failed to connect to database"),
            DatabaseError::UserNotFound(id) => write!(f, "User with ID {} not found", id),
            DatabaseError::QueryFailed(query) => write!(f, "Database query failed: {}", query),
            DatabaseError::PermissionDenied => write!(f, "Permission denied for database operation"),
        }
    }
}

impl Error for DatabaseError {}

impl DatabaseError {
    fn is_retryable(&self) -> bool {
        match self {
            DatabaseError::ConnectionFailed => true,
            DatabaseError::QueryFailed(_) => true,
            DatabaseError::UserNotFound(_) => false,
            DatabaseError::PermissionDenied => false,
        }
    }
    
    fn error_code(&self) -> &'static str {
        match self {
            DatabaseError::ConnectionFailed => "DB_CONNECTION_FAILED",
            DatabaseError::UserNotFound(_) => "DB_USER_NOT_FOUND",
            DatabaseError::QueryFailed(_) => "DB_QUERY_FAILED",
            DatabaseError::PermissionDenied => "DB_PERMISSION_DENIED",
        }
    }
}

fn get_user_from_database(id: u32) -> Result<String, DatabaseError> {
    // Simulate database lookup with different failure scenarios
    match id {
        1 => Ok("Alice Johnson".to_string()),
        2 => Ok("Bob Smith".to_string()),
        42 => Ok("Douglas Adams".to_string()),
        999 => Err(DatabaseError::UserNotFound(id)),
        100 => Err(DatabaseError::ConnectionFailed),
        200 => Err(DatabaseError::PermissionDenied),
        0 => Err(DatabaseError::QueryFailed("Invalid user ID: cannot be zero".to_string())),
        _ => Ok(format!("User{}", id)),
    }
}

fn handle_database_error(error: &DatabaseError) {
    println!("🏷️  Error Code: {}", error.error_code());
    
    match error {
        DatabaseError::ConnectionFailed => {
            println!("💡 Suggestion: Check database server status and network connectivity");
            println!("🔧 Action: Verify connection string and database availability");
        }
        DatabaseError::UserNotFound(id) => {
            println!("💡 Suggestion: Verify user ID {} exists in the system", id);
            println!("🔧 Action: Check if user was deleted or ID is correct");
        }
        DatabaseError::QueryFailed(query) => {
            println!("💡 Suggestion: Review query syntax and parameters");
            println!("🔧 Action: Check query: {}", query);
        }
        DatabaseError::PermissionDenied => {
            println!("💡 Suggestion: Contact database administrator for access rights");
            println!("🔧 Action: Verify user has required database permissions");
        }
    }
    
    if error.is_retryable() {
        println!("🔄 This error might succeed if retried");
    } else {
        println!("❌ This error requires manual intervention");
    }
}

// ============================================================================
// Exercise 3.4: Application-level error that combines multiple error types - SOLVED
// ============================================================================

#[derive(Debug)]
enum AppError {
    Validation(ValidationError),
    Network(NetworkError),
    Database(DatabaseError),
    Io(io::Error),
    Parse(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Validation(e) => write!(f, "Validation failed: {}", e),
            AppError::Network(e) => write!(f, "Network operation failed: {}", e),
            AppError::Database(e) => write!(f, "Database operation failed: {}", e),
            AppError::Io(e) => write!(f, "I/O operation failed: {}", e),
            AppError::Parse(e) => write!(f, "Parsing failed: {}", e),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Validation(e) => Some(e),
            AppError::Network(e) => Some(e),
            AppError::Database(e) => Some(e),
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
        }
    }
}

// Exercise 3.5: Implement From traits for automatic error conversion - SOLVED
impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::Validation(err)
    }
}

impl From<NetworkError> for AppError {
    fn from(err: NetworkError) -> Self {
        AppError::Network(err)
    }
}

impl From<DatabaseError> for AppError {
    fn from(err: DatabaseError) -> Self {
        AppError::Database(err)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

// File processing function that uses AppError - SOLVED
fn process_file(filename: &str) -> Result<String, AppError> {
    // Simulate file processing with multiple failure points
    match filename {
        "read_config.json" => {
            // Simulate successful processing
            Ok("Configuration loaded and validated successfully".to_string())
        }
        "process_data.csv" => {
            // Simulate validation error (auto-converted to AppError)
            validate_user("", "invalid@email", 25)?;
            Ok("Data processed successfully".to_string())
        }
        "nonexistent_file.txt" => {
            // Simulate I/O error (auto-converted to AppError)
            std::fs::read_to_string(filename)?;
            Ok("File read successfully".to_string())
        }
        "corrupted_data.json" => {
            // Simulate parse error (auto-converted to AppError)
            let _number = "not_a_number".parse::<i32>()?;
            Ok("Data parsed successfully".to_string())
        }
        "network_resource.xml" => {
            // Simulate network error (auto-converted to AppError)
            simulate_http_request("https://broken-api.example.com/404")?;
            Ok("Network resource processed successfully".to_string())
        }
        _ => {
            // Simulate database error (auto-converted to AppError)
            get_user_from_database(999)?;
            Ok("Database operation completed successfully".to_string())
        }
    }
}

// Function that prints the full error chain - SOLVED
fn print_error_chain(error: &dyn Error) {
    println!("🔍 Error chain analysis:");
    println!("┌─ Primary error: {}", error);
    
    let mut source = error.source();
    let mut level = 1;
    
    while let Some(err) = source {
        let prefix = if source.is_some() && err.source().is_some() {
            "├─"
        } else {
            "└─"
        };
        println!("{}{}Caused by: {}", "│ ".repeat(level), prefix, err);
        source = err.source();
        level += 1;
    }
    
    if level == 1 {
        println!("└─ No underlying cause");
    }
}

// Function that combines multiple error-prone operations - SOLVED
fn parse_and_validate(input: &str) -> Result<u32, AppError> {
    // Parse string to number (auto-converts ParseIntError to AppError)
    let number = input.parse::<u32>()?;
    
    // Validate the parsed number (auto-converts ValidationError to AppError)
    if number == 0 {
        return Err(ValidationError::InvalidAge(number).into());
    }
    
    if number > 150 {
        return Err(ValidationError::TooOld.into());
    }
    
    Ok(number)
}

// Complex operation with error chaining - SOLVED
fn complex_operation() -> Result<String, AppError> {
    // Validate a user (ValidationError -> AppError)
    let user = validate_user("Alice", "alice@example.com", 30)?;
    
    // Get user data from database (DatabaseError -> AppError)  
    let user_data = get_user_from_database(1)?;
    
    // Make network request (NetworkError -> AppError)
    let response = simulate_http_request("https://api.example.com/users")?;
    
    // Parse some configuration (ParseIntError -> AppError)
    let config_value = "42".parse::<i32>()?;
    
    Ok(format!(
        "Operation successful: User {}, Data: {}, Response: {}, Config: {}",
        user.name, user_data, response, config_value
    ))
}

// Advanced error handling demonstrations
fn demonstrate_error_recovery() {
    println!("=== Error Recovery Strategies ===");
    
    // Strategy 1: Retry with backoff
    fn retry_with_backoff<F, T, E>(mut operation: F, max_attempts: u32) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        E: std::fmt::Debug,
    {
        for attempt in 1..=max_attempts {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_attempts {
                        return Err(e);
                    }
                    println!("Attempt {} failed: {:?}, retrying...", attempt, e);
                    std::thread::sleep(std::time::Duration::from_millis(100 * attempt as u64));
                }
            }
        }
        unreachable!()
    }
    
    // Strategy 2: Fallback chain
    fn load_config_with_fallbacks() -> Result<String, AppError> {
        // Try loading from environment
        std::env::var("APP_CONFIG")
            .map_err(|_| AppError::Io(io::Error::new(io::ErrorKind::NotFound, "env var not found")))
            .or_else(|_| {
                // Fallback to config file
                std::fs::read_to_string("config.json")
                    .map_err(AppError::from)
            })
            .or_else(|_| {
                // Final fallback to default config
                Ok("default configuration".to_string())
            })
    }
    
    // Strategy 3: Partial success collection
    fn process_multiple_items(items: &[&str]) -> (Vec<String>, Vec<AppError>) {
        let mut successes = Vec::new();
        let mut failures = Vec::new();
        
        for item in items {
            match parse_and_validate(item) {
                Ok(value) => successes.push(value.to_string()),
                Err(e) => failures.push(e),
            }
        }
        
        (successes, failures)
    }
    
    // Demonstrate recovery strategies
    println!("Testing retry strategy:");
    let result = retry_with_backoff(|| {
        simulate_http_request("https://timeout.example.com/data")
    }, 3);
    println!("Retry result: {:?}", result);
    
    println!("\nTesting fallback strategy:");
    match load_config_with_fallbacks() {
        Ok(config) => println!("Loaded config: {}", config),
        Err(e) => println!("All fallbacks failed: {}", e),
    }
    
    println!("\nTesting partial success:");
    let items = ["42", "not_a_number", "100", "invalid", "25"];
    let (successes, failures) = process_multiple_items(&items);
    println!("Successes: {:?}", successes);
    println!("Failures: {} items failed", failures.len());
}

// Helper function for email validation
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::EmptyName;
        assert_eq!(format!("{}", error), "Name cannot be empty");
        
        let error = ValidationError::InvalidEmail("bad@email".to_string());
        assert_eq!(format!("{}", error), "Invalid email format: bad@email");
    }
    
    #[test]
    fn test_user_validation() {
        // Test empty name
        assert!(validate_user("", "test@example.com", 25).is_err());
        
        // Test invalid email
        assert!(validate_user("John", "", 25).is_err());
        assert!(validate_user("John", "invalid-email", 25).is_err());
        
        // Test invalid age
        assert!(validate_user("John", "test@example.com", 0).is_err());
        assert!(validate_user("John", "test@example.com", 10).is_err());
        assert!(validate_user("John", "test@example.com", 150).is_err());
        
        // Test valid user
        assert!(validate_user("John", "test@example.com", 25).is_ok());
    }
    
    #[test]
    fn test_error_conversion() {
        let validation_err = ValidationError::EmptyName;
        let app_err: AppError = validation_err.into();
        assert!(matches!(app_err, AppError::Validation(_)));
        
        // Test automatic conversion with ? operator
        fn test_conversion() -> Result<(), AppError> {
            validate_user("", "test@example.com", 25)?;
            Ok(())
        }
        
        assert!(test_conversion().is_err());
    }
    
    #[test]
    fn test_error_chaining() {
        let validation_err = ValidationError::EmptyName;
        let app_err = AppError::Validation(validation_err);
        assert!(app_err.source().is_some());
        
        let source = app_err.source().unwrap();
        assert_eq!(format!("{}", source), "Name cannot be empty");
    }
    
    #[test]
    fn test_network_error_retry_logic() {
        let timeout_error = NetworkError::Timeout;
        assert!(timeout_error.is_retryable());
        assert!(timeout_error.retry_delay() > std::time::Duration::from_secs(0));
        
        let invalid_url_error = NetworkError::InvalidUrl("bad-url".to_string());
        assert!(!invalid_url_error.is_retryable());
        assert_eq!(invalid_url_error.retry_delay(), std::time::Duration::from_secs(0));
    }
    
    #[test]
    fn test_database_error_classification() {
        let connection_error = DatabaseError::ConnectionFailed;
        assert!(connection_error.is_retryable());
        assert_eq!(connection_error.error_code(), "DB_CONNECTION_FAILED");
        
        let not_found_error = DatabaseError::UserNotFound(123);
        assert!(!not_found_error.is_retryable());
        assert_eq!(not_found_error.error_code(), "DB_USER_NOT_FOUND");
    }
    
    #[test]
    fn test_parse_and_validate() {
        // Valid inputs
        assert!(parse_and_validate("42").is_ok());
        assert!(parse_and_validate("25").is_ok());
        
        // Invalid inputs
        assert!(parse_and_validate("not_a_number").is_err());
        assert!(parse_and_validate("0").is_err());
        assert!(parse_and_validate("200").is_err());
    }
    
    #[test]
    fn test_complex_operation_error_chain() {
        // This test demonstrates error propagation through complex operations
        match complex_operation() {
            Ok(_) => {
                // Operation might succeed depending on simulation
            }
            Err(e) => {
                // Verify we can access the error chain
                assert!(format!("{}", e).len() > 0);
                
                // Test error source chain
                let mut depth = 0;
                let mut current = Some(&e as &dyn Error);
                while let Some(err) = current {
                    depth += 1;
                    current = err.source();
                    assert!(depth < 10, "Error chain too deep");
                }
            }
        }
    }
}
```

## 🎓 Key Learning Points from Complete Solutions

### Custom Error Type Design Principles
1. **Specific Variants**: Each enum variant represents a distinct failure mode
2. **Contextual Data**: Include relevant information for debugging and recovery
3. **User-Friendly Messages**: Display trait provides clear, actionable error messages
4. **Error Chaining**: Error trait with source() preserves underlying error information
5. **Automatic Conversion**: From traits enable seamless ? operator usage

### Production-Ready Error Patterns

#### Error Classification
```rust
impl NetworkError {
    fn is_retryable(&self) -> bool { /* classify errors */ }
    fn retry_delay(&self) -> Duration { /* suggest retry timing */ }
    fn error_code(&self) -> &'static str { /* machine-readable codes */ }
}
```

#### Error Context Building
```rust
#[derive(Debug)]
struct ErrorContext {
    operation: String,
    timestamp: SystemTime,
    user_id: Option<u32>,
    request_id: Option<String>,
}

impl AppError {
    fn with_context(self, ctx: ErrorContext) -> ContextualError {
        ContextualError { error: self, context: ctx }
    }
}
```

#### Error Recovery Strategies
```rust
fn resilient_operation() -> Result<Data, AppError> {
    primary_method()
        .or_else(|e| {
            if e.is_retryable() {
                thread::sleep(e.retry_delay());
                retry_method()
            } else {
                fallback_method()
            }
        })
        .or_else(|_| default_response())
}
```

### Error Handling Integration

#### With Logging
```rust
impl AppError {
    fn log_error(&self) {
        match self {
            AppError::Network(e) if e.is_retryable() => {
                log::warn!("Retryable network error: {}", e);
            }
            AppError::Database(e) => {
                log::error!("Database error [{}]: {}", e.error_code(), e);
            }
            _ => {
                log::error!("Application error: {}", self);
            }
        }
    }
}
```

#### With Metrics
```rust
use std::sync::atomic::{AtomicU64, Ordering};

static VALIDATION_ERRORS: AtomicU64 = AtomicU64::new(0);
static NETWORK_ERRORS: AtomicU64 = AtomicU64::new(0);

impl AppError {
    fn increment_metrics(&self) {
        match self {
            AppError::Validation(_) => {
                VALIDATION_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            AppError::Network(_) => {
                NETWORK_ERRORS.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
    }
}
```

#### With Structured Logging
```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorEvent {
    error_type: String,
    message: String,
    timestamp: SystemTime,
    context: HashMap<String, String>,
}

impl From<&AppError> for ErrorEvent {
    fn from(error: &AppError) -> Self {
        ErrorEvent {
            error_type: match error {
                AppError::Validation(_) => "validation".to_string(),
                AppError::Network(_) => "network".to_string(),
                AppError::Database(_) => "database".to_string(),
                AppError::Io(_) => "io".to_string(),
                AppError::Parse(_) => "parse".to_string(),
            },
            message: format!("{}", error),
            timestamp: SystemTime::now(),
            context: HashMap::new(),
        }
    }
}
```

## 🚀 Advanced Error Handling Patterns

### Error Aggregation
```rust
#[derive(Debug)]
struct BatchError {
    successes: u32,
    failures: Vec<(usize, AppError)>,
}

impl BatchError {
    fn from_results(results: Vec<(usize, Result<T, AppError>)>) -> Result<Vec<T>, BatchError> {
        let mut successes = Vec::new();
        let mut failures = Vec::new();
        
        for (idx, result) in results {
            match result {
                Ok(value) => successes.push(value),
                Err(error) => failures.push((idx, error)),
            }
        }
        
        if failures.is_empty() {
            Ok(successes)
        } else {
            Err(BatchError {
                successes: successes.len() as u32,
                failures,
            })
        }
    }
}
```

### Error Transformation Pipelines
```rust
trait ErrorTransform<E> {
    fn transform_error(self) -> Result<Self, E>
    where
        Self: Sized;
}

impl<T> ErrorTransform<AppError> for Result<T, ValidationError> {
    fn transform_error(self) -> Result<T, AppError> {
        self.map_err(AppError::Validation)
    }
}
```

### Error Rate Limiting
```rust
use std::time::{Duration, Instant};

struct ErrorRateLimiter {
    last_error: Option<Instant>,
    error_count: u32,
    window: Duration,
}

impl ErrorRateLimiter {
    fn should_report_error(&mut self) -> bool {
        let now = Instant::now();
        
        match self.last_error {
            None => {
                self.last_error = Some(now);
                self.error_count = 1;
                true
            }
            Some(last) if now.duration_since(last) > self.window => {
                self.last_error = Some(now);
                self.error_count = 1;
                true
            }
            Some(_) => {
                self.error_count += 1;
                self.error_count <= 5  // Report first 5 errors in window
            }
        }
    }
}
```

## 💡 Error Design Best Practices

### Documentation and Examples
```rust
/// Represents validation errors that can occur during user input processing.
/// 
/// # Examples
/// 
/// ```
/// use myapp::ValidationError;
/// 
/// let error = ValidationError::InvalidEmail("not-an-email".to_string());
/// println!("User-friendly: {}", error);
/// println!("Debug info: {:?}", error);
/// ```
#[derive(Debug)]
pub enum ValidationError {
    /// The user's name field is empty or contains only whitespace
    EmptyName,
    
    /// The provided email address is not in a valid format
    /// 
    /// Contains the invalid email address for context
    InvalidEmail(String),
    
    /// The user's age is outside the acceptable range (13-119)
    /// 
    /// Contains the invalid age value
    InvalidAge(u32),
}
```

### Error Testing Strategies
```rust
#[cfg(test)]
mod error_tests {
    use super::*;
    
    #[test]
    fn test_error_display_messages() {
        let errors = vec![
            (ValidationError::EmptyName, "Name cannot be empty"),
            (ValidationError::InvalidEmail("bad".to_string()), "Invalid email format: bad"),
            (ValidationError::InvalidAge(5), "Invalid age: 5"),
        ];
        
        for (error, expected) in errors {
            assert_eq!(format!("{}", error), expected);
        }
    }
    
    #[test]
    fn test_error_source_chains() {
        let parse_error: ParseIntError = "abc".parse::<i32>().unwrap_err();
        let app_error = AppError::Parse(parse_error);
        
        assert!(app_error.source().is_some());
        
        let source = app_error.source().unwrap();
        assert!(format!("{}", source).contains("invalid digit"));
    }
    
    #[test]
    fn test_error_conversion_chains() {
        fn test_conversion() -> Result<i32, AppError> {
            let _user = validate_user("", "test@example.com", 25)?;  // ValidationError -> AppError
            let _data = get_user_from_database(999)?;                // DatabaseError -> AppError  
            let _response = simulate_http_request("invalid-url")?;   // NetworkError -> AppError
            let _number = "abc".parse::<i32>()?;                     // ParseIntError -> AppError
            Ok(42)
        }
        
        assert!(test_conversion().is_err());
    }
}
```

## 🏆 Mastery Checkpoint

You've mastered custom error types when you can:
- ✅ Design error enums with appropriate variants and data
- ✅ Implement Display and Error traits for proper error behavior
- ✅ Create From conversions for automatic error transformation
- ✅ Build error source chains for debugging
- ✅ Add helper methods for error classification and recovery
- ✅ Integrate errors with logging, metrics, and monitoring
- ✅ Design error types that guide users toward solutions

## ➡️ Next Steps

Ready for advanced error conversion patterns? Continue with:
- **[Exercise 4](../ex04-conversions.rs)** - Advanced error conversions and flexible error handling
- **[File Processor Project](../../project-file-processor/)** - Production error handling in a real application

You've now mastered designing structured, actionable error types! 🦀