# Testing Framework Project - Level 3 Hints 🌳

## Complete Implementation Guide

### Complete Assertions Module
```rust
// src/assertions.rs - Complete implementation

/// Custom assertion error with detailed information
#[derive(Debug)]
pub struct AssertionError {
    pub message: String,
    pub context: Option<String>,
}

impl std::fmt::Display for AssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(context) = &self.context {
            write!(f, "\nContext: {}", context)?;
        }
        Ok(())
    }
}

impl std::error::Error for AssertionError {}

/// Assertion macro with better error messages
#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        if !$haystack.contains($needle) {
            panic!(
                "Assertion failed: string does not contain expected substring\n\
                 Haystack: {:?}\n\
                 Needle:   {:?}\n\
                 Location: {}:{}",
                $haystack,
                $needle,
                file!(),
                line!()
            );
        }
    };
    ($haystack:expr, $needle:expr, $msg:expr) => {
        if !$haystack.contains($needle) {
            panic!(
                "Assertion failed: {}\n\
                 Haystack: {:?}\n\
                 Needle:   {:?}\n\
                 Location: {}:{}",
                $msg,
                $haystack,
                $needle,
                file!(),
                line!()
            );
        }
    };
}

#[macro_export]
macro_rules! assert_between {
    ($value:expr, $min:expr, $max:expr) => {
        if !($min <= $value && $value <= $max) {
            panic!(
                "Assertion failed: value not within range\n\
                 Value: {:?}\n\
                 Range: {:?}..={:?}\n\
                 Location: {}:{}",
                $value,
                $min,
                $max,
                file!(),
                line!()
            );
        }
    };
}

/// JSON comparison with detailed diff
pub fn assert_json_eq(actual: &str, expected: &str) -> Result<(), AssertionError> {
    use serde_json::Value;
    
    let actual_value: Value = serde_json::from_str(actual)
        .map_err(|e| AssertionError {
            message: format!("Failed to parse actual JSON: {}", e),
            context: Some(actual.to_string()),
        })?;
    
    let expected_value: Value = serde_json::from_str(expected)
        .map_err(|e| AssertionError {
            message: format!("Failed to parse expected JSON: {}", e),
            context: Some(expected.to_string()),
        })?;
    
    if actual_value != expected_value {
        return Err(AssertionError {
            message: "JSON values do not match".to_string(),
            context: Some(format!(
                "Expected: {}\nActual: {}", 
                serde_json::to_string_pretty(&expected_value).unwrap(),
                serde_json::to_string_pretty(&actual_value).unwrap()
            )),
        });
    }
    
    Ok(())
}

/// File existence assertion
pub fn assert_file_exists(path: &str) -> Result<(), AssertionError> {
    if !std::path::Path::new(path).exists() {
        return Err(AssertionError {
            message: format!("File does not exist: {}", path),
            context: None,
        });
    }
    Ok(())
}

/// Collection comparison with detailed differences
pub fn assert_collection_equal<T: PartialEq + std::fmt::Debug>(
    actual: &[T], 
    expected: &[T]
) -> Result<(), AssertionError> {
    if actual.len() != expected.len() {
        return Err(AssertionError {
            message: format!(
                "Collection lengths differ: actual={}, expected={}", 
                actual.len(), 
                expected.len()
            ),
            context: Some(format!("Actual: {:?}\nExpected: {:?}", actual, expected)),
        });
    }
    
    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        if a != e {
            return Err(AssertionError {
                message: format!("Elements at index {} differ", i),
                context: Some(format!("Actual: {:?}\nExpected: {:?}", a, e)),
            });
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_assert_contains() {
        assert_contains!("hello world", "world");
        assert_contains!("test string", "str", "Custom message");
    }
    
    #[test]
    #[should_panic(expected = "does not contain")]
    fn test_assert_contains_failure() {
        assert_contains!("hello world", "foo");
    }
    
    #[test]
    fn test_json_assertions() {
        let json1 = r#"{"name": "John", "age": 30}"#;
        let json2 = r#"{"age": 30, "name": "John"}"#;
        
        assert!(assert_json_eq(json1, json2).is_ok());
    }
}
```

### Complete Builders Module
```rust
// src/builders.rs - Complete implementation

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Option<u64>,
    pub username: String,
    pub email: String,
    pub age: u32,
    pub is_active: bool,
    pub role: UserRole,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Regular,
    Guest,
}

#[derive(Debug, Clone)]
pub struct UserBuilder {
    id: Option<u64>,
    username: String,
    email: String,
    age: u32,
    is_active: bool,
    role: UserRole,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: None,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
            is_active: true,
            role: UserRole::Regular,
        }
    }
}

impl UserBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }
    
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }
    
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }
    
    pub fn with_age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }
    
    pub fn inactive(mut self) -> Self {
        self.is_active = false;
        self
    }
    
    pub fn admin(mut self) -> Self {
        self.role = UserRole::Admin;
        self
    }
    
    pub fn guest(mut self) -> Self {
        self.role = UserRole::Guest;
        self
    }
    
    pub fn build(self) -> User {
        User {
            id: self.id,
            username: self.username,
            email: self.email,
            age: self.age,
            is_active: self.is_active,
            role: self.role,
        }
    }
}

// Object Mother pattern for common scenarios
pub mod fixtures {
    use super::*;
    
    pub struct Users;
    
    impl Users {
        pub fn admin() -> User {
            UserBuilder::new()
                .with_username("admin")
                .with_email("admin@example.com")
                .admin()
                .build()
        }
        
        pub fn regular_user() -> User {
            UserBuilder::new().build()
        }
        
        pub fn guest() -> User {
            UserBuilder::new()
                .guest()
                .build()
        }
        
        pub fn inactive_user() -> User {
            UserBuilder::new()
                .inactive()
                .build()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_builder() {
        let user = UserBuilder::new()
            .with_username("alice")
            .with_age(30)
            .admin()
            .build();
        
        assert_eq!(user.username, "alice");
        assert_eq!(user.age, 30);
        assert!(matches!(user.role, UserRole::Admin));
    }
    
    #[test]
    fn test_fixtures() {
        let admin = fixtures::Users::admin();
        assert!(matches!(admin.role, UserRole::Admin));
        
        let guest = fixtures::Users::guest();
        assert!(matches!(guest.role, UserRole::Guest));
    }
}
```

### Complete Mocks Module
```rust
// src/mocks.rs - Complete implementation

use std::cell::RefCell;
use std::collections::HashMap;

/// Email service trait for mocking
pub trait EmailService: Send + Sync {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, String>;
    fn validate_email(&self, email: &str) -> bool;
}

/// Manual mock implementation
pub struct MockEmailService {
    calls: RefCell<Vec<EmailCall>>,
    expected_calls: RefCell<Vec<ExpectedCall>>,
    default_response: RefCell<Result<String, String>>,
}

#[derive(Debug, Clone)]
pub struct EmailCall {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug)]
struct ExpectedCall {
    to: String,
    subject: String,
    body: String,
    response: Result<String, String>,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            expected_calls: RefCell::new(Vec::new()),
            default_response: RefCell::new(Ok("default_id".to_string())),
        }
    }
    
    pub fn expect_send_email(&self, to: &str, subject: &str, body: &str) -> &Self {
        self.expected_calls.borrow_mut().push(ExpectedCall {
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
            response: Ok("mocked_id".to_string()),
        });
        self
    }
    
    pub fn returning(&self, response: Result<String, String>) -> &Self {
        if let Some(last_call) = self.expected_calls.borrow_mut().last_mut() {
            last_call.response = response;
        }
        self
    }
    
    pub fn verify(&self) -> bool {
        let calls = self.calls.borrow();
        let expected = self.expected_calls.borrow();
        
        if calls.len() != expected.len() {
            return false;
        }
        
        for (call, expected) in calls.iter().zip(expected.iter()) {
            if call.to != expected.to || 
               call.subject != expected.subject || 
               call.body != expected.body {
                return false;
            }
        }
        
        true
    }
    
    pub fn call_count(&self) -> usize {
        self.calls.borrow().len()
    }
    
    pub fn calls(&self) -> Vec<EmailCall> {
        self.calls.borrow().clone()
    }
}

impl EmailService for MockEmailService {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, String> {
        let call = EmailCall {
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        };
        
        self.calls.borrow_mut().push(call.clone());
        
        // Find matching expected call
        let expected_calls = self.expected_calls.borrow();
        for expected in expected_calls.iter() {
            if expected.to == call.to && 
               expected.subject == call.subject && 
               expected.body == call.body {
                return expected.response.clone();
            }
        }
        
        // Return default response
        self.default_response.borrow().clone()
    }
    
    fn validate_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mock_email_service() {
        let mock = MockEmailService::new();
        
        mock.expect_send_email("test@example.com", "Test", "Body")
            .returning(Ok("test_id".to_string()));
        
        let result = mock.send_email("test@example.com", "Test", "Body");
        assert_eq!(result, Ok("test_id".to_string()));
        
        assert!(mock.verify());
        assert_eq!(mock.call_count(), 1);
    }
}
```

### Complete Async Helpers
```rust
// src/async_helpers.rs - Complete implementation

use std::future::Future;
use std::time::Duration;

/// Timeout wrapper for async operations
pub async fn with_timeout<F, T>(
    duration: Duration,
    future: F,
) -> Result<T, TimeoutError>
where
    F: Future<Output = T>,
{
    match tokio::time::timeout(duration, future).await {
        Ok(result) => Ok(result),
        Err(_) => Err(TimeoutError {
            duration,
            operation: "async operation".to_string(),
        }),
    }
}

#[derive(Debug)]
pub struct TimeoutError {
    pub duration: Duration,
    pub operation: String,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} timed out after {:?}",
            self.operation, self.duration
        )
    }
}

impl std::error::Error for TimeoutError {}

/// Async assertion helpers
pub async fn assert_eventually<F, Fut>(
    condition: F,
    timeout: Duration,
    interval: Duration,
) -> Result<(), String>
where
    F: Fn() -> Fut,
    Fut: Future<Output = bool>,
{
    let start = tokio::time::Instant::now();
    
    while start.elapsed() < timeout {
        if condition().await {
            return Ok(());
        }
        tokio::time::sleep(interval).await;
    }
    
    Err(format!(
        "Condition was never true within {:?}",
        timeout
    ))
}

/// Mock for async operations
pub struct AsyncMockService {
    responses: std::cell::RefCell<Vec<Result<String, String>>>,
    delays: std::cell::RefCell<Vec<Duration>>,
    call_count: std::cell::RefCell<usize>,
}

impl AsyncMockService {
    pub fn new() -> Self {
        Self {
            responses: std::cell::RefCell::new(Vec::new()),
            delays: std::cell::RefCell::new(Vec::new()),
            call_count: std::cell::RefCell::new(0),
        }
    }
    
    pub fn expect_call(&self, response: Result<String, String>) -> &Self {
        self.responses.borrow_mut().push(response);
        self
    }
    
    pub fn with_delay(&self, delay: Duration) -> &Self {
        self.delays.borrow_mut().push(delay);
        self
    }
    
    pub async fn call(&self) -> Result<String, String> {
        let call_index = *self.call_count.borrow();
        *self.call_count.borrow_mut() += 1;
        
        // Apply delay if configured
        if let Some(delay) = self.delays.borrow().get(call_index) {
            tokio::time::sleep(*delay).await;
        }
        
        // Return configured response
        self.responses
            .borrow()
            .get(call_index)
            .cloned()
            .unwrap_or_else(|| Ok("default".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_timeout_wrapper() {
        let fast_operation = async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            "completed"
        };
        
        let result = with_timeout(Duration::from_millis(100), fast_operation).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_assert_eventually() {
        let mut counter = 0;
        let condition = || {
            counter += 1;
            async move { counter >= 3 }
        };
        
        let result = assert_eventually(
            condition,
            Duration::from_millis(100),
            Duration::from_millis(10),
        ).await;
        
        assert!(result.is_ok());
    }
}
```

### Complete lib.rs Integration
```rust
// src/lib.rs - Complete integration

pub mod assertions;
pub mod builders;
pub mod mocks;

#[cfg(feature = "async")]
pub mod async_helpers;

// Re-export main functionality
pub use assertions::*;
pub use builders::*;
pub use mocks::*;

#[cfg(feature = "async")]
pub use async_helpers::*;

// Framework metadata
pub const VERSION: &str = "0.1.0";
pub const FRAMEWORK_NAME: &str = "RustTest Framework";

/// Initialize the testing framework
pub fn init() {
    println!("🧪 {} v{} initialized", FRAMEWORK_NAME, VERSION);
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_framework_integration() {
        init();
        
        // Test builder + assertion integration
        let user = builders::UserBuilder::new()
            .with_username("integration_test")
            .build();
        
        assert_contains!(&user.username, "integration");
        
        // Test mock integration
        let mock = mocks::MockEmailService::new();
        mock.expect_send_email("test@example.com", "Test", "Body");
        
        let result = mock.send_email("test@example.com", "Test", "Body");
        assert!(result.is_ok());
        assert!(mock.verify());
    }
}
```

### Complete Cargo.toml
```toml
[package]
name = "testing-framework"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"], optional = true }

[dev-dependencies]
tokio-test = "0.4"

[features]
default = []
async = ["tokio"]

[[bin]]
name = "demo"
path = "src/main.rs"
```

### Complete Demo Application
```rust
// src/main.rs - Complete demo

use testing_framework::*;

fn main() {
    println!("🧪 Testing Framework Demo");
    println!("========================");
    
    testing_framework::init();
    
    demo_assertions();
    demo_builders();
    demo_mocks();
    
    println!("\n✅ All demos completed successfully!");
}

fn demo_assertions() {
    println!("\n📋 Custom Assertions Demo:");
    
    // String assertions
    assert_contains!("hello world", "world");
    println!("  ✅ String contains assertion");
    
    // Range assertions
    assert_between!(5, 1, 10);
    println!("  ✅ Numeric range assertion");
    
    // JSON assertions
    let json1 = r#"{"name": "John", "age": 30}"#;
    let json2 = r#"{"age": 30, "name": "John"}"#;
    assert!(assert_json_eq(json1, json2).is_ok());
    println!("  ✅ JSON equality assertion");
}

fn demo_builders() {
    println!("\n🏗️ Test Builders Demo:");
    
    let user = UserBuilder::new()
        .with_username("demo_user")
        .with_age(25)
        .admin()
        .build();
    
    println!("  ✅ Created user: {:?}", user);
    
    let admin = fixtures::Users::admin();
    println!("  ✅ Admin fixture: {:?}", admin);
}

fn demo_mocks() {
    println!("\n🎭 Mock Utilities Demo:");
    
    let mock = MockEmailService::new();
    mock.expect_send_email("demo@example.com", "Demo", "Test email")
        .returning(Ok("demo_id".to_string()));
    
    let result = mock.send_email("demo@example.com", "Demo", "Test email");
    println!("  ✅ Mock email result: {:?}", result);
    
    assert!(mock.verify());
    println!("  ✅ Mock verification passed");
}
```

## 🎉 Congratulations!

You now have a complete testing framework! Your implementation includes:
- **Custom assertions** with detailed error messages
- **Test data builders** with fluent API
- **Mock utilities** for dependency injection
- **Async testing helpers** (with feature flag)
- **Integration tests** to verify everything works together
- **Demo application** showcasing all features

This framework demonstrates advanced Rust testing patterns and could be extended with property testing, coverage reporting, and more sophisticated mocking capabilities!