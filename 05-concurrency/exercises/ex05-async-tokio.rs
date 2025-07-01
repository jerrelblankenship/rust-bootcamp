// Exercise 5: Async/Await and Tokio - Fix the Broken Code!
// 
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (10 checkpoints to fix)
// 
// Your task: Make this async code compile and work correctly by fixing
// all the compilation errors and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Fix ONE error at a time - don't try to fix everything at once!
// 2. Compile after each fix: `cargo check` (async needs tokio dependency)
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (basic async syntax)
// - Each checkpoint builds on the previous one
// - Compare with C# async/await patterns you know
//
// C# VS RUST ASYNC COMPARISON:
// C#: public async Task<string> FetchData(string url) { await Task.Delay(1000); }
// Rust: async fn fetch_data(url: &str) -> String { tokio::time::sleep(Duration::from_secs(1)).await; }
//
// KEY DIFFERENCES:
// - C#: Task<T> return type, Rust: async fn returns impl Future<Output = T>
// - C#: Task.Run() for spawning, Rust: tokio::spawn() 
// - C#: Task.Delay() for async sleep, Rust: tokio::time::sleep()
// - C#: ConfigureAwait(false), Rust: No equivalent needed
// - C#: SynchronizationContext captures, Rust: Send + Sync traits
//
// COMPLETED CONCEPTS:
// [] Basic async function syntax
// [] Tokio runtime setup and main function
// [] Async sleep vs blocking sleep
// [] Error handling in async contexts
// [] Task spawning with tokio::spawn
// [] Concurrent vs sequential execution
// [] Task cancellation patterns

// CHECKPOINT 1: Fix the imports
// FIXME: Missing async runtime - we need tokio!
use std::time::Duration;
// TODO: Add missing tokio imports

// Exercise 5.1: Fix async function definition
// FIXME: Wrong async syntax
async fn fetch_data(url: &str) -> String {
    println!("Fetching data from: {}", url);
    
    // CHECKPOINT 2: Fix the await call
    // FIXME: Wrong sleep function for async context
    std::thread::sleep(Duration::from_secs(1)); // COMPILE ERROR: This blocks the thread!
    
    format!("Data from {}", url)
}

// ✅ CHECKPOINT 1: Basic async function should compile
// Progress: [█░░░░░░░░░] 10% Complete

// Exercise 5.2: Fix async function with Result
// FIXME: Wrong Result type for async
async fn fetch_with_error(url: &str) -> Result<String, String> {
    if url.contains("error") {
        return Err("Simulated network error".to_string());
    }
    
    // CHECKPOINT 3: Fix async sleep usage
    // FIXME: Same blocking sleep issue
    std::thread::sleep(Duration::from_millis(500)); // COMPILE ERROR: Wrong sleep!
    Ok(format!("Success data from {}", url))
}

// ✅ CHECKPOINT 2: Async sleep should work properly
// Progress: [██░░░░░░░░] 20% Complete

// Exercise 5.3: Fix async function calling other async functions
async fn process_multiple_urls() -> Vec<String> {
    let urls = vec![
        "https://api.example.com/data1",
        "https://api.example.com/data2", 
        "https://api.example.com/error", // This will cause an error
        "https://api.example.com/data3",
    ];
    
    let mut results = Vec::new();
    
    for url in urls {
        // CHECKPOINT 4: Fix sequential vs concurrent execution
        // FIXME: This is sequential, not concurrent!
        let result = fetch_data(url); // COMPILE ERROR: Missing await!
        results.push(result);
        
        // CHECKPOINT 5: Fix error handling
        match fetch_with_error(url) { // COMPILE ERROR: Missing await!
            Ok(data) => println!("✅ Success: {}", data),
            Err(e) => println!("❌ Error: {}", e),
        }
    }
    
    results
}

// ✅ CHECKPOINT 3: Async function calls should work
// Progress: [███░░░░░░░] 30% Complete

// Exercise 5.4: Fix tokio task spawning with shared data
async fn spawn_with_shared_data() {
    let shared_data = vec!["item1", "item2", "item3", "item4"];
    let mut handles = Vec::new();
    
    for (i, item) in shared_data.iter().enumerate() {
        // CHECKPOINT 6: Fix ownership issues with spawning tasks
        // FIXME: Ownership issues with spawning tasks
        let handle = tokio::spawn(async {
            println!("Processing: {}", item); // COMPILE ERROR: item not owned by this closure
            tokio::time::sleep(Duration::from_millis(100)).await;
            format!("Processed: {}", item)
        });
        handles.push(handle);
    }
    
    // CHECKPOINT 7: Fix task result collection
    for handle in handles {
        // FIXME: Not handling the Result from join()
        let result = handle.join(); // COMPILE ERROR: Missing await and error handling
        println!("Task result: {}", result);
    }
}

// ✅ CHECKPOINT 4: Tokio task spawning should work
// Progress: [████░░░░░░] 40% Complete

// Exercise 5.5: Fix concurrent task execution with error handling
async fn concurrent_tasks_with_errors() {
    let task_durations = vec![100, 200, 50, 300, 150];
    let mut handles = Vec::new();
    
    for (id, duration) in task_durations.into_iter().enumerate() {
        // CHECKPOINT 8: Fix moving id and duration into closure
        // FIXME: Moving id and duration into closure
        let handle = tokio::spawn(async {
            background_task(id as u32, duration).await // COMPILE ERROR: id and duration moved
        });
        handles.push(handle);
    }
    
    // CHECKPOINT 9: Fix error handling for panicked tasks
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join().await {
            Ok(result) => println!("Task {} success: {}", i, result),
            Err(e) => println!("Task {} failed: {:?}", i, e), // Handle the JoinError
        }
    }
}

// Helper function for background task
async fn background_task(task_id: u32, duration_ms: u64) -> String {
    println!("Task {}: Starting work for {}ms", task_id, duration_ms);
    
    tokio::time::sleep(Duration::from_millis(duration_ms)).await;
    
    // Simulate potential error
    if task_id == 2 {
        panic!("Task {} encountered an error!", task_id);
    }
    
    let result = format!("Task {} completed successfully", task_id);
    println!("{}", result);
    result
}

// ✅ CHECKPOINT 5: Concurrent tasks with error handling should work
// Progress: [█████░░░░░] 50% Complete

// Exercise 5.6: Fix task cancellation
async fn cancellable_task() {
    // CHECKPOINT 10: Fix missing imports for cancellation
    // FIXME: Missing import
    // use tokio_util::sync::CancellationToken; // COMPILE ERROR: Missing import
    
    // TODO: Implement proper cancellation token usage
    let cancel_token = unimplemented!(); // Replace with proper CancellationToken::new()
    let cancel_token_clone = cancel_token.clone();
    
    let long_running_task = tokio::spawn(async move {
        for i in 1..=10 {
            // FIXME: Not checking for cancellation
            println!("Long task iteration: {}", i);
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            // TODO: Check if cancelled and break if so
            // if cancel_token_clone.is_cancelled() { break; }
        }
        "Long task completed"
    });
    
    // Cancel after 2 seconds
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        cancel_token.cancel();
        println!("Cancellation requested!");
    });
    
    // FIXME: Not handling potential cancellation
    let result = long_running_task.join().await.unwrap();
    println!("Result: {}", result);
}

// ✅ CHECKPOINT 6: Task cancellation should work
// Progress: [██████░░░░] 60% Complete

// Exercise 5.7: Fix concurrent vs sequential execution
async fn optimize_concurrent_execution() {
    let urls = vec![
        "https://api.example.com/fast",    // 100ms
        "https://api.example.com/medium",  // 300ms
        "https://api.example.com/slow",    // 500ms
        "https://api.example.com/quick",   // 50ms
    ];
    
    println!("🐌 Sequential execution:");
    let start = std::time::Instant::now();
    
    // FIXME: This is sequential - make it concurrent!
    for url in &urls {
        let _result = fetch_data(url).await;
    }
    
    let sequential_time = start.elapsed();
    println!("Sequential took: {:?}", sequential_time);
    
    println!("\n🚀 Concurrent execution:");
    let start = std::time::Instant::now();
    
    // TODO: Make this truly concurrent using futures::future::join_all or similar
    // HINT: Collect futures first, then await them all at once
    
    let concurrent_time = start.elapsed();
    println!("Concurrent took: {:?}", concurrent_time);
    
    // Should be much faster!
    assert!(concurrent_time < sequential_time);
}

// ✅ CHECKPOINT 7: Concurrent execution should be much faster
// Progress: [███████░░░] 70% Complete

// Exercise 5.8: Fix the main function
// FIXME: main is not async, but we're calling async functions
fn main() {
    println!("🚀 Starting async example...");
    
    // CHECKPOINT 8: Fix async main function
    // FIXME: Cannot await in non-async context
    let result = fetch_data("https://api.example.com/test"); // COMPILE ERROR: Missing await
    println!("Result: {}", result);
    
    // FIXME: Same issue with processing multiple URLs
    let results = process_multiple_urls(); // COMPILE ERROR: Missing await
    println!("Processed {} URLs", results.len());
    
    println!("✅ Program completed!");
}

// ✅ CHECKPOINT 8: Async main function should work
// Progress: [████████░░] 80% Complete

// Exercise 5.9: Add comprehensive async test
// TODO: Create a function that demonstrates all concepts:
// - Spawn 5 concurrent tasks
// - Each task simulates different work (50ms to 500ms)
// - Collect all results
// - Handle any panics gracefully
// - Show total execution time
async fn comprehensive_async_test() {
    println!("🧪 Running comprehensive async test...");
    
    // FIXME: This implementation doesn't spawn any tasks or measure performance
    let results = vec!["Task 1", "Task 2", "Task 3"];
    for result in results {
        println!("Result: {}", result);
    }
    // FIXME: Should use tokio::spawn, join_all, and time measurement
}

// ✅ CHECKPOINT 9: Comprehensive test should demonstrate mastery
// Progress: [█████████░] 90% Complete

// Exercise 5.10: Advanced - async trait objects (optional)
// TODO: Advanced challenge - implement async trait for different data sources
trait AsyncDataSource {
    async fn fetch(&self, id: &str) -> Result<String, String>;
}

struct FastSource;
struct SlowSource;

// TODO: Implement the trait for both sources
// FastSource should return in 50ms
// SlowSource should return in 500ms
// Then create a function that works with any AsyncDataSource

// ✅ CHECKPOINT 10: Async traits should work
// Progress: [██████████] 100% Complete

// COMPILATION CHALLENGES:
// 1. Add proper tokio dependency and imports
// 2. Fix async function syntax and await usage
// 3. Replace std::thread::sleep with tokio::time::sleep
// 4. Handle ownership in tokio::spawn closures
// 5. Properly await and handle task results
// 6. Implement task cancellation
// 7. Make concurrent execution truly concurrent
// 8. Set up async main function with tokio

// LEARNING OBJECTIVES:
// - Understanding Rust async/await vs C# async/await
// - Tokio runtime and task spawning
// - Concurrent vs sequential async execution
// - Error handling in async contexts
// - Task cancellation patterns
// - Performance implications of async choices

// C# COMPARISON:
// C#: public async Task<string> FetchDataAsync(string url)
// Rust: async fn fetch_data(url: &str) -> String
//
// C#: await Task.Delay(1000);
// Rust: tokio::time::sleep(Duration::from_secs(1)).await;
//
// C#: Task.Run(() => DoWork())
// Rust: tokio::spawn(async { do_work().await })
//
// C#: await Task.WhenAll(tasks)
// Rust: futures::future::join_all(futures).await

// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Basic async syntax): [ ] Complete
// Checkpoint 2 (Async sleep): [ ] Complete  
// Checkpoint 3 (Async function calls): [ ] Complete
// Checkpoint 4 (Tokio task spawning): [ ] Complete
// Checkpoint 5 (Concurrent tasks with errors): [ ] Complete
// Checkpoint 6 (Task cancellation): [ ] Complete
// Checkpoint 7 (Concurrent execution): [ ] Complete
// Checkpoint 8 (Async main function): [ ] Complete
// Checkpoint 9 (Comprehensive test): [ ] Complete
// Checkpoint 10 (Async traits): [ ] Complete

// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Async/await syntax and semantics
// ✅ Tokio runtime and task management
// ✅ Concurrent async execution patterns
// ✅ Error handling in async contexts
// ✅ Performance optimization with async code

// 🚀 Ready for the next challenge?
// Move on to the web scraper project to apply everything you've learned!
// Or check your work with: `cargo check && cargo run`
