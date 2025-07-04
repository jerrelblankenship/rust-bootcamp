// This example is INTENTIONALLY BROKEN to teach ecosystem integration!
// Your mission: Fix the dependencies and make this example work

use crate_ecosystem::{ApiClient, Database, DataExporter, utils};
use anyhow::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ecosystem Crate Usage Example");
    println!("==============================");
    
    // This will fail because tracing-subscriber is missing
    tracing_subscriber::fmt::init();
    
    // Generate some sample users
    println!("\n1. Generating sample users...");
    let users: Vec<crate_ecosystem::User> = (1..=3)
        .map(|_| utils::generate_random_user())
        .collect();
    
    for user in &users {
        println!("  - {} ({})", user.name, user.email);
        
        // This will fail because email validation requires regex crate
        if utils::validate_email(&user.email) {
            println!("    ✓ Email format is valid");
        } else {
            println!("    ✗ Email format is invalid");
        }
    }
    
    // Export data to different formats
    println!("\n2. Exporting data...");
    
    // JSON export (might work if serde is properly configured)
    match DataExporter::to_json(&users) {
        Ok(json) => println!("JSON export successful:\n{}", json),
        Err(e) => println!("JSON export failed: {}", e),
    }
    
    // CSV export (will fail because csv crate is missing)
    match DataExporter::to_csv(&users) {
        Ok(csv) => println!("CSV export successful:\n{}", csv),
        Err(e) => println!("CSV export failed: {}", e),
    }
    
    // YAML export (will fail because serde_yaml crate is missing)
    match DataExporter::to_yaml(&users) {
        Ok(yaml) => println!("YAML export successful:\n{}", yaml),
        Err(e) => println!("YAML export failed: {}", e),
    }
    
    // HTTP client operations
    println!("\n3. Testing HTTP client...");
    let client = ApiClient::new("https://httpbin.org".to_string());
    
    // This will likely fail due to missing reqwest features or API mismatch
    match client.fetch_users().await {
        Ok(fetched_users) => {
            println!("Fetched {} users from API", fetched_users.len());
        }
        Err(e) => {
            println!("Failed to fetch users: {}", e);
        }
    }
    
    // Database operations (will fail without proper sqlx setup)
    println!("\n4. Testing database operations...");
    
    // This will fail because we don't have a real database connection
    // and sqlx features are likely missing
    let database_url = "postgresql://localhost/test_db";
    match Database::connect(database_url).await {
        Ok(db) => {
            println!("Connected to database successfully");
            
            // Try to initialize schema
            match db.init_schema().await {
                Ok(_) => println!("Schema initialized"),
                Err(e) => println!("Schema initialization failed: {}", e),
            }
            
            // Try to insert users
            for user in &users {
                match db.insert_user(user).await {
                    Ok(inserted_user) => {
                        println!("Inserted user: {}", inserted_user.name);
                    }
                    Err(e) => {
                        println!("Failed to insert user {}: {}", user.name, e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            println!("(This is expected if you don't have PostgreSQL running)");
        }
    }
    
    // Password hashing (will fail because bcrypt crate is missing)
    println!("\n5. Testing password operations...");
    let password = "secure_password123";
    
    match utils::hash_password(password) {
        Ok(hash) => {
            println!("Password hashed successfully: {}", hash);
            
            match utils::verify_password(password, &hash) {
                Ok(is_valid) => {
                    if is_valid {
                        println!("Password verification successful");
                    } else {
                        println!("Password verification failed");
                    }
                }
                Err(e) => println!("Password verification error: {}", e),
            }
        }
        Err(e) => println!("Password hashing failed: {}", e),
    }
    
    // Platform-specific operations
    println!("\n6. Testing platform-specific operations...");
    
    #[cfg(windows)]
    {
        match crate_ecosystem::windows_specific::get_computer_name() {
            Ok(name) => println!("Computer name: {}", name),
            Err(e) => println!("Failed to get computer name: {}", e),
        }
    }
    
    #[cfg(unix)]
    {
        match crate_ecosystem::unix_specific::get_hostname() {
            Ok(hostname) => println!("Hostname: {}", hostname),
            Err(e) => println!("Failed to get hostname: {}", e),
        }
    }
    
    // Metrics demonstration (will fail because metrics crate is missing)
    println!("\n7. Testing metrics collection...");
    let metrics = crate_ecosystem::Metrics::new();
    
    for i in 1..=5 {
        metrics.increment_requests();
        let response_time = std::time::Duration::from_millis(i * 10);
        metrics.record_response_time(response_time);
        println!("Request {} processed in {:?}", i, response_time);
    }
    
    println!("\n8. Configuration loading...");
    // This will fail because config crate is missing
    match crate_ecosystem::Config::from_file("config.toml") {
        Ok(config) => {
            println!("Configuration loaded successfully");
            println!("Database URL: {}", config.database_url);
            println!("API Base URL: {}", config.api_base_url);
        }
        Err(e) => {
            println!("Failed to load configuration: {}", e);
            println!("(This is expected if config.toml doesn't exist)");
        }
    }
    
    println!("\n✅ Example completed!");
    println!("Some operations may have failed due to missing dependencies or features.");
    println!("Your task is to fix the Cargo.toml to make everything work!");
    
    Ok(())
}