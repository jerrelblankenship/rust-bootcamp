# Exercise 4 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Advanced Error Patterns

You've tried Level 1 hints but need concrete implementation guidance. Here are specific solutions for building flexible, production-ready error handling systems.

## 🔧 Exercise 4.1: Unified Error Implementation

**Problem**: Create UnifiedError that can wrap any error type with additional context.

**Specific Solution:**
```rust
#[derive(Debug)]
struct UnifiedError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl std::fmt::Display for UnifiedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UnifiedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl UnifiedError {
    fn new(message: String) -> Self {
        UnifiedError {
            message,
            source: None,
        }
    }
    
    fn with_source<E: std::error::Error + Send + Sync + 'static>(message: String, source: E) -> Self {
        UnifiedError {
            message,
            source: Some(Box::new(source)),
        }
    }
}

// Automatic conversions from common error types
impl From<ParseIntError> for UnifiedError {
    fn from(err: ParseIntError) -> Self {
        UnifiedError::with_source("Failed to parse integer".to_string(), err)
    }
}

impl From<std::io::Error> for UnifiedError {
    fn from(err: std::io::Error) -> Self {
        UnifiedError::with_source("I/O operation failed".to_string(), err)
    }
}

impl From<serde_json::Error> for UnifiedError {
    fn from(err: serde_json::Error) -> Self {
        UnifiedError::with_source("JSON processing failed".to_string(), err)
    }
}

// Implement operation that uses unified error handling
fn perform_operation(operation: &str) -> Result<String, UnifiedError> {
    let parts: Vec<&str> = operation.split(':').collect();
    
    match parts.get(0) {
        Some(&"parse_int") => {
            let value = parts.get(1).ok_or_else(|| {
                UnifiedError::new("Missing value for parse_int operation".to_string())
            })?;
            
            let number = value.parse::<i32>()?; // Auto-converts ParseIntError
            Ok(format!("Parsed integer: {}", number))
        }
        Some(&"read_file") => {
            let path = parts.get(1).ok_or_else(|| {
                UnifiedError::new("Missing path for read_file operation".to_string())
            })?;
            
            let content = std::fs::read_to_string(path)?; // Auto-converts io::Error
            Ok(format!("File content: {} bytes", content.len()))
        }
        Some(&"divide") => {
            let a = parts.get(1).and_then(|s| s.parse::<f64>().ok()).ok_or_else(|| {
                UnifiedError::new("Invalid first number for divide operation".to_string())
            })?;
            
            let b = parts.get(2).and_then(|s| s.parse::<f64>().ok()).ok_or_else(|| {
                UnifiedError::new("Invalid second number for divide operation".to_string())
            })?;
            
            if b == 0.0 {
                return Err(UnifiedError::new("Division by zero".to_string()));
            }
            
            Ok(format!("Division result: {}", a / b))
        }
        _ => Err(UnifiedError::new(format!("Unknown operation: {}", operation))),
    }
}
```

**Key Learning**: UnifiedError can wrap any error while adding meaningful context.

## 🔧 Exercise 4.2: Flexible Error Handling

**Problem**: Implement anyhow-style error handling for maximum flexibility.

**Specific Solution:**
```rust
// Type alias for maximum flexibility
type FlexibleResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// Configuration processing with flexible error handling
fn load_and_process_config(filename: &str) -> FlexibleResult<String> {
    // Different filenames trigger different error scenarios
    match filename {
        "valid_config.json" => {
            // Simulate successful JSON processing
            let config = serde_json::json!({
                "name": "test_app",
                "version": "1.0.0",
                "features": ["logging", "metrics"]
            });
            Ok(format!("Loaded config: {}", config))
        }
        "invalid_json.json" => {
            // Simulate JSON parsing error (auto-boxed)
            let invalid_json = "{invalid json content";
            serde_json::from_str::<serde_json::Value>(invalid_json)?;
            Ok("Should not reach here".to_string())
        }
        "missing_field.json" => {
            // Simulate custom validation error
            Err("Required field 'name' is missing from configuration".into())
        }
        "wrong_type.json" => {
            // Simulate type validation error
            Err("Field 'port' must be a number, found string".into())
        }
        _ => {
            // Simulate file not found error (auto-boxed)
            std::fs::read_to_string(filename)?;
            Ok("File read successfully".to_string())
        }
    }
}

// Helper function to print detailed error context
fn print_error_context(error: &(dyn std::error::Error + Send + Sync)) {
    println!("Error: {}", error);
    
    let mut source = error.source();
    let mut level = 1;
    
    while let Some(err) = source {
        println!("{}Caused by: {}", "  ".repeat(level), err);
        source = err.source();
        level += 1;
        
        // Prevent infinite loops
        if level > 10 {
            println!("{}... (error chain truncated)", "  ".repeat(level));
            break;
        }
    }
}
```

**Key Learning**: `Box<dyn Error>` provides maximum flexibility for diverse error types.

## 🔧 Exercise 4.3: Error Context Implementation

**Problem**: Build error context that accumulates as errors bubble up.

**Specific Solution:**
```rust
#[derive(Debug)]
struct ContextError {
    message: String,
    context: Vec<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        
        if !self.context.is_empty() {
            write!(f, "\nContext trail:")?;
            for (i, ctx) in self.context.iter().enumerate() {
                write!(f, "\n  {}. {}", i + 1, ctx)?;
            }
        }
        
        Ok(())
    }
}

impl std::error::Error for ContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl ContextError {
    fn new(message: String) -> Self {
        ContextError {
            message,
            context: Vec::new(),
            source: None,
        }
    }
    
    fn add_context(mut self, context: String) -> Self {
        self.context.push(context);
        self
    }
    
    fn wrap<E: std::error::Error + Send + Sync + 'static>(source: E, message: String) -> Self {
        ContextError {
            message,
            context: Vec::new(),
            source: Some(Box::new(source)),
        }
    }
}

// User operation processing with rich context
fn process_user_operation(user: &str, operation: &str) -> Result<String, ContextError> {
    match user {
        "alice" => {
            match operation {
                "process" => Ok("Alice's data processed successfully".to_string()),
                "validate" => {
                    // Simulate validation error with context
                    Err(ContextError::new("Validation failed: invalid email format".to_string())
                        .add_context(format!("validating user: {}", user))
                        .add_context(format!("operation: {}", operation)))
                }
                "save" => Ok("Alice's data saved successfully".to_string()),
                _ => Err(ContextError::new(format!("Unknown operation: {}", operation))
                    .add_context(format!("processing user: {}", user))),
            }
        }
        "bob" => {
            match operation {
                "process" => {
                    // Simulate nested error with source
                    let parse_error = "invalid_number".parse::<i32>().unwrap_err();
                    Err(ContextError::wrap(parse_error, "Failed to process Bob's data".to_string())
                        .add_context(format!("parsing configuration for user: {}", user))
                        .add_context(format!("operation: {}", operation)))
                }
                _ => Ok(format!("Bob's {} operation completed", operation)),
            }
        }
        "charlie" => {
            // Simulate operation-specific error
            Err(ContextError::new(format!("Operation '{}' not permitted for user", operation))
                .add_context(format!("user: {} has restricted permissions", user))
                .add_context("checking user authorization".to_string()))
        }
        _ => {
            Err(ContextError::new(format!("Unknown user: {}", user))
                .add_context("looking up user in system".to_string())
                .add_context(format!("requested operation: {}", operation)))
        }
    }
}

fn print_full_error_context(error: &ContextError) {
    println!("🚨 Operation Failed");
    println!("├─ Error: {}", error.message);
    
    if !error.context.is_empty() {
        println!("├─ Context:");
        for (i, ctx) in error.context.iter().enumerate() {
            let prefix = if i == error.context.len() - 1 { "└─" } else { "├─" };
            println!("│  {} {}", prefix, ctx);
        }
    }
    
    if let Some(source) = error.source() {
        println!("└─ Root cause: {}", source);
        
        // Print additional source chain
        let mut current_source = source.source();
        let mut depth = 1;
        while let Some(src) = current_source {
            println!("{}└─ {}", "  ".repeat(depth), src);
            current_source = src.source();
            depth += 1;
        }
    }
}
```

**Key Learning**: Context errors build debugging breadcrumbs as errors propagate.

## 🔧 Exercise 4.4: Retry Logic Implementation

**Problem**: Implement retry logic that analyzes errors and makes intelligent retry decisions.

**Specific Solution:**
```rust
#[derive(Debug)]
struct RetryError {
    operation: String,
    attempts: u32,
    errors: Vec<String>,
}

impl std::fmt::Display for RetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Operation '{}' failed after {} attempts. Errors: [{}]",
            self.operation,
            self.attempts,
            self.errors.join(", ")
        )
    }
}

impl std::error::Error for RetryError {}

fn retry_operation(operation: &str) -> Result<String, RetryError> {
    let max_attempts = 3;
    let mut errors = Vec::new();
    
    for attempt in 1..=max_attempts {
        match simulate_unreliable_operation(operation) {
            Ok(result) => {
                if attempt > 1 {
                    println!("✅ Operation succeeded on attempt {}", attempt);
                }
                return Ok(result);
            }
            Err(e) => {
                let error_msg = e.to_string();
                errors.push(format!("Attempt {}: {}", attempt, error_msg));
                
                if attempt < max_attempts {
                    let delay = std::time::Duration::from_millis(100 * attempt as u64);
                    println!("🔄 Attempt {} failed: {}. Retrying in {:?}...", attempt, error_msg, delay);
                    std::thread::sleep(delay);
                }
            }
        }
    }
    
    Err(RetryError {
        operation: operation.to_string(),
        attempts: max_attempts,
        errors,
    })
}

fn analyze_retry_failure(error: &RetryError) {
    println!("🔍 Retry Analysis:");
    println!("├─ Operation: {}", error.operation);
    println!("├─ Total attempts: {}", error.attempts);
    println!("├─ Error pattern analysis:");
    
    let mut error_types = std::collections::HashMap::new();
    for error in &error.errors {
        let error_type = if error.contains("timeout") {
            "timeout"
        } else if error.contains("connection") {
            "connection"
        } else if error.contains("busy") {
            "resource_busy"
        } else {
            "other"
        };
        *error_types.entry(error_type).or_insert(0) += 1;
    }
    
    for (error_type, count) in error_types {
        println!("│  ├─ {}: {} occurrences", error_type, count);
    }
    
    // Provide recommendations
    println!("└─ Recommendations:");
    if error.operation.contains("always_fails") {
        println!("   └─ This operation is designed to fail - check implementation");
    } else if error.errors.iter().any(|e| e.contains("timeout")) {
        println!("   └─ Consider increasing timeout values or checking network conditions");
    } else if error.errors.iter().any(|e| e.contains("connection")) {
        println!("   └─ Check service availability and network connectivity");
    } else {
        println!("   └─ Consider exponential backoff or circuit breaker pattern");
    }
}
```

**Key Learning**: Intelligent retry logic analyzes error patterns to make better decisions.

## 🔧 Exercise 4.5: Fallback Chain Implementation

**Problem**: Create resilient data fetching with multiple fallback sources.

**Specific Solution:**
```rust
#[derive(Debug)]
struct FallbackError {
    sources_tried: Vec<String>,
    errors: Vec<String>,
}

impl std::fmt::Display for FallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "All fallback sources failed. Tried: [{}]. Errors: [{}]",
            self.sources_tried.join(", "),
            self.errors.join("; ")
        )
    }
}

impl std::error::Error for FallbackError {}

fn fetch_with_fallback(sources: &[&str]) -> Result<String, FallbackError> {
    let mut tried_sources = Vec::new();
    let mut errors = Vec::new();
    
    for source in sources {
        tried_sources.push(source.to_string());
        
        match simulate_data_source(source) {
            Ok(data) => {
                if tried_sources.len() > 1 {
                    println!("✅ Fallback successful: {} provided data", source);
                }
                return Ok(data);
            }
            Err(e) => {
                errors.push(format!("{}: {}", source, e));
                println!("❌ Source '{}' failed: {}", source, e);
            }
        }
    }
    
    Err(FallbackError {
        sources_tried: tried_sources,
        errors,
    })
}

fn log_fallback_failure(sources: &[&str], error: &FallbackError) {
    println!("📊 Fallback Failure Analysis:");
    println!("├─ Sources attempted: {}", sources.len());
    println!("├─ Failure details:");
    
    for (i, (source, error)) in sources.iter().zip(error.errors.iter()).enumerate() {
        let prefix = if i == sources.len() - 1 { "└─" } else { "├─" };
        println!("│  {} {}: {}", prefix, source, error);
    }
    
    println!("└─ Recommendations:");
    
    // Analyze error patterns for recommendations
    let connection_failures = error.errors.iter().filter(|e| e.contains("connection")).count();
    let unavailable_services = error.errors.iter().filter(|e| e.contains("unavailable")).count();
    
    if connection_failures > 0 {
        println!("   ├─ {} connection failures detected - check network connectivity", connection_failures);
    }
    if unavailable_services > 0 {
        println!("   ├─ {} services unavailable - check service health", unavailable_services);
    }
    if error.sources_tried.contains(&"cache".to_string()) {
        println!("   ├─ Consider pre-populating cache with default data");
    }
    println!("   └─ Consider implementing circuit breaker pattern for failing services");
}

// Helper function to simulate unreliable operations
fn simulate_unreliable_operation(operation: &str) -> Result<String, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let random = (now.as_nanos() % 100) as u32;
    
    match operation {
        op if op.contains("sometimes_works") => {
            if random < 30 {  // 30% success rate
                Ok("Operation completed successfully".to_string())
            } else {
                Err("Temporary failure: service overloaded".to_string())
            }
        }
        op if op.contains("connection_flaky") => {
            if random < 25 {  // 25% success rate
                Ok("Database connection established".to_string())
            } else {
                Err("Connection timeout: database unreachable".to_string())
            }
        }
        op if op.contains("might_be_busy") => {
            if random < 40 {  // 40% success rate
                Ok("Resource lock acquired".to_string())
            } else {
                Err("Resource busy: try again later".to_string())
            }
        }
        op if op.contains("always_fails") => {
            Err("Permanent failure: service deprecated".to_string())
        }
        _ => Ok("Default operation successful".to_string()),
    }
}

fn simulate_data_source(source: &str) -> Result<String, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let random = (now.as_nanos() % 100) as u32;
    
    match source {
        "primary_db" => {
            if random < 20 {  // 20% failure rate
                Err("Connection refused".to_string())
            } else {
                Ok("Primary database data: [user_records: 1000]".to_string())
            }
        }
        "backup_db" => {
            if random < 10 {  // 10% failure rate
                Err("Backup database unavailable".to_string())
            } else {
                Ok("Backup database data: [user_records: 950]".to_string())
            }
        }
        "cache" => {
            if random < 5 {  // 5% failure rate
                Err("Cache miss".to_string())
            } else {
                Ok("Cached data: [user_records: 800, last_updated: 2h ago]".to_string())
            }
        }
        "secondary_source" => {
            Ok("Secondary source data: [limited_records: 500]".to_string())
        }
        _ => Err("Unknown data source".to_string()),
    }
}
```

**Key Learning**: Fallback chains provide system resilience by trying alternatives systematically.

## 💡 Advanced Error Handling Patterns

### Error Conversion Utilities
```rust
// Helper trait for adding context to any Result
trait ResultExt<T, E> {
    fn with_context<F>(self, f: F) -> Result<T, ContextError>
    where
        F: FnOnce() -> String,
        E: std::error::Error + Send + Sync + 'static;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn with_context<F>(self, f: F) -> Result<T, ContextError>
    where
        F: FnOnce() -> String,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.map_err(|e| ContextError::wrap(e, f()))
    }
}

// Usage:
fn example() -> Result<String, ContextError> {
    std::fs::read_to_string("config.json")
        .with_context(|| "Failed to load application configuration".to_string())
}
```

### Error Aggregation
```rust
#[derive(Debug)]
struct BatchError {
    successes: u32,
    failures: Vec<(usize, String)>,
}

impl BatchError {
    fn from_results<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, BatchError>
    where
        E: std::fmt::Display,
    {
        let mut successes = Vec::new();
        let mut failures = Vec::new();
        
        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(value) => successes.push(value),
                Err(e) => failures.push((i, e.to_string())),
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

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex04-level3.md) for full solution code.

## 🎓 Understanding Check

You should now understand:
1. **Unified error types**: Single error type that can represent any failure
2. **Flexible error handling**: Box<dyn Error> for maximum adaptability
3. **Error context building**: Adding debugging breadcrumbs to errors
4. **Retry logic**: Intelligent error analysis and retry strategies
5. **Fallback systems**: Resilient data fetching with multiple sources
6. **Error utilities**: Helper functions and traits for common patterns

Ready to build production-ready error handling systems! 🦀