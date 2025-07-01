// Exercise 1: Threading Basics - Fix the Broken Code!
// 
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (10 checkpoints to fix)
// 
// Your task: Make this code compile and work correctly by fixing
// all the compilation errors and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Fix ONE error at a time - don't try to fix everything at once!
// 2. Compile after each fix: `rustc ex01-threading-basics.rs`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (basic thread spawning)
// - Each checkpoint builds on the previous one
// - Celebrate each successful compilation - you're learning!
//
// COMPLETED CONCEPTS:
// [] Basic thread spawning with std::thread
// [] Move semantics and ownership transfer
// [] Cloning vs moving data
// [] Shared ownership with Arc
// [] Thread joining and panic handling

use std::thread;
use std::time::Duration;
use std::sync::Arc;

fn main() {
    println!("=== Exercise 1: Threading Basics (Fix the Code!) ===\n");
    
    // Exercise 1.1: Fix basic thread spawning
    // FIXME: This doesn't compile - ownership issue!
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(|| {
        println!("Thread data: {:?}", data); // COMPILE ERROR: Capture of moved value
    });
    handle.join().unwrap();
    
    // ✅ CHECKPOINT 1: Run `rustc ex01-threading-basics.rs` - should show fewer errors
    // If the ownership error is fixed, move to the next section
    // Progress: [█░░░░░░░░░] 10% Complete
    
    // Exercise 1.2: Fix the move keyword
    // FIXME: Missing move keyword
    let message = String::from("Hello from thread!");
    let handle = thread::spawn(|| {
        println!("{}", message); // COMPILE ERROR: Closure may outlive current function
    });
    handle.join().unwrap();
    
    // ✅ CHECKPOINT 2: Compile again - move keyword error should be resolved
    // Progress: [██░░░░░░░░] 20% Complete
    
    // Exercise 1.3: Fix multiple threads with shared data
    // FIXME: Can't use the same data in multiple threads!
    let numbers = vec![10, 20, 30];
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(|| {
            println!("Thread {}: {:?}", i, numbers); // COMPILE ERROR: Multiple ownership
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // ✅ CHECKPOINT 3: Multiple thread ownership issue should be resolved
    // Progress: [███░░░░░░░] 30% Complete
    
    // Exercise 1.4: Fix expensive data cloning
    // FIXME: Choose between cloning and moving
    let expensive_data = vec![1; 1000000]; // Large vector
    let handle = thread::spawn(|| {
        let sum: usize = expensive_data.iter().sum(); // COMPILE ERROR: Value moved
        println!("Sum: {}", sum);
    });
    
    // We want to use expensive_data here too!
    println!("Original length: {}", expensive_data.len()); // COMPILE ERROR: Value moved
    handle.join().unwrap();
    
    // ✅ CHECKPOINT 4: Should clone expensive data properly
    // Progress: [████░░░░░░] 40% Complete
    
    // Exercise 1.5: Fix shared ownership with Arc
    // FIXME: Multiple threads need access to the same data
    let shared_data = vec!["Rust", "is", "awesome"];
    
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            println!("Thread {}: {:?}", i, shared_data); // COMPILE ERROR: Each closure needs ownership
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // ✅ CHECKPOINT 5: Arc should be implemented for shared ownership
    // Progress: [█████░░░░░] 50% Complete
    
    // Exercise 1.6: Fix reference counting
    // FIXME: Implement proper reference counting
    let reference_counted = vec!["Share", "me", "safely"];
    let mut handles = vec![];
    
    for i in 0..5 {
        // Each thread needs a reference to the same data
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            println!("Thread {} accessing: {:?}", i, reference_counted); // COMPILE ERROR: Move occurs
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // ✅ CHECKPOINT 6: Reference counting with Arc should work
    // Progress: [██████░░░░] 60% Complete
    
    // Exercise 1.7: Fix thread with return value
    // FIXME: Can't capture mutable reference!
    let mut result = 0;
    let handle = thread::spawn(|| {
        result = 42; // COMPILE ERROR: Can't capture mutable reference
    });
    handle.join().unwrap();
    println!("Result: {}", result);
    
    // ✅ CHECKPOINT 7: Thread should return value instead of mutating
    // Progress: [███████░░░] 70% Complete
    
    // Exercise 1.8: Fix scoped threads (advanced)
    // FIXME: Regular threads can't borrow local data!
    let local_data = "Hello from main thread!";
    thread::spawn(|| {
        println!("Borrowed: {}", local_data); // COMPILE ERROR: Lifetime issue
    }).join().unwrap();
    
    // ✅ CHECKPOINT 8: Scoped threads should work with borrowed data
    // Progress: [████████░░] 80% Complete
    
    // Exercise 1.9: Fix thread panic handling
    // FIXME: How do we handle thread panics gracefully?
    let handle = thread::spawn(|| {
        panic!("Thread panicked!"); // This will crash the thread
    });
    
    // Handle the panic without crashing main thread
    match handle.join() {
        Ok(_) => println!("Thread completed successfully"),
        Err(_) => println!("Thread panicked, but we handled it!"),
    }
    
    // ✅ CHECKPOINT 9: Panic handling should work properly
    // Progress: [█████████░] 90% Complete
    
    // Exercise 1.10: Test your understanding
    // TODO: Create 3 threads that each process part of this data concurrently
    let big_data = (0..300).collect::<Vec<u32>>();
    // Each thread should process 100 numbers and return the sum
    // Use Arc for sharing, collect results properly
    // HINT: Split the data into 3 chunks: 0-99, 100-199, 200-299
    
    // TODO: Implement concurrent processing here
    println!("Processing {} numbers across 3 threads...", big_data.len());
    // Your implementation should print: "Thread 0 sum: X, Thread 1 sum: Y, Thread 2 sum: Z"
    
    // ✅ CHECKPOINT 10: Concurrent data processing should work
    // Progress: [██████████] 100% Complete
    
    println!("\n🎉 Exercise 1 completed! You've mastered threading basics!");
}

// COMPILATION CHALLENGES:
// 1. Fix ownership issues with thread spawning
// 2. Add move keyword where needed  
// 3. Choose between cloning expensive data vs sharing with Arc
// 4. Implement proper reference counting for multiple threads
// 5. Handle thread return values correctly
// 6. Use scoped threads for borrowing local data
// 7. Handle thread panics gracefully
// 8. Implement concurrent data processing

// LEARNING OBJECTIVES:
// - Understanding Rust's ownership model in concurrent contexts
// - When to move vs clone vs share data between threads
// - Using Arc for shared ownership across threads
// - Proper thread joining and error handling
// - Performance implications of different sharing strategies

// C# COMPARISON:
// C#: var task = Task.Run(() => ProcessData(data));
// Rust: let handle = thread::spawn(move || process_data(data));
//
// C#: lock(sharedData) { /* access */ }
// Rust: let shared = Arc::new(data); // Share immutable data
//
// C#: await Task.WhenAll(tasks);
// Rust: for handle in handles { handle.join().unwrap(); }

// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Basic thread spawn): [ ] Complete
// Checkpoint 2 (Move keyword): [ ] Complete  
// Checkpoint 3 (Multiple threads): [ ] Complete
// Checkpoint 4 (Expensive data cloning): [ ] Complete
// Checkpoint 5 (Arc shared ownership): [ ] Complete
// Checkpoint 6 (Reference counting): [ ] Complete
// Checkpoint 7 (Thread return values): [ ] Complete
// Checkpoint 8 (Scoped threads): [ ] Complete
// Checkpoint 9 (Panic handling): [ ] Complete
// Checkpoint 10 (Concurrent processing): [ ] Complete

// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Thread spawning and ownership transfer
// ✅ Move semantics in concurrent contexts
// ✅ Shared ownership patterns with Arc
// ✅ Thread joining and error handling
// ✅ Performance-conscious concurrent programming

// 🚀 Ready for the next challenge?
// Move on to ex02-channels.rs to explore thread communication!
// Or check your work with: `rustc ex01-threading-basics.rs && ./ex01-threading-basics`
