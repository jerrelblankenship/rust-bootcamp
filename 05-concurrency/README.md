# Module 05: Concurrency and Parallelism

Master Rust's powerful concurrency features, from threads to async/await, and build high-performance concurrent applications.

## 🎯 Learning Objectives

After completing this module, you will:
- Understand Rust's threading model and how it prevents data races
- Master message passing with channels
- Safely share state between threads with Arc and Mutex
- Write async code with Tokio
- Compare Rust's concurrency with C#'s Task and async/await
- Build scalable concurrent applications

## 📚 Module Overview

Rust's ownership system makes concurrent programming safer than in most languages. The compiler prevents data races at compile time!

## 📖 Lessons

1. **[Threads](01-threads.md)** - Creating and managing threads
2. **[Message Passing](02-message-passing.md)** - Channels for thread communication
3. **[Shared State](03-shared-state.md)** - Mutex, RwLock, and Arc
4. **[Async/Await](04-async-await.md)** - Asynchronous programming with Tokio

## 💻 Project: Parallel Data Processor

Build a multi-threaded data processing pipeline that:
- Processes large datasets in parallel
- Uses channels for work distribution
- Implements thread pools
- Includes async I/O operations
- Demonstrates performance improvements

## 🔄 C# to Rust Concurrency

| C# Concept | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| `Task` | `Future` | Lazy evaluation |
| `async/await` | `async/await` | No runtime by default |
| `lock` statement | `Mutex::lock()` | RAII-based |
| `Thread` | `std::thread` | Ownership-based |
| `ConcurrentQueue` | `mpsc::channel` | Message passing preferred |
| `Parallel.For` | Rayon crate | Data parallelism |

## 🏃 Quick Example

### C# Concurrent Collection
```csharp
var dict = new ConcurrentDictionary<int, string>();
Parallel.For(0, 1000, i => {
    dict.TryAdd(i, $"Value {i}");
});
```

### Rust Concurrent Access
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(HashMap::new()));
let handles: Vec<_> = (0..1000).map(|i| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let mut map = data.lock().unwrap();
        map.insert(i, format!("Value {}", i));
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

## 🚀 Key Concepts

1. **Fearless Concurrency**: Compiler prevents data races
2. **Send and Sync**: Traits that define thread safety
3. **No Shared Mutable State**: Use channels or synchronization
4. **Zero-Cost Abstractions**: High-level APIs, low-level performance

## 📊 Module Structure

```
05-concurrency/
├── README.md
├── 01-threads.md
├── 02-message-passing.md
├── 03-shared-state.md
├── 04-async-await.md
├── exercises/
│   ├── ex01-basic-threads.rs
│   ├── ex02-channels.rs
│   ├── ex03-mutex-practice.rs
│   └── ex04-async-tasks.rs
└── project-parallel-processor/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── worker.rs
    │   ├── pipeline.rs
    │   └── async_io.rs
    └── README.md
```

---

Ready to write concurrent code without fear? Let's dive in!
