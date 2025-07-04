// Exercise 01: Dependency Hell - Fix the version conflicts!
// 
// This exercise demonstrates the infamous "dependency hell" problem.
// Multiple crates depend on different versions of the same library,
// causing compilation failures.
//
// Your mission: Fix the Cargo.toml and resolve the conflicts!

// Expected: This should parse JSON and make HTTP requests
// Currently: Won't compile due to version conflicts

use serde_json;
use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to fetch some JSON data
    let response = reqwest::get("https://api.github.com/repos/rust-lang/rust")
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    println!("Repository name: {}", response["name"]);
    println!("Stars: {}", response["stargazers_count"]);
    
    // Try to parse some JSON manually
    let json_str = r#"{"name": "Rust", "type": "Systems Language"}"#;
    let parsed: serde_json::Value = serde_json::from_str(json_str)?;
    
    println!("Parsed: {}", parsed["name"]);
    
    Ok(())
}

// Cargo.toml for this exercise (intentionally broken):
/*
[package]
name = "ex01-dependency-hell"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
# This creates a conflict - reqwest internally uses a different tokio version
# and the features don't align properly
*/