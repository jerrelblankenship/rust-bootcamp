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

### C# Async
```csharp
public async Task<string> FetchDataAsync() {
    using var client = new HttpClient();
    var response = await client.GetAsync("https://api.example.com");
    return await response.Content.ReadAsStringAsync();
}
```

### Rust Async
```rust
use reqwest;

async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.example.com").await?;
    let text = response.text().await?;
    Ok(text)
}
```

## Key Differences

| Aspect | C# | Rust |
|--------|-----|------|
| **Memory Safety** | GC prevents most issues | Compiler prevents data races |
| **Performance** | Runtime overhead | Zero-cost abstractions |
| **Error Handling** | Exceptions | Result types |
| **Cancellation** | CancellationToken | Drop the future |
| **Synchronization** | Many built-in types | Explicit with Arc/Mutex |
| **Runtime** | Built-in runtime | Choose your runtime (Tokio) |