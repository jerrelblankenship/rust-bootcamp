// Exercise 6: Parallel Processing Waste - Fix the Threading Performance Disasters!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix catastrophic parallel processing performance disasters
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex06-parallel-waste.rs`
// 2. Run: `./ex06-parallel-waste`
// 3. Fix the threading anti-patterns one by one
// 4. Use hints in /hints/ directory if stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Each checkpoint shows a different parallel processing anti-pattern
// - Fix one threading disaster at a time and measure improvement
// - Compare with equivalent C# parallel processing optimizations
//
// COMPLETED CONCEPTS:
// [] False sharing and cache line contention
// [] Thread creation overhead vs thread pools
// [] Lock contention and synchronization overhead
// [] Work distribution and load balancing
// [] Memory access patterns in parallel code
// [] Parallel vs sequential threshold tuning

use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    println!("=== Exercise 6: Parallel Processing Waste (Fix the Threading Performance Disasters!) ===\n");
    
    // Exercise 6.1: Thread creation for every task
    println!("🔥 CHECKPOINT 1: Thread creation overhead disaster");
    let start = Instant::now();
    
    let numbers = (0..1000).collect::<Vec<i32>>();
    let mut handles = vec![];
    
    // PERFORMANCE DISASTER: Creating new thread for every small task!
    for chunk in numbers.chunks(10) {
        let chunk = chunk.to_vec();  // FIXME: Unnecessary allocation!
        let handle = thread::spawn(move || {  // FIXME: Thread creation overhead!
            chunk.iter().map(|x| x * x).sum::<i32>()
        });
        handles.push(handle);
    }
    
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("Computed sum of squares: {} in {:?}", total, duration);
    println!("💡 Use thread pools or rayon for task parallelism! Check hints/ex06-level1.md\n");
    
    // ✅ CHECKPOINT 1: Use thread pools instead of creating threads per task
    
    // Exercise 6.2: False sharing cache line contention
    println!("🔥 CHECKPOINT 2: False sharing between threads");
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: False sharing between thread counters!
    struct Counters {
        counter1: i64,  // FIXME: Same cache line as counter2!
        counter2: i64,  // FIXME: Cache line bouncing between cores!
        counter3: i64,  // FIXME: More false sharing!
        counter4: i64,  // FIXME: All counters in same cache line!
    }
    
    let counters = Arc::new(Mutex::new(Counters {
        counter1: 0, counter2: 0, counter3: 0, counter4: 0
    }));
    
    let mut handles = vec![];
    for i in 0..4 {
        let counters = Arc::clone(&counters);
        let handle = thread::spawn(move || {
            for _ in 0..250_000 {
                let mut c = counters.lock().unwrap();  // FIXME: Heavy lock contention!
                match i {
                    0 => c.counter1 += 1,
                    1 => c.counter2 += 1,
                    2 => c.counter3 += 1,
                    _ => c.counter4 += 1,
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    let c = counters.lock().unwrap();
    println!("Counters: {}, {}, {}, {} in {:?}", c.counter1, c.counter2, c.counter3, c.counter4, duration);
    println!("💡 Separate counters to different cache lines and reduce lock contention!\n");
    
    // ✅ CHECKPOINT 2: Eliminate false sharing and reduce lock contention
    
    // Exercise 6.3: Poor work distribution
    println!("🔥 CHECKPOINT 3: Unbalanced work distribution");
    let start = Instant::now();
    
    let data = (0..1_000_000).collect::<Vec<usize>>();
    
    // PERFORMANCE DISASTER: Uneven work distribution!
    let chunk_size = data.len() / 4;
    let mut handles = vec![];
    
    for i in 0..4 {
        let start_idx = i * chunk_size;
        let end_idx = if i == 3 { data.len() } else { (i + 1) * chunk_size };
        let chunk = data[start_idx..end_idx].to_vec();
        
        let handle = thread::spawn(move || {
            let mut sum = 0;
            for &num in &chunk {
                // FIXME: Work complexity varies dramatically by input!
                let work_amount = num % 1000 + 1;  // 1 to 1000 iterations
                for _ in 0..work_amount {
                    sum += 1;
                }
            }
            sum
        });
        handles.push(handle);
    }
    
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("Computed variable work total: {} in {:?}", total, duration);
    println!("💡 Use work-stealing or dynamic task distribution!\n");
    
    // ✅ CHECKPOINT 3: Implement better work distribution strategy
    
    // Exercise 6.4: Sequential memory access in parallel
    println!("🔥 CHECKPOINT 4: Cache-unfriendly parallel memory access");
    let start = Instant::now();
    
    let mut large_array = vec![0i32; 4_000_000];
    let mut handles = vec![];
    
    // PERFORMANCE DISASTER: Threads accessing interleaved memory!
    for thread_id in 0..4 {
        let array_ptr = large_array.as_mut_ptr();
        let handle = thread::spawn(move || {
            unsafe {
                for i in 0..1_000_000 {
                    let index = thread_id + i * 4;  // FIXME: Interleaved access pattern!
                    *array_ptr.add(index) = (i * thread_id) as i32;
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    let sum: i64 = large_array.iter().map(|&x| x as i64).sum();
    println!("Parallel array fill sum: {} in {:?}", sum, duration);
    println!("💡 Use contiguous memory blocks per thread for better cache usage!\n");
    
    // ✅ CHECKPOINT 4: Optimize memory access patterns for parallel processing
    
    // Exercise 6.5: Over-parallelization for small work
    println!("🔥 CHECKPOINT 5: Over-parallelization overhead");
    let start = Instant::now();
    
    let small_data = (0..1000).collect::<Vec<i32>>();
    
    // PERFORMANCE DISASTER: Parallel overhead exceeds work cost!
    let handles: Vec<_> = small_data
        .chunks(25)  // FIXME: Too small chunks for parallelization!
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                chunk.iter().sum::<i32>()  // FIXME: Trivial work per thread!
            })
        })
        .collect();
    
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("Over-parallelized sum: {} in {:?}", total, duration);
    println!("💡 Only parallelize when work exceeds thread overhead!\n");
    
    // ✅ CHECKPOINT 5: Apply appropriate parallelization thresholds
    
    // Exercise 6.6: Shared mutable state with heavy contention
    println!("🔥 CHECKPOINT 6: Heavy lock contention disaster");
    let start = Instant::now();
    
    let shared_counter = Arc::new(Mutex::new(0i64));
    let mut handles = vec![];
    
    // PERFORMANCE DISASTER: All threads fighting for same lock!
    for _ in 0..8 {
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            for _ in 0..125_000 {
                let mut count = counter.lock().unwrap();  // FIXME: Severe lock contention!
                *count += 1;  // FIXME: Minimal work per lock acquisition!
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    let final_count = *shared_counter.lock().unwrap();
    println!("Contended counter final value: {} in {:?}", final_count, duration);
    println!("💡 Use local accumulation and final aggregation to reduce contention!\n");
    
    // ✅ CHECKPOINT 6: Reduce lock contention with better synchronization patterns
    
    println!("🎯 CHALLENGE: Optimize all checkpoints to achieve near-linear speedup!");
    println!("🔄 C# COMPARISON: These are like Parallel.For, PLINQ, and threading best practices!");
}

// COMPILATION CHALLENGES:
// 1. Use thread pools (rayon) instead of creating threads per task
// 2. Eliminate false sharing with proper data layout and reduce lock contention
// 3. Implement work-stealing or dynamic work distribution
// 4. Optimize memory access patterns for cache-friendly parallel processing
// 5. Apply appropriate parallelization thresholds based on work complexity
// 6. Use local accumulation patterns to reduce shared state contention
//
// LEARNING OBJECTIVES:
// - Understanding thread creation vs thread pool performance
// - False sharing and cache line contention in parallel code
// - Work distribution strategies for balanced parallel processing
// - Memory access patterns and cache behavior in threaded code
// - When parallelization overhead exceeds benefits
// - Synchronization patterns and lock contention reduction
// - Performance comparison with C# parallel processing patterns
//
// C# COMPARISON:
// C#: Task.Run() vs Parallel.ForEach() vs ThreadPool
// Rust: thread::spawn() vs rayon::par_iter() vs thread pools
//
// C#: false sharing with [FieldOffset] and padding
// Rust: false sharing with manual padding and memory layout
//
// C#: Parallel.ForEach with Partitioner for work distribution
// Rust: rayon work-stealing and custom parallel iterators
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Thread creation): [ ] Complete
// Checkpoint 2 (False sharing): [ ] Complete
// Checkpoint 3 (Work distribution): [ ] Complete
// Checkpoint 4 (Memory access): [ ] Complete
// Checkpoint 5 (Over-parallelization): [ ] Complete
// Checkpoint 6 (Lock contention): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Efficient thread pool usage patterns
// ✅ Eliminating false sharing and cache contention
// ✅ Balanced work distribution strategies
// ✅ Cache-friendly parallel memory access
// ✅ Appropriate parallelization thresholds
// ✅ Low-contention synchronization patterns
//
// 🚀 Ready for the next challenge?
// Move on to ex07-async-blocking.rs to explore async performance!
// Or check your work with: `rustc ex06-parallel-waste.rs && ./ex06-parallel-waste`