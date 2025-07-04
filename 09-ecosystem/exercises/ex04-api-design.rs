// Exercise 04: API Design Disaster - Fix the terrible API!
//
// This exercise shows a poorly designed Rust API that fights against
// the language's idioms. It's hard to use and error-prone.
//
// Your mission: Redesign the API to be idiomatic and user-friendly!

// Expected: Clean, builder-pattern API that's hard to misuse
// Currently: C-style API that's verbose and error-prone

use std::collections::HashMap;

// BAD API DESIGN - This is what you need to fix!

pub struct HttpClient {
    base_url: String,
    headers: HashMap<String, String>,
    timeout: u64,
    retries: u32,
}

impl HttpClient {
    // Problem: Too many required parameters, easy to mess up order
    pub fn new(
        base_url: String,
        timeout: u64,
        retries: u32,
        user_agent: String,
        auth_token: Option<String>,
    ) -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), user_agent);
        if let Some(token) = auth_token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        
        HttpClient {
            base_url,
            headers,
            timeout,
            retries,
        }
    }
    
    // Problem: Returns raw strings instead of proper types
    pub fn get(&self, path: &str) -> Result<String, String> {
        // Simulate HTTP request
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        
        Ok(format!("Response from {}{}", self.base_url, path))
    }
    
    // Problem: Takes too many parameters, hard to remember order
    pub fn post(&self, path: &str, body: &str, content_type: &str) -> Result<String, String> {
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        
        Ok(format!("Posted to {}{}", self.base_url, path))
    }
    
    // Problem: Mutates the client, should return a new instance
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
    
    // Problem: Exposes internal implementation
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

// Usage example showing how painful this API is to use
fn main() {
    // This is terrible - too many parameters, wrong order is easy
    let mut client = HttpClient::new(
        "https://api.example.com".to_string(),
        30000, // timeout
        3,     // retries
        "MyApp/1.0".to_string(),
        Some("secret-token".to_string()),
    );
    
    // Mutation-based API is not idiomatic
    client.add_header("X-Custom".to_string(), "value".to_string());
    
    // Poor error handling with string errors
    match client.get("/users") {
        Ok(response) => println!("Success: {}", response),
        Err(e) => println!("Error: {}", e),
    }
    
    // Too many parameters for post
    match client.post("/users", r#"{"name": "Alice"}"#, "application/json") {
        Ok(response) => println!("Posted: {}", response),
        Err(e) => println!("Error: {}", e),
    }
}

// What you should create instead:
// 1. Builder pattern for construction
// 2. Proper error types
// 3. Method chaining
// 4. Type-safe request building
// 5. Immutable by default

// Example of good API design:
/*
let client = HttpClient::builder()
    .base_url("https://api.example.com")
    .timeout(Duration::from_secs(30))
    .retries(3)
    .user_agent("MyApp/1.0")
    .auth_token("secret-token")
    .build()?;

let response = client
    .get("/users")
    .header("X-Custom", "value")
    .send()
    .await?;
    
let user: User = client
    .post("/users")
    .json(&new_user)
    .send()
    .await?
    .json()
    .await?;
*/