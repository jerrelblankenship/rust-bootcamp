# Async Testing in Rust ⚡

*A comprehensive guide to testing asynchronous code in Rust*

## 🚀 Quick Reference

### Essential Async Test Patterns (Copy & Paste Ready)

```rust
// 1. Basic async test
#[tokio::test]
async fn test_async_function() {
    let result = my_async_function().await;
    assert!(result.is_ok());
}

// 2. Test with timeout
#[tokio::test]
async fn test_with_timeout() {
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        slow_function()
    ).await;
    assert!(result.is_ok());
}

// 3. Time control for deterministic tests
#[tokio::test]
async fn test_with_time_control() {
    tokio::time::pause();
    
    let start = tokio::time::Instant::now();
    let task = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    });
    
    tokio::time::advance(Duration::from_secs(1)).await;
    task.await.unwrap();
    
    assert!(start.elapsed() >= Duration::from_secs(1));
}

// 4. Async mock with mockall
#[automock]
#[async_trait]
trait AsyncService {
    async fn fetch_data(&self, id: u64) -> Result<String, Error>;
}

#[tokio::test]
async fn test_async_mock() {
    let mut mock = MockAsyncService::new();
    mock.expect_fetch_data()
        .returning(|_| Box::pin(async { Ok("test_data".to_string()) }));
    
    let result = mock.fetch_data(123).await;
    assert_eq!(result.unwrap(), "test_data");
}
```

### Emergency Async Setup

| **Problem** | **Solution** | **Code Pattern** |
|-------------|-------------|------------------|
| Test won't compile | Add async test macro | `#[tokio::test]` |
| Non-deterministic timing | Use time control | `tokio::time::pause()` |
| Test hangs forever | Add timeout | `tokio::time::timeout(duration, future)` |
| Mock async trait | Use async-trait + mockall | `#[automock] #[async_trait]` |
| Test concurrent code | Use Arc + spawn | `Arc::new()` + `tokio::spawn()` |
| Test channels | Use mpsc/oneshot | `tokio::sync::mpsc::channel()` |

### Cargo.toml Dependencies

```toml
[dev-dependencies]
tokio = { version = "1.0", features = ["test-util", "macros", "rt", "time"] }
tokio-test = "0.4"            # Additional test utilities
async-trait = "0.1"           # For async trait mocking
mockall = "0.11"              # Async mocking support
```

### Most Common Async Test Types

```rust
// Time-based test
#[tokio::test]
async fn test_time_based() {
    tokio::time::pause();
    // Use advance() to control time
}

// Concurrent test  
#[tokio::test]
async fn test_concurrent() {
    let handles: Vec<_> = (0..10)
        .map(|i| tokio::spawn(async move { work(i).await }))
        .collect();
    futures::future::join_all(handles).await;
}

// Stream test
#[tokio::test]
async fn test_stream() {
    let stream = create_stream();
    let results: Vec<_> = stream.collect().await;
    assert_eq!(results.len(), 5);
}

// Channel test
#[tokio::test]
async fn test_channels() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    tokio::spawn(async move { tx.send(42).await.unwrap(); });
    assert_eq!(rx.recv().await.unwrap(), 42);
}
```

---

## Introduction

Testing asynchronous code in Rust requires understanding both the async runtime and testing patterns specific to concurrent execution. This guide covers patterns, tools, and best practices for effective async testing.

## Async Test Fundamentals

### Basic Async Test Setup

```rust
// Cargo.toml dependencies
[dev-dependencies]
tokio = { version = "1.0", features = ["test-util", "macros", "rt", "time"] }
tokio-test = "0.4"

// Basic async test
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert_eq!(result, "expected");
}

// Alternative with custom runtime
#[test]
fn test_with_custom_runtime() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let result = async_function().await;
        assert_eq!(result, "expected");
    });
}

// Test configuration
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_with_specific_runtime() {
    // Test with multiple threads
}
```

### Async Function Testing Patterns

```rust
use std::time::Duration;
use tokio::time;

// Function under test
pub async fn fetch_user_data(user_id: u64) -> Result<UserData, ApiError> {
    let url = format!("https://api.example.com/users/{}", user_id);
    let response = reqwest::get(&url).await?;
    let user_data: UserData = response.json().await?;
    Ok(user_data)
}

// Testing success case
#[tokio::test]
async fn test_fetch_user_data_success() {
    // Use a mock server or mock the HTTP client
    let user_data = fetch_user_data(123).await;
    assert!(user_data.is_ok());
}

// Testing error handling
#[tokio::test]
async fn test_fetch_user_data_network_error() {
    // Test with invalid URL or network failure
    let result = fetch_user_data(0).await;
    assert!(matches!(result, Err(ApiError::NetworkError(_))));
}

// Testing with timeouts
#[tokio::test]
async fn test_fetch_user_data_timeout() {
    let timeout_duration = Duration::from_millis(100);
    
    let result = time::timeout(timeout_duration, fetch_user_data(999)).await;
    
    match result {
        Ok(user_data) => {
            // Function completed within timeout
            assert!(user_data.is_ok());
        }
        Err(_) => {
            // Function timed out
            println!("Function timed out as expected");
        }
    }
}
```

## Time Control in Async Tests

### Using tokio::time for Deterministic Testing

```rust
use tokio::time::{self, Duration, Instant};

// Function that depends on time
pub async fn process_with_delay(data: &str) -> String {
    time::sleep(Duration::from_secs(1)).await;
    format!("Processed: {}", data)
}

// Function with retry logic
pub async fn retry_operation<F, T, E>(
    mut operation: F,
    max_attempts: usize,
    delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> futures::future::BoxFuture<'static, Result<T, E>>,
{
    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts => return Err(e),
            Err(_) => {
                time::sleep(delay).await;
            }
        }
    }
    unreachable!()
}

#[tokio::test]
async fn test_process_with_delay() {
    // Pause time to control it manually
    time::pause();
    
    let start = Instant::now();
    let task = tokio::spawn(process_with_delay("test"));
    
    // Advance time manually
    time::advance(Duration::from_secs(1)).await;
    
    let result = task.await.unwrap();
    assert_eq!(result, "Processed: test");
    
    // Verify the elapsed time
    assert!(start.elapsed() >= Duration::from_secs(1));
}

#[tokio::test]
async fn test_retry_operation_success_on_second_attempt() {
    time::pause();
    
    let mut attempt_count = 0;
    let operation = || {
        attempt_count += 1;
        Box::pin(async move {
            if attempt_count == 1 {
                Err("First attempt fails")
            } else {
                Ok("Success")
            }
        })
    };
    
    let task = tokio::spawn(async move {
        retry_operation(operation, 3, Duration::from_millis(100)).await
    });
    
    // Fast-forward through the retry delay
    time::advance(Duration::from_millis(100)).await;
    
    let result = task.await.unwrap();
    assert_eq!(result, Ok("Success"));
}

#[tokio::test]
async fn test_timeout_behavior() {
    time::pause();
    
    // Function that never completes
    let never_completes = async {
        loop {
            time::sleep(Duration::from_secs(60)).await;
        }
    };
    
    let timeout_task = time::timeout(Duration::from_secs(30), never_completes);
    
    // Advance time beyond the timeout
    time::advance(Duration::from_secs(31)).await;
    
    let result = timeout_task.await;
    assert!(result.is_err()); // Should timeout
}
```

### Rate Limiting and Throttling Tests

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct RateLimiter {
    max_requests_per_second: usize,
    current_requests: Arc<AtomicUsize>,
}

impl RateLimiter {
    pub fn new(max_requests_per_second: usize) -> Self {
        Self {
            max_requests_per_second,
            current_requests: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub async fn acquire(&self) -> Result<(), RateLimitError> {
        let current = self.current_requests.load(Ordering::SeqCst);
        
        if current >= self.max_requests_per_second {
            return Err(RateLimitError::Exceeded);
        }
        
        self.current_requests.fetch_add(1, Ordering::SeqCst);
        
        // Reset counter after 1 second
        let counter = self.current_requests.clone();
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(1)).await;
            counter.fetch_sub(1, Ordering::SeqCst);
        });
        
        Ok(())
    }
}

#[tokio::test]
async fn test_rate_limiter_allows_requests_within_limit() {
    time::pause();
    
    let limiter = RateLimiter::new(5);
    
    // Should allow 5 requests
    for _ in 0..5 {
        assert!(limiter.acquire().await.is_ok());
    }
    
    // 6th request should fail
    assert!(limiter.acquire().await.is_err());
}

#[tokio::test]
async fn test_rate_limiter_resets_after_time_window() {
    time::pause();
    
    let limiter = RateLimiter::new(2);
    
    // Use up the limit
    assert!(limiter.acquire().await.is_ok());
    assert!(limiter.acquire().await.is_ok());
    assert!(limiter.acquire().await.is_err());
    
    // Advance time to reset the window
    time::advance(Duration::from_secs(1)).await;
    
    // Should be able to make requests again
    assert!(limiter.acquire().await.is_ok());
}
```

## Testing Async Streams

### Stream Processing Tests

```rust
use futures::stream::{self, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;

// Function that processes a stream
pub async fn process_number_stream<S>(mut stream: S) -> Vec<i32>
where
    S: StreamExt<Item = i32> + Unpin,
{
    let mut results = Vec::new();
    
    while let Some(number) = stream.next().await {
        if number % 2 == 0 {
            results.push(number * 2);
        }
    }
    
    results
}

// Function that creates a stream from async operations
pub async fn fetch_data_stream(urls: Vec<String>) -> impl StreamExt<Item = Result<String, reqwest::Error>> {
    stream::iter(urls)
        .map(|url| async move {
            reqwest::get(&url).await?.text().await
        })
        .buffer_unordered(3) // Process up to 3 requests concurrently
}

#[tokio::test]
async fn test_process_number_stream() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let stream = stream::iter(numbers);
    
    let results = process_number_stream(stream).await;
    
    // Should contain only even numbers doubled
    assert_eq!(results, vec![4, 8, 12]);
}

#[tokio::test]
async fn test_stream_from_channel() {
    let (tx, rx) = mpsc::channel(10);
    
    // Spawn task to send data
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            time::sleep(Duration::from_millis(10)).await;
        }
    });
    
    let stream = ReceiverStream::new(rx);
    let results = process_number_stream(stream).await;
    
    assert_eq!(results, vec![4, 8]);
}

#[tokio::test]
async fn test_stream_with_timeout() {
    let (tx, rx) = mpsc::channel(10);
    
    // Send one item then delay
    tokio::spawn(async move {
        tx.send(2).await.unwrap();
        time::sleep(Duration::from_secs(10)).await; // Long delay
        tx.send(4).await.unwrap();
    });
    
    let stream = ReceiverStream::new(rx)
        .timeout(Duration::from_millis(100));
    
    let mut results = Vec::new();
    let mut stream = Box::pin(stream);
    
    while let Some(item) = stream.next().await {
        match item {
            Ok(number) => {
                if number % 2 == 0 {
                    results.push(number * 2);
                }
            }
            Err(_) => break, // Timeout occurred
        }
    }
    
    // Should only get the first item before timeout
    assert_eq!(results, vec![4]);
}
```

### Async Iterator Testing

```rust
use async_stream::stream;
use futures::pin_mut;

// Async generator function
pub fn fibonacci_stream() -> impl StreamExt<Item = u64> {
    stream! {
        let mut a = 0;
        let mut b = 1;
        
        loop {
            yield a;
            let next = a + b;
            a = b;
            b = next;
            
            // Add a small delay to make it async
            time::sleep(Duration::from_millis(1)).await;
        }
    }
}

#[tokio::test]
async fn test_fibonacci_stream() {
    let stream = fibonacci_stream().take(10);
    pin_mut!(stream);
    
    let mut results = Vec::new();
    while let Some(value) = stream.next().await {
        results.push(value);
    }
    
    assert_eq!(
        results,
        vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    );
}

#[tokio::test]
async fn test_fibonacci_stream_with_time_control() {
    time::pause();
    
    let stream = fibonacci_stream().take(3);
    pin_mut!(stream);
    
    let start = Instant::now();
    
    let results: Vec<_> = stream.collect().await;
    
    // Advance time to account for the delays
    time::advance(Duration::from_millis(3)).await;
    
    assert_eq!(results, vec![0, 1, 1]);
    assert!(start.elapsed() >= Duration::from_millis(3));
}
```

## Concurrent Testing Patterns

### Testing Race Conditions

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Counter {
    value: Arc<Mutex<i32>>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }
    
    pub async fn increment(&self) {
        let mut guard = self.value.lock().await;
        *guard += 1;
    }
    
    pub async fn get(&self) -> i32 {
        let guard = self.value.lock().await;
        *guard
    }
}

#[tokio::test]
async fn test_concurrent_counter_increments() {
    let counter = Arc::new(Counter::new());
    let mut handles = Vec::new();
    
    // Spawn 100 concurrent increment tasks
    for _ in 0..100 {
        let counter_clone = counter.clone();
        let handle = tokio::spawn(async move {
            counter_clone.increment().await;
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Verify the final count
    assert_eq!(counter.get().await, 100);
}

#[tokio::test]
async fn test_deadlock_detection() {
    // Create two mutexes to test potential deadlock
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let mutex1_clone = mutex1.clone();
    let mutex2_clone = mutex2.clone();
    
    let task1 = tokio::spawn(async move {
        let _guard1 = mutex1_clone.lock().await;
        time::sleep(Duration::from_millis(10)).await;
        let _guard2 = mutex2_clone.lock().await;
        "task1 completed"
    });
    
    let task2 = tokio::spawn(async move {
        let _guard2 = mutex2.lock().await;
        time::sleep(Duration::from_millis(10)).await;
        let _guard1 = mutex1.lock().await;
        "task2 completed"
    });
    
    // Use timeout to detect potential deadlock
    let timeout_duration = Duration::from_secs(1);
    
    let result1 = time::timeout(timeout_duration, task1).await;
    let result2 = time::timeout(timeout_duration, task2).await;
    
    // Both should complete without timeout (no deadlock)
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}
```

### Channel-Based Communication Testing

```rust
use tokio::sync::{mpsc, oneshot, broadcast};

// Producer-consumer pattern
pub async fn producer(tx: mpsc::Sender<i32>, count: usize) {
    for i in 0..count {
        if tx.send(i as i32).await.is_err() {
            break;
        }
        time::sleep(Duration::from_millis(1)).await;
    }
}

pub async fn consumer(mut rx: mpsc::Receiver<i32>) -> Vec<i32> {
    let mut results = Vec::new();
    
    while let Some(value) = rx.recv().await {
        results.push(value);
    }
    
    results
}

#[tokio::test]
async fn test_producer_consumer() {
    let (tx, rx) = mpsc::channel(10);
    
    let producer_handle = tokio::spawn(producer(tx, 5));
    let consumer_handle = tokio::spawn(consumer(rx));
    
    let (_, results) = tokio::join!(producer_handle, consumer_handle);
    
    assert_eq!(results.unwrap(), vec![0, 1, 2, 3, 4]);
}

#[tokio::test]
async fn test_broadcast_channel() {
    let (tx, mut rx1) = broadcast::channel(10);
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();
    
    // Send some messages
    tx.send("message1").unwrap();
    tx.send("message2").unwrap();
    tx.send("message3").unwrap();
    
    // Drop sender to close channel
    drop(tx);
    
    // Collect messages from all receivers
    let mut messages1 = Vec::new();
    while let Ok(msg) = rx1.recv().await {
        messages1.push(msg);
    }
    
    let mut messages2 = Vec::new();
    while let Ok(msg) = rx2.recv().await {
        messages2.push(msg);
    }
    
    let mut messages3 = Vec::new();
    while let Ok(msg) = rx3.recv().await {
        messages3.push(msg);
    }
    
    // All receivers should get the same messages
    assert_eq!(messages1, vec!["message1", "message2", "message3"]);
    assert_eq!(messages2, vec!["message1", "message2", "message3"]);
    assert_eq!(messages3, vec!["message1", "message2", "message3"]);
}

#[tokio::test]
async fn test_oneshot_channel() {
    let (tx, rx) = oneshot::channel();
    
    let sender_task = tokio::spawn(async move {
        time::sleep(Duration::from_millis(100)).await;
        tx.send("result").unwrap();
    });
    
    let receiver_task = tokio::spawn(async move {
        rx.await.unwrap()
    });
    
    let (_, result) = tokio::join!(sender_task, receiver_task);
    assert_eq!(result.unwrap(), "result");
}
```

## Async Mocking Patterns

### Mocking Async Dependencies

```rust
use async_trait::async_trait;
use mockall::{automock, predicate::*};

#[automock]
#[async_trait]
pub trait AsyncEmailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError>;
    async fn verify_email(&self, email: &str) -> Result<bool, EmailError>;
}

#[automock]
#[async_trait]
pub trait AsyncUserRepository {
    async fn save(&self, user: &User) -> Result<u64, RepositoryError>;
    async fn find_by_id(&self, id: u64) -> Result<Option<User>, RepositoryError>;
}

pub struct AsyncUserService {
    email_service: Box<dyn AsyncEmailService>,
    user_repository: Box<dyn AsyncUserRepository>,
}

impl AsyncUserService {
    pub fn new(
        email_service: Box<dyn AsyncEmailService>,
        user_repository: Box<dyn AsyncUserRepository>,
    ) -> Self {
        Self {
            email_service,
            user_repository,
        }
    }
    
    pub async fn register_user(&self, name: &str, email: &str) -> Result<User, UserError> {
        // Verify email asynchronously
        let is_valid = self.email_service.verify_email(email).await?;
        if !is_valid {
            return Err(UserError::InvalidEmail);
        }
        
        // Create and save user
        let user = User::new(name.to_string(), email.to_string());
        let user_id = self.user_repository.save(&user).await?;
        
        // Send welcome email asynchronously
        self.email_service.send_email(
            email,
            "Welcome!",
            &format!("Welcome, {}!", name),
        ).await?;
        
        Ok(User { id: Some(user_id), ..user })
    }
}

#[tokio::test]
async fn test_async_user_registration() {
    let mut mock_email = MockAsyncEmailService::new();
    let mut mock_repo = MockAsyncUserRepository::new();
    
    // Set up async mock expectations
    mock_email
        .expect_verify_email()
        .with(eq("alice@example.com"))
        .times(1)
        .returning(|_| Box::pin(async { Ok(true) }));
    
    mock_email
        .expect_send_email()
        .with(
            eq("alice@example.com"),
            eq("Welcome!"),
            eq("Welcome, Alice!"),
        )
        .times(1)
        .returning(|_, _, _| Box::pin(async { Ok("msg_123".to_string()) }));
    
    mock_repo
        .expect_save()
        .times(1)
        .returning(|_| Box::pin(async { Ok(42) }));
    
    let service = AsyncUserService::new(
        Box::new(mock_email),
        Box::new(mock_repo),
    );
    
    let result = service.register_user("Alice", "alice@example.com").await;
    
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.id, Some(42));
}
```

### Manual Async Mocks

```rust
use std::future::Future;
use std::pin::Pin;

pub struct ManualAsyncEmailService {
    send_responses: RefCell<Vec<Result<String, EmailError>>>,
    verify_responses: RefCell<Vec<Result<bool, EmailError>>>,
    send_delays: RefCell<Vec<Duration>>,
    calls: RefCell<Vec<AsyncEmailCall>>,
}

#[derive(Debug, Clone)]
pub struct AsyncEmailCall {
    pub operation: String,
    pub timestamp: Instant,
    pub args: Vec<String>,
}

impl ManualAsyncEmailService {
    pub fn new() -> Self {
        Self {
            send_responses: RefCell::new(Vec::new()),
            verify_responses: RefCell::new(Vec::new()),
            send_delays: RefCell::new(Vec::new()),
            calls: RefCell::new(Vec::new()),
        }
    }
    
    pub fn expect_send_email(&self, response: Result<String, EmailError>) -> &Self {
        self.send_responses.borrow_mut().push(response);
        self
    }
    
    pub fn with_send_delay(&self, delay: Duration) -> &Self {
        self.send_delays.borrow_mut().push(delay);
        self
    }
    
    pub fn expect_verify_email(&self, response: Result<bool, EmailError>) -> &Self {
        self.verify_responses.borrow_mut().push(response);
        self
    }
    
    pub fn get_calls(&self) -> Vec<AsyncEmailCall> {
        self.calls.borrow().clone()
    }
}

#[async_trait]
impl AsyncEmailService for ManualAsyncEmailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, EmailError> {
        // Record the call
        self.calls.borrow_mut().push(AsyncEmailCall {
            operation: "send_email".to_string(),
            timestamp: Instant::now(),
            args: vec![to.to_string(), subject.to_string(), body.to_string()],
        });
        
        // Apply delay if configured
        if let Some(delay) = self.send_delays.borrow_mut().pop() {
            time::sleep(delay).await;
        }
        
        // Return configured response
        self.send_responses
            .borrow_mut()
            .pop()
            .unwrap_or(Ok("default_msg_id".to_string()))
    }
    
    async fn verify_email(&self, email: &str) -> Result<bool, EmailError> {
        self.calls.borrow_mut().push(AsyncEmailCall {
            operation: "verify_email".to_string(),
            timestamp: Instant::now(),
            args: vec![email.to_string()],
        });
        
        self.verify_responses
            .borrow_mut()
            .pop()
            .unwrap_or(Ok(true))
    }
}

#[tokio::test]
async fn test_with_manual_async_mock() {
    let mock = ManualAsyncEmailService::new();
    mock.expect_verify_email(Ok(true))
        .expect_send_email(Ok("msg_123".to_string()))
        .with_send_delay(Duration::from_millis(100));
    
    let start = Instant::now();
    
    let verify_result = mock.verify_email("test@example.com").await;
    assert_eq!(verify_result, Ok(true));
    
    let send_result = mock.send_email("test@example.com", "Test", "Body").await;
    assert_eq!(send_result, Ok("msg_123".to_string()));
    
    // Verify the delay was applied
    assert!(start.elapsed() >= Duration::from_millis(100));
    
    // Verify call order
    let calls = mock.get_calls();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].operation, "verify_email");
    assert_eq!(calls[1].operation, "send_email");
}
```

## Advanced Async Testing Patterns

### Testing Backpressure and Flow Control

```rust
use tokio::sync::Semaphore;

pub struct ThrottledProcessor {
    semaphore: Semaphore,
}

impl ThrottledProcessor {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Semaphore::new(max_concurrent),
        }
    }
    
    pub async fn process_item(&self, item: i32) -> Result<i32, ProcessingError> {
        let _permit = self.semaphore.acquire().await.unwrap();
        
        // Simulate processing time
        time::sleep(Duration::from_millis(100)).await;
        
        if item < 0 {
            Err(ProcessingError::InvalidInput)
        } else {
            Ok(item * 2)
        }
    }
    
    pub async fn process_batch(&self, items: Vec<i32>) -> Vec<Result<i32, ProcessingError>> {
        let tasks: Vec<_> = items
            .into_iter()
            .map(|item| self.process_item(item))
            .collect();
        
        futures::future::join_all(tasks).await
    }
}

#[tokio::test]
async fn test_throttled_processing() {
    time::pause();
    
    let processor = ThrottledProcessor::new(2); // Max 2 concurrent
    let items = vec![1, 2, 3, 4, 5];
    
    let start = Instant::now();
    let task = tokio::spawn(async move {
        processor.process_batch(items).await
    });
    
    // First 2 items should start immediately
    // Remaining 3 should wait for permits
    time::advance(Duration::from_millis(100)).await; // First batch completes
    time::advance(Duration::from_millis(100)).await; // Second batch completes
    time::advance(Duration::from_millis(100)).await; // Last item completes
    
    let results = task.await.unwrap();
    
    // Verify results
    assert_eq!(results.len(), 5);
    assert!(results.iter().all(|r| r.is_ok()));
    
    // Verify timing - should take at least 3 * 100ms due to throttling
    assert!(start.elapsed() >= Duration::from_millis(300));
}

#[tokio::test]
async fn test_backpressure_handling() {
    let (tx, mut rx) = mpsc::channel::<i32>(2); // Small buffer
    
    // Producer that sends faster than consumer can handle
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            match tx.send(i).await {
                Ok(_) => println!("Sent: {}", i),
                Err(_) => {
                    println!("Channel closed");
                    break;
                }
            }
        }
    });
    
    // Slow consumer
    let consumer = tokio::spawn(async move {
        let mut received = Vec::new();
        while let Some(value) = rx.recv().await {
            received.push(value);
            time::sleep(Duration::from_millis(50)).await; // Slow processing
            
            if received.len() >= 5 {
                break; // Stop early to test backpressure
            }
        }
        received
    });
    
    let (_, received) = tokio::join!(producer, consumer);
    let received = received.unwrap();
    
    // Should receive first 5 items due to backpressure
    assert_eq!(received, vec![0, 1, 2, 3, 4]);
}
```

### Testing Cancellation and Cleanup

```rust
use tokio_util::sync::CancellationToken;

pub struct CancellableTask {
    cancellation_token: CancellationToken,
}

impl CancellableTask {
    pub fn new() -> Self {
        Self {
            cancellation_token: CancellationToken::new(),
        }
    }
    
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }
    
    pub async fn run_with_cleanup(&self) -> Result<String, TaskError> {
        // Simulate work with periodic cancellation checks
        for i in 0..10 {
            tokio::select! {
                _ = self.cancellation_token.cancelled() => {
                    // Perform cleanup
                    self.cleanup().await;
                    return Err(TaskError::Cancelled);
                }
                _ = time::sleep(Duration::from_millis(100)) => {
                    println!("Step {} completed", i);
                }
            }
        }
        
        Ok("Task completed".to_string())
    }
    
    async fn cleanup(&self) {
        println!("Performing cleanup...");
        time::sleep(Duration::from_millis(50)).await;
        println!("Cleanup completed");
    }
}

#[tokio::test]
async fn test_task_cancellation() {
    let task = CancellableTask::new();
    let task_ref = &task;
    
    let work_task = tokio::spawn(async move {
        task_ref.run_with_cleanup().await
    });
    
    // Let the task run for a bit
    time::sleep(Duration::from_millis(250)).await;
    
    // Cancel the task
    task.cancel();
    
    let result = work_task.await.unwrap();
    assert!(matches!(result, Err(TaskError::Cancelled)));
}

#[tokio::test]
async fn test_task_completion_without_cancellation() {
    let task = CancellableTask::new();
    
    let result = task.run_with_cleanup().await;
    assert!(matches!(result, Ok(_)));
}

#[tokio::test]
async fn test_graceful_shutdown() {
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
    let (completion_tx, completion_rx) = oneshot::channel();
    
    // Long-running task
    let worker = tokio::spawn(async move {
        let mut counter = 0;
        
        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    println!("Shutdown signal received, cleaning up...");
                    time::sleep(Duration::from_millis(100)).await; // Cleanup time
                    completion_tx.send(counter).unwrap();
                    break;
                }
                _ = time::sleep(Duration::from_millis(50)) => {
                    counter += 1;
                    if counter >= 20 {
                        completion_tx.send(counter).unwrap();
                        break; // Natural completion
                    }
                }
            }
        }
    });
    
    // Let it run for a bit
    time::sleep(Duration::from_millis(200)).await;
    
    // Send shutdown signal
    shutdown_tx.send(()).unwrap();
    
    // Wait for graceful completion
    let final_count = completion_rx.await.unwrap();
    worker.await.unwrap();
    
    // Should have processed some items but not all
    assert!(final_count > 0);
    assert!(final_count < 20);
}
```

## Error Handling in Async Tests

### Testing Error Propagation

```rust
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    #[error("Validation error: {0}")]
    Validation(String),
}

pub async fn complex_operation(
    input: &str,
) -> Result<String, ServiceError> {
    // Validate input
    if input.is_empty() {
        return Err(ServiceError::Validation("Input cannot be empty".to_string()));
    }
    
    // Fetch data from network
    let network_data = fetch_from_network(input).await?;
    
    // Save to database
    save_to_database(&network_data).await?;
    
    Ok(format!("Processed: {}", network_data))
}

#[tokio::test]
async fn test_error_propagation_chain() {
    // Test validation error
    let result = complex_operation("").await;
    assert!(matches!(result, Err(ServiceError::Validation(_))));
    
    // Test network error propagation
    // (Would need to mock network call to return error)
    
    // Test database error propagation  
    // (Would need to mock database to return error)
}

#[tokio::test]
async fn test_error_context_preservation() {
    use std::error::Error;
    
    let result = complex_operation("").await;
    
    match result {
        Err(ServiceError::Validation(msg)) => {
            assert_eq!(msg, "Input cannot be empty");
        }
        _ => panic!("Expected validation error"),
    }
}
```

## Best Practices for Async Testing

### 1. Time Management
- ✅ Use `tokio::time::pause()` for deterministic testing
- ✅ Test timeout behavior explicitly
- ✅ Use `time::advance()` to control time flow
- ❌ Don't rely on wall-clock time in tests

### 2. Concurrency Testing
- ✅ Test race conditions and thread safety
- ✅ Use proper synchronization primitives
- ✅ Test backpressure and flow control
- ❌ Don't assume execution order without synchronization

### 3. Resource Management
- ✅ Test cancellation and cleanup
- ✅ Use timeout wrappers for long-running operations
- ✅ Verify proper resource cleanup
- ❌ Don't leak resources in test failures

### 4. Error Handling
- ✅ Test all async error paths
- ✅ Verify error propagation chains
- ✅ Test partial failures in batch operations
- ❌ Don't ignore async errors

### 5. Mock Configuration
- ✅ Use async-compatible mocking tools
- ✅ Test both success and failure scenarios
- ✅ Verify call ordering and timing
- ❌ Don't forget to test async mock behavior

## Common Async Testing Pitfalls

### 1. Forgetting to await
```rust
// BAD: Test may pass incorrectly
#[tokio::test]
async fn test_forgets_await() {
    let future = async_function(); // Not awaited!
    // Test ends before future completes
}

// GOOD: Properly await the result
#[tokio::test]
async fn test_with_await() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### 2. Deadlocks in tests
```rust
// BAD: Potential deadlock
#[tokio::test]
async fn test_potential_deadlock() {
    let mutex = Arc::new(Mutex::new(0));
    let mutex_clone = mutex.clone();
    
    let guard1 = mutex.lock().await;
    
    let task = tokio::spawn(async move {
        let _guard2 = mutex_clone.lock().await; // May deadlock
    });
    
    // guard1 still held here
    task.await.unwrap();
}

// GOOD: Proper guard management
#[tokio::test] 
async fn test_proper_guard_management() {
    let mutex = Arc::new(Mutex::new(0));
    let mutex_clone = mutex.clone();
    
    {
        let _guard1 = mutex.lock().await;
        // Use guard1 here
    } // guard1 dropped
    
    let task = tokio::spawn(async move {
        let _guard2 = mutex_clone.lock().await; // Safe
    });
    
    task.await.unwrap();
}
```

### 3. Non-deterministic timing
```rust
// BAD: Relies on real time
#[tokio::test]
async fn test_with_real_time() {
    let start = std::time::Instant::now();
    time::sleep(Duration::from_millis(100)).await;
    assert!(start.elapsed() >= Duration::from_millis(100)); // Flaky
}

// GOOD: Use controlled time
#[tokio::test]
async fn test_with_controlled_time() {
    time::pause();
    
    let start = time::Instant::now();
    let sleep_task = tokio::spawn(async {
        time::sleep(Duration::from_millis(100)).await;
    });
    
    time::advance(Duration::from_millis(100)).await;
    sleep_task.await.unwrap();
    
    assert!(start.elapsed() >= Duration::from_millis(100));
}
```

## Conclusion

Effective async testing in Rust requires:

1. **Understanding async runtimes** and their testing implications
2. **Using time control** for deterministic tests
3. **Testing concurrency patterns** thoroughly
4. **Proper error handling** in async contexts
5. **Appropriate mocking strategies** for async dependencies
6. **Avoiding common pitfalls** like deadlocks and timing issues

With these patterns and practices, you can confidently test complex async Rust applications and ensure they behave correctly under various conditions.