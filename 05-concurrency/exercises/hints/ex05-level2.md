# Exercise 05 Hints - Level 2: Specific Guidance 🟡

## 🎯 Specific Tokio Fixes

Here are the exact changes you need to make:

## ✅ Fix 1: Add Missing Imports

```rust
// Add these imports at the top of your file:
use std::time::Duration;
use tokio::time::sleep;  // ← This is the async sleep!
```

## ✅ Fix 2: Convert Blocking Sleep to Async Sleep

```rust
// ❌ WRONG: Blocks the thread
std::thread::sleep(Duration::from_secs(1));

// ✅ FIXED: Async sleep that yields
tokio::time::sleep(Duration::from_secs(1)).await;
```

**Apply this pattern everywhere you see `std::thread::sleep`!**

## ✅ Fix 3: Fix Async Main Function

```rust
// ❌ WRONG: Can't await in sync main
fn main() {
    let result = fetch_data("url");  // Missing await
}

// ✅ FIXED: Async main with tokio
#[tokio::main]
async fn main() {
    let result = fetch_data("url").await;  // Now we can await!
}
```

## ✅ Fix 4: Fix Sequential vs Concurrent Execution

```rust
// ❌ SEQUENTIAL: Waits for each one to finish
for url in urls {
    let result = fetch_data(url).await;  // One at a time
    results.push(result);
}

// ✅ CONCURRENT: All execute simultaneously
let futures: Vec<_> = urls.into_iter()
    .map(|url| fetch_data(url))  // Create futures (don't await yet!)
    .collect();

let results = futures::future::join_all(futures).await;  // Await all at once
```

## ✅ Fix 5: Fix Tokio Task Spawning

```rust
// ❌ WRONG: Ownership issues
for (i, item) in shared_data.iter().enumerate() {
    let handle = tokio::spawn(async {
        println!("Processing: {}", item); // ERROR: item not owned
    });
}

// ✅ FIXED: Clone data into task
for (i, item) in shared_data.iter().enumerate() {
    let item_clone = item.clone();  // Clone for each task
    let handle = tokio::spawn(async move {  // Move the clone
        println!("Processing: {}", item_clone);
    });
}
```

## ✅ Fix 6: Fix Task Join with Error Handling

```rust
// ❌ WRONG: Missing await and error handling
let result = handle.join();
println!("Task result: {}", result);

// ✅ FIXED: Await and handle JoinError
match handle.join().await {
    Ok(result) => println!("Task result: {}", result),
    Err(e) => println!("Task panicked: {:?}", e),
}
```

## ✅ Fix 7: Fix Cancellation Token (Advanced)

```rust
// Add this import:
use tokio_util::sync::CancellationToken;

// Replace unimplemented! with:
let cancel_token = CancellationToken::new();

// In the long-running task, check for cancellation:
for i in 1..=10 {
    if cancel_token_clone.is_cancelled() {
        println!("Task cancelled at iteration {}", i);
        break;
    }
    println!("Long task iteration: {}", i);
    tokio::time::sleep(Duration::from_millis(500)).await;
}
```

## 🎯 C# Async/Await Comparison

| **Pattern** | **C#** | **Rust** |
|-------------|---------|----------|
| **Async function** | `async Task<T>` | `async fn -> T` |
| **Main function** | `async Task Main()` | `#[tokio::main] async fn main()` |
| **Delay** | `await Task.Delay(1000)` | `tokio::time::sleep(Duration::from_secs(1)).await` |
| **Spawn task** | `Task.Run(() => work())` | `tokio::spawn(async { work().await })` |
| **Wait for all** | `await Task.WhenAll(tasks)` | `join_all(futures).await` |
| **Task result** | `await task` | `task.await.unwrap()` |

## 🎯 Common Patterns to Apply

### Pattern 1: Basic Async Function
```rust
async fn my_async_function() -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "result".to_string()
}
```

### Pattern 2: Error Handling in Async
```rust
async fn with_error_handling() -> Result<String, String> {
    if some_condition {
        return Err("error message".to_string());
    }
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok("success".to_string())
}
```

### Pattern 3: Spawning Multiple Tasks
```rust
let mut handles = Vec::new();
for i in 0..5 {
    let handle = tokio::spawn(async move {
        // Work for each task
        i * 2
    });
    handles.push(handle);
}

// Collect results
for handle in handles {
    let result = handle.await.unwrap();
    println!("Result: {}", result);
}
```

## 🎯 Step-by-Step Checkpoint Guide

1. **Add imports** → `use tokio::time::sleep;`
2. **Fix sleeps** → `tokio::time::sleep(...).await`
3. **Fix main** → `#[tokio::main] async fn main()`
4. **Add awaits** → `.await` on all async function calls
5. **Fix ownership** → Clone data before moving into tasks
6. **Handle errors** → `match handle.join().await`

Need the complete working solutions? Check Level 3 hints!