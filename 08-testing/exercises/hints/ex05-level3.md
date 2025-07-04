# Exercise 05 - Async Tests: Level 3 Hints 🌳

## Complete Solutions

### Checkpoint 1: Complete Async Test Setup
```rust
use tokio;

// Add tokio::test attribute
#[tokio::test]
async fn test_fetch_user_data() {
    let user_id = 42;
    let result = fetch_user_from_api(user_id).await;
    
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.id, user_id);
    assert!(!user.name.is_empty());
}

// For multiple runtime flavors:
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_operations() {
    let futures = (0..10).map(|i| async move {
        process_item(i).await
    });
    
    let results = futures::future::join_all(futures).await;
    assert_eq!(results.len(), 10);
    assert!(results.iter().all(|r| r.is_ok()));
}
```

### Checkpoint 2: Complete Async Function Testing
```rust
// The async function to test
async fn fetch_and_process_data(url: &str) -> Result<ProcessedData, Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let processed = process_text(&text)?;
    Ok(processed)
}

// Complete async test
#[tokio::test]
async fn test_fetch_and_process() {
    // Start a mock server
    let mock_server = MockServer::start().await;
    
    // Configure mock response
    Mock::given(method("GET"))
        .and(path("/data"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string("test data"))
        .mount(&mock_server)
        .await;
    
    // Test the async function
    let url = format!("{}/data", &mock_server.uri());
    let result = fetch_and_process_data(&url).await;
    
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data.content, "processed: test data");
}
```

### Checkpoint 3: Complete Non-Blocking Solutions
```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;

// Service with proper async I/O
pub struct FileProcessor;

impl FileProcessor {
    pub async fn process_file(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Use async file operations
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        // Process without blocking
        let processed = tokio::task::spawn(async move {
            contents.to_uppercase()  // Simulate processing
        }).await?;
        
        // Write result asynchronously
        let output_path = format!("{}.processed", path);
        let mut output = File::create(&output_path).await?;
        output.write_all(processed.as_bytes()).await?;
        
        Ok(output_path)
    }
}

#[tokio::test]
async fn test_file_processor() {
    // Create test file
    let test_file = "test_input.txt";
    tokio::fs::write(test_file, "hello world").await.unwrap();
    
    let processor = FileProcessor;
    let result = processor.process_file(test_file).await;
    
    assert!(result.is_ok());
    let output_path = result.unwrap();
    
    // Verify output
    let output_content = tokio::fs::read_to_string(&output_path).await.unwrap();
    assert_eq!(output_content, "HELLO WORLD");
    
    // Cleanup
    tokio::fs::remove_file(test_file).await.unwrap();
    tokio::fs::remove_file(output_path).await.unwrap();
}
```

### Checkpoint 4: Complete Timeout Handling
```rust
use tokio::time::{timeout, Duration};
use std::future::Future;

// Helper function for tests with timeout
async fn with_timeout<F, T>(duration: Duration, future: F) -> Result<T, String>
where
    F: Future<Output = T>,
{
    timeout(duration, future)
        .await
        .map_err(|_| "Operation timed out".to_string())
}

// Service that might hang
pub struct NetworkService {
    timeout: Duration,
}

impl NetworkService {
    pub async fn fetch_with_retry(&self, url: &str, max_retries: u32) -> Result<String, Error> {
        for attempt in 0..=max_retries {
            match timeout(self.timeout, self.fetch_internal(url)).await {
                Ok(Ok(data)) => return Ok(data),
                Ok(Err(e)) if attempt < max_retries => {
                    println!("Attempt {} failed: {}, retrying...", attempt + 1, e);
                    tokio::time::sleep(Duration::from_millis(100 * (attempt + 1) as u64)).await;
                }
                Ok(Err(e)) => return Err(e),
                Err(_) => {
                    if attempt < max_retries {
                        println!("Attempt {} timed out, retrying...", attempt + 1);
                    } else {
                        return Err(Error::Timeout);
                    }
                }
            }
        }
        unreachable!()
    }
    
    async fn fetch_internal(&self, url: &str) -> Result<String, Error> {
        // Simulate network operation
        tokio::time::sleep(Duration::from_millis(500)).await;
        Ok(format!("Data from {}", url))
    }
}

#[tokio::test]
async fn test_fetch_with_timeout() {
    let service = NetworkService {
        timeout: Duration::from_secs(1),
    };
    
    // Should succeed within timeout
    let result = with_timeout(
        Duration::from_secs(2),
        service.fetch_with_retry("https://fast.example.com", 2)
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_timeout_exceeded() {
    let service = NetworkService {
        timeout: Duration::from_millis(100),  // Very short timeout
    };
    
    // Mock slow network by not providing mock server
    let result = service.fetch_with_retry("https://slow.example.com", 0).await;
    
    assert!(matches!(result, Err(Error::Timeout)));
}
```

### Checkpoint 5: Complete Time Control Testing
```rust
use tokio::time::{self, Duration, Instant};

pub struct RateLimiter {
    requests_per_second: u32,
    last_request: tokio::sync::Mutex<Option<Instant>>,
}

impl RateLimiter {
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            requests_per_second,
            last_request: tokio::sync::Mutex::new(None),
        }
    }
    
    pub async fn check_rate_limit(&self) -> Result<(), Duration> {
        let mut last = self.last_request.lock().await;
        let now = Instant::now();
        let min_interval = Duration::from_secs(1) / self.requests_per_second;
        
        if let Some(last_time) = *last {
            let elapsed = now.duration_since(last_time);
            if elapsed < min_interval {
                return Err(min_interval - elapsed);
            }
        }
        
        *last = Some(now);
        Ok(())
    }
}

#[tokio::test]
async fn test_rate_limiter_with_time_control() {
    time::pause();  // Pause time advancement
    
    let limiter = RateLimiter::new(2);  // 2 requests per second
    
    // First request should succeed
    assert!(limiter.check_rate_limit().await.is_ok());
    
    // Immediate second request should fail
    let result = limiter.check_rate_limit().await;
    assert!(result.is_err());
    let wait_time = result.unwrap_err();
    assert!(wait_time > Duration::from_millis(400));
    
    // Advance time by half a second
    time::advance(Duration::from_millis(500)).await;
    
    // Now it should succeed
    assert!(limiter.check_rate_limit().await.is_ok());
    
    time::resume();  // Resume normal time
}

#[tokio::test]
async fn test_scheduled_task() {
    time::pause();
    
    let start = Instant::now();
    let task = async {
        time::sleep(Duration::from_secs(60)).await;
        "completed"
    };
    
    // Start the task
    let handle = tokio::spawn(task);
    
    // Advance time by 30 seconds
    time::advance(Duration::from_secs(30)).await;
    
    // Task should still be pending
    assert!(!handle.is_finished());
    
    // Advance remaining time
    time::advance(Duration::from_secs(31)).await;
    
    // Now task should complete
    let result = handle.await.unwrap();
    assert_eq!(result, "completed");
    assert!(start.elapsed() >= Duration::from_secs(60));
}
```

### Checkpoint 6: Complete Parallel Test Coordination
```rust
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use once_cell::sync::Lazy;

// Global test resources
static TEST_SEMAPHORE: Lazy<Arc<Semaphore>> = Lazy::new(|| {
    Arc::new(Semaphore::new(3))  // Max 3 concurrent tests using this resource
});

static NEXT_PORT: AtomicU16 = AtomicU16::new(9000);

// Test-specific database isolation
struct TestContext {
    db_name: String,
    port: u16,
}

impl TestContext {
    async fn setup() -> Self {
        let db_name = format!("test_db_{}", uuid::Uuid::new_v4());
        let port = NEXT_PORT.fetch_add(1, Ordering::SeqCst);
        
        // Create isolated test database
        create_test_database(&db_name).await;
        
        Self { db_name, port }
    }
    
    async fn teardown(self) {
        drop_test_database(&self.db_name).await;
    }
}

#[tokio::test]
async fn test_concurrent_api_calls() {
    let _permit = TEST_SEMAPHORE.acquire().await.unwrap();
    let ctx = TestContext::setup().await;
    
    // Run test with isolated resources
    let server = TestServer::start(ctx.port).await;
    let client = TestClient::new(&format!("http://localhost:{}", ctx.port));
    
    // Spawn multiple concurrent requests
    let handles: Vec<_> = (0..10).map(|i| {
        let client = client.clone();
        tokio::spawn(async move {
            client.create_user(&format!("user{}", i)).await
        })
    }).collect();
    
    // Wait for all requests
    let results = futures::future::join_all(handles).await;
    
    // Verify all succeeded
    assert!(results.iter().all(|r| r.is_ok()));
    
    // Cleanup
    ctx.teardown().await;
}

// Alternative: Channel-based coordination
#[tokio::test]  
async fn test_event_processing() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    
    // Spawn event processor
    let processor_handle = tokio::spawn(async move {
        let mut events = Vec::new();
        while let Some(event) = rx.recv().await {
            events.push(event);
        }
        events
    });
    
    // Send events concurrently
    let senders = (0..5).map(|i| {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("event_{}", i)).await.unwrap();
        })
    });
    
    futures::future::join_all(senders).await;
    drop(tx);  // Close channel
    
    // Get processed events
    let events = processor_handle.await.unwrap();
    assert_eq!(events.len(), 5);
}
```

## 🎉 Congratulations!

You've mastered async testing in Rust! Key takeaways:
- Use `#[tokio::test]` for async test functions
- Always use async I/O operations in async contexts
- Control time with `tokio::time::pause()` and `advance()`
- Coordinate shared resources with semaphores or mutexes
- Test timeouts and retries with controlled time
- Isolate test resources to enable parallel execution

Your async testing skills now match your C# async/await expertise!