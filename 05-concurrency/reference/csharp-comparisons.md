# C# to Rust Concurrency: Complete Comparison 🔄

## Threading Models

### C# Threading
```csharp
// Thread creation
var thread = new Thread(() => {
    Console.WriteLine("Hello from thread!");
});
thread.Start();
thread.Join();

// Task-based
var task = Task.Run(() => {
    return DoWork();
});
var result = await task;
```

### Rust Threading
```rust
// Thread creation
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from thread!");
});
handle.join().unwrap();

// Future-based
use tokio;

let handle = tokio::spawn(async {
    do_work().await
});
let result = handle.await.unwrap();
```

## Shared State Patterns

### C# Lock Statement
```csharp
private readonly object _lock = new object();
private int _counter = 0;

public void Increment() {
    lock (_lock) {
        _counter++;
    }
}
```

### Rust Mutex
```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));

fn increment(counter: Arc<Mutex<i32>>) {
    let mut num = counter.lock().unwrap();
    *num += 1;
}
```

## Channel Communication

### C# Channel
```csharp
var channel = Channel.CreateUnbounded<string>();
var writer = channel.Writer;
var reader = channel.Reader;

// Producer
await writer.WriteAsync("message");

// Consumer
var message = await reader.ReadAsync();
```

### Rust Channel
```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

// Producer
tx.send("message").unwrap();

// Consumer
let message = rx.recv().unwrap();
```

## Async/Await Patterns

### Basic Async Functions

#### C# Async Methods
```csharp
// Simple async method
public async Task<string> FetchDataAsync(string url) {
    using var client = new HttpClient();
    var response = await client.GetAsync(url);
    return await response.Content.ReadAsStringAsync();
}

// Fire-and-forget
public async void ProcessInBackground() {  // Usually avoid async void
    await DoBackgroundWork();
}

// CPU-bound with Task.Run
public async Task<int> CalculateAsync(int[] numbers) {
    return await Task.Run(() => numbers.Sum());
}
```

#### Rust Async Functions
```rust
// Simple async function
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    Ok(response.text().await?)
}

// Fire-and-forget with tokio::spawn
fn process_in_background() {
    tokio::spawn(async {
        do_background_work().await;
    });
}

// CPU-bound with spawn_blocking
async fn calculate(numbers: Vec<i32>) -> i32 {
    tokio::task::spawn_blocking(move || {
        numbers.iter().sum()
    }).await.unwrap()
}
```

### Concurrent Execution Patterns

#### C# Task Coordination
```csharp
// Parallel execution with Task.WhenAll
public async Task<string[]> FetchMultipleAsync(string[] urls) {
    var tasks = urls.Select(async url => {
        using var client = new HttpClient();
        var response = await client.GetAsync(url);
        return await response.Content.ReadAsStringAsync();
    });
    
    return await Task.WhenAll(tasks);
}

// Task.WhenAny for first completion
public async Task<string> GetFastestResponseAsync(string[] urls) {
    var tasks = urls.Select(FetchDataAsync);
    var completed = await Task.WhenAny(tasks);
    return await completed;
}

// Cancellation support
public async Task<string> FetchWithTimeoutAsync(string url, TimeSpan timeout) {
    using var cts = new CancellationTokenSource(timeout);
    using var client = new HttpClient();
    
    try {
        var response = await client.GetAsync(url, cts.Token);
        return await response.Content.ReadAsStringAsync();
    }
    catch (OperationCanceledException) {
        throw new TimeoutException("Request timed out");
    }
}
```

#### Rust Async Coordination  
```rust
// Parallel execution with join_all
async fn fetch_multiple(urls: &[&str]) -> Vec<Result<String, reqwest::Error>> {
    let client = reqwest::Client::new();
    
    let futures = urls.iter().map(|&url| {
        let client = client.clone();
        async move {
            let response = client.get(url).send().await?;
            response.text().await
        }
    });
    
    futures::future::join_all(futures).await
}

// select! for first completion
async fn get_fastest_response(urls: &[&str]) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    
    let mut futures = urls.iter().map(|&url| {
        Box::pin(client.get(url).send())
    });
    
    // Use tokio::select! or futures::select!
    tokio::select! {
        result = futures.next().unwrap() => {
            result?.text().await
        }
    }
}

// Timeout with tokio::time::timeout
async fn fetch_with_timeout(url: &str, timeout: Duration) -> Result<String, Box<dyn std::error::Error>> {
    let future = async {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        response.text().await
    };
    
    match tokio::time::timeout(timeout, future).await {
        Ok(result) => result.map_err(Into::into),
        Err(_) => Err("Request timed out".into()),
    }
}
```

## Enterprise Async Patterns Comparison

### Background Services

#### C# Background Service
```csharp
public class DataProcessingService : BackgroundService {
    protected override async Task ExecuteAsync(CancellationToken stoppingToken) {
        while (!stoppingToken.IsCancellationRequested) {
            try {
                await ProcessBatch();
                await Task.Delay(TimeSpan.FromMinutes(5), stoppingToken);
            }
            catch (OperationCanceledException) {
                break;
            }
            catch (Exception ex) {
                // Log error and continue
                await Task.Delay(TimeSpan.FromSeconds(30), stoppingToken);
            }
        }
    }
}
```

#### Rust Background Task
```rust
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

async fn data_processing_service(cancel_token: CancellationToken) {
    while !cancel_token.is_cancelled() {
        tokio::select! {
            _ = cancel_token.cancelled() => break,
            result = process_batch() => {
                match result {
                    Ok(_) => {
                        sleep(Duration::from_secs(300)).await;
                    }
                    Err(e) => {
                        eprintln!("Processing error: {}", e);
                        sleep(Duration::from_secs(30)).await;
                    }
                }
            }
        }
    }
}
```

## Key Differences Summary

| Aspect | C# Tasks | Rust Futures |
|--------|----------|--------------|
| **Syntax** | `async Task<T>` | `async fn -> T` |
| **Runtime** | Built into .NET | Explicit runtime (tokio) |
| **Spawning** | `Task.Run()` | `tokio::spawn()` |
| **Waiting** | `await task` | `task.await` |
| **Cancellation** | `CancellationToken` | `select!` + cancellation |
| **Error Handling** | `try/catch` | `Result<T, E>` + `?` operator |
| **Concurrency** | `Task.WhenAll/WhenAny` | `join_all/select!` |
| **CPU Work** | `Task.Run()` | `spawn_blocking()` |
| **Timeouts** | `CancellationTokenSource` | `tokio::time::timeout` |

## Performance Implications

### C# Async Performance
- **ThreadPool overhead**: Task scheduling overhead
- **GC pressure**: Async state machines create objects
- **SynchronizationContext**: UI thread marshaling overhead
- **ConfigureAwait(false)**: Needed to avoid context capture

### Rust Async Performance  
- **Zero-cost futures**: No allocation until `.await`
- **Compile-time optimization**: State machines optimized by compiler
- **No GC overhead**: No garbage collection pauses
- **Send + Sync**: Explicit thread safety, no hidden overhead

## Migration Strategy

1. **Start with simple async functions** - Direct translation often works
2. **Replace Task.Run with spawn_blocking** - For CPU-bound work
3. **Use select! instead of WhenAny** - More expressive cancellation
4. **Embrace Result types** - Replace exceptions with explicit error handling
5. **Leverage zero-cost futures** - Don't spawn unless necessary

The transition from C# Tasks to Rust Futures offers both better performance and more explicit control over concurrent execution.

