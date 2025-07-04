// Exercise 08: Ecosystem Tour - Integrate major crates!
//
// This exercise attempts to use many popular Rust crates together,
// but has integration issues, version conflicts, and missing features.
//
// Your mission: Fix all the integration issues and make it work!

// Expected: A working example showcasing the Rust ecosystem
// Currently: Won't compile due to various integration problems

use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio;
use reqwest;
use sqlx;
use tracing;
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// CLI argument parsing
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// API endpoint to fetch data from
    #[arg(short, long)]
    url: String,
    
    /// Database connection string
    #[arg(short, long)]
    database: String,
    
    /// Number of requests to make
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

// Data structures
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    users: Vec<User>,
    total: u32,
}

// Database operations
async fn setup_database(database_url: &str) -> Result<sqlx::PgPool> {
    let pool = sqlx::PgPool::connect(database_url).await?;
    
    // Create table if it doesn't exist
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL
        )
    "#)
    .execute(&pool)
    .await?;
    
    Ok(pool)
}

async fn insert_user(pool: &sqlx::PgPool, user: &User) -> Result<()> {
    sqlx::query(r#"
        INSERT INTO users (id, name, email, created_at)
        VALUES ($1, $2, $3, $4)
    "#)
    .bind(user.id)
    .bind(&user.name)
    .bind(&user.email)
    .bind(user.created_at)
    .execute(pool)
    .await?;
    
    Ok(())
}

// HTTP client operations
async fn fetch_users(client: &reqwest::Client, url: &str) -> Result<Vec<User>> {
    let response = client
        .get(url)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;
    
    Ok(response.users)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Parse command line arguments
    let args = Args::parse();
    
    tracing::info!("Starting application with {} requests", args.count);
    
    // Set up database connection
    let pool = setup_database(&args.database).await?;
    
    // Create HTTP client
    let client = reqwest::Client::new();
    
    // Fetch and store users
    for i in 0..args.count {
        tracing::info!("Making request {} of {}", i + 1, args.count);
        
        let users = fetch_users(&client, &args.url).await?;
        
        for user in users {
            tracing::debug!("Storing user: {:?}", user);
            insert_user(&pool, &user).await?;
        }
    }
    
    tracing::info!("Completed all requests successfully");
    
    Ok(())
}

// Cargo.toml for this exercise (integration issues):
/*
[package]
name = "ex08-ecosystem-tour"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = "4.0"
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
reqwest = "0.11"
sqlx = "0.7"
tracing = "0.1"
anyhow = "1.0"
uuid = "1.0"
chrono = "0.4"

# Problems:
# 1. Missing feature flags for several crates
# 2. Version conflicts between dependencies
# 3. Missing tracing-subscriber dependency
# 4. sqlx needs database-specific features
# 5. reqwest needs json feature
# 6. serde needs derive feature
# 7. uuid needs v4 feature
# 8. chrono needs serde feature
# 9. clap needs derive feature
# 10. Some crates may have conflicting versions
*/