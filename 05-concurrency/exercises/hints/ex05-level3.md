# Exercise 05 Hints - Level 3: Near-Complete Solutions 🔴

## 🎯 Complete Working Solutions

Here are the working implementations for each checkpoint:

## ✅ Complete Import Section

```rust
// Add these imports at the top
use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;  // For cancellation
```

## ✅ Fixed Async Functions

```rust
// Exercise 5.1: Fixed async function
async fn fetch_data(url: &str) -> String {
    println!("Fetching data from: {}", url);
    
    // FIXED: Use tokio async sleep
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    format!("Data from {}", url)
}

// Exercise 5.2: Fixed async function with Result
async fn fetch_with_error(url: &str) -> Result<String, String> {
    if url.contains("error") {
        return Err("Simulated network error".to_string());
    }
    
    // FIXED: Use tokio async sleep
    tokio::time::sleep(Duration::from_millis(500)).await;
    Ok(format!("Success data from {}", url))
}
```

## ✅ Fixed Sequential vs Concurrent Processing

```rust
// Exercise 5.3: Fixed async function calls
async fn process_multiple_urls() -> Vec<String> {
    let urls = vec![
        "https://api.example.com/data1",
        "https://api.example.com/data2", 
        "https://api.example.com/error",
        "https://api.example.com/data3",
    ];
    
    let mut results = Vec::new();
    
    for url in urls {
        // FIXED: Add .await to async function calls
        let result = fetch_data(url).await;
        results.push(result);
        
        // FIXED: Add .await to error handling
        match fetch_with_error(url).await {
            Ok(data) => println!("✅ Success: {}", data),
            Err(e) => println!("❌ Error: {}", e),
        }
    }
    
    results
}
```

## ✅ Fixed Tokio Task Spawning

```rust
// Exercise 5.4: Fixed task spawning with ownership
async fn spawn_with_shared_data() {
    let shared_data = vec!["item1", "item2", "item3", "item4"];
    let mut handles = Vec::new();
    
    for (i, item) in shared_data.iter().enumerate() {
        // FIXED: Clone the item for each task
        let item_clone = item.to_string();
        let handle = tokio::spawn(async move {
            println!("Processing: {}", item_clone);
            tokio::time::sleep(Duration::from_millis(100)).await;
            format!("Processed: {}", item_clone)
        });
        handles.push(handle);
    }
    
    // FIXED: Proper task result collection with error handling
    for handle in handles {
        match handle.await {
            Ok(result) => println!("Task result: {}", result),
            Err(e) => println!("Task failed: {:?}", e),
        }
    }
}
```

## ✅ Fixed Concurrent Tasks with Error Handling

```rust
// Exercise 5.5: Fixed concurrent execution
async fn concurrent_tasks_with_errors() {
    let task_durations = vec![100, 200, 50, 300, 150];
    let mut handles = Vec::new();
    
    for (id, duration) in task_durations.into_iter().enumerate() {
        // FIXED: Move variables into closure properly
        let handle = tokio::spawn(async move {
            background_task(id as u32, duration).await
        });
        handles.push(handle);
    }
    
    // FIXED: Proper error handling for panicked tasks
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => println!("Task {} success: {}", i, result),
            Err(e) => println!("Task {} failed: {:?}", i, e),
        }
    }
}
```

## ✅ Fixed Task Cancellation

```rust
// Exercise 5.6: Fixed cancellation implementation
async fn cancellable_task() {
    // FIXED: Proper cancellation token creation
    let cancel_token = CancellationToken::new();
    let cancel_token_clone = cancel_token.clone();
    
    let long_running_task = tokio::spawn(async move {
        for i in 1..=10 {
            // FIXED: Check for cancellation
            if cancel_token_clone.is_cancelled() {
                println!("Task cancelled at iteration {}", i);
                return "Task was cancelled".to_string();
            }
            
            println!("Long task iteration: {}", i);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        "Long task completed".to_string()
    });
    
    // Cancel after 2 seconds
    let cancel_token_for_canceller = cancel_token.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        cancel_token_for_canceller.cancel();
        println!("Cancellation requested!");
    });
    
    // FIXED: Handle potential cancellation in result
    match long_running_task.await {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Task panicked: {:?}", e),
    }
}
```

## ✅ Fixed Concurrent vs Sequential Performance

```rust
// Exercise 5.7: Fixed concurrent execution optimization
async fn optimize_concurrent_execution() {
    let urls = vec![
        "https://api.example.com/fast",
        "https://api.example.com/medium", 
        "https://api.example.com/slow",
        "https://api.example.com/quick",
    ];
    
    println!("🐌 Sequential execution:");
    let start = std::time::Instant::now();
    
    for url in &urls {
        let _result = fetch_data(url).await;
    }
    
    let sequential_time = start.elapsed();
    println!("Sequential took: {:?}", sequential_time);
    
    println!("\n🚀 Concurrent execution:");
    let start = std::time::Instant::now();
    
    // FIXED: True concurrent execution
    let futures: Vec<_> = urls.iter()
        .map(|url| fetch_data(url))
        .collect();
    
    let _results = futures::future::join_all(futures).await;
    
    let concurrent_time = start.elapsed();
    println!("Concurrent took: {:?}", concurrent_time);
    
    // Should be much faster!
    assert!(concurrent_time < sequential_time);
}
```

## ✅ Fixed Main Function

```rust
// Exercise 5.8: Fixed async main function
#[tokio::main]  // FIXED: Add tokio main attribute
async fn main() {
    println!("🚀 Starting async example...");
    
    // FIXED: Add .await to async function calls
    let result = fetch_data("https://api.example.com/test").await;
    println!("Result: {}", result);
    
    // FIXED: Add .await to async function calls
    let results = process_multiple_urls().await;
    println!("Processed {} URLs", results.len());
    
    // Test all the exercises
    spawn_with_shared_data().await;
    concurrent_tasks_with_errors().await;
    cancellable_task().await;
    optimize_concurrent_execution().await;
    
    println!("✅ Program completed!");
}
```

## ✅ Fixed Comprehensive Async Test

```rust
// Exercise 5.9: Complete implementation
async fn comprehensive_async_test() {
    println!("🧪 Running comprehensive async test...");
    
    let start = std::time::Instant::now();
    
    // Spawn 5 concurrent tasks with different durations
    let durations = vec![50, 150, 75, 200, 100];
    let mut handles = Vec::new();
    
    for (id, duration) in durations.into_iter().enumerate() {
        let handle = tokio::spawn(async move {
            println!("Task {} starting ({}ms)", id, duration);
            tokio::time::sleep(Duration::from_millis(duration)).await;
            
            // Simulate potential error
            if id == 2 {
                panic!("Task {} failed!", id);
            }
            
            format!("Task {} completed in {}ms", id, duration)
        });
        handles.push(handle);
    }
    
    // Collect all results with error handling
    let mut successful_results = Vec::new();
    let mut failed_count = 0;
    
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => {
                println!("✅ {}", result);
                successful_results.push(result);
            },
            Err(e) => {
                println!("❌ Task {} panicked: {:?}", i, e);
                failed_count += 1;
            }
        }
    }
    
    let total_time = start.elapsed();
    println!("🏁 Comprehensive test completed in {:?}", total_time);
    println!("📊 Results: {} successful, {} failed", successful_results.len(), failed_count);
}
```

## ✅ Cargo.toml Dependencies

Make sure your `Cargo.toml` includes:

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"
```

## 🏆 Final Implementation Tips

1. **Copy these solutions gradually** - Don't copy everything at once
2. **Understand each fix** - Ask yourself why each change was necessary
3. **Test incrementally** - Run `cargo check` after each fix
4. **Compare with C#** - Notice the similarities and differences

## 🎯 Expected Final Output

When working correctly, your program should:
- Complete without hanging or panicking
- Show concurrent execution being faster than sequential
- Demonstrate task cancellation working properly
- Handle both successful and failed async operations

Run with: `cargo run --bin ex05-async-tokio`