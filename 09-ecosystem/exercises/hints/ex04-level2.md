# Exercise 04 - Level 2 Hints: API Design Disaster

## 🎯 Specific Design Patterns

### 🔧 Builder Pattern
```rust
pub struct HttpClientBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    // ...
}

impl HttpClientBuilder {
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }
    
    pub fn build(self) -> Result<HttpClient, BuildError> {
        // validation and construction
    }
}
```

### 🔗 Method Chaining
```rust
let response = client
    .get("/users")
    .header("X-Custom", "value")
    .send()
    .await?;
```

## 🔍 Error Types vs Strings

```rust
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Network error: {0}")]
    Network(String),
}
```

## 🤔 C# Comparison

This is like:
- **Fluent interfaces**: `StringBuilder.Append().Append()`
- **Builder pattern**: `HttpClientBuilder.AddHeader().SetTimeout()`
- **Typed exceptions**: Custom exception types vs `Exception`

## ⏰ Time Check

Still stuck after 30 minutes? Move to Level 3 for the complete redesign.