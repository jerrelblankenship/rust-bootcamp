# Mocking Patterns in Rust 🎭

*A comprehensive guide to mocking, test doubles, and dependency injection in Rust*

## 🚀 Quick Reference

### Essential Patterns (Copy & Paste Ready)

```rust
// 1. Basic trait for mocking
trait EmailService: Send + Sync {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError>;
}

// 2. Dependency injection
struct UserService {
    email_service: Box<dyn EmailService>,
}

// 3. Manual mock
struct MockEmailService {
    calls: RefCell<Vec<EmailCall>>,
    responses: RefCell<Vec<Result<String, EmailError>>>,
}

// 4. Mockall automatic mock
#[automock]
trait PaymentService {
    fn process(&self, amount: f64) -> Result<String, PaymentError>;
}

// 5. Test with mock expectations
#[test]
fn test_with_mockall() {
    let mut mock = MockPaymentService::new();
    mock.expect_process()
        .with(eq(99.99))
        .times(1)
        .returning(|_| Ok("txn_123".to_string()));
    
    let service = OrderService::new(Box::new(mock));
    let result = service.process_order(99.99);
    assert!(result.is_ok());
}
```

### Emergency Mock Setup

| **Need** | **Solution** | **Quick Pattern** |
|----------|-------------|-------------------|
| Make code testable | Add trait abstraction | `trait MyService { fn do_thing(&self); }` |
| Manual mock | RefCell for state | `calls: RefCell<Vec<Call>>` |
| Automatic mock | Use mockall | `#[automock] trait MyService` |
| Mock expectations | `.expect_method()` | `.with(eq(value)).times(1).returning(...)` |
| Verify interactions | Check after test | `assert!(mock.verify_called_times(1))` |

### Most Common Mock Types

```rust
// Stub - Returns fixed values
struct StubService { success: bool }
impl MyService for StubService {
    fn call(&self) -> Result<(), Error> {
        if self.success { Ok(()) } else { Err(Error::Failed) }
    }
}

// Spy - Records interactions  
struct SpyService { calls: RefCell<Vec<Call>> }

// Mock - Verifies expectations
struct MockService { expectations: RefCell<Vec<Expectation>> }

// Fake - Working implementation with shortcuts
struct FakeDatabase { data: RefCell<HashMap<u64, User>> }
```

### Cargo.toml Setup

```toml
[dev-dependencies]
mockall = "0.11"           # For automatic mocks
async-trait = "0.1"        # For async trait mocking
```

---

## Introduction

Mocking in Rust requires a different approach than traditional OOP languages due to Rust's ownership model and trait system. This guide covers various mocking strategies, from manual implementations to automated solutions.

## Core Concepts

### Test Doubles Hierarchy

```rust
// 1. Dummy - Passed but never used
struct DummyEmailService;
impl EmailService for DummyEmailService {
    fn send_email(&self, _: &str, _: &str, _: &str) -> Result<(), EmailError> {
        unreachable!("Should never be called")
    }
}

// 2. Stub - Returns hardcoded responses
struct StubEmailService {
    always_succeeds: bool,
}
impl EmailService for StubEmailService {
    fn send_email(&self, _: &str, _: &str, _: &str) -> Result<(), EmailError> {
        if self.always_succeeds {
            Ok(())
        } else {
            Err(EmailError::DeliveryFailed)
        }
    }
}

// 3. Spy - Records how it was called
struct SpyEmailService {
    calls: RefCell<Vec<EmailCall>>,
}

// 4. Mock - Verifies behavior expectations
struct MockEmailService {
    expectations: RefCell<Vec<Expectation>>,
    calls: RefCell<Vec<EmailCall>>,
}

// 5. Fake - Working implementation with shortcuts
struct FakeEmailService {
    sent_emails: RefCell<Vec<Email>>,
}
```

### Trait-Based Dependency Injection

```rust
// Define behavior with traits
pub trait EmailService: Send + Sync {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError>;
    fn validate_email(&self, email: &str) -> bool;
}

pub trait UserRepository: Send + Sync {
    fn save(&self, user: &User) -> Result<u64, RepositoryError>;
    fn find_by_id(&self, id: u64) -> Result<Option<User>, RepositoryError>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
}

// Service that depends on traits (dependency injection)
pub struct UserService {
    email_service: Box<dyn EmailService>,
    user_repository: Box<dyn UserRepository>,
}

impl UserService {
    pub fn new(
        email_service: Box<dyn EmailService>,
        user_repository: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            email_service,
            user_repository,
        }
    }
    
    pub fn register_user(&self, name: &str, email: &str) -> Result<User, UserError> {
        // Validate email using injected service
        if !self.email_service.validate_email(email) {
            return Err(UserError::InvalidEmail);
        }
        
        // Check if user already exists
        if let Ok(Some(_)) = self.user_repository.find_by_email(email) {
            return Err(UserError::EmailAlreadyExists);
        }
        
        // Create and save user
        let user = User::new(name.to_string(), email.to_string());
        let user_id = self.user_repository.save(&user)?;
        
        // Send welcome email
        self.email_service.send_email(
            email,
            "Welcome!",
            &format!("Welcome, {}!", name)
        )?;
        
        Ok(User { id: Some(user_id), ..user })
    }
}
```

## Manual Mocking Patterns

### 1. Simple Mock with Call Recording

```rust
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct EmailCall {
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub struct MockEmailService {
    calls: RefCell<Vec<EmailCall>>,
    responses: RefCell<Vec<Result<String, EmailError>>>,
    call_index: RefCell<usize>,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            responses: RefCell::new(Vec::new()),
            call_index: RefCell::new(0),
        }
    }
    
    // Setup expectations
    pub fn expect_send_email(&self, response: Result<String, EmailError>) -> &Self {
        self.responses.borrow_mut().push(response);
        self
    }
    
    // Verification methods
    pub fn verify_called_times(&self, expected: usize) -> bool {
        self.calls.borrow().len() == expected
    }
    
    pub fn verify_called_with(&self, to: &str, subject: &str, body: &str) -> bool {
        self.calls.borrow().iter().any(|call| {
            call.to == to && call.subject == subject && call.body == body
        })
    }
    
    pub fn get_calls(&self) -> Vec<EmailCall> {
        self.calls.borrow().clone()
    }
    
    pub fn reset(&self) {
        self.calls.borrow_mut().clear();
        self.responses.borrow_mut().clear();
        *self.call_index.borrow_mut() = 0;
    }
}

impl EmailService for MockEmailService {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError> {
        // Record the call
        self.calls.borrow_mut().push(EmailCall {
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        });
        
        // Get the response for this call
        let index = *self.call_index.borrow();
        *self.call_index.borrow_mut() = index + 1;
        
        self.responses
            .borrow()
            .get(index)
            .cloned()
            .unwrap_or_else(|| Ok("default_message_id".to_string()))
    }
    
    fn validate_email(&self, email: &str) -> bool {
        // Simple validation for testing
        email.contains('@') && email.contains('.')
    }
}

// Usage in tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn register_user_sends_welcome_email() {
        // Arrange
        let mock_email = MockEmailService::new();
        mock_email.expect_send_email(Ok("message_123".to_string()));
        
        let mock_repo = MockUserRepository::new();
        mock_repo.expect_save(Ok(42));
        mock_repo.expect_find_by_email(Ok(None));
        
        let service = UserService::new(
            Box::new(mock_email),
            Box::new(mock_repo),
        );
        
        // Act
        let result = service.register_user("Alice", "alice@example.com");
        
        // Assert
        assert!(result.is_ok());
        assert!(mock_email.verify_called_times(1));
        assert!(mock_email.verify_called_with(
            "alice@example.com",
            "Welcome!",
            "Welcome, Alice!"
        ));
    }
}
```

### 2. Parameterized Mock with Builder Pattern

```rust
pub struct MockEmailServiceBuilder {
    send_responses: Vec<Result<String, EmailError>>,
    validate_responses: Vec<bool>,
}

impl MockEmailServiceBuilder {
    pub fn new() -> Self {
        Self {
            send_responses: Vec::new(),
            validate_responses: Vec::new(),
        }
    }
    
    pub fn send_email_returns(mut self, response: Result<String, EmailError>) -> Self {
        self.send_responses.push(response);
        self
    }
    
    pub fn validate_email_returns(mut self, response: bool) -> Self {
        self.validate_responses.push(response);
        self
    }
    
    pub fn build(self) -> MockEmailService {
        MockEmailService {
            calls: RefCell::new(Vec::new()),
            send_responses: RefCell::new(self.send_responses),
            validate_responses: RefCell::new(self.validate_responses),
            send_call_index: RefCell::new(0),
            validate_call_index: RefCell::new(0),
        }
    }
}

// Usage
#[test]
fn test_with_builder() {
    let mock_email = MockEmailServiceBuilder::new()
        .validate_email_returns(true)
        .send_email_returns(Ok("msg_123".to_string()))
        .send_email_returns(Err(EmailError::DeliveryFailed))
        .build();
    
    // Use mock...
}
```

### 3. State-Based Mock

```rust
pub struct MockUserRepository {
    users: RefCell<HashMap<u64, User>>,
    next_id: RefCell<u64>,
    fail_save: RefCell<bool>,
    fail_find: RefCell<bool>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            users: RefCell::new(HashMap::new()),
            next_id: RefCell::new(1),
            fail_save: RefCell::new(false),
            fail_find: RefCell::new(false),
        }
    }
    
    // Test configuration methods
    pub fn set_save_failure(&self, should_fail: bool) {
        *self.fail_save.borrow_mut() = should_fail;
    }
    
    pub fn set_find_failure(&self, should_fail: bool) {
        *self.fail_find.borrow_mut() = should_fail;
    }
    
    pub fn add_existing_user(&self, user: User) {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() = id + 1;
        self.users.borrow_mut().insert(id, User { id: Some(id), ..user });
    }
    
    // Query methods for verification
    pub fn user_count(&self) -> usize {
        self.users.borrow().len()
    }
    
    pub fn contains_user_with_email(&self, email: &str) -> bool {
        self.users.borrow().values().any(|u| u.email == email)
    }
}

impl UserRepository for MockUserRepository {
    fn save(&self, user: &User) -> Result<u64, RepositoryError> {
        if *self.fail_save.borrow() {
            return Err(RepositoryError::DatabaseError);
        }
        
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() = id + 1;
        
        let user_with_id = User { id: Some(id), ..user.clone() };
        self.users.borrow_mut().insert(id, user_with_id);
        
        Ok(id)
    }
    
    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        if *self.fail_find.borrow() {
            return Err(RepositoryError::DatabaseError);
        }
        
        let user = self.users
            .borrow()
            .values()
            .find(|u| u.email == email)
            .cloned();
            
        Ok(user)
    }
    
    fn find_by_id(&self, id: u64) -> Result<Option<User>, RepositoryError> {
        if *self.fail_find.borrow() {
            return Err(RepositoryError::DatabaseError);
        }
        
        Ok(self.users.borrow().get(&id).cloned())
    }
}
```

## Automated Mocking with mockall

### Basic mockall Usage

```rust
use mockall::{automock, predicate::*};

#[automock]
pub trait PaymentService {
    fn process_payment(&self, amount: f64, card_token: &str) -> Result<PaymentResult, PaymentError>;
    fn refund_payment(&self, transaction_id: &str) -> Result<RefundResult, PaymentError>;
    fn validate_card(&self, card_token: &str) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn process_order_successful_payment() {
        // Create mock
        let mut mock_payment = MockPaymentService::new();
        
        // Set expectations
        mock_payment
            .expect_validate_card()
            .with(eq("card_123"))
            .times(1)
            .returning(|_| true);
            
        mock_payment
            .expect_process_payment()
            .with(eq(99.99), eq("card_123"))
            .times(1)
            .returning(|_, _| Ok(PaymentResult {
                transaction_id: "txn_456".to_string(),
                amount: 99.99,
            }));
        
        // Use mock
        let order_service = OrderService::new(Box::new(mock_payment));
        let result = order_service.process_order(99.99, "card_123");
        
        // Verify
        assert!(result.is_ok());
        // mockall automatically verifies expectations on drop
    }
}
```

### Advanced mockall Patterns

```rust
use mockall::{automock, predicate::*, Sequence};

#[automock]
pub trait NotificationService {
    fn send_sms(&self, phone: &str, message: &str) -> Result<(), NotificationError>;
    fn send_email(&self, email: &str, subject: &str, body: &str) -> Result<(), NotificationError>;
    fn send_push(&self, user_id: u64, title: &str, body: &str) -> Result<(), NotificationError>;
}

#[test]
fn test_notification_sequence() {
    let mut mock_notifications = MockNotificationService::new();
    let mut sequence = Sequence::new();
    
    // Expect calls in specific order
    mock_notifications
        .expect_send_email()
        .with(eq("user@example.com"), always(), always())
        .times(1)
        .in_sequence(&mut sequence)
        .returning(|_, _, _| Ok(()));
    
    mock_notifications
        .expect_send_sms()
        .with(eq("+1234567890"), always())
        .times(1)
        .in_sequence(&mut sequence)
        .returning(|_, _| Ok(()));
    
    mock_notifications
        .expect_send_push()
        .with(eq(123), always(), always())
        .times(1)
        .in_sequence(&mut sequence)
        .returning(|_, _, _| Ok(()));
    
    // Use service that should call notifications in sequence
    let service = MultiChannelNotificationService::new(Box::new(mock_notifications));
    service.notify_user_urgent(123, "user@example.com", "+1234567890", "Alert!").unwrap();
}

#[test]
fn test_with_complex_predicates() {
    let mut mock_notifications = MockNotificationService::new();
    
    // Custom predicate for email validation
    mock_notifications
        .expect_send_email()
        .with(
            function(|email: &str| email.contains('@')),  // Valid email
            eq("Welcome"),                                 // Exact subject
            function(|body: &str| body.len() > 10),       // Non-trivial body
        )
        .returning(|_, _, _| Ok(()));
    
    let service = NotificationService::new(Box::new(mock_notifications));
    service.send_welcome_email("alice@example.com").unwrap();
}
```

### Mocking Async Traits

```rust
use mockall::{automock, predicate::*};
use async_trait::async_trait;

#[automock]
#[async_trait]
pub trait AsyncUserService {
    async fn fetch_user(&self, id: u64) -> Result<User, UserError>;
    async fn save_user(&mut self, user: &User) -> Result<(), UserError>;
    async fn delete_user(&mut self, id: u64) -> Result<(), UserError>;
}

#[tokio::test]
async fn test_async_mock() {
    let mut mock_service = MockAsyncUserService::new();
    
    mock_service
        .expect_fetch_user()
        .with(eq(42))
        .times(1)
        .returning(|_| Box::pin(async {
            Ok(User {
                id: Some(42),
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            })
        }));
    
    let user = mock_service.fetch_user(42).await.unwrap();
    assert_eq!(user.id, Some(42));
}
```

## Mocking External Dependencies

### 1. HTTP Client Mocking

```rust
// Wrap external HTTP client
pub trait HttpClient: Send + Sync {
    fn get(&self, url: &str) -> Result<String, HttpError>;
    fn post(&self, url: &str, body: &str) -> Result<String, HttpError>;
}

// Real implementation
pub struct ReqwestClient {
    client: reqwest::blocking::Client,
}

impl HttpClient for ReqwestClient {
    fn get(&self, url: &str) -> Result<String, HttpError> {
        let response = self.client.get(url).send()?;
        Ok(response.text()?)
    }
    
    fn post(&self, url: &str, body: &str) -> Result<String, HttpError> {
        let response = self.client.post(url).body(body.to_string()).send()?;
        Ok(response.text()?)
    }
}

// Service using HTTP client
pub struct ApiService {
    http_client: Box<dyn HttpClient>,
    base_url: String,
}

impl ApiService {
    pub fn fetch_user_data(&self, user_id: u64) -> Result<UserData, ApiError> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        let response = self.http_client.get(&url)?;
        serde_json::from_str(&response).map_err(ApiError::ParseError)
    }
}

// Mock for testing
#[cfg(test)]
mockall::mock! {
    TestHttpClient {}
    
    impl HttpClient for TestHttpClient {
        fn get(&self, url: &str) -> Result<String, HttpError>;
        fn post(&self, url: &str, body: &str) -> Result<String, HttpError>;
    }
}

#[test]
fn test_fetch_user_data() {
    let mut mock_http = MockTestHttpClient::new();
    
    mock_http
        .expect_get()
        .with(eq("https://api.example.com/users/123"))
        .returning(|_| Ok(r#"{"id": 123, "name": "Alice"}"#.to_string()));
    
    let api_service = ApiService {
        http_client: Box::new(mock_http),
        base_url: "https://api.example.com".to_string(),
    };
    
    let user_data = api_service.fetch_user_data(123).unwrap();
    assert_eq!(user_data.id, 123);
    assert_eq!(user_data.name, "Alice");
}
```

### 2. Database Mocking

```rust
// Database abstraction
pub trait Database: Send + Sync {
    fn execute(&self, query: &str, params: &[&dyn std::fmt::Display]) -> Result<Vec<Row>, DbError>;
    fn transaction<F, R>(&self, f: F) -> Result<R, DbError>
    where
        F: FnOnce(&dyn Database) -> Result<R, DbError>;
}

// Repository using database
pub struct UserRepository {
    db: Box<dyn Database>,
}

impl UserRepository {
    pub fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let rows = self.db.execute(
            "SELECT id, name, email FROM users WHERE email = ?",
            &[&email],
        )?;
        
        if rows.is_empty() {
            Ok(None)
        } else {
            let row = &rows[0];
            Ok(Some(User {
                id: row.get("id")?,
                name: row.get("name")?,
                email: row.get("email")?,
            }))
        }
    }
}

// Mock database for testing
pub struct MockDatabase {
    query_results: RefCell<HashMap<String, Vec<Row>>>,
    executed_queries: RefCell<Vec<(String, Vec<String>)>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self {
            query_results: RefCell::new(HashMap::new()),
            executed_queries: RefCell::new(Vec::new()),
        }
    }
    
    pub fn expect_query(&self, query: &str, result: Vec<Row>) {
        self.query_results.borrow_mut().insert(query.to_string(), result);
    }
    
    pub fn verify_query_executed(&self, query: &str) -> bool {
        self.executed_queries
            .borrow()
            .iter()
            .any(|(q, _)| q == query)
    }
}

impl Database for MockDatabase {
    fn execute(&self, query: &str, params: &[&dyn std::fmt::Display]) -> Result<Vec<Row>, DbError> {
        // Record the query execution
        let param_strings: Vec<String> = params.iter().map(|p| p.to_string()).collect();
        self.executed_queries
            .borrow_mut()
            .push((query.to_string(), param_strings));
        
        // Return mocked result
        self.query_results
            .borrow()
            .get(query)
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }
    
    fn transaction<F, R>(&self, f: F) -> Result<R, DbError>
    where
        F: FnOnce(&dyn Database) -> Result<R, DbError>,
    {
        // Simple mock - just execute the function
        f(self)
    }
}
```

## Advanced Mocking Patterns

### 1. Partial Mocking with Real Implementation

```rust
// Sometimes you want to mock only some methods
pub struct PartialMockEmailService {
    real_service: RealEmailService,
    mock_validate: bool,
    validation_results: RefCell<Vec<bool>>,
}

impl PartialMockEmailService {
    pub fn new(real_service: RealEmailService) -> Self {
        Self {
            real_service,
            mock_validate: false,
            validation_results: RefCell::new(Vec::new()),
        }
    }
    
    pub fn mock_validation_with_results(mut self, results: Vec<bool>) -> Self {
        self.mock_validate = true;
        self.validation_results = RefCell::new(results);
        self
    }
}

impl EmailService for PartialMockEmailService {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError> {
        // Use real implementation
        self.real_service.send_email(to, subject, body)
    }
    
    fn validate_email(&self, email: &str) -> bool {
        if self.mock_validate {
            // Use mock implementation
            let mut results = self.validation_results.borrow_mut();
            results.pop().unwrap_or(false)
        } else {
            // Use real implementation
            self.real_service.validate_email(email)
        }
    }
}
```

### 2. Mock Chaining and Fluent Interface

```rust
pub struct FluentMockBuilder<T> {
    mock: T,
}

impl<T> FluentMockBuilder<T> {
    pub fn new(mock: T) -> Self {
        Self { mock }
    }
    
    pub fn and(self) -> Self {
        self
    }
    
    pub fn then(self) -> Self {
        self
    }
    
    pub fn build(self) -> T {
        self.mock
    }
}

// Extension trait for fluent mocking
trait FluentMock: Sized {
    fn fluent(self) -> FluentMockBuilder<Self> {
        FluentMockBuilder::new(self)
    }
}

impl FluentMock for MockEmailService {}

#[test]
fn test_fluent_mock_setup() {
    let mock = MockEmailService::new()
        .fluent()
        .and()
        .then()
        .build();
    
    // Configure expectations...
}
```

### 3. Mock Registry Pattern

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Global mock registry for integration tests
pub struct MockRegistry {
    email_service: Arc<Mutex<Option<Box<dyn EmailService>>>>,
    user_repository: Arc<Mutex<Option<Box<dyn UserRepository>>>>,
}

impl MockRegistry {
    pub fn new() -> Self {
        Self {
            email_service: Arc::new(Mutex::new(None)),
            user_repository: Arc::new(Mutex::new(None)),
        }
    }
    
    pub fn register_email_service(&self, service: Box<dyn EmailService>) {
        *self.email_service.lock().unwrap() = Some(service);
    }
    
    pub fn get_email_service(&self) -> Option<Box<dyn EmailService>> {
        self.email_service.lock().unwrap().take()
    }
    
    pub fn reset(&self) {
        *self.email_service.lock().unwrap() = None;
        *self.user_repository.lock().unwrap() = None;
    }
}

lazy_static::lazy_static! {
    static ref MOCK_REGISTRY: MockRegistry = MockRegistry::new();
}

// Use in integration tests
#[test]
fn integration_test_with_mock_registry() {
    // Setup
    let mock_email = MockEmailService::new();
    mock_email.expect_send_email(Ok("test_id".to_string()));
    
    MOCK_REGISTRY.register_email_service(Box::new(mock_email));
    
    // Run test that uses the mock through dependency injection
    // ...
    
    // Cleanup
    MOCK_REGISTRY.reset();
}
```

## Testing Anti-Patterns and Solutions

### Anti-Pattern 1: Over-Mocking

**Problem**: Mocking everything, including value objects
```rust
// BAD: Mocking simple value objects
#[automock]
trait EmailAddress {
    fn value(&self) -> &str;
    fn is_valid(&self) -> bool;
}
```

**Solution**: Only mock dependencies with side effects
```rust
// GOOD: Use real value objects
#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress {
    value: String,
}

impl EmailAddress {
    pub fn new(email: &str) -> Result<Self, ValidationError> {
        if email.contains('@') {
            Ok(Self { value: email.to_string() })
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }
}

// Only mock the service that sends emails
#[automock]
trait EmailService {
    fn send(&self, to: &EmailAddress, subject: &str, body: &str) -> Result<(), EmailError>;
}
```

### Anti-Pattern 2: Testing Implementation Details

**Problem**: Mocks that are too specific to implementation
```rust
// BAD: Testing internal method calls
#[test]
fn test_user_creation_calls_validation_methods() {
    let mut mock_validator = MockValidator::new();
    mock_validator.expect_validate_name().times(1);
    mock_validator.expect_validate_email().times(1);
    mock_validator.expect_validate_age().times(1);
    
    // This test is brittle - it breaks if implementation changes
}
```

**Solution**: Test behavior, not implementation
```rust
// GOOD: Test the behavior
#[test]
fn test_user_creation_rejects_invalid_data() {
    let service = UserService::new();
    
    // Test the actual behavior
    let result = service.create_user("", "invalid-email", -1);
    assert!(matches!(result, Err(UserError::ValidationFailed(_))));
}
```

### Anti-Pattern 3: Shared Mock State

**Problem**: Mocks shared between tests
```rust
// BAD: Global mock state
lazy_static! {
    static ref MOCK_EMAIL: MockEmailService = MockEmailService::new();
}

#[test]
fn test_a() {
    MOCK_EMAIL.expect_send_email(Ok("a".to_string()));
    // Test may fail if test_b runs first
}

#[test]
fn test_b() {
    MOCK_EMAIL.expect_send_email(Ok("b".to_string()));
    // Test may fail if test_a runs first
}
```

**Solution**: Fresh mocks for each test
```rust
// GOOD: Fresh mock instances
#[test]
fn test_a() {
    let mock_email = MockEmailService::new();
    mock_email.expect_send_email(Ok("a".to_string()));
    // Each test gets its own mock
}

#[test]
fn test_b() {
    let mock_email = MockEmailService::new();
    mock_email.expect_send_email(Ok("b".to_string()));
    // Isolated from other tests
}
```

## Mock Testing Utilities

### Custom Assertion Helpers

```rust
// Helper functions for common mock verifications
pub mod mock_assertions {
    use super::*;
    
    pub fn assert_email_sent_to(mock: &MockEmailService, expected_recipient: &str) {
        let calls = mock.get_calls();
        assert!(
            calls.iter().any(|call| call.to == expected_recipient),
            "Expected email to be sent to '{}', but found calls to: {:?}",
            expected_recipient,
            calls.iter().map(|c| &c.to).collect::<Vec<_>>()
        );
    }
    
    pub fn assert_email_not_sent(mock: &MockEmailService) {
        let calls = mock.get_calls();
        assert!(
            calls.is_empty(),
            "Expected no emails to be sent, but found {} calls",
            calls.len()
        );
    }
    
    pub fn assert_user_saved_with_email(mock: &MockUserRepository, expected_email: &str) {
        let saved_users = mock.get_saved_users();
        assert!(
            saved_users.iter().any(|user| user.email == expected_email),
            "Expected user with email '{}' to be saved",
            expected_email
        );
    }
}

// Usage
#[test]
fn test_with_custom_assertions() {
    let mock_email = MockEmailService::new();
    let mock_repo = MockUserRepository::new();
    
    let service = UserService::new(Box::new(mock_email), Box::new(mock_repo));
    service.register_user("Alice", "alice@example.com").unwrap();
    
    // Use custom assertions
    mock_assertions::assert_email_sent_to(&mock_email, "alice@example.com");
    mock_assertions::assert_user_saved_with_email(&mock_repo, "alice@example.com");
}
```

## Best Practices Summary

### 1. When to Mock
- ✅ **Mock dependencies with side effects** (databases, HTTP calls, file I/O)
- ✅ **Mock slow or unreliable services**
- ✅ **Mock external systems you don't control**
- ❌ **Don't mock value objects or simple data structures**
- ❌ **Don't mock the system under test**

### 2. How to Mock
- **Use traits** for dependency abstraction
- **Prefer composition** over inheritance
- **Keep mocks simple** and focused
- **Use builders** for complex mock setup
- **Verify behavior**, not implementation details

### 3. Mock Lifecycle
- **Create fresh mocks** for each test
- **Reset mock state** between tests if sharing
- **Use RAII** for automatic cleanup
- **Document mock expectations** clearly

### 4. Choosing Mock Strategy
- **Manual mocks** for simple cases and learning
- **mockall** for complex scenarios and rapid development
- **Hybrid approach** combining both as needed

## Conclusion

Effective mocking in Rust requires understanding:

1. **Trait-based dependency injection** instead of interface inheritance
2. **Ownership semantics** for mock lifecycle management
3. **Interior mutability** for mock state tracking
4. **The difference between testing behavior vs implementation**

With these patterns, you can create maintainable, reliable tests that effectively isolate the code under test while maintaining confidence in your system's behavior.