// Exercise 03: Crate Selection Confusion - Choose the right crates!
//
// This exercise uses inappropriate or outdated crates for common tasks.
// The Rust ecosystem has evolved, and some crates are better choices.
//
// Your mission: Replace the crates with better alternatives!

// Expected: Fast JSON parsing, efficient HTTP client, proper error handling
// Currently: Uses suboptimal crates that are slow/deprecated/problematic

use rustc_serialize::json; // Old JSON library
use hyper; // Low-level HTTP (overkill for simple requests)
use failure; // Deprecated error handling

// This should be a simple API client
struct ApiClient {
    base_url: String,
}

impl ApiClient {
    fn new(base_url: &str) -> Self {
        ApiClient {
            base_url: base_url.to_string(),
        }
    }
    
    // This should make a simple GET request and parse JSON
    async fn get_user(&self, id: u32) -> Result<User, failure::Error> {
        // Using hyper directly is overkill for this simple case
        let uri = format!("{}/users/{}", self.base_url, id);
        let client = hyper::Client::new();
        
        // This is way too verbose for a simple GET request
        let uri = uri.parse::<hyper::Uri>()?;
        let response = client.get(uri).await?;
        
        // Old JSON parsing approach
        let body = hyper::body::to_bytes(response.into_body()).await?;
        let body_str = std::str::from_utf8(&body)?;
        
        // rustc_serialize is deprecated and slower than modern alternatives
        let user: User = json::decode(body_str)?;
        Ok(user)
    }
}

// Using rustc_serialize derive (deprecated)
#[derive(RustcEncodable, RustcDecodable)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let client = ApiClient::new("https://jsonplaceholder.typicode.com");
    let user = client.get_user(1).await?;
    
    println!("User: {} ({})", user.name, user.email);
    
    Ok(())
}

// Cargo.toml for this exercise (uses suboptimal crates):
/*
[package]
name = "ex03-crate-selection"
version = "0.1.0"
edition = "2021"

[dependencies]
rustc-serialize = "0.3"  # Deprecated, use serde instead
hyper = "0.14"           # Too low-level, use reqwest instead
failure = "0.1"          # Deprecated, use thiserror/anyhow instead
tokio = { version = "1.0", features = ["full"] }

# Better alternatives:
# serde + serde_json for JSON
# reqwest for HTTP
# thiserror or anyhow for errors
*/