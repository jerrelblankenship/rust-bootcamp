# Web Scraper Project - Level 1 Hints 🟢

## Understanding the Concurrent Web Scraper Architecture

You're building a multi-threaded web scraper that demonstrates all the concurrency concepts from the exercises.

### 🎯 What You're Building

A web scraper that:
- **Spawns worker threads** to fetch URLs concurrently
- **Uses channels** for work distribution and result collection  
- **Shares state** with Arc/Mutex for progress tracking
- **Handles errors** gracefully across threads
- **Implements async I/O** with Tokio for HTTP requests

### 🏗️ High-Level Architecture

Think of it like a restaurant kitchen (from ex02):

```
Main Thread (Head Chef)
    ↓ dishes channels
Worker Threads (Line Cooks) → Results Channel → Result Collector
    ↑ shared state
Progress Tracker (Arc<Mutex<Stats>>)
```

### 🔄 C# Mental Model

Similar to C# parallel processing:

```csharp
// C# approach
var tasks = urls.Select(async url => {
    var result = await httpClient.GetAsync(url);
    return await ProcessResponse(result);
}).ToArray();

await Task.WhenAll(tasks);
```

```rust
// Rust approach (what you're building)
let handles: Vec<_> = (0..num_workers)
    .map(|_| {
        let rx = work_receiver.clone();
        let tx = result_sender.clone();
        thread::spawn(move || worker_loop(rx, tx))
    })
    .collect();
```

### 🧩 Key Components to Implement

1. **URL Queue Management**
   - Channel for distributing work to threads
   - Proper shutdown signaling when done

2. **Worker Thread Pool**
   - Spawn multiple threads for parallel fetching
   - Each thread processes URLs from the queue

3. **Result Collection**
   - Channel for collecting results from workers
   - Aggregate statistics and success/failure counts

4. **Shared Progress Tracking**
   - Arc<Mutex<Stats>> for thread-safe progress updates
   - Real-time progress reporting

5. **Error Handling**
   - Network timeouts and connection failures
   - Invalid URLs and HTTP errors
   - Thread panic recovery

### 🚧 Common Architecture Pitfalls

1. **Forgetting to close channels** - Workers wait forever
2. **Not handling thread panics** - Silent failures
3. **Race conditions in stats** - Inconsistent progress
4. **Blocking the main thread** - UI freezes
5. **Resource leaks** - Too many connections

### 💭 Design Questions to Consider

1. **How many worker threads?** Consider CPU cores and I/O bound work
2. **How to signal completion?** When all URLs are processed
3. **What to do with results?** Store, display, or save to file
4. **How to handle failures?** Retry, skip, or abort
5. **How to track progress?** Count completed, failed, remaining

### 🎯 Your First Steps

1. **Start with the data structures** - What needs to be shared?
2. **Design the channel patterns** - How does work flow?
3. **Implement a single worker** - Get one thread working first
4. **Add result collection** - How do results come back?
5. **Scale to multiple workers** - Then add more threads

### 🔍 Debugging Strategy

1. **Add lots of println!** - See what each thread is doing
2. **Start with one thread** - Simpler to debug
3. **Use thread names** - Easier to track which thread has issues
4. **Test with simple URLs** - httpbin.org, example.com
5. **Handle panics gracefully** - Don't let one URL crash everything

### 📚 Concepts from Exercises Applied

- **ex01 (Threading)**: Spawning worker threads with move semantics
- **ex02 (Channels)**: MPSC for work distribution and result collection  
- **ex03 (Shared State)**: Arc/Mutex for progress tracking
- **ex04 (Deadlocks)**: Proper shutdown without deadlocks
- **ex05 (Async)**: HTTP requests with async/await

Don't try to implement everything at once - start with basic threading and channels!

Need specific code patterns? Check Level 2 hints!