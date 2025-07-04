# Testing Framework Project - Level 2 Hints 🌿

## More Specific Implementation Guidance

### Module Implementation Order
```rust
// Recommended implementation sequence:
// 1. assertions.rs - Core assertion macros
// 2. builders.rs - Test data creation
// 3. mocks.rs - Mock utilities  
// 4. async_helpers.rs - Async test support
// 5. property.rs - Property testing integration
// 6. reporting.rs - Test analysis and reporting
```

### Assertions Module (`assertions.rs`)
```rust
// Start with simple macros:
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        if !$haystack.contains($needle) {
            panic!(
                "Assertion failed: '{}' does not contain '{}'\n  haystack: {}\n  needle: {}",
                stringify!($haystack),
                stringify!($needle), 
                $haystack,
                $needle
            );
        }
    };
}

// Then add better error handling:
pub struct AssertionError {
    pub message: String,
    pub expected: String,
    pub actual: String,
}

impl std::fmt::Display for AssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\nExpected: {}\nActual: {}", 
               self.message, self.expected, self.actual)
    }
}
```

### Builders Module (`builders.rs`)
```rust
// Pattern: Default implementation with builder methods
#[derive(Debug, Clone)]
pub struct UserBuilder {
    username: String,
    email: String,
    age: u32,
    // Add more fields as needed
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
        }
    }
}

impl UserBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    // Builder methods for customization
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }
    
    pub fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            age: self.age,
        }
    }
}
```

### Mocks Module (`mocks.rs`)
```rust
// Manual mock implementation approach:
use std::cell::RefCell;

pub struct MockEmailService {
    calls: RefCell<Vec<EmailCall>>,
    responses: RefCell<Vec<Result<String, String>>>,
}

#[derive(Debug)]
pub struct EmailCall {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            responses: RefCell::new(Vec::new()),
        }
    }
    
    pub fn expect_call(&self, response: Result<String, String>) {
        self.responses.borrow_mut().push(response);
    }
    
    pub fn verify_call_count(&self, expected: usize) -> bool {
        self.calls.borrow().len() == expected
    }
}

// For automatic mocks, check if mockall is available:
// #[cfg(feature = "mockall")]
// pub use mockall::*;
```

### Async Helpers (`async_helpers.rs`)
```rust
use std::future::Future;
use std::time::Duration;

// Timeout wrapper for async operations
pub async fn with_timeout<F, T>(
    duration: Duration,
    future: F
) -> Result<T, String>
where
    F: Future<Output = T>,
{
    match tokio::time::timeout(duration, future).await {
        Ok(result) => Ok(result),
        Err(_) => Err("Operation timed out".to_string()),
    }
}

// Time control for testing
pub async fn pause_time() {
    tokio::time::pause();
}

pub async fn advance_time(duration: Duration) {
    tokio::time::advance(duration).await;
}
```

### Dependency Management
```toml
# Add these to Cargo.toml as you implement features:

[dependencies]
# For JSON assertions
serde_json = "1.0"

# For async helpers (if implementing)
tokio = { version = "1.0", features = ["full"], optional = true }

# For mocking (if implementing)
mockall = { version = "0.11", optional = true }

# For property testing (if implementing) 
proptest = { version = "1.0", optional = true }

[features]
default = []
async = ["tokio"]
mocking = ["mockall"]
property = ["proptest"]
```

### Re-exports in `lib.rs`
```rust
// Uncomment and implement gradually:
pub mod assertions;
pub use assertions::*;

// pub mod builders;
// pub use builders::*;

// Add modules as you implement them
```

### Testing Your Framework
```rust
// Create tests for your testing utilities:
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_assert_contains_success() {
        // Should not panic
        assert_contains!("hello world", "world");
    }
    
    #[test]
    #[should_panic]
    fn test_assert_contains_failure() {
        // Should panic with good error message
        assert_contains!("hello world", "foo");
    }
}
```

## 🎯 Architecture Tips

**Key Principles:**
- Start simple, add complexity gradually
- Test your testing utilities thoroughly
- Focus on developer experience and clear error messages
- Make the API feel natural for Rust developers
- Consider what would make YOUR testing easier

**Common Pitfalls:**
- Don't try to implement everything at once
- Watch out for circular dependencies between modules
- Be careful with macro hygiene and variable capture
- Remember async requires `#[tokio::test]` attribute

Ready for Level 3 if you need complete implementations!