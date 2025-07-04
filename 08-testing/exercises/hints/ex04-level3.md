# Exercise 04 - Mock Troubles: Level 3 Hints 🌳

## Complete Solutions

### Checkpoint 1: Complete Trait-Based Solution
```rust
// Define the trait for mockability
pub trait EmailService: Send + Sync {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError>;
    fn validate_email(&self, email: &str) -> bool;
}

// Real implementation
pub struct SmtpEmailService {
    smtp_server: String,
    port: u16,
}

impl EmailService for SmtpEmailService {
    fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError> {
        // Real SMTP implementation
        println!("Sending email to {} via {}", to, self.smtp_server);
        Ok(format!("Message-ID: {}", uuid::Uuid::new_v4()))
    }
    
    fn validate_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
}

// Service that uses the trait
pub struct NotificationService {
    email_service: Box<dyn EmailService>,
}

impl NotificationService {
    pub fn new(email_service: Box<dyn EmailService>) -> Self {
        Self { email_service }
    }
    
    pub fn notify_user(&self, user_email: &str, message: &str) -> Result<(), String> {
        if !self.email_service.validate_email(user_email) {
            return Err("Invalid email address".to_string());
        }
        
        self.email_service
            .send_email(user_email, "Notification", message)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
```

### Checkpoint 2: Complete mockall Implementation
```rust
use mockall::{automock, predicate::*};

#[derive(Debug)]
pub enum DatabaseError {
    NotFound,
    ConnectionFailed,
}

#[automock]
pub trait UserRepository {
    fn find_by_id(&self, id: u64) -> Result<User, DatabaseError>;
    fn find_by_email(&self, email: &str) -> Result<User, DatabaseError>;
    fn save(&mut self, user: &User) -> Result<u64, DatabaseError>;
    fn delete(&mut self, id: u64) -> Result<(), DatabaseError>;
}

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    pub fn get_user_name(&self, id: u64) -> Result<String, String> {
        self.repository
            .find_by_id(id)
            .map(|user| user.name)
            .map_err(|_| "User not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_user_name_success() {
        let mut mock_repo = MockUserRepository::new();
        
        mock_repo
            .expect_find_by_id()
            .with(eq(42))
            .times(1)
            .returning(|_| Ok(User {
                id: 42,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            }));
        
        let service = UserService::new(mock_repo);
        let result = service.get_user_name(42);
        
        assert_eq!(result, Ok("Alice".to_string()));
    }
    
    #[test]
    fn test_get_user_name_not_found() {
        let mut mock_repo = MockUserRepository::new();
        
        mock_repo
            .expect_find_by_id()
            .with(eq(999))
            .times(1)
            .returning(|_| Err(DatabaseError::NotFound));
        
        let service = UserService::new(mock_repo);
        let result = service.get_user_name(999);
        
        assert_eq!(result, Err("User not found".to_string()));
    }
}
```

### Checkpoint 3: Complete Call Verification
```rust
use std::cell::RefCell;

pub trait EventLogger {
    fn log_event(&self, event_type: &str, details: &str);
    fn log_error(&self, error: &str);
}

pub struct MockEventLogger {
    events: RefCell<Vec<(String, String)>>,
    errors: RefCell<Vec<String>>,
}

impl MockEventLogger {
    pub fn new() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            errors: RefCell::new(Vec::new()),
        }
    }
    
    pub fn assert_event_logged(&self, event_type: &str, details_contains: &str) {
        let events = self.events.borrow();
        let found = events.iter().any(|(t, d)| t == event_type && d.contains(details_contains));
        assert!(found, "Event '{}' with details containing '{}' not found", event_type, details_contains);
    }
    
    pub fn assert_no_errors(&self) {
        let errors = self.errors.borrow();
        assert!(errors.is_empty(), "Expected no errors but found: {:?}", errors);
    }
    
    pub fn event_count(&self) -> usize {
        self.events.borrow().len()
    }
}

impl EventLogger for MockEventLogger {
    fn log_event(&self, event_type: &str, details: &str) {
        self.events.borrow_mut().push((event_type.to_string(), details.to_string()));
    }
    
    fn log_error(&self, error: &str) {
        self.errors.borrow_mut().push(error.to_string());
    }
}

#[test]
fn test_event_logging() {
    let logger = MockEventLogger::new();
    let service = OrderService::new(Box::new(logger));
    
    service.process_order(Order { id: 1, total: 99.99 });
    
    logger.assert_event_logged("order_processed", "Order 1");
    logger.assert_no_errors();
    assert_eq!(logger.event_count(), 1);
}
```

### Checkpoint 4: Complete Async Mocking
```rust
use async_trait::async_trait;
use mockall::*;

#[derive(Clone, Debug)]
pub struct WeatherData {
    pub temperature: f64,
    pub humidity: u8,
}

#[automock]
#[async_trait]
pub trait WeatherApi {
    async fn get_current_weather(&self, city: &str) -> Result<WeatherData, String>;
    async fn get_forecast(&self, city: &str, days: u8) -> Result<Vec<WeatherData>, String>;
}

pub struct WeatherService<A: WeatherApi> {
    api: A,
}

impl<A: WeatherApi> WeatherService<A> {
    pub fn new(api: A) -> Self {
        Self { api }
    }
    
    pub async fn get_temperature(&self, city: &str) -> Result<f64, String> {
        let weather = self.api.get_current_weather(city).await?;
        Ok(weather.temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_temperature() {
        let mut mock_api = MockWeatherApi::new();
        
        mock_api
            .expect_get_current_weather()
            .with(eq("London"))
            .times(1)
            .returning(|_| Box::pin(async {
                Ok(WeatherData {
                    temperature: 20.5,
                    humidity: 65,
                })
            }));
        
        let service = WeatherService::new(mock_api);
        let temp = service.get_temperature("London").await;
        
        assert_eq!(temp, Ok(20.5));
    }
}
```

### Checkpoint 5: Complete External Type Wrapper
```rust
// External HTTP client we can't modify
use external_http::Client as ExternalClient;

// Our trait abstraction
pub trait HttpClient: Send + Sync {
    fn get(&self, url: &str) -> Result<String, HttpError>;
    fn post(&self, url: &str, body: &str) -> Result<String, HttpError>;
}

// Wrapper for the external client
pub struct HttpClientWrapper {
    client: ExternalClient,
}

impl HttpClientWrapper {
    pub fn new() -> Self {
        Self {
            client: ExternalClient::new(),
        }
    }
}

impl HttpClient for HttpClientWrapper {
    fn get(&self, url: &str) -> Result<String, HttpError> {
        self.client
            .get(url)
            .map_err(|e| HttpError::RequestFailed(e.to_string()))
    }
    
    fn post(&self, url: &str, body: &str) -> Result<String, HttpError> {
        self.client
            .post(url, body)
            .map_err(|e| HttpError::RequestFailed(e.to_string()))
    }
}

// Now we can mock HttpClient trait
pub struct ApiService {
    client: Box<dyn HttpClient>,
}

impl ApiService {
    pub fn new(client: Box<dyn HttpClient>) -> Self {
        Self { client }
    }
    
    pub fn fetch_user(&self, id: u64) -> Result<User, String> {
        let url = format!("https://api.example.com/users/{}", id);
        let response = self.client.get(&url).map_err(|e| e.to_string())?;
        serde_json::from_str(&response).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    
    mock! {
        TestHttpClient {}
        
        impl HttpClient for TestHttpClient {
            fn get(&self, url: &str) -> Result<String, HttpError>;
            fn post(&self, url: &str, body: &str) -> Result<String, HttpError>;
        }
    }
    
    #[test]
    fn test_fetch_user() {
        let mut mock_client = MockTestHttpClient::new();
        
        mock_client
            .expect_get()
            .with(eq("https://api.example.com/users/42"))
            .returning(|_| Ok(r#"{"id":42,"name":"Bob"}"#.to_string()));
        
        let service = ApiService::new(Box::new(mock_client));
        let user = service.fetch_user(42).unwrap();
        
        assert_eq!(user.id, 42);
        assert_eq!(user.name, "Bob");
    }
}
```

### Checkpoint 6: Complete Mock Isolation
```rust
use std::sync::Arc;
use mockall::*;

#[automock]
pub trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String);
    fn clear(&mut self);
}

// Factory function for test mocks
fn create_cache_mock() -> MockCache {
    MockCache::new()
}

// Service that uses cache
pub struct DataService<C: Cache> {
    cache: C,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_hit() {
        let mut mock = create_cache_mock();
        
        // Set up this specific test's expectations
        mock.expect_get()
            .with(eq("user:42"))
            .returning(|_| Some("cached_data".to_string()));
        
        let service = DataService::new(mock);
        let result = service.get_user_data(42);
        
        assert_eq!(result, "cached_data");
    }
    
    #[test]
    fn test_cache_miss() {
        let mut mock = create_cache_mock();  // Fresh mock!
        
        // Different expectations for this test
        mock.expect_get()
            .with(eq("user:99"))
            .returning(|_| None);
        
        mock.expect_set()
            .with(eq("user:99"), eq("fetched_data".to_string()))
            .times(1)
            .returning(|_, _| ());
        
        let mut service = DataService::new(mock);
        let result = service.get_user_data(99);
        
        assert_eq!(result, "fetched_data");
    }
    
    // Each test gets its own isolated mock instance!
}
```

## 🎉 Congratulations!

You've mastered Rust mocking patterns! Key takeaways:
- Always use traits for dependencies (like interfaces in C#)
- `mockall` automates mock generation (like Moq)
- Wrap external types you can't control
- Keep mock instances isolated per test
- Async mocking requires `async_trait` and special handling
- Design for testability from the start

Your testing game is now as strong in Rust as it was in C#!