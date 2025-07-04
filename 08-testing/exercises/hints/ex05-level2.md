# Exercise 05 - Async Tests: Level 2 Hints 🌿

## More Specific Guidance

### Checkpoint 1: Async Test Attribute
```rust
// Regular test won't work for async:
#[test]
async fn test_async_operation() {  // This won't run!
    // ...
}

// Use tokio::test instead:
#[tokio::test]
async fn test_async_operation() {  // This works!
    let result = async_function().await;
    assert_eq!(result, expected);
}

// Or with async-std:
#[async_std::test]
async fn test_with_async_std() {
    // ...
}
```

### Checkpoint 2: Making Tests Async
```rust
// The test function itself must be async:
#[tokio::test]
async fn test_api_call() {
    // Now you can use .await
    let response = fetch_data("https://api.example.com").await;
    assert!(response.is_ok());
}

// For custom runtime configuration:
#[test]
fn test_with_runtime() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // Your async test code here
    });
}
```

### Checkpoint 3: Avoiding Blocking Operations
```rust
// DON'T: Use blocking I/O in async context
#[tokio::test]
async fn bad_test() {
    std::thread::sleep(Duration::from_secs(1));  // BLOCKS THE THREAD!
    let data = std::fs::read_to_string("file.txt").unwrap();  // BLOCKING I/O!
}

// DO: Use async equivalents
#[tokio::test]
async fn good_test() {
    tokio::time::sleep(Duration::from_secs(1)).await;  // Async sleep
    let data = tokio::fs::read_to_string("file.txt").await.unwrap();  // Async I/O
}

// For CPU-intensive work:
#[tokio::test]
async fn test_cpu_work() {
    let result = tokio::task::spawn_blocking(|| {
        // CPU-intensive work here
        expensive_computation()
    }).await.unwrap();
}
```

### Checkpoint 4: Handling Test Timeouts
```rust
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_with_timeout() {
    let future = async {
        // Some operation that might hang
        potentially_slow_operation().await
    };
    
    // Timeout after 5 seconds
    match timeout(Duration::from_secs(5), future).await {
        Ok(result) => assert_eq!(result, expected),
        Err(_) => panic!("Operation timed out!"),
    }
}

// Using test attribute with timeout:
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[timeout(1000)]  // milliseconds
async fn test_with_attribute_timeout() {
    // Test will fail if it takes more than 1 second
}
```

### Checkpoint 5: Testing Timeout Behavior
```rust
use tokio::time::{pause, advance, Duration, Instant};

#[tokio::test]
async fn test_timeout_behavior() {
    // Pause time - time won't advance automatically
    tokio::time::pause();
    
    let start = Instant::now();
    
    let timeout_future = tokio::time::timeout(
        Duration::from_secs(30),
        never_completes()
    );
    
    // Advance time manually
    tokio::time::advance(Duration::from_secs(31)).await;
    
    // Now the timeout should trigger
    assert!(timeout_future.await.is_err());
    
    // Verify elapsed time
    assert!(start.elapsed() >= Duration::from_secs(30));
}

async fn never_completes() -> String {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
```

### Checkpoint 6: Coordinating Parallel Async Tests
```rust
use tokio::sync::Mutex;
use std::sync::Arc;

// Shared state between tests
static SHARED_PORT: AtomicU16 = AtomicU16::new(8000);

// Or use a mutex for more complex state
lazy_static! {
    static ref TEST_DB: Arc<Mutex<TestDatabase>> = Arc::new(Mutex::new(TestDatabase::new()));
}

#[tokio::test]
async fn test_server_1() {
    // Get unique port for this test
    let port = SHARED_PORT.fetch_add(1, Ordering::SeqCst);
    let server = start_server(port).await;
    
    // Test with isolated port
    let response = client.get(&format!("http://localhost:{}", port)).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_with_shared_resource() {
    let db = TEST_DB.lock().await;
    // Use shared resource with mutex protection
    db.insert_test_data().await;
    
    // Don't forget to clean up!
    db.cleanup().await;
}

// Alternative: Use serial testing
#[serial_test::serial]
#[tokio::test]
async fn test_that_needs_isolation() {
    // This test won't run in parallel with other [serial] tests
}
```

## 🎯 Pattern Recognition

Key insights:
- `#[tokio::test]` is like `async Task` test methods in C#
- Always use async versions of I/O operations
- Time control in tests is powerful for timeout testing
- Resource coordination is crucial for parallel async tests

Ready for Level 3 if you need complete solutions!