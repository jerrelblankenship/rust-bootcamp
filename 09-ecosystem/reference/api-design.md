# API Design Patterns in Rust

## 🎯 Overview

Designing great APIs in Rust requires understanding idiomatic patterns that may differ from C#. This guide shows you how to create APIs that feel natural to Rust developers.

## 🔄 C# vs Rust API Philosophy

| Aspect | C# Approach | Rust Approach | Key Difference |
|--------|-------------|---------------|----------------|
| **Object Construction** | Constructors + Properties | Builder Pattern | More flexible initialization |
| **Error Handling** | Exceptions | `Result<T, E>` | Explicit error handling |
| **Null Safety** | Nullable reference types | `Option<T>` | Built into type system |
| **Method Chaining** | Fluent interfaces | Method chaining | More common in Rust |
| **Immutability** | Opt-in (readonly) | Default (mut opt-in) | Safer by default |

## 🏗️ Builder Pattern

### C# Style
```csharp
// C# - Constructor with many parameters
public class HttpClient
{
    public HttpClient(string baseUrl, TimeSpan timeout, int retries, 
                     string userAgent, string authToken) { ... }
}

// Usage - hard to remember parameter order
var client = new HttpClient(
    "https://api.example.com", 
    TimeSpan.FromSeconds(30), 
    3, 
    "MyApp/1.0", 
    "bearer-token"
);
```

### Rust Style
```rust
// Rust - Builder pattern
pub struct HttpClient {
    // Private fields
}

pub struct HttpClientBuilder {
    base_url: Option<String>,
    timeout: Duration,
    retries: u32,
    user_agent: Option<String>,
    auth_token: Option<String>,
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            timeout: Duration::from_secs(30),
            retries: 3,
            user_agent: None,
            auth_token: None,
        }
    }
    
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }
    
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
    
    pub fn user_agent(mut self, ua: &str) -> Self {
        self.user_agent = Some(ua.to_string());
        self
    }
    
    pub fn auth_token(mut self, token: &str) -> Self {
        self.auth_token = Some(token.to_string());
        self
    }
    
    pub fn build(self) -> Result<HttpClient, BuildError> {
        let base_url = self.base_url
            .ok_or(BuildError::MissingBaseUrl)?;
        
        Ok(HttpClient::new_internal(
            base_url,
            self.timeout,
            self.retries,
            self.user_agent,
            self.auth_token,
        ))
    }
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }
}

// Usage - clear and flexible
let client = HttpClient::builder()
    .base_url("https://api.example.com")
    .timeout(Duration::from_secs(30))
    .retries(3)
    .user_agent("MyApp/1.0")
    .auth_token("bearer-token")
    .build()?;
```

## 🔗 Method Chaining for Operations

### C# Style
```csharp
// C# - Separate method calls
var request = new HttpRequestMessage(HttpMethod.Get, "/users");
request.Headers.Add("X-Custom", "value");
request.Headers.Add("Accept", "application/json");

var response = await client.SendAsync(request);
var json = await response.Content.ReadAsStringAsync();
```

### Rust Style
```rust
// Rust - Fluent method chaining
let response = client
    .get("/users")
    .header("X-Custom", "value")
    .header("Accept", "application/json")
    .send()
    .await?
    .json::<User>()
    .await?;
```

**Implementation Pattern:**
```rust
pub struct RequestBuilder<'a> {
    client: &'a HttpClient,
    method: Method,
    url: String,
    headers: HeaderMap,
}

impl<'a> RequestBuilder<'a> {
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.parse().unwrap(), value.parse().unwrap());
        self
    }
    
    pub fn json<T: Serialize>(mut self, body: &T) -> Self {
        // Add JSON content type and serialize body
        self
    }
    
    pub async fn send(self) -> Result<Response, Error> {
        // Execute the request
    }
}
```

## ❌ Error Handling Patterns

### C# Style
```csharp
// C# - Exceptions
public User GetUser(int id)
{
    if (id <= 0)
        throw new ArgumentException("ID must be positive");
        
    var user = database.FindUser(id);
    if (user == null)
        throw new UserNotFoundException($"User {id} not found");
        
    return user;
}

// Usage - try/catch required
try
{
    var user = api.GetUser(123);
    Console.WriteLine(user.Name);
}
catch (ArgumentException ex)
{
    Console.WriteLine($"Invalid argument: {ex.Message}");
}
catch (UserNotFoundException ex)
{
    Console.WriteLine($"User not found: {ex.Message}");
}
```

### Rust Style
```rust
// Rust - Result types
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("ID must be positive, got {0}")]
    InvalidId(i32),
    #[error("User {0} not found")]
    NotFound(i32),
    #[error("Database error: {0}")]
    Database(String),
}

pub fn get_user(id: i32) -> Result<User, UserError> {
    if id <= 0 {
        return Err(UserError::InvalidId(id));
    }
    
    let user = database.find_user(id)
        .map_err(|e| UserError::Database(e.to_string()))?
        .ok_or(UserError::NotFound(id))?;
    
    Ok(user)
}

// Usage - explicit error handling
match api.get_user(123) {
    Ok(user) => println!("{}", user.name),
    Err(UserError::InvalidId(id)) => println!("Invalid ID: {}", id),
    Err(UserError::NotFound(id)) => println!("User {} not found", id),
    Err(UserError::Database(msg)) => println!("Database error: {}", msg),
}

// Or with ? operator for propagation
let user = api.get_user(123)?;
println!("{}", user.name);
```

## 🎛️ Configuration and Options

### C# Style
```csharp
// C# - Options pattern
public class HttpClientOptions
{
    public string BaseUrl { get; set; }
    public TimeSpan Timeout { get; set; } = TimeSpan.FromSeconds(30);
    public int MaxRetries { get; set; } = 3;
}

services.Configure<HttpClientOptions>(options =>
{
    options.BaseUrl = "https://api.example.com";
    options.Timeout = TimeSpan.FromSeconds(60);
});
```

### Rust Style
```rust
// Rust - Builder + Default
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub user_agent: String,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            user_agent: "rust-client/1.0".to_string(),
        }
    }
}

impl HttpClientConfig {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            ..Default::default()
        }
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
}

// Usage
let config = HttpClientConfig::new("https://api.example.com")
    .with_timeout(Duration::from_secs(60))
    .with_retries(5);

let client = HttpClient::with_config(config)?;
```

## 🔒 Type Safety and Invalid States

### C# Approach
```csharp
// C# - Runtime validation
public class BankAccount
{
    private decimal _balance;
    
    public decimal Balance 
    { 
        get => _balance;
        set 
        {
            if (value < 0)
                throw new ArgumentException("Balance cannot be negative");
            _balance = value;
        }
    }
}
```

### Rust Approach
```rust
// Rust - Make invalid states unrepresentable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Balance(u64); // Stored in cents, can't be negative

impl Balance {
    pub fn new(cents: u64) -> Self {
        Self(cents)
    }
    
    pub fn from_dollars(dollars: f64) -> Result<Self, BalanceError> {
        if dollars < 0.0 {
            return Err(BalanceError::Negative);
        }
        if dollars > 1_000_000.0 {
            return Err(BalanceError::TooLarge);
        }
        Ok(Self((dollars * 100.0) as u64))
    }
    
    pub fn as_dollars(&self) -> f64 {
        self.0 as f64 / 100.0
    }
    
    pub fn as_cents(&self) -> u64 {
        self.0
    }
}

pub struct BankAccount {
    balance: Balance,  // Can never be invalid
}

impl BankAccount {
    pub fn new() -> Self {
        Self {
            balance: Balance::new(0),
        }
    }
    
    pub fn balance(&self) -> Balance {
        self.balance
    }
    
    pub fn deposit(&mut self, amount: Balance) {
        self.balance = Balance::new(self.balance.0 + amount.0);
    }
    
    pub fn withdraw(&mut self, amount: Balance) -> Result<(), WithdrawError> {
        if amount.0 > self.balance.0 {
            return Err(WithdrawError::InsufficientFunds);
        }
        self.balance = Balance::new(self.balance.0 - amount.0);
        Ok(())
    }
}
```

## 📦 Module Organization

### C# Style
```csharp
// C# - Namespace organization
namespace MyLibrary.Http
{
    public class HttpClient { }
    public class Response { }
    public class RequestBuilder { }
}

namespace MyLibrary.Database
{
    public class Connection { }
    public class Query { }
}
```

### Rust Style
```rust
// Rust - Module organization
// src/lib.rs
pub mod http {
    mod client;
    mod response;
    mod request;
    
    pub use client::HttpClient;
    pub use response::Response;
    pub use request::RequestBuilder;
}

pub mod database {
    mod connection;
    mod query;
    
    pub use connection::Connection;
    pub use query::Query;
}

// Re-export common items at crate root
pub use http::HttpClient;
pub use database::Connection;
```

## 🧪 Testing APIs

### C# Style
```csharp
[Test]
public async Task HttpClient_GetUser_ReturnsUser()
{
    // Arrange
    var mockHandler = new Mock<HttpMessageHandler>();
    var client = new HttpClient(mockHandler.Object);
    
    // Act
    var user = await client.GetUserAsync(123);
    
    // Assert
    Assert.AreEqual("Alice", user.Name);
}
```

### Rust Style
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};
    
    mock! {
        HttpTransport {}
        
        #[async_trait]
        impl HttpTransport for HttpTransport {
            async fn send(&self, request: Request) -> Result<Response, Error>;
        }
    }
    
    #[tokio::test]
    async fn http_client_get_user_returns_user() {
        // Arrange
        let mut mock_transport = MockHttpTransport::new();
        mock_transport
            .expect_send()
            .with(predicate::function(|req: &Request| req.url.contains("/users/123")))
            .returning(|_| Ok(Response::json(r#"{"id": 123, "name": "Alice"}"#)));
        
        let client = HttpClient::with_transport(Box::new(mock_transport));
        
        // Act
        let user = client.get_user(123).await.unwrap();
        
        // Assert
        assert_eq!("Alice", user.name);
    }
}
```

## 🎯 API Design Checklist

### ✅ Constructor Patterns
- [ ] Use builder pattern for complex initialization
- [ ] Provide sensible defaults
- [ ] Make required parameters obvious
- [ ] Return `Result` from fallible constructors

### ✅ Method Design
- [ ] Use `&self` for read-only operations
- [ ] Use `&mut self` for mutations
- [ ] Use `self` for consuming operations
- [ ] Return `Result<T, E>` for fallible operations
- [ ] Support method chaining where appropriate

### ✅ Error Handling
- [ ] Define custom error types with `thiserror`
- [ ] Provide helpful error messages
- [ ] Include context in error chains
- [ ] Document error conditions

### ✅ Type Safety
- [ ] Use newtype pattern for domain values
- [ ] Make invalid states unrepresentable
- [ ] Use `Option<T>` instead of null
- [ ] Leverage the type system for correctness

### ✅ Documentation
- [ ] Include working examples in docs
- [ ] Document panics and error conditions
- [ ] Provide migration guides for breaking changes
- [ ] Use doctests for verification

### ✅ Ergonomics
- [ ] Minimize required imports
- [ ] Provide convenient constructors
- [ ] Support common use cases easily
- [ ] Follow naming conventions

## 🔄 Migration Tips

When porting C# APIs to Rust:

1. **Replace exceptions with `Result`** - Make errors explicit
2. **Use builders instead of constructors** - More flexible
3. **Leverage the type system** - Encode invariants in types
4. **Embrace immutability** - Default to `&self` methods
5. **Design for composition** - Small, focused types
6. **Document with examples** - Show real usage

---

**Next:** Learn about [Publishing to crates.io](publishing-guide.md)!