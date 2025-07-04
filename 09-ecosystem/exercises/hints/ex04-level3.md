# Exercise 04 - Level 3 Hints: API Design Disaster

## 🎯 Complete Redesign

```rust
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Request failed: {0}")]
    RequestFailed(String),
}

pub struct HttpClient {
    base_url: String,
    headers: HashMap<String, String>,
    timeout: Duration,
}

pub struct HttpClientBuilder {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    timeout: Duration,
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            headers: HashMap::new(),
            timeout: Duration::from_secs(30),
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
    
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn user_agent(self, ua: &str) -> Self {
        self.header("User-Agent", ua)
    }
    
    pub fn auth_token(self, token: &str) -> Self {
        self.header("Authorization", &format!("Bearer {}", token))
    }
    
    pub fn build(self) -> Result<HttpClient, HttpError> {
        let base_url = self.base_url
            .ok_or_else(|| HttpError::InvalidUrl("Base URL required".to_string()))?;
        
        Ok(HttpClient {
            base_url,
            headers: self.headers,
            timeout: self.timeout,
        })
    }
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }
    
    pub fn get(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self, "GET", path)
    }
    
    pub fn post(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self, "POST", path)
    }
}

pub struct RequestBuilder<'a> {
    client: &'a HttpClient,
    method: &'static str,
    path: String,
    headers: HashMap<String, String>,
}

impl<'a> RequestBuilder<'a> {
    fn new(client: &'a HttpClient, method: &'static str, path: &str) -> Self {
        Self {
            client,
            method,
            path: path.to_string(),
            headers: HashMap::new(),
        }
    }
    
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn send(self) -> Result<String, HttpError> {
        // Simulate request
        Ok(format!("{} {}{}", self.method, self.client.base_url, self.path))
    }
}

fn main() -> Result<(), HttpError> {
    let client = HttpClient::builder()
        .base_url("https://api.example.com")
        .timeout(Duration::from_secs(30))
        .user_agent("MyApp/1.0")
        .auth_token("secret-token")
        .build()?;
    
    let response = client
        .get("/users")
        .header("X-Custom", "value")
        .send()?;
    
    println!("Response: {}", response);
    
    Ok(())
}
```

## 🎯 What This Achieves

1. **Builder pattern** for easy construction
2. **Method chaining** for fluent API
3. **Proper error types** instead of strings
4. **Immutable by default** design
5. **Type safety** prevents misuse

## 🏆 Success Criteria

Your API should be easy to use, hard to misuse, and follow Rust idioms!