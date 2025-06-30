# Lesson 03: Async Programming with Tokio

Rust's async/await is similar to C#'s but with some key differences. Unlike C# where async is built into the runtime, Rust provides async as a language feature with pluggable runtimes. Tokio is the most popular async runtime for building scalable network applications.

## 🎯 Learning Objectives

- Master async/await syntax and concepts in Rust
- Understand the differences from C# async/await
- Learn Tokio runtime and ecosystem
- Build high-performance async applications
- Handle async error patterns and cancellation

## 🔄 C# vs Rust Async Comparison

### C# Async/Await Model
```csharp
// C# - Built-in Task-based async
public async Task<string> FetchDataAsync(string url)
{
    using var client = new HttpClient();
    var response = await client.GetAsync(url);
    response.EnsureSuccessStatusCode();
    return await response.Content.ReadAsStringAsync();
}

public async Task ProcessMultipleUrlsAsync()
{
    var urls = new[] { "http://api1.com", "http://api2.com" };
    var tasks = urls.Select(FetchDataAsync).ToArray();
    var results = await Task.WhenAll(tasks);
    
    foreach (var result in results)
    {
        Console.WriteLine($"Result: {result}");
    }
}
```

### Rust Async/Await Model
```rust
use tokio;

// Rust - Runtime-agnostic async with Tokio
async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

async fn process_multiple_urls() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec!["http://api1.com", "http://api2.com"];
    
    // Concurrent execution
    let futures: Vec<_> = urls.iter().map(|url| fetch_data(url)).collect();
    let results = futures::future::try_join_all(futures).await?;
    
    for result in results {
        println!("Result: {}", result);
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_multiple_urls().await
}
```

## 🏃 Getting Started with Tokio

### Basic Tokio Setup

First, add Tokio to your `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
```

### Simple Async Examples

```rust
use tokio::time::{sleep, Duration, Instant};
use std::future::Future;

// Basic async function
async fn say_hello() {
    println!("Hello");
    sleep(Duration::from_millis(100)).await;
    println!("World");
}

// Async function with return value
async fn calculate_async(x: i32, y: i32) -> i32 {
    // Simulate async work
    sleep(Duration::from_millis(10)).await;
    x + y
}

// Using async blocks
async fn async_blocks_demo() {
    let future1 = async {
        sleep(Duration::from_millis(100)).await;
        "First result"
    };
    
    let future2 = async {
        sleep(Duration::from_millis(200)).await;
        "Second result"
    };
    
    // Execute concurrently
    let (result1, result2) = tokio::join!(future1, future2);
    println!("Results: {}, {}", result1, result2);
}

#[tokio::main]
async fn main() {
    say_hello().await;
    
    let result = calculate_async(5, 3).await;
    println!("Calculation result: {}", result);
    
    async_blocks_demo().await;
}
```

### Understanding Futures

```rust
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use tokio::time::{sleep, Duration};

// Manual Future implementation (educational purposes)
struct DelayedValue {
    value: i32,
    delay: Duration,
    started: Option<Instant>,
}

impl DelayedValue {
    fn new(value: i32, delay: Duration) -> Self {
        Self {
            value,
            delay,
            started: None,
        }
    }
}

impl Future for DelayedValue {
    type Output = i32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.started.is_none() {
            self.started = Some(Instant::now());
        }
        
        if self.started.unwrap().elapsed() >= self.delay {
            Poll::Ready(self.value)
        } else {
            // Schedule a wake-up
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn custom_future_demo() {
    let delayed = DelayedValue::new(42, Duration::from_millis(100));
    let result = delayed.await;
    println!("Delayed result: {}", result);
}
```

## 🌐 Async I/O with Tokio

### Network Programming

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use std::error::Error;

// Async TCP server
async fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        
        // Spawn a new task for each connection
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    
    loop {
        let n = socket.read(&mut buffer).await?;
        
        if n == 0 {
            // Connection closed
            break;
        }
        
        // Echo the data back
        socket.write_all(&buffer[0..n]).await?;
    }
    
    Ok(())
}

// Async HTTP client
async fn http_client_demo() -> Result<(), Box<dyn Error>> {
    let response = reqwest::get("https://httpbin.org/json").await?;
    let status = response.status();
    let text = response.text().await?;
    
    println!("Status: {}", status);
    println!("Body: {}", text);
    
    Ok(())
}

// File I/O
async fn file_operations() -> Result<(), Box<dyn Error>> {
    use tokio::fs;
    
    // Write to file
    fs::write("async_test.txt", "Hello, async world!").await?;
    
    // Read from file
    let content = fs::read_to_string("async_test.txt").await?;
    println!("File content: {}", content);
    
    // Read file line by line
    let file = fs::File::open("async_test.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await? {
        println!("Line: {}", line);
    }
    
    // Clean up
    fs::remove_file("async_test.txt").await?;
    
    Ok(())
}
```

### Concurrent Task Management

```rust
use tokio::task::{JoinHandle, JoinError};
use tokio::time::{timeout, Duration};
use futures::future::{select, Either};

// Spawning and managing tasks
async fn task_management_demo() -> Result<(), Box<dyn Error>> {
    // Spawn background tasks
    let task1: JoinHandle<i32> = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        42
    });
    
    let task2: JoinHandle<String> = tokio::spawn(async {
        sleep(Duration::from_millis(200)).await;
        "Hello".to_string()
    });
    
    // Wait for both tasks
    let (result1, result2) = tokio::try_join!(task1, task2)?;
    println!("Task results: {}, {}", result1, result2);
    
    // Race between tasks (first to complete wins)
    let fast_task = async {
        sleep(Duration::from_millis(50)).await;
        "Fast"
    };
    
    let slow_task = async {
        sleep(Duration::from_millis(200)).await;
        "Slow"
    };
    
    match select(fast_task, slow_task).await {
        Either::Left((result, _)) => println!("Fast task won: {}", result),
        Either::Right((result, _)) => println!("Slow task won: {}", result),
    }
    
    Ok(())
}

// Timeout and cancellation
async fn timeout_demo() -> Result<(), Box<dyn Error>> {
    let slow_operation = async {
        sleep(Duration::from_secs(5)).await;
        "Finally done!"
    };
    
    match timeout(Duration::from_secs(1), slow_operation).await {
        Ok(result) => println!("Operation completed: {}", result),
        Err(_) => println!("Operation timed out!"),
    }
    
    Ok(())
}
```

## 🔄 Async Channels and Communication

```rust
use tokio::sync::{mpsc, oneshot, broadcast};
use tokio::time::{interval, Duration};

// Multi-producer, single-consumer channel
async fn mpsc_demo() {
    let (tx, mut rx) = mpsc::channel::<String>(100);
    
    // Spawn multiple producers
    for i in 0..3 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            for j in 0..5 {
                let message = format!("Producer {} - Message {}", i, j);
                if tx_clone.send(message).await.is_err() {
                    break;
                }
                sleep(Duration::from_millis(100)).await;
            }
        });
    }
    
    // Drop the original sender
    drop(tx);
    
    // Receive all messages
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}

// One-shot channel for single-value communication
async fn oneshot_demo() {
    let (tx, rx) = oneshot::channel::<i32>();
    
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        let _ = tx.send(42);
    });
    
    match rx.await {
        Ok(value) => println!("Received value: {}", value),
        Err(_) => println!("Sender dropped"),
    }
}

// Broadcast channel for one-to-many communication
async fn broadcast_demo() {
    let (tx, _) = broadcast::channel::<String>(16);
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    
    // Sender task
    let sender_tx = tx.clone();
    tokio::spawn(async move {
        for i in 0..5 {
            let message = format!("Broadcast message {}", i);
            let _ = sender_tx.send(message);
            sleep(Duration::from_millis(100)).await;
        }
    });
    
    // Receiver tasks
    tokio::spawn(async move {
        while let Ok(message) = rx1.recv().await {
            println!("Receiver 1: {}", message);
        }
    });
    
    tokio::spawn(async move {
        while let Ok(message) = rx2.recv().await {
            println!("Receiver 2: {}", message);
        }
    });
    
    sleep(Duration::from_millis(600)).await;
}
```

## 🌊 Streams and Async Iteration

```rust
use futures::stream::{self, Stream, StreamExt};
use tokio::time::{interval, Duration};

// Working with async streams
async fn streams_demo() {
    // Create a stream from a vector
    let numbers = stream::iter(vec![1, 2, 3, 4, 5]);
    
    // Transform and collect
    let doubled: Vec<i32> = numbers
        .map(|x| x * 2)
        .collect()
        .await;
    
    println!("Doubled numbers: {:?}", doubled);
    
    // Create a timed stream
    let mut timer = interval(Duration::from_millis(100));
    let mut count = 0;
    
    loop {
        timer.tick().await;
        count += 1;
        println!("Timer tick: {}", count);
        
        if count >= 5 {
            break;
        }
    }
}

// Custom async stream
use std::pin::Pin;
use std::task::{Context, Poll};

struct NumberStream {
    current: i32,
    max: i32,
}

impl NumberStream {
    fn new(max: i32) -> Self {
        Self { current: 0, max }
    }
}

impl Stream for NumberStream {
    type Item = i32;
    
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Poll::Ready(Some(current))
        } else {
            Poll::Ready(None)
        }
    }
}

async fn custom_stream_demo() {
    let stream = NumberStream::new(5);
    
    stream.for_each(|number| async move {
        println!("Stream number: {}", number);
    }).await;
}
```

## 🛡️ Error Handling in Async Code

```rust
use thiserror::Error;
use tokio::time::{timeout, Duration};

#[derive(Error, Debug)]
enum AsyncError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Timeout occurred")]
    Timeout,
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

type AsyncResult<T> = Result<T, AsyncError>;

// Async function with comprehensive error handling
async fn fetch_and_parse_data(url: &str) -> AsyncResult<serde_json::Value> {
    // Add timeout to prevent hanging
    let response = timeout(Duration::from_secs(10), async {
        reqwest::get(url)
            .await
            .map_err(|e| AsyncError::Network(e.to_string()))?
            .text()
            .await
            .map_err(|e| AsyncError::Network(e.to_string()))
    })
    .await
    .map_err(|_| AsyncError::Timeout)??;
    
    // Parse JSON
    let parsed: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| AsyncError::Parse(e.to_string()))?;
    
    Ok(parsed)
}

// Error propagation with async
async fn process_multiple_apis() -> AsyncResult<Vec<serde_json::Value>> {
    let urls = vec![
        "https://httpbin.org/json",
        "https://api.github.com/users/octocat",
    ];
    
    let mut results = Vec::new();
    
    for url in urls {
        match fetch_and_parse_data(url).await {
            Ok(data) => results.push(data),
            Err(e) => {
                eprintln!("Failed to fetch {}: {}", url, e);
                // Continue with other URLs
            }
        }
    }
    
    if results.is_empty() {
        return Err(AsyncError::Network("All requests failed".to_string()));
    }
    
    Ok(results)
}

// Graceful shutdown pattern
use tokio::signal;
use tokio::sync::broadcast;

async fn graceful_shutdown_demo() -> Result<(), Box<dyn Error>> {
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let shutdown_rx = shutdown_tx.subscribe();
    
    // Background worker task
    let worker_shutdown = shutdown_tx.subscribe();
    let worker_handle = tokio::spawn(async move {
        let mut shutdown = worker_shutdown;
        let mut interval = interval(Duration::from_secs(1));
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    println!("Worker doing work...");
                }
                _ = shutdown.recv() => {
                    println!("Worker shutting down gracefully");
                    break;
                }
            }
        }
    });
    
    // Wait for CTRL+C
    signal::ctrl_c().await?;
    println!("Received shutdown signal");
    
    // Send shutdown signal
    let _ = shutdown_tx.send(());
    
    // Wait for worker to finish
    worker_handle.await?;
    
    println!("Shutdown complete");
    Ok(())
}
```

## 🏗️ Real-World Example: Async Web Scraper

```rust
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::time::{sleep, Duration};
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ScrapedData {
    url: String,
    title: String,
    links: Vec<String>,
    word_count: usize,
}

#[derive(Debug)]
struct WebScraper {
    client: Client,
    max_concurrent: usize,
    delay_between_requests: Duration,
}

impl WebScraper {
    fn new(max_concurrent: usize, delay_ms: u64) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            max_concurrent,
            delay_between_requests: Duration::from_millis(delay_ms),
        }
    }
    
    async fn scrape_url(&self, url: &str) -> Result<ScrapedData, Box<dyn Error + Send + Sync>> {
        // Rate limiting
        sleep(self.delay_between_requests).await;
        
        println!("Scraping: {}", url);
        
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        
        self.parse_html(url, &html).await
    }
    
    async fn parse_html(&self, url: &str, html: &str) -> Result<ScrapedData, Box<dyn Error + Send + Sync>> {
        let document = Html::parse_document(html);
        
        // Extract title
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_else(|| "No title".to_string());
        
        // Extract links
        let link_selector = Selector::parse("a[href]").unwrap();
        let links: Vec<String> = document
            .select(&link_selector)
            .filter_map(|el| el.value().attr("href"))
            .map(|href| href.to_string())
            .collect();
        
        // Count words
        let text_content = document.root_element().text().collect::<String>();
        let word_count = text_content.split_whitespace().count();
        
        Ok(ScrapedData {
            url: url.to_string(),
            title,
            links,
            word_count,
        })
    }
    
    async fn scrape_multiple(&self, urls: Vec<String>) -> Vec<Result<ScrapedData, Box<dyn Error + Send + Sync>>> {
        let mut futures = FuturesUnordered::new();
        let mut results = Vec::new();
        let mut pending = 0;
        
        let mut urls_iter = urls.into_iter();
        
        // Start initial batch
        for _ in 0..self.max_concurrent {
            if let Some(url) = urls_iter.next() {
                futures.push(self.scrape_url(&url));
                pending += 1;
            }
        }
        
        // Process results and start new requests
        while pending > 0 {
            if let Some(result) = futures.next().await {
                results.push(result);
                pending -= 1;
                
                // Start next request if available
                if let Some(url) = urls_iter.next() {
                    futures.push(self.scrape_url(&url));
                    pending += 1;
                }
            }
        }
        
        results
    }
}

// Usage example
async fn web_scraper_demo() -> Result<(), Box<dyn Error>> {
    let scraper = WebScraper::new(3, 100); // Max 3 concurrent, 100ms delay
    
    let urls = vec![
        "https://example.com".to_string(),
        "https://httpbin.org".to_string(),
        "https://rust-lang.org".to_string(),
    ];
    
    let results = scraper.scrape_multiple(urls).await;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(data) => {
                println!("Result {}: {} (title: {}, {} words, {} links)", 
                         i, data.url, data.title, data.word_count, data.links.len());
            }
            Err(e) => {
                println!("Error {}: {}", i, e);
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Run all the demos
    println!("=== Task Management Demo ===");
    task_management_demo().await?;
    
    println!("\n=== Timeout Demo ===");
    timeout_demo().await?;
    
    println!("\n=== MPSC Demo ===");
    mpsc_demo().await;
    
    println!("\n=== Oneshot Demo ===");
    oneshot_demo().await;
    
    println!("\n=== Broadcast Demo ===");
    broadcast_demo().await;
    
    println!("\n=== Streams Demo ===");
    streams_demo().await;
    
    println!("\n=== Custom Stream Demo ===");
    custom_stream_demo().await;
    
    println!("\n=== Web Scraper Demo ===");
    // web_scraper_demo().await?;  // Uncomment to run actual scraping
    
    Ok(())
}
```

## 🎯 Key Takeaways

1. **Runtime Choice**: Unlike C#, Rust async requires choosing a runtime (Tokio is most popular)
2. **Zero-Cost**: Rust async has no runtime overhead when not used
3. **Explicit**: Async functions must be explicitly awaited
4. **Cancellation**: Use `select!` and channels for cancellation patterns
5. **Error Handling**: Combine `Result` types with async for robust error handling

## 💻 Exercises

### Exercise 1: Async File Processor
```rust
// TODO: Create an async file processing pipeline
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};

struct AsyncFileProcessor {
    // TODO: Add fields
}

impl AsyncFileProcessor {
    async fn process_files(&self, file_paths: Vec<String>) -> Result<ProcessingResult, Box<dyn Error>> {
        // TODO: Process multiple files concurrently
        // Count lines, words, and characters
        // Implement proper error handling and progress reporting
        todo!()
    }
    
    async fn process_single_file(&self, path: &str) -> Result<FileStats, Box<dyn Error>> {
        // TODO: Process a single file asynchronously
        todo!()
    }
}

struct ProcessingResult {
    total_files: usize,
    successful: usize,
    total_lines: usize,
    total_words: usize,
}

struct FileStats {
    path: String,
    lines: usize,
    words: usize,
    chars: usize,
}
```

### Exercise 2: Async Rate Limiter
```rust
// TODO: Implement an async rate limiter
use tokio::time::{Duration, Instant};

struct RateLimiter {
    // TODO: Add fields for rate limiting
}

impl RateLimiter {
    fn new(max_requests: u32, window: Duration) -> Self {
        todo!()
    }
    
    async fn acquire(&self) -> RateLimitPermit {
        // TODO: Wait if necessary to respect rate limits
        todo!()
    }
    
    async fn try_acquire(&self) -> Option<RateLimitPermit> {
        // TODO: Return permit if available, None if rate limited
        todo!()
    }
}

struct RateLimitPermit {
    // TODO: Add permit data
}
```

### Exercise 3: Async Worker Pool
```rust
// TODO: Create an async worker pool for job processing
use tokio::sync::mpsc;

struct AsyncWorkerPool<T, R> {
    // TODO: Add fields
}

impl<T, R> AsyncWorkerPool<T, R> 
where 
    T: Send + 'static,
    R: Send + 'static,
{
    async fn new<F, Fut>(worker_count: usize, job_handler: F) -> Self 
    where 
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = R> + Send,
    {
        // TODO: Create worker pool with async job processing
        todo!()
    }
    
    async fn submit_job(&self, job: T) -> Result<R, Box<dyn Error>> {
        // TODO: Submit job and wait for result
        todo!()
    }
    
    async fn shutdown(self) {
        // TODO: Gracefully shutdown all workers
        todo!()
    }
}
```

## 📚 Additional Resources

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Official Tokio guide
- [Async Book](https://rust-lang.github.io/async-book/) - Comprehensive async programming guide
- [reqwest](https://docs.rs/reqwest/) - HTTP client for async applications
- [futures](https://docs.rs/futures/) - Additional async utilities

---

Next: [Performance Optimization](../06-performance/README.md) →
