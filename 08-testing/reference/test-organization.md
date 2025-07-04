# Test Organization in Rust 📁

*A comprehensive guide to structuring and organizing tests in Rust projects*

## 🚀 Quick Reference

### Basic Test Setup (Copy & Paste)

```rust
// Unit tests in source file
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic() {
        assert_eq!(my_function(), expected);
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic_function();
    }
}
```

### Integration Test Template

```rust
// In tests/integration_test.rs
use my_crate::*;

#[test]
fn integration_test() {
    // Test public API only
    let result = public_function();
    assert!(result.is_ok());
}
```

### Test Commands Cheat Sheet

```bash
cargo test                    # Run all tests
cargo test --lib             # Unit tests only
cargo test --test my_test    # Specific integration test
cargo test my_function       # Tests matching pattern
cargo test -- --nocapture    # Show println! output
cargo test -- --test-threads=1  # Run serially
```

### File Location Quick Guide

| Test Type | Location | Purpose | Access |
|-----------|----------|---------|---------|
| **Unit** | Same file as code | Test functions/methods | Private + Public |
| **Integration** | `tests/` directory | Test public API | Public only |
| **Doc** | `///` comments | Test examples | Public only |

---

## Overview

Rust has a unique approach to test organization that differs from traditional testing frameworks. This guide covers best practices, patterns, and conventions for organizing tests effectively.

## Types of Tests

### 1. Unit Tests
- **Location**: Same file as the code being tested
- **Purpose**: Test individual functions and methods
- **Scope**: Private and public APIs
- **Attribute**: `#[cfg(test)]`

### 2. Integration Tests  
- **Location**: `tests/` directory at project root
- **Purpose**: Test public API as external users would
- **Scope**: Only public APIs
- **Compilation**: Each file is a separate binary

### 3. Documentation Tests
- **Location**: In documentation comments
- **Purpose**: Ensure examples in docs work
- **Scope**: Public APIs shown in examples
- **Attribute**: Automatic with `///` comments

## Project Structure

### Recommended Layout

```
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs                 // Library root with re-exports
│   ├── main.rs               // Binary entry point (if applicable)
│   ├── models/
│   │   ├── mod.rs            // Module declarations
│   │   ├── user.rs           // User model + unit tests
│   │   └── order.rs          // Order model + unit tests
│   ├── services/
│   │   ├── mod.rs
│   │   ├── user_service.rs   // Business logic + unit tests
│   │   └── email_service.rs  // Email service + unit tests
│   ├── utils/
│   │   ├── mod.rs
│   │   └── validation.rs     // Utilities + unit tests
│   └── test_helpers/         // Optional: shared test utilities
│       ├── mod.rs
│       ├── builders.rs       // Test data builders
│       └── fixtures.rs       // Common test fixtures
├── tests/                    // Integration tests
│   ├── api_tests.rs         // API integration tests
│   ├── database_tests.rs    // Database integration tests
│   ├── user_workflow_tests.rs // End-to-end workflow tests
│   └── common/              // Shared integration test utilities
│       ├── mod.rs           // Re-exports and utilities
│       ├── setup.rs         // Test environment setup
│       └── fixtures.rs      // Integration test fixtures
├── benches/                 // Benchmark tests (optional)
│   └── performance_tests.rs
└── examples/               // Usage examples (optional)
    └── basic_usage.rs
```

## Unit Test Organization

### Basic Pattern

```rust
// src/calculator.rs

pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn divide(&self, a: f64, b: f64) -> Result<f64, DivisionError> {
        if b == 0.0 {
            Err(DivisionError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    // Private helper function
    fn validate_input(&self, value: i32) -> bool {
        value >= 0
    }
}

#[derive(Debug, PartialEq)]
pub enum DivisionError {
    DivisionByZero,
}

// Unit tests in the same file
#[cfg(test)]
mod tests {
    use super::*;  // Import everything from parent module
    
    // Helper function for test setup
    fn setup() -> Calculator {
        Calculator::new()
    }
    
    // Group related tests with nested modules
    mod addition_tests {
        use super::*;
        
        #[test]
        fn add_positive_numbers() {
            let calc = setup();
            assert_eq!(calc.add(2, 3), 5);
        }
        
        #[test]
        fn add_negative_numbers() {
            let calc = setup();
            assert_eq!(calc.add(-2, -3), -5);
        }
        
        #[test]
        fn add_mixed_numbers() {
            let calc = setup();
            assert_eq!(calc.add(-2, 3), 1);
        }
    }
    
    mod division_tests {
        use super::*;
        
        #[test]
        fn divide_success() {
            let calc = setup();
            let result = calc.divide(10.0, 2.0);
            assert_eq!(result, Ok(5.0));
        }
        
        #[test]
        fn divide_by_zero_returns_error() {
            let calc = setup();
            let result = calc.divide(10.0, 0.0);
            assert_eq!(result, Err(DivisionError::DivisionByZero));
        }
    }
    
    // Test private functions
    #[test]
    fn validate_input_accepts_positive() {
        let calc = setup();
        assert!(calc.validate_input(5));
    }
    
    #[test]
    fn validate_input_rejects_negative() {
        let calc = setup();
        assert!(!calc.validate_input(-1));
    }
}
```

### Advanced Unit Test Organization

```rust
// src/user_service.rs

use crate::models::User;
use crate::repositories::UserRepository;

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    pub fn create_user(&self, name: &str, email: &str) -> Result<User, UserError> {
        self.validate_email(email)?;
        self.validate_name(name)?;
        
        let user = User::new(name.to_string(), email.to_string());
        self.repository.save(&user)
    }
    
    pub fn find_user_by_email(&self, email: &str) -> Option<User> {
        self.repository.find_by_email(email)
    }
    
    fn validate_email(&self, email: &str) -> Result<(), UserError> {
        if email.contains('@') {
            Ok(())
        } else {
            Err(UserError::InvalidEmail)
        }
    }
    
    fn validate_name(&self, name: &str) -> Result<(), UserError> {
        if name.len() >= 2 {
            Ok(())
        } else {
            Err(UserError::InvalidName)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UserError {
    InvalidEmail,
    InvalidName,
    DuplicateEmail,
    NotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::MockUserRepository;
    
    // Test fixtures and setup
    mod setup {
        use super::*;
        
        pub fn service_with_mock() -> UserService<MockUserRepository> {
            let mock_repo = MockUserRepository::new();
            UserService::new(mock_repo)
        }
        
        pub fn valid_user_data() -> (&'static str, &'static str) {
            ("John Doe", "john@example.com")
        }
    }
    
    // Grouped by functionality
    mod user_creation {
        use super::*;
        use super::setup::*;
        
        #[test]
        fn creates_user_with_valid_data() {
            let service = service_with_mock();
            let (name, email) = valid_user_data();
            
            let result = service.create_user(name, email);
            
            assert!(result.is_ok());
            let user = result.unwrap();
            assert_eq!(user.name, name);
            assert_eq!(user.email, email);
        }
        
        #[test]
        fn rejects_invalid_email() {
            let service = service_with_mock();
            
            let result = service.create_user("John", "invalid-email");
            
            assert_eq!(result, Err(UserError::InvalidEmail));
        }
        
        #[test]
        fn rejects_short_name() {
            let service = service_with_mock();
            
            let result = service.create_user("J", "john@example.com");
            
            assert_eq!(result, Err(UserError::InvalidName));
        }
    }
    
    mod user_lookup {
        use super::*;
        use super::setup::*;
        
        #[test]
        fn finds_existing_user() {
            // Test implementation
        }
        
        #[test]
        fn returns_none_for_nonexistent_user() {
            // Test implementation
        }
    }
    
    // Private function tests
    mod validation {
        use super::*;
        use super::setup::*;
        
        #[test]
        fn email_validation_accepts_valid_email() {
            let service = service_with_mock();
            assert!(service.validate_email("test@example.com").is_ok());
        }
        
        #[test]
        fn email_validation_rejects_invalid_email() {
            let service = service_with_mock();
            assert_eq!(
                service.validate_email("invalid"), 
                Err(UserError::InvalidEmail)
            );
        }
    }
}
```

## Integration Test Organization

### Basic Integration Tests

```rust
// tests/user_service_integration.rs

use my_project::{UserService, PostgresUserRepository, User};
use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        // Initialize logging, database, etc.
        env_logger::init();
    });
}

#[test]
fn user_creation_workflow() {
    setup();
    
    // Create real database connection
    let repo = PostgresUserRepository::new("test_db_url");
    let service = UserService::new(repo);
    
    // Test complete workflow
    let result = service.create_user("Integration Test", "integration@test.com");
    assert!(result.is_ok());
    
    let user = result.unwrap();
    let found_user = service.find_user_by_email("integration@test.com");
    assert_eq!(found_user, Some(user));
}
```

### Advanced Integration Test Organization

```rust
// tests/common/mod.rs

use my_project::*;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Global test resources
lazy_static! {
    static ref TEST_DB: Mutex<TestDatabase> = Mutex::new(TestDatabase::new());
}

pub struct TestDatabase {
    url: String,
}

impl TestDatabase {
    fn new() -> Self {
        Self {
            url: "postgres://localhost/test_db".to_string(),
        }
    }
    
    pub fn clean(&self) {
        // Clean database between tests
    }
    
    pub fn url(&self) -> &str {
        &self.url
    }
}

pub struct TestEnvironment {
    pub database: String,
    pub config: Config,
}

impl TestEnvironment {
    pub fn new() -> Self {
        let db = TEST_DB.lock().unwrap();
        db.clean();
        
        Self {
            database: db.url().to_string(),
            config: Config::test_config(),
        }
    }
    
    pub fn user_service(&self) -> UserService<PostgresUserRepository> {
        let repo = PostgresUserRepository::new(&self.database);
        UserService::new(repo)
    }
}

// Test data builders for integration tests
pub mod builders {
    use super::*;
    
    pub struct IntegrationUserBuilder {
        name: String,
        email: String,
    }
    
    impl IntegrationUserBuilder {
        pub fn new() -> Self {
            Self {
                name: "Integration User".to_string(),
                email: "integration@example.com".to_string(),
            }
        }
        
        pub fn with_unique_email(mut self) -> Self {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.email = format!("user_{}@example.com", timestamp);
            self
        }
        
        pub fn build_and_save(self, service: &UserService<PostgresUserRepository>) -> User {
            service.create_user(&self.name, &self.email).unwrap()
        }
    }
}
```

```rust
// tests/user_workflow_tests.rs

mod common;
use common::*;

#[test]
fn complete_user_lifecycle() {
    let env = TestEnvironment::new();
    let service = env.user_service();
    
    // Create user
    let user = builders::IntegrationUserBuilder::new()
        .with_unique_email()
        .build_and_save(&service);
    
    // Verify user exists
    let found_user = service.find_user_by_email(&user.email);
    assert_eq!(found_user, Some(user.clone()));
    
    // Update user
    let updated_result = service.update_user(user.id, "Updated Name");
    assert!(updated_result.is_ok());
    
    // Delete user
    let delete_result = service.delete_user(user.id);
    assert!(delete_result.is_ok());
    
    // Verify user no longer exists
    let not_found = service.find_user_by_email(&user.email);
    assert_eq!(not_found, None);
}
```

## Test Helper Organization

### Shared Test Utilities

```rust
// src/test_helpers/mod.rs

pub mod builders;
pub mod fixtures;
pub mod mocks;
pub mod assertions;

// Re-export commonly used items
pub use builders::*;
pub use fixtures::*;
pub use mocks::*;
pub use assertions::*;

// Common test setup utilities
pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn temp_file_with_content(content: &str) -> tempfile::NamedTempFile {
    use std::io::Write;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(file, "{}", content).unwrap();
    file
}
```

```rust
// src/test_helpers/builders.rs

use crate::models::*;

pub struct UserBuilder {
    id: Option<u64>,
    name: String,
    email: String,
    age: u32,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
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
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }
    
    pub fn adult(mut self) -> Self {
        self.age = 25;
        self
    }
    
    pub fn minor(mut self) -> Self {
        self.age = 16;
        self
    }
    
    pub fn build(self) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
            age: self.age,
        }
    }
}
```

```rust
// src/test_helpers/fixtures.rs

use super::builders::*;
use crate::models::*;

pub struct UserFixtures;

impl UserFixtures {
    pub fn alice() -> User {
        UserBuilder::new()
            .with_name("Alice")
            .with_email("alice@example.com")
            .adult()
            .build()
    }
    
    pub fn bob() -> User {
        UserBuilder::new()
            .with_name("Bob")
            .with_email("bob@example.com")
            .adult()
            .build()
    }
    
    pub fn minor_user() -> User {
        UserBuilder::new()
            .with_name("Young User")
            .minor()
            .build()
    }
}

pub struct TestScenarios;

impl TestScenarios {
    pub fn new_user_registration() -> (User, RegistrationData) {
        let user = UserFixtures::alice();
        let registration_data = RegistrationData {
            username: user.name.clone(),
            email: user.email.clone(),
            password: "secure_password".to_string(),
        };
        (user, registration_data)
    }
}
```

## Test Configuration and Conditional Compilation

### Conditional Test Compilation

```rust
// Different test configurations
#[cfg(test)]
mod tests {
    use super::*;
    
    // Always runs in test mode
    #[test]
    fn basic_test() {
        assert_eq!(2 + 2, 4);
    }
    
    // Only runs with specific features
    #[cfg(feature = "integration-tests")]
    #[test]
    fn integration_test() {
        // Expensive integration test
    }
    
    // Platform-specific tests
    #[cfg(target_os = "linux")]
    #[test]
    fn linux_specific_test() {
        // Linux-only functionality
    }
    
    // Environment-specific tests
    #[cfg(not(target_env = "musl"))]
    #[test]
    fn non_musl_test() {
        // Test that doesn't work with musl
    }
}
```

### Test-Only Dependencies

```toml
# Cargo.toml
[dev-dependencies]
mockall = "0.11"          # Only available during testing
proptest = "1.0"          # Property-based testing
tempfile = "3.0"          # Temporary files for tests
serial_test = "0.9"       # Serial test execution

[features]
default = []
integration-tests = []    # Feature flag for expensive tests
```

## Running Tests

### Basic Test Commands

```bash
# Run all tests
cargo test

# Run only unit tests (exclude integration tests)
cargo test --lib

# Run only integration tests
cargo test --test integration_test_name

# Run tests matching a pattern
cargo test user_service

# Run tests in a specific module
cargo test tests::user_service::creation

# Run with output visible
cargo test -- --nocapture

# Run tests sequentially (not in parallel)
cargo test -- --test-threads=1

# Run ignored tests
cargo test -- --ignored
```

### Advanced Test Running

```bash
# Run with specific features
cargo test --features integration-tests

# Run with environment variables
RUST_LOG=debug cargo test

# Run specific test binary
cargo test --bin my_binary

# Run documentation tests
cargo test --doc

# Generate test coverage
cargo tarpaulin --out Html
```

## Best Practices

### 1. Test Organization Principles

- **Colocate unit tests** with the code they test
- **Use nested modules** to group related tests
- **Create helper functions** for common setup
- **Keep integration tests** in separate files
- **Share utilities** through common modules

### 2. Naming Conventions

```rust
// Good test names (descriptive)
#[test]
fn create_user_with_valid_data_succeeds() { }

#[test]
fn create_user_with_invalid_email_returns_error() { }

#[test]
fn find_user_by_email_returns_none_when_not_found() { }

// Avoid generic names
#[test]
fn test_user() { }  // Too vague

#[test]
fn test1() { }      // Not descriptive
```

### 3. Test Module Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Common setup at module level
    fn setup() -> TestEnvironment {
        TestEnvironment::new()
    }
    
    // Group by functionality, not by test type
    mod user_creation {
        use super::*;
        
        #[test]
        fn succeeds_with_valid_data() { }
        
        #[test]
        fn fails_with_invalid_email() { }
    }
    
    mod user_updates {
        use super::*;
        
        #[test]
        fn updates_existing_user() { }
        
        #[test]
        fn rejects_invalid_updates() { }
    }
}
```

### 4. Error Testing Patterns

```rust
#[test]
fn test_error_scenarios() {
    // Test Result types
    let result = fallible_function();
    assert!(matches!(result, Err(SpecificError::Variant)));
    
    // Test Option types
    let option = might_return_none();
    assert!(option.is_none());
    
    // Test panics (sparingly)
    // Use should_panic only when panic is the intended behavior
}
```

### 5. Resource Management

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Use RAII for cleanup
    struct TestEnvironment {
        temp_dir: tempfile::TempDir,
    }
    
    impl TestEnvironment {
        fn new() -> Self {
            Self {
                temp_dir: tempfile::TempDir::new().unwrap(),
            }
        }
        
        fn file_path(&self, name: &str) -> std::path::PathBuf {
            self.temp_dir.path().join(name)
        }
    }
    
    // temp_dir automatically cleaned up when dropped
    #[test]
    fn test_with_temp_files() {
        let env = TestEnvironment::new();
        let file_path = env.file_path("test.txt");
        // Use file_path...
        // Cleanup happens automatically
    }
}
```

## Common Pitfalls and Solutions

### 1. Test Isolation Issues

**Problem**: Tests interfere with each other
```rust
// BAD: Shared mutable state
static mut COUNTER: i32 = 0;

#[test]
fn test_increment() {
    unsafe {
        COUNTER += 1;
        assert_eq!(COUNTER, 1); // May fail if other tests run first
    }
}
```

**Solution**: Use proper isolation
```rust
// GOOD: Isolated state
#[test]
fn test_increment() {
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}

// Or use thread-local storage
thread_local! {
    static COUNTER: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
}
```

### 2. Async Test Issues

**Problem**: Forgetting async test setup
```rust
// BAD: Won't compile
#[test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

**Solution**: Use proper async test attributes
```rust
// GOOD: Proper async test
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### 3. Module Visibility Issues

**Problem**: Can't access private items in tests
```rust
// In same file, tests can access private items
fn private_function() -> i32 { 42 }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_private_function() {
        assert_eq!(private_function(), 42); // This works
    }
}
```

## Conclusion

Effective test organization in Rust requires understanding the unique features of the language and tooling. Key principles:

1. **Colocate unit tests** with source code using `#[cfg(test)]`
2. **Use integration tests** for public API testing
3. **Organize with modules** and clear naming conventions
4. **Share utilities** through common modules
5. **Leverage Rust's type system** for test safety
6. **Use proper async patterns** for async code
7. **Maintain test isolation** and resource cleanup

With these patterns, your Rust tests will be well-organized, maintainable, and effective at catching bugs!