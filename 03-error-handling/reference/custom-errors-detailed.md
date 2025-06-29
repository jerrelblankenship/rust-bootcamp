# Lesson 03: Custom Error Types

Learn to create robust, domain-specific error types that make your APIs clear and your error handling precise. This is where Rust's type system truly shines for building maintainable applications.

## 🎯 Learning Objectives

- Design custom error types that guide users to solutions
- Use the `thiserror` crate for cleaner error definitions
- Implement error hierarchies and error chains for complex applications
- Create actionable error types with specific context
- Build error types that integrate seamlessly with the ? operator
- Compare custom error design with C# exception hierarchies

## 📚 Introduction

Generic error messages like "something went wrong" or "invalid input" are frustrating for users and developers alike. Custom error types let you create precise, actionable errors that guide users to solutions and make debugging straightforward.

In this lesson, you'll learn to design error types that are both user-friendly and developer-friendly.

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
    Ok(User { email: data.to_string() })
}

#[derive(Debug)]
struct User {
    email: String,
}

// Caller can't distinguish between different error types
fn main() {
    match process_user_data("invalid-data") {
        Ok(user) => println!("Success: {:?}", user),
        Err(e) => {
            // What kind of error? How should we respond?
            eprintln!("Something went wrong: {}", e);
            // Can't provide specific help to the user!
        }
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

impl std::fmt::Display for UserProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserProcessingError::EmptyData => {
                write!(f, "No user data provided")
            }
            UserProcessingError::InvalidEmail { provided } => {
                write!(f, "Invalid email '{}'. Expected format: user@domain.com", provided)
            }
            UserProcessingError::MissingRequiredField { field } => {
                write!(f, "Missing required field: {}", field)
            }
            UserProcessingError::InvalidAge { provided, min, max } => {
                write!(f, "Age {} is invalid. Must be between {} and {}", provided, min, max)
            }
        }
    }
}

impl std::error::Error for UserProcessingError {}

fn process_user_data_safe(email: &str, age: i32) -> Result<User, UserProcessingError> {
    if email.is_empty() {
        return Err(UserProcessingError::EmptyData);
    }
    
    if !email.contains('@') {
        return Err(UserProcessingError::InvalidEmail {
            provided: email.to_string(),
        });
    }
    
    if age < 13 || age > 120 {
        return Err(UserProcessingError::InvalidAge {
            provided: age,
            min: 13,
            max: 120,
        });
    }
    
    Ok(User { 
        email: email.to_string(),
        age,
    })
}

#[derive(Debug)]
struct User {
    email: String,
    age: i32,
}

// Now caller can handle each error type appropriately
fn main() {
    match process_user_data_safe("invalid-email", 150) {
        Ok(user) => println!("Success: {:?}", user),
        Err(UserProcessingError::EmptyData) => {
            eprintln!("Please provide user data");
        }
        Err(UserProcessingError::InvalidEmail { provided }) => {
            eprintln!("Fix your email: '{}' -> user@domain.com", provided);
        }
        Err(UserProcessingError::InvalidAge { provided, min, max }) => {
            eprintln!("Age {} is invalid. Please enter age between {} and {}", 
                     provided, min, max);
        }
        Err(UserProcessingError::MissingRequiredField { field }) => {
            eprintln!("Please provide the missing field: {}", field);
        }
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

#[derive(serde::Deserialize, Debug)]
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
    
    #[error("Insufficient balance: need ${required:.2}, have ${available:.2}")]
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
    // Each operation can fail with different error types
    let config = load_config("app.toml")?;              // ConfigError -> ApplicationError
    let user = authenticate_user(user_id, &config)?;    // AuthError -> ApplicationError
    let order = load_order(order_id)?;                  // DatabaseError -> ApplicationError
    validate_order(&order, &user)?;                     // BusinessError -> ApplicationError
    let payment = process_payment(&order)?;             // Various errors -> ApplicationError
    
    Ok(OrderResult {
        order_id: order_id.to_string(),
        amount: payment.amount,
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
            eprintln!("Please add ${:.2} to your account and try again.", required - available);
        }
        Err(e) => {
            eprintln!("Order processing failed: {}", e);
            
            // Print the full error chain for debugging
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {}", err);
                source = err.source();
            }
        }
    }
}

// Supporting types and functions for the example
#[derive(Debug)]
struct AppConfig {
    auth_token: String,
}

#[derive(Debug)]
struct User {
    id: u32,
    balance: f64,
}

#[derive(Debug)]
struct Order {
    id: String,
    amount: f64,
    user_id: u32,
}

#[derive(Debug)]
struct OrderResult {
    order_id: String,
    amount: f64,
    status: String,
}

struct PaymentResult {
    amount: f64,
}

fn load_config(path: &str) -> Result<AppConfig, ConfigError> {
    if path.is_empty() {
        return Err(ConfigError::FileNotFound { path: path.to_string() });
    }
    Ok(AppConfig { auth_token: "secret".to_string() })
}

fn authenticate_user(user_id: u32, _config: &AppConfig) -> Result<User, AuthError> {
    if user_id == 0 {
        return Err(AuthError::InvalidCredentials {
            username: format!("user-{}", user_id),
        });
    }
    Ok(User { id: user_id, balance: 100.0 })
}

fn load_order(order_id: &str) -> Result<Order, DatabaseError> {
    if order_id.is_empty() {
        return Err(DatabaseError::QueryFailed {
            query: "SELECT * FROM orders WHERE id = ?".to_string(),
            reason: "Invalid order ID".to_string(),
        });
    }
    Ok(Order { id: order_id.to_string(), amount: 25.99, user_id: 456 })
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
    Ok(PaymentResult { amount: order.amount })
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
        source: Box<dyn std::error::Error + Send + Sync>,
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
    
    #[error("TLS handshake failed: {reason}")]
    TlsError { reason: String },
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

## 🔄 Comparison with C#

| C# Exception Design | Rust Error Design | Key Difference |
|--------------------|-------------------|----------------|
| Exception inheritance | Error enum variants | Composition over inheritance |
| Runtime type checking | Compile-time matching | Safety at compile time |
| `InnerException` | `#[source]` attribute | Explicit error chaining |
| `Message` property | `Display` implementation | Same concept, different syntax |
| Stack traces | Error source chains | Manual context building |
| Uncaught exceptions | Must handle all cases | Compiler enforces handling |

## 💻 Practice Exercises

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
    // Use your custom error types for specific failure cases
    todo!()
}

struct ProcessedFile {
    path: String,
    size: u64,
    content_type: String,
}

fn main() {
    match process_file("test.json") {
        Ok(file) => println!("Processed: {:?}", file),
        Err(e) => {
            eprintln!("Processing failed: {}", e);
            
            // TODO: Provide specific guidance based on error type
            match e {
                // Handle each error variant with specific advice
                _ => {}
            }
        }
    }
}
```

### Exercise 2: Create an Error Hierarchy

```rust
use thiserror::Error;

// TODO: Create a three-level error hierarchy:
// 1. Low-level: NetworkError, DatabaseError, FileSystemError
// 2. Mid-level: ServiceError (wraps low-level errors)
// 3. High-level: ApplicationError (wraps service errors)

// Each level should add appropriate context and abstraction

#[derive(Error, Debug)]
pub enum NetworkError {
    // TODO: Define network-specific errors
    // - Connection timeout
    // - DNS resolution failed
    // - TLS errors
}

#[derive(Error, Debug)]
pub enum ServiceError {
    // TODO: Define service-level errors that wrap NetworkError
    // - API unavailable
    // - Authentication failed
    // - Rate limited
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    // TODO: Define application-level errors
    // - User errors
    // - Configuration errors
    // - Service errors
}

fn perform_complex_operation() -> Result<String, ApplicationError> {
    // TODO: Implement an operation that can fail at multiple levels
    // Show how errors bubble up through the hierarchy
    todo!()
}

fn main() {
    match perform_complex_operation() {
        Ok(result) => println!("Success: {}", result),
        Err(e) => {
            eprintln!("Operation failed: {}", e);
            
            // TODO: Handle different error levels appropriately
            // Show specific user guidance for each error type
        }
    }
}
```

### Exercise 3: Error Recovery Strategies

```rust
use thiserror::Error;

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

fn retry_operation<F, T, E>(
    mut operation: F,
    max_attempts: u32,
) -> Result<T, RetryableError>
where
    F: FnMut() -> Result<T, E>,
    E: Into<RetryableError>,
{
    // TODO: Implement retry logic with exponential backoff
    // Handle different error types appropriately:
    // - Retry temporary errors
    // - Don't retry permanent errors
    // - Wait for rate limit to expire
    todo!()
}

// Example operation that can fail
fn flaky_network_call() -> Result<String, RetryableError> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    match now % 5 {
        0 => Ok("Success!".to_string()),
        1 => Err(RetryableError::Temporary {
            message: "Network timeout".to_string(),
            attempt: 1,
            max_attempts: 3,
        }),
        2 => Err(RetryableError::RateLimited { seconds: 5 }),
        _ => Err(RetryableError::Permanent {
            message: "Invalid API key".to_string(),
        }),
    }
}

fn main() {
    match retry_operation(|| flaky_network_call(), 3) {
        Ok(result) => println!("Operation succeeded: {}", result),
        Err(e) => eprintln!("Operation failed permanently: {}", e),
    }
}
```

## 🚀 Mini-Project: E-commerce Validation System

Build a comprehensive validation system with custom error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderValidationError {
    #[error("Invalid customer: {0}")]
    Customer(#[from] CustomerError),
    
    #[error("Invalid product: {0}")]
    Product(#[from] ProductError),
    
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    
    #[error("Shipping error: {0}")]
    Shipping(#[from] ShippingError),
}

#[derive(Error, Debug)]
pub enum CustomerError {
    #[error("Customer not found: {customer_id}")]
    NotFound { customer_id: String },
    
    #[error("Customer account suspended: {customer_id}")]
    Suspended { customer_id: String },
    
    #[error("Invalid email format: {email}")]
    InvalidEmail { email: String },
}

#[derive(Error, Debug)]
pub enum ProductError {
    #[error("Product not found: {product_id}")]
    NotFound { product_id: String },
    
    #[error("Product out of stock: {product_id} (requested: {requested}, available: {available})")]
    OutOfStock { product_id: String, requested: u32, available: u32 },
    
    #[error("Product discontinued: {product_id}")]
    Discontinued { product_id: String },
}

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Insufficient funds: ${required:.2} required, ${available:.2} available")]
    InsufficientFunds { required: f64, available: f64 },
    
    #[error("Invalid payment method: {method}")]
    InvalidMethod { method: String },
    
    #[error("Payment processing failed: {reason}")]
    ProcessingFailed { reason: String },
}

#[derive(Error, Debug)]
pub enum ShippingError {
    #[error("Invalid shipping address: {reason}")]
    InvalidAddress { reason: String },
    
    #[error("Shipping not available to: {location}")]
    UnavailableLocation { location: String },
    
    #[error("Package too large: {weight}kg (max: {max_weight}kg)")]
    OverWeight { weight: f64, max_weight: f64 },
}

#[derive(Debug)]
struct Order {
    customer_id: String,
    items: Vec<OrderItem>,
    payment_method: String,
    shipping_address: String,
}

#[derive(Debug)]
struct OrderItem {
    product_id: String,
    quantity: u32,
}

struct OrderValidator;

impl OrderValidator {
    fn validate_order(&self, order: &Order) -> Result<(), OrderValidationError> {
        self.validate_customer(&order.customer_id)?;
        
        for item in &order.items {
            self.validate_product(&item.product_id, item.quantity)?;
        }
        
        self.validate_payment(&order.payment_method)?;
        self.validate_shipping(&order.shipping_address)?;
        
        Ok(())
    }
    
    fn validate_customer(&self, customer_id: &str) -> Result<(), CustomerError> {
        if customer_id.is_empty() {
            return Err(CustomerError::InvalidEmail {
                email: "empty".to_string(),
            });
        }
        
        if customer_id == "suspended_user" {
            return Err(CustomerError::Suspended {
                customer_id: customer_id.to_string(),
            });
        }
        
        Ok(())
    }
    
    fn validate_product(&self, product_id: &str, quantity: u32) -> Result<(), ProductError> {
        if product_id == "discontinued" {
            return Err(ProductError::Discontinued {
                product_id: product_id.to_string(),
            });
        }
        
        if quantity > 5 {
            return Err(ProductError::OutOfStock {
                product_id: product_id.to_string(),
                requested: quantity,
                available: 5,
            });
        }
        
        Ok(())
    }
    
    fn validate_payment(&self, method: &str) -> Result<(), PaymentError> {
        if method == "invalid_card" {
            return Err(PaymentError::ProcessingFailed {
                reason: "Card expired".to_string(),
            });
        }
        
        Ok(())
    }
    
    fn validate_shipping(&self, address: &str) -> Result<(), ShippingError> {
        if address.is_empty() {
            return Err(ShippingError::InvalidAddress {
                reason: "Address cannot be empty".to_string(),
            });
        }
        
        Ok(())
    }
}

fn main() {
    let order = Order {
        customer_id: "suspended_user".to_string(),
        items: vec![
            OrderItem { product_id: "laptop".to_string(), quantity: 1 },
            OrderItem { product_id: "mouse".to_string(), quantity: 10 }, // Too many
        ],
        payment_method: "credit_card".to_string(),
        shipping_address: "123 Main St".to_string(),
    };
    
    let validator = OrderValidator;
    
    match validator.validate_order(&order) {
        Ok(()) => println!("Order is valid!"),
        Err(e) => {
            eprintln!("Order validation failed: {}", e);
            
            // Provide specific guidance based on error type
            match &e {
                OrderValidationError::Customer(CustomerError::Suspended { customer_id }) => {
                    eprintln!("Please contact support to reactivate account: {}", customer_id);
                }
                OrderValidationError::Product(ProductError::OutOfStock { product_id, available, .. }) => {
                    eprintln!("Please reduce quantity for {} to {} or less", product_id, available);
                }
                OrderValidationError::Payment(PaymentError::ProcessingFailed { reason }) => {
                    eprintln!("Payment issue: {}. Please try a different payment method", reason);
                }
                _ => {
                    eprintln!("Please fix the error and try again");
                }
            }
        }
    }
}
```

## 🔑 Key Takeaways

1. **Custom error types** make APIs clearer and error handling more precise
2. **thiserror crate** eliminates boilerplate for error type definitions
3. **Error hierarchies** provide appropriate abstraction levels
4. **Actionable errors** include context and guidance for resolution
5. **Pattern matching** enables specific handling for specific error types
6. **Error chaining** preserves context through multiple abstraction layers

## 📚 Additional Resources

- [thiserror crate documentation](https://docs.rs/thiserror/)
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [Rust API Guidelines - Error Design](https://rust-lang.github.io/api-guidelines/type-safety.html#error-types-are-meaningful-and-well-behaved-c-good-err)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Design custom error types for specific domains
- [ ] Use thiserror to reduce boilerplate code
- [ ] Create error hierarchies for complex applications
- [ ] Write actionable error messages with context
- [ ] Handle error chains and source errors
- [ ] Choose appropriate error abstraction levels

---

**Congratulations!** You've mastered Rust's error handling system. You now know how to build applications that handle errors gracefully and guide users to solutions.

Next Module: [04 - Systems Programming](../04-systems-programming/README.md) - Learn low-level Rust programming with the safety guarantees you've mastered →
