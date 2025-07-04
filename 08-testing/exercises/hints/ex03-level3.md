# Exercise 03 - Integration Tests: Level 3 Hints 🌳

## Complete Solutions

### Checkpoint 1: Complete Integration Test Setup
```rust
// In tests/calculator_integration.rs:

// Import from your library crate
use integration_testing::{Calculator, Operation};

#[test]
fn test_calculator_operations() {
    let mut calc = Calculator::new();
    
    calc.push(10.0);
    calc.push(5.0);
    calc.apply(Operation::Add);
    
    assert_eq!(calc.result(), Some(15.0));
}

#[test]
fn test_calculator_clear() {
    let mut calc = Calculator::new();
    
    calc.push(42.0);
    calc.clear();
    
    assert_eq!(calc.result(), None);
}
```

### Checkpoint 2: Fixing Import Issues
```rust
// Assuming your Cargo.toml has:
// [package]
// name = "my_api_client"

// In tests/api_tests.rs:
use my_api_client::{Client, Config, Response};
use my_api_client::errors::ApiError;

#[test]
fn test_client_creation() {
    let config = Config::new("https://api.example.com");
    let client = Client::with_config(config);
    
    assert_eq!(client.base_url(), "https://api.example.com");
}

#[test]
fn test_api_error_handling() {
    let client = Client::new();
    let result = client.get("/invalid-endpoint");
    
    assert!(matches!(result, Err(ApiError::NotFound)));
}
```

### Checkpoint 3: Proper Test Setup with Cleanup
```rust
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// Using TempDir for automatic cleanup
#[test]
fn test_file_processor() {
    // TempDir automatically cleans up when dropped
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    
    // Create test data
    fs::write(&test_file, "test content").unwrap();
    
    // Test your file processor
    let processor = FileProcessor::new();
    let result = processor.process(&test_file).unwrap();
    
    assert_eq!(result, "processed: test content");
    // temp_dir automatically cleaned up here
}

// Manual cleanup with Drop guard
struct TestDatabase {
    name: String,
}

impl TestDatabase {
    fn new() -> Self {
        let name = format!("test_db_{}", uuid::Uuid::new_v4());
        // Create test database
        create_database(&name);
        Self { name }
    }
    
    fn connection_string(&self) -> String {
        format!("postgres://localhost/{}", self.name)
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Cleanup database when test ends
        drop_database(&self.name);
    }
}

#[test]
fn test_database_operations() {
    let test_db = TestDatabase::new();
    let conn = connect(&test_db.connection_string()).unwrap();
    
    // Run your database tests
    let result = conn.execute("SELECT 1").unwrap();
    assert_eq!(result, vec![1]);
    
    // test_db automatically dropped and cleaned up
}
```

### Checkpoint 4: Making Tests Discoverable
```rust
// File: tests/server_tests.rs

// No mod declaration needed - this is a root test file
use my_server::{Server, Request, Response};

#[test]
fn test_server_startup() {
    let server = Server::new("127.0.0.1:0"); // Use port 0 for random port
    let addr = server.local_addr();
    
    assert!(addr.port() > 0);
}

#[test]
fn test_request_handling() {
    let server = Server::new("127.0.0.1:0");
    let response = server.handle_request(Request::get("/health"));
    
    assert_eq!(response.status(), 200);
    assert_eq!(response.body(), "OK");
}
```

### Checkpoint 5: Sharing Test Utilities
```rust
// File: tests/common/mod.rs
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_test_logging() {
    INIT.call_once(|| {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
    });
}

pub fn create_test_server() -> TestServer {
    init_test_logging();
    TestServer::new()
}

pub struct TestServer {
    port: u16,
}

impl TestServer {
    pub fn new() -> Self {
        Self { port: get_free_port() }
    }
    
    pub fn url(&self, path: &str) -> String {
        format!("http://localhost:{}{}", self.port, path)
    }
}

// File: tests/api_integration.rs
mod common;
use common::{create_test_server, init_test_logging};

#[test]
fn test_api_endpoint() {
    let server = create_test_server();
    let client = reqwest::blocking::Client::new();
    
    let response = client
        .get(&server.url("/api/users"))
        .send()
        .unwrap();
    
    assert_eq!(response.status(), 200);
}
```

### Checkpoint 6: Reliable Database Tests
```rust
use serial_test::serial;
use sqlx::{PgPool, postgres::PgPoolOptions};

async fn setup_test_db() -> PgPool {
    let pool = PgPoolOptions::new()
        .connect("postgres://localhost/test_db")
        .await
        .unwrap();
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
    
    // Clear any existing data
    sqlx::query("TRUNCATE TABLE users, orders CASCADE")
        .execute(&pool)
        .await
        .unwrap();
    
    pool
}

#[serial]  // Ensures this test runs alone
#[tokio::test]
async fn test_user_creation() {
    let pool = setup_test_db().await;
    
    // Insert test data
    let user_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id"
    )
    .bind("Alice")
    .bind("alice@example.com")
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert!(user_id > 0);
    
    // Verify insertion
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    assert_eq!(count, 1);
}

#[serial]
#[tokio::test]  
async fn test_order_processing() {
    let pool = setup_test_db().await;
    
    // Create test user first
    let user_id = create_test_user(&pool).await;
    
    // Test order creation
    let order = create_order(&pool, user_id, vec!["item1", "item2"]).await;
    
    assert_eq!(order.status, "pending");
    assert_eq!(order.items.len(), 2);
}
```

## 🎉 Congratulations!

You've mastered Rust integration testing! Key takeaways:
- Integration tests live in `tests/` directory
- Each file is a separate test binary
- Tests can only access public API
- Use proper setup/teardown for test isolation
- Share utilities via `tests/common/mod.rs`
- Use serial testing for shared resources

This approach ensures your public API works correctly, just like testing a C# library from a separate test project!