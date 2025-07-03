// Exercise 7: Async Blocking Performance - Fix the Async/Await Disasters!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 checkpoints to fix)
//
// Your task: Fix catastrophic async/await performance disasters
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex07-async-blocking.rs`
// 2. Run: `./ex07-async-blocking`
// 3. Fix the async anti-patterns one by one
// 4. Use hints in /hints/ directory if stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Each checkpoint shows a different async performance anti-pattern
// - Fix one async disaster at a time and measure improvement
// - Compare with equivalent C# async/await optimizations
//
// COMPLETED CONCEPTS:
// [] Blocking operations in async contexts
// [] Sequential vs concurrent async operations
// [] Excessive task creation overhead
// [] Async runtime and thread pool saturation
// [] Memory overhead from async state machines

use std::time::{Duration, Instant};
use std::thread;

// Simulated async runtime for demonstration
async fn simulate_io_operation(duration_ms: u64) -> String {
    thread::sleep(Duration::from_millis(duration_ms));
    format!("Operation completed after {}ms", duration_ms)
}

async fn simulate_database_query(id: u32) -> String {
    thread::sleep(Duration::from_millis(50));
    format!("User data for ID: {}", id)
}

async fn simulate_api_call(endpoint: &str) -> String {
    thread::sleep(Duration::from_millis(100));
    format!("API response from {}", endpoint)
}

fn main() {
    println!("=== Exercise 7: Async Blocking Performance (Fix the Async/Await Disasters!) ===\n");
    
    // Exercise 7.1: Blocking operations in async context
    println!("🔥 CHECKPOINT 1: Blocking operations killing async performance");
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: Blocking operations in async context!
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let mut results = Vec::new();
        
        for i in 0..10 {
            // FIXME: This blocks the entire async runtime!
            thread::sleep(Duration::from_millis(100));  // FIXME: Blocking operation!
            results.push(format!("Processed item {}", i));
        }
        
        results
    });
    
    let duration = start.elapsed();
    println!("Processed {} items in {:?}", result.len(), duration);
    println!("💡 Use async operations instead of blocking calls! Check hints/ex07-level1.md\n");
    
    // ✅ CHECKPOINT 1: Replace blocking operations with async equivalents
    
    // Exercise 7.2: Sequential async operations instead of concurrent
    println!("🔥 CHECKPOINT 2: Sequential async operations missing concurrency");
    let start = Instant::now();
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let results = rt.block_on(async {
        let mut all_results = Vec::new();
        
        // PERFORMANCE DISASTER: Sequential async operations!
        for i in 0..5 {
            let result = simulate_io_operation(200).await;  // FIXME: Sequential execution!
            all_results.push(result);
        }
        // In C#, this would be: await operation1; await operation2; (sequential)
        
        all_results
    });
    
    let duration = start.elapsed();
    println!("Completed {} operations in {:?}", results.len(), duration);
    println!("💡 Use join! or select! for concurrent async operations!\n");
    
    // ✅ CHECKPOINT 2: Make async operations concurrent instead of sequential
    
    // Exercise 7.3: Excessive async task creation
    println!("🔥 CHECKPOINT 3: Excessive async task spawning overhead");
    let start = Instant::now();
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let mut handles = Vec::new();
        
        // PERFORMANCE DISASTER: Creating async task for every tiny operation!
        for i in 0..1000 {
            let handle = tokio::spawn(async move {  // FIXME: Task creation overhead!
                i * 2  // FIXME: Trivial work per task!
            });
            handles.push(handle);
        }
        
        let mut sum = 0;
        for handle in handles {
            sum += handle.await.unwrap();
        }
        sum
    });
    
    let duration = start.elapsed();
    println!("Computed sum {} with excessive tasks in {:?}", result, duration);
    println!("💡 Batch operations or use rayon for CPU-bound work!\n");
    
    // ✅ CHECKPOINT 3: Reduce async task creation overhead
    
    // Exercise 7.4: Mixed CPU and IO work in async
    println!("🔥 CHECKPOINT 4: CPU-bound work blocking async runtime");
    let start = Instant::now();
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let results = rt.block_on(async {
        let mut handles = Vec::new();
        
        for i in 0..4 {
            let handle = tokio::spawn(async move {
                // PERFORMANCE DISASTER: CPU-intensive work in async context!
                let mut sum = 0;
                for j in 0..1_000_000 {  // FIXME: CPU-bound work blocks runtime!
                    sum += j * i;
                }
                
                // Mixed with actual async work
                let api_result = simulate_api_call(&format!("endpoint_{}", i)).await;
                (sum, api_result)
            });
            handles.push(handle);
        }
        
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }
        results
    });
    
    let duration = start.elapsed();
    println!("Completed {} mixed operations in {:?}", results.len(), duration);
    println!("💡 Use spawn_blocking for CPU-intensive work in async contexts!\n");
    
    // ✅ CHECKPOINT 4: Properly handle CPU-bound work in async runtime
    
    // Exercise 7.5: Async state machine memory bloat
    println!("🔥 CHECKPOINT 5: Excessive async state machine memory usage");
    let start = Instant::now();
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let mut futures = Vec::new();
        
        // PERFORMANCE DISASTER: Large async state machines with unnecessary data!
        for i in 0..1000 {
            let large_data = vec![0u8; 1024];  // FIXME: Large data in async state!
            let another_large_data = vec![i as u8; 1024];  // FIXME: More bloat!
            
            let future = async move {
                let _data1 = large_data;  // FIXME: Carried in state machine!
                let _data2 = another_large_data;  // FIXME: More state bloat!
                
                simulate_database_query(i).await;  // FIXME: Simple operation with huge state!
                i
            };
            futures.push(future);
        }
        
        let mut sum = 0;
        for future in futures {
            sum += future.await;
        }
        sum
    });
    
    let duration = start.elapsed();
    println!("Completed {} operations with bloated state in {:?}", result, duration);
    println!("💡 Minimize data captured in async closures and move heavy data outside!\n");
    
    // ✅ CHECKPOINT 5: Optimize async state machine memory usage
    
    println!("🎯 CHALLENGE: Optimize all checkpoints to achieve true async concurrency!");
    println!("🔄 C# COMPARISON: These are like Task.Run, async/await, and ConfigureAwait optimizations!");
}

#[cfg(feature = "async")]
mod async_examples {
    use super::*;
    
    // Additional async examples that would require tokio dependency
    async fn example_concurrent_operations() {
        // This shows the correct pattern for concurrent operations
        let (result1, result2, result3) = tokio::join!(
            simulate_io_operation(100),
            simulate_io_operation(150),
            simulate_io_operation(200)
        );
        
        println!("Concurrent results: {}, {}, {}", result1, result2, result3);
    }
    
    async fn example_proper_cpu_work() {
        let cpu_result = tokio::task::spawn_blocking(|| {
            // CPU-intensive work in separate thread pool
            (0..1_000_000).sum::<usize>()
        }).await.unwrap();
        
        let io_result = simulate_api_call("example").await;
        
        println!("CPU result: {}, IO result: {}", cpu_result, io_result);
    }
}

// COMPILATION CHALLENGES:
// 1. Replace thread::sleep with proper async sleep/delay
// 2. Use tokio::join! or futures::join! for concurrent operations
// 3. Batch small operations or use appropriate parallelization
// 4. Use spawn_blocking for CPU-bound work in async contexts
// 5. Minimize data captured in async closures to reduce state machine size
//
// LEARNING OBJECTIVES:
// - Understanding blocking vs non-blocking operations in async contexts
// - Sequential vs concurrent async operation patterns
// - When to use async tasks vs other parallelization approaches
// - Proper separation of CPU-bound and IO-bound work in async
// - Memory optimization for async state machines
// - Performance comparison with C# async/await patterns
//
// C# COMPARISON:
// C#: Thread.Sleep vs await Task.Delay
// Rust: thread::sleep vs tokio::time::sleep
//
// C#: await task1; await task2; vs await Task.WhenAll(task1, task2)
// Rust: op1.await; op2.await; vs tokio::join!(op1, op2)
//
// C#: Task.Run for CPU-bound work in async context
// Rust: tokio::task::spawn_blocking for CPU-bound work
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Blocking operations): [ ] Complete
// Checkpoint 2 (Sequential operations): [ ] Complete
// Checkpoint 3 (Task creation overhead): [ ] Complete
// Checkpoint 4 (CPU work in async): [ ] Complete
// Checkpoint 5 (State machine bloat): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Proper async/await operation patterns
// ✅ Concurrent vs sequential async execution
// ✅ Efficient async task management
// ✅ CPU vs IO work separation in async contexts
// ✅ Memory-efficient async state machines
// ✅ Performance equivalent to optimized C# async patterns
//
// 🚀 Ready for the next challenge?
// Move on to ex08-simd-opportunity.rs to explore vectorization performance!
// Or check your work with: `rustc ex07-async-blocking.rs && ./ex07-async-blocking`