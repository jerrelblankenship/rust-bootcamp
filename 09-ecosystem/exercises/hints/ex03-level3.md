# Exercise 03 - Level 3 Hints: Crate Selection Confusion

## 🎯 Complete Modern Rewrite

Here's the completely rewritten code using modern crates:

```rust
use anyhow::Result;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    fn new(base_url: &str) -> Self {
        ApiClient {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    async fn get_user(&self, id: u32) -> Result<User> {
        let url = format!("{}/users/{}", self.base_url, id);
        let user = self.client
            .get(&url)
            .send()
            .await?
            .json::<User>()
            .await?;
        Ok(user)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ApiClient::new("https://jsonplaceholder.typicode.com");
    let user = client.get_user(1).await?;
    
    println!("User: {} ({})", user.name, user.email);
    
    Ok(())
}
```

## 🔧 Working Cargo.toml

```toml
[package]
name = "ex03-crate-selection"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## 🎓 What Changed

1. **JSON**: `rustc_serialize` → `serde + serde_json`
   - Faster, better error messages, derive macros
   - Industry standard

2. **HTTP**: `hyper` → `reqwest`
   - Much simpler API
   - Built-in JSON support
   - Async/await friendly

3. **Errors**: `failure` → `anyhow`
   - Modern error handling
   - Great for applications
   - Easy error propagation with `?`

## 📊 Code Comparison

- **Old**: 60+ lines of complex code
- **New**: 30 lines of simple, readable code
- **Performance**: Much faster
- **Maintainability**: Much easier to understand

## 🤔 C# Comparison

This is like upgrading from:
- **WebClient** → **HttpClient**
- **JavaScriptSerializer** → **System.Text.Json**
- **Exception handling** → **Result<T, Error>** pattern

## 🏆 Success Criteria

Your program should compile and run much faster, with cleaner code that's easier to understand and maintain!