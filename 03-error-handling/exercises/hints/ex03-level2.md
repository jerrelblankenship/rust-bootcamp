# Exercise 3 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Custom Error Type Design

You've tried Level 1 hints but need concrete implementation guidance. Here are specific solutions for designing and implementing custom error types.

## 🔧 Exercise 3.1: Validation Error Implementation

**Problem**: Create a ValidationError enum that captures different user validation failures.

**Specific Solution:**
```rust
#[derive(Debug)]
enum ValidationError {
    EmptyName,
    InvalidEmail(String),
    InvalidAge(u32),
    TooYoung,
    TooOld,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "Name cannot be empty"),
            ValidationError::InvalidEmail(email) => write!(f, "Invalid email format: {}", email),
            ValidationError::InvalidAge(age) => write!(f, "Invalid age: {}", age),
            ValidationError::TooYoung => write!(f, "Age must be at least 13"),
            ValidationError::TooOld => write!(f, "Age must be less than 120"),
        }
    }
}

impl std::error::Error for ValidationError {}

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
    if email.is_empty() {
        return Err(ValidationError::InvalidEmail(email.to_string()));
    }
    if !email.contains('@') || !email.contains('.') {
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
```

**Key Learning**: Use enum variants to represent different failure modes with relevant data.

## 🔧 Exercise 3.2: Network Error Implementation

**Problem**: Design NetworkError enum for HTTP client operations.

**Specific Solution:**
```rust
#[derive(Debug)]
enum NetworkError {
    Timeout,
    ConnectionFailed(String),
    HttpError(u16),
    InvalidUrl(String),
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NetworkError::Timeout => write!(f, "Request timed out"),
            NetworkError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            NetworkError::HttpError(status) => {
                match *status {
                    400 => write!(f, "Bad Request (400)"),
                    401 => write!(f, "Unauthorized (401)"),
                    403 => write!(f, "Forbidden (403)"),
                    404 => write!(f, "Not Found (404)"),
                    500 => write!(f, "Internal Server Error (500)"),
                    _ => write!(f, "HTTP Error: {}", status),
                }
            }
            NetworkError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
        }
    }
}

impl std::error::Error for NetworkError {}

fn simulate_http_request(url: &str) -> Result<String, NetworkError> {
    // Simulate different failure scenarios based on URL
    if url.is_empty() || !url.starts_with("http") {
        return Err(NetworkError::InvalidUrl(url.to_string()));
    }
    
    if url.contains("timeout") {
        return Err(NetworkError::Timeout);
    }
    
    if url.contains("connection") {
        return Err(NetworkError::ConnectionFailed("DNS resolution failed".to_string()));
    }
    
    if url.contains("404") {
        return Err(NetworkError::HttpError(404));
    }
    
    if url.contains("500") {
        return Err(NetworkError::HttpError(500));
    }
    
    // Simulate success
    Ok(format!("Response from {}", url))
}
```

**Key Learning**: HTTP errors benefit from status-code-specific handling in Display implementation.

## 🔧 Exercise 3.3: Database Error Implementation

**Problem**: Create DatabaseError enum with retry guidance.

**Specific Solution:**
```rust
#[derive(Debug)]
enum DatabaseError {
    ConnectionFailed,
    UserNotFound(u32),
    QueryFailed(String),
    PermissionDenied,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionFailed => write!(f, "Failed to connect to database"),
            DatabaseError::UserNotFound(id) => write!(f, "User with ID {} not found", id),
            DatabaseError::QueryFailed(query) => write!(f, "Query failed: {}", query),
            DatabaseError::PermissionDenied => write!(f, "Permission denied for database operation"),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl DatabaseError {
    // Helper method to determine if error is retryable
    fn is_retryable(&self) -> bool {
        match self {
            DatabaseError::ConnectionFailed => true,  // Might succeed on retry
            DatabaseError::QueryFailed(_) => true,    // Might be temporary
            DatabaseError::UserNotFound(_) => false,  // Won't change on retry
            DatabaseError::PermissionDenied => false, // Won't change on retry
        }
    }
}

fn get_user_from_database(id: u32) -> Result<String, DatabaseError> {
    // Simulate database lookup with different failure scenarios
    match id {
        1 => Ok("Alice".to_string()),
        2 => Ok("Bob".to_string()),
        999 => Err(DatabaseError::UserNotFound(id)),
        0 => Err(DatabaseError::QueryFailed("Invalid user ID".to_string())),
        100 => Err(DatabaseError::ConnectionFailed),
        200 => Err(DatabaseError::PermissionDenied),
        _ => Ok(format!("User{}", id)),
    }
}

fn handle_database_error(error: &DatabaseError) {
    match error {
        DatabaseError::ConnectionFailed => {
            println!("💡 Suggestion: Check database connection and retry");
        }
        DatabaseError::UserNotFound(id) => {
            println!("💡 Suggestion: Verify user ID {} exists or create user", id);
        }
        DatabaseError::QueryFailed(query) => {
            println!("💡 Suggestion: Check query syntax: {}", query);
        }
        DatabaseError::PermissionDenied => {
            println!("💡 Suggestion: Contact administrator for database permissions");
        }
    }
    
    if error.is_retryable() {
        println!("🔄 This error might succeed if retried");
    } else {
        println!("❌ This error will not succeed on retry");
    }
}
```

**Key Learning**: Add helper methods to error types for common operations like retry logic.

## 🔧 Exercise 3.4: Application-Level Error Aggregation

**Problem**: Create AppError that can represent any error in the application.

**Specific Solution:**
```rust
#[derive(Debug)]
enum AppError {
    Validation(ValidationError),
    Network(NetworkError),
    Database(DatabaseError),
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Validation(e) => write!(f, "Validation error: {}", e),
            AppError::Network(e) => write!(f, "Network error: {}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Validation(e) => Some(e),
            AppError::Network(e) => Some(e),
            AppError::Database(e) => Some(e),
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
        }
    }
}

// Automatic conversion implementations
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

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err)
    }
}
```

**Key Learning**: `From` trait implementations enable seamless error conversion with `?` operator.

## 🔧 Exercise 3.5: File Processing with Multiple Error Types

**Problem**: Implement file processing function using the unified AppError.

**Specific Solution:**
```rust
fn process_file(filename: &str) -> Result<String, AppError> {
    // Different filenames trigger different error scenarios
    match filename {
        "read_config.json" => {
            // Simulate successful file processing
            Ok("Configuration loaded successfully".to_string())
        }
        "process_data.csv" => {
            // Simulate validation error
            Err(AppError::Validation(ValidationError::InvalidEmail("bad@email".to_string())))
        }
        "nonexistent_file.txt" => {
            // Simulate I/O error (will be auto-converted)
            std::fs::read_to_string(filename)?;  // This will fail
            Ok("Should not reach here".to_string())
        }
        "corrupted_data.json" => {
            // Simulate parse error (will be auto-converted)
            let bad_number = "not_a_number".parse::<i32>()?;  // This will fail
            Ok(format!("Parsed: {}", bad_number))
        }
        _ => {
            // Simulate network error
            Err(AppError::Network(NetworkError::HttpError(404)))
        }
    }
}

fn print_error_chain(error: &dyn std::error::Error) {
    println!("Error: {}", error);
    
    let mut source = error.source();
    let mut level = 1;
    
    while let Some(err) = source {
        println!("{}└─ Caused by: {}", "  ".repeat(level), err);
        source = err.source();
        level += 1;
    }
}
```

**Key Learning**: Use `?` operator with `From` conversions for seamless error propagation.

## 🔧 Exercise 3.6: Error Context and Chaining

**Problem**: Add helpful context to errors for better debugging.

**Specific Solution:**
```rust
fn parse_and_validate(input: &str) -> Result<u32, AppError> {
    // Parse string to number (auto-converts ParseIntError to AppError)
    let number = input.parse::<u32>()?;
    
    // Validate the parsed number
    if number == 0 {
        return Err(AppError::Validation(ValidationError::InvalidAge(number)));
    }
    
    if number > 150 {
        return Err(AppError::Validation(ValidationError::TooOld));
    }
    
    Ok(number)
}

fn complex_operation() -> Result<String, AppError> {
    // Chain multiple operations that can each fail with different error types
    let user = validate_user("Alice", "alice@example.com", 25)?;  // Validation error
    let user_data = get_user_from_database(1)?;                   // Database error  
    let response = simulate_http_request("https://api.example.com/users")?;  // Network error
    
    Ok(format!("Success: User {}, Data: {}, Response: {}", 
               user.name, user_data, response))
}
```

**Key Learning**: Complex operations benefit from automatic error conversion chains.

## 💡 Error Design Patterns

### Pattern 1: Specific Error Data
```rust
#[derive(Debug)]
enum ValidationError {
    InvalidEmail { 
        email: String, 
        reason: String 
    },
    InvalidAge { 
        age: u32, 
        min: u32, 
        max: u32 
    },
}
```

### Pattern 2: Error Categories with Methods
```rust
impl NetworkError {
    fn is_temporary(&self) -> bool {
        matches!(self, NetworkError::Timeout | NetworkError::ConnectionFailed(_))
    }
    
    fn should_retry(&self) -> bool {
        self.is_temporary()
    }
    
    fn retry_delay(&self) -> std::time::Duration {
        match self {
            NetworkError::Timeout => std::time::Duration::from_secs(1),
            NetworkError::ConnectionFailed(_) => std::time::Duration::from_secs(5),
            _ => std::time::Duration::from_secs(0),
        }
    }
}
```

### Pattern 3: Error Context Building
```rust
#[derive(Debug)]
struct ErrorContext {
    operation: String,
    location: String,
    timestamp: std::time::SystemTime,
}

impl AppError {
    fn with_context(self, operation: &str, location: &str) -> ContextualError {
        ContextualError {
            error: self,
            context: ErrorContext {
                operation: operation.to_string(),
                location: location.to_string(),
                timestamp: std::time::SystemTime::now(),
            },
        }
    }
}
```

### Pattern 4: Error Recovery Strategies
```rust
fn robust_operation(input: &str) -> Result<String, AppError> {
    // Try primary method
    match primary_processing(input) {
        Ok(result) => return Ok(result),
        Err(AppError::Network(NetworkError::Timeout)) => {
            // Retry for timeout
            std::thread::sleep(std::time::Duration::from_millis(100));
            return retry_processing(input);
        }
        Err(AppError::Database(DatabaseError::ConnectionFailed)) => {
            // Use cache as fallback
            return fallback_to_cache(input);
        }
        Err(e) => return Err(e),  // Propagate non-recoverable errors
    }
}
```

## 🔄 Error Handling Best Practices

### Display vs Debug
```rust
// Display: User-friendly messages
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::NotFound => write!(f, "The requested item was not found"),
            MyError::PermissionDenied => write!(f, "You don't have permission for this operation"),
        }
    }
}

// Debug: Technical details (derived automatically)
#[derive(Debug)]
enum MyError {
    NotFound,
    PermissionDenied,
}
```

### Error Source Chains
```rust
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),      // Chain to I/O error
            AppError::Parse(e) => Some(e),   // Chain to parse error
            _ => None,                       // No source for domain errors
        }
    }
}
```

### Conversion Convenience
```rust
// Enable ? operator usage
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

// Now you can use:
fn read_file(path: &str) -> Result<String, AppError> {
    let content = std::fs::read_to_string(path)?;  // Auto-converts I/O error
    Ok(content)
}
```

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex03-level3.md) for full solution code.

## 🎓 Understanding Check

You should now understand:
1. **Error enum design**: Variants for different failure modes with relevant data
2. **Trait implementations**: Display for users, Error for proper error behavior
3. **Error conversion**: From traits for automatic ? operator usage
4. **Error chaining**: source() method for preserving error context
5. **Helper methods**: Adding utility methods to error types
6. **Application errors**: Unified error types that wrap domain-specific errors

Ready to design production-quality error handling systems! 🦀