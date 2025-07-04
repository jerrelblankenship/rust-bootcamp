# Exercise 04 - Mock Troubles: Level 2 Hints 🌿

## More Specific Guidance

### Checkpoint 1: Making Types Mockable with Traits
```rust
// Problem: Can't mock a concrete struct
struct EmailService { /* ... */ }

// Solution: Define a trait
trait EmailSender {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), Error>;
}

impl EmailSender for EmailService {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), Error> {
        // Real implementation
    }
}

// Now you can mock the trait!
```

This is exactly like creating an `IEmailService` interface in C#!

### Checkpoint 2: Using mockall for Automatic Mocks
```rust
use mockall::{automock, predicate::*};

#[automock]  // This generates MockDatabase automatically!
trait Database {
    fn get_user(&self, id: u64) -> Option<User>;
    fn save_user(&mut self, user: &User) -> Result<u64, Error>;
}

// In tests:
#[test]
fn test_with_mock() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user()
        .with(eq(42))  // Expect call with id=42
        .times(1)      // Expect exactly one call
        .returning(|_| Some(User { id: 42, name: "Alice".into() }));
}
```

### Checkpoint 3: Manual Call Verification
```rust
// If not using mockall, track calls manually:
struct MockLogger {
    messages: RefCell<Vec<String>>,
}

impl MockLogger {
    fn new() -> Self {
        Self { messages: RefCell::new(Vec::new()) }
    }
    
    fn verify_message_logged(&self, expected: &str) -> bool {
        self.messages.borrow().iter().any(|m| m.contains(expected))
    }
    
    fn call_count(&self) -> usize {
        self.messages.borrow().len()
    }
}

impl Logger for MockLogger {
    fn log(&self, message: &str) {
        self.messages.borrow_mut().push(message.to_string());
    }
}
```

### Checkpoint 4: Async Trait Mocking
```rust
use async_trait::async_trait;
use mockall::*;

#[automock]
#[async_trait]
trait AsyncService {
    async fn fetch_data(&self, id: u64) -> Result<Data, Error>;
}

// In tests:
#[tokio::test]
async fn test_async_mock() {
    let mut mock = MockAsyncService::new();
    mock.expect_fetch_data()
        .returning(|_| Box::pin(async { Ok(Data::default()) }));
    
    let result = mock.fetch_data(1).await;
    assert!(result.is_ok());
}
```

### Checkpoint 5: Mocking External Types
```rust
// Can't mock external types directly, so wrap them:

// External crate type
use external_crate::HttpClient;

// Create a trait for what you need
trait HttpRequester {
    fn get(&self, url: &str) -> Result<String, Error>;
}

// Wrapper that implements your trait
struct HttpClientWrapper {
    client: HttpClient,
}

impl HttpRequester for HttpClientWrapper {
    fn get(&self, url: &str) -> Result<String, Error> {
        self.client.get(url).map_err(|e| Error::Http(e))
    }
}

// Now you can mock HttpRequester!
#[cfg(test)]
mockall::mock! {
    TestHttpRequester {}
    impl HttpRequester for TestHttpRequester {
        fn get(&self, url: &str) -> Result<String, Error>;
    }
}
```

### Checkpoint 6: Isolated Mock Instances
```rust
// Problem: Shared mock state
// DON'T DO THIS:
static mut MOCK: Option<MockService> = None;

// DO THIS: Create fresh mocks for each test
#[test]
fn test_one() {
    let mut mock = MockService::new();  // Fresh instance
    mock.expect_call().returning(|| "test1");
    // Use mock...
}

#[test]
fn test_two() {
    let mut mock = MockService::new();  // Different instance
    mock.expect_call().returning(|| "test2");
    // Use mock...
}

// Or use a factory function:
fn create_mock_with_defaults() -> MockService {
    let mut mock = MockService::new();
    // Set up common expectations
    mock
}
```

## 🎯 Pattern Recognition

The key insight: Traits in Rust are like interfaces in C#, and `mockall` is like Moq - but you need to design with testability in mind from the start!

Ready for Level 3 if you need complete solutions!