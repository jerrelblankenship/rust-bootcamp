// Exercise 02: Feature Flag Fiasco - Enable the right features!
//
// This exercise shows how missing or wrong feature flags can break compilation.
// Many Rust crates use feature flags to reduce compile time and binary size.
//
// Your mission: Add the correct feature flags to make this compile!

// Expected: Should serialize to JSON, make HTTP requests, and work with dates
// Currently: Missing feature flags cause compilation errors

use serde::{Deserialize, Serialize};
use reqwest;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a user
    let user = User {
        id: Uuid::new_v4(),
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        created_at: Utc::now(),
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&user)?;
    println!("User JSON: {}", json);
    
    // Make a POST request with JSON
    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?;
    
    println!("Status: {}", response.status());
    
    Ok(())
}

// Cargo.toml for this exercise (missing features):
/*
[package]
name = "ex02-feature-flags"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
serde_json = "1.0"
reqwest = "0.11"
chrono = "0.4"
uuid = "1.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

# Problem: Missing derive features for serde, json features for reqwest,
# serde features for chrono and uuid, etc.
*/