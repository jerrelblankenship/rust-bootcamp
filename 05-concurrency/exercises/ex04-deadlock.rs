// Exercise 4: Deadlock Detection and Resolution - Fix the Broken Code!
// 
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
// 
// Your task: Identify and fix the deadlock in this code!
// This program WILL HANG FOREVER until you fix the deadlock.
//
// INSTRUCTIONS:
// 1. First, run the code to see the deadlock happen: `cargo run --bin ex04-deadlock`
// 2. Press Ctrl+C to kill the hanging program
// 3. Analyze the deadlock pattern and fix it
// 4. Each checkpoint teaches a different deadlock prevention technique
//
// LEARNING STRATEGY:
// - Understand why deadlocks happen first
// - Learn multiple prevention techniques
// - Apply C# lock statement knowledge to Rust
//
// COMPLETED CONCEPTS:
// [] Identifying classic deadlock patterns
// [] Lock ordering for deadlock prevention
// [] Timeout-based lock acquisition
// [] Try-lock patterns for non-blocking access
// [] Hierarchical locking strategies

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 4: Deadlock Detection and Resolution ===\n");
    
    // WARNING: This code will deadlock! Run it to see the problem.
    println!("🚨 About to demonstrate a deadlock...");
    println!("If this hangs, press Ctrl+C and fix the code!");
    
    deadlock_demonstration();
    
    println!("\n--- After you fix the deadlock, uncomment these sections: ---");
    // lock_ordering_solution();
    // timeout_solution();
    // try_lock_solution();
    // hierarchical_locking();
    // advanced_deadlock_detection();
}

// Exercise 4.1: The Classic Deadlock (DO NOT FIX - This is the example!)
fn deadlock_demonstration() {
    println!("\n=== Deadlock Demonstration ===");
    
    // Two shared resources that threads will compete for
    let resource1 = Arc::new(Mutex::new("Database Connection"));
    let resource2 = Arc::new(Mutex::new("File Handle"));

    // Thread 1: Locks resource1 first, then resource2
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        println!("Thread 1: Attempting to lock Database Connection...");
        let _guard1 = r1_clone.lock().unwrap();
        println!("Thread 1: ✅ Got Database Connection, working for 200ms...");
        
        // Simulate some work while holding the first lock
        thread::sleep(Duration::from_millis(200));
        
        println!("Thread 1: Now attempting to lock File Handle...");
        let _guard2 = r2_clone.lock().unwrap(); // 🔒 DEADLOCK POINT!
        println!("Thread 1: ✅ Got both resources! (This won't print)");
    });

    // Thread 2: Locks resource2 first, then resource1 (OPPOSITE ORDER!)
    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        println!("Thread 2: Attempting to lock File Handle...");
        let _guard2 = r2_clone2.lock().unwrap();
        println!("Thread 2: ✅ Got File Handle, working for 200ms...");
        
        // Simulate some work while holding the first lock
        thread::sleep(Duration::from_millis(200));
        
        println!("Thread 2: Now attempting to lock Database Connection...");
        let _guard1 = r1_clone2.lock().unwrap(); // 🔒 DEADLOCK POINT!
        println!("Thread 2: ✅ Got both resources! (This won't print)");
    });

    // This will hang forever! 
    println!("Main thread: Waiting for threads to complete...");
    println!("⏰ If you see this message for more than 5 seconds, you have a deadlock!");
    
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ Threads completed (this won't print until deadlock is fixed)");
    
    // ✅ CHECKPOINT 1: Understand why this deadlocks
    // Progress: [█░░░░░░░░░] 16.7% Complete
}

// Exercise 4.2: Fix with Lock Ordering
fn lock_ordering_solution() {
    println!("\n=== Solution 1: Lock Ordering ===");
    
    let resource1 = Arc::new(Mutex::new("Database Connection"));
    let resource2 = Arc::new(Mutex::new("File Handle"));

    // FIXME: Both threads should acquire locks in the SAME order!
    // Rule: Always lock resource1 before resource2
    
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        println!("Thread 1: Locking resources in order: DB first, then File");
        let _guard1 = r1_clone.lock().unwrap();
        println!("Thread 1: ✅ Got Database Connection");
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = r2_clone.lock().unwrap();
        println!("Thread 1: ✅ Got File Handle - success!");
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        println!("Thread 2: Locking resources in order: DB first, then File");
        // FIXME: Change this to lock resource1 first, then resource2
        let _guard2 = r2_clone2.lock().unwrap(); // BUG: Wrong order!
        println!("Thread 2: ✅ Got File Handle");
        thread::sleep(Duration::from_millis(100));
        
        let _guard1 = r1_clone2.lock().unwrap(); // BUG: Wrong order!
        println!("Thread 2: ✅ Got Database Connection - success!");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ Lock ordering solution completed!");
    
    // ✅ CHECKPOINT 2: Implement consistent lock ordering
    // Progress: [██░░░░░░░░] 33.3% Complete
}

// Exercise 4.3: Fix with Timeout
fn timeout_solution() {
    println!("\n=== Solution 2: Timeout-based Locking ===");
    
    let resource1 = Arc::new(Mutex::new("Database Connection"));
    let resource2 = Arc::new(Mutex::new("File Handle"));

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        println!("Thread 1: Trying to lock DB with timeout...");
        let _guard1 = r1_clone.lock().unwrap();
        println!("Thread 1: ✅ Got Database Connection");
        thread::sleep(Duration::from_millis(100));
        
        // FIXME: Use try_lock or implement timeout mechanism
        // Hint: Rust's Mutex doesn't have try_lock_for, but has try_lock()
        match r2_clone.try_lock() {
            Ok(_guard2) => println!("Thread 1: ✅ Got File Handle with try_lock!"),
            Err(_) => println!("Thread 1: ❌ Could not get File Handle, retrying..."),
        }
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        println!("Thread 2: Trying to lock File with timeout...");
        let _guard2 = r2_clone2.lock().unwrap();
        println!("Thread 2: ✅ Got File Handle");
        thread::sleep(Duration::from_millis(100));
        
        // FIXME: Use try_lock pattern here too
        match r1_clone2.try_lock() {
            Ok(_guard1) => println!("Thread 2: ✅ Got Database Connection with try_lock!"),
            Err(_) => println!("Thread 2: ❌ Could not get Database Connection, retrying..."),
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ Timeout solution completed!");
    
    // ✅ CHECKPOINT 3: Implement timeout-based locking
    // Progress: [███░░░░░░░] 50% Complete
}

// Exercise 4.4: Advanced Try-Lock Pattern
fn try_lock_solution() {
    println!("\n=== Solution 3: Try-Lock with Retry ===");
    
    let resource1 = Arc::new(Mutex::new("Database Connection"));
    let resource2 = Arc::new(Mutex::new("File Handle"));

    // Helper function for retry logic
    fn acquire_both_resources(
        name: &str,
        res1: &Arc<Mutex<&str>>,
        res2: &Arc<Mutex<&str>>,
        max_retries: u32
    ) -> Result<(), String> {
        for attempt in 1..=max_retries {
            println!("{}: Attempt {} to acquire both resources", name, attempt);
            
            // FIXME: Implement proper try-lock pattern
            // Try to get first lock
            if let Ok(_guard1) = res1.try_lock() {
                println!("{}: ✅ Got first resource", name);
                
                // Try to get second lock
                if let Ok(_guard2) = res2.try_lock() {
                    println!("{}: ✅ Got both resources successfully!", name);
                    thread::sleep(Duration::from_millis(50)); // Do work
                    return Ok(());
                } else {
                    println!("{}: ❌ Could not get second resource, will retry", name);
                }
            } else {
                println!("{}: ❌ Could not get first resource, will retry", name);
            }
            
            // Wait before retry
            thread::sleep(Duration::from_millis(10));
        }
        
        Err(format!("{}: Failed after {} attempts", name, max_retries))
    }

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        match acquire_both_resources("Thread 1", &r1_clone, &r2_clone, 10) {
            Ok(_) => println!("Thread 1: Success!"),
            Err(e) => println!("Thread 1: {}", e),
        }
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        match acquire_both_resources("Thread 2", &r2_clone2, &r1_clone2, 10) {
            Ok(_) => println!("Thread 2: Success!"),
            Err(e) => println!("Thread 2: {}", e),
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ Try-lock solution completed!");
    
    // ✅ CHECKPOINT 4: Implement retry-based try-lock pattern
    // Progress: [████░░░░░░] 66.7% Complete
}

// Exercise 4.5: Hierarchical Locking
fn hierarchical_locking() {
    println!("\n=== Solution 4: Hierarchical Locking ===");
    
    // Resources with hierarchy levels (lower numbers locked first)
    struct Resource {
        name: String,
        level: u32,
        data: Mutex<String>,
    }
    
    let resource1 = Arc::new(Resource {
        name: "Database".to_string(),
        level: 1, // Lower level = acquired first
        data: Mutex::new("DB Data".to_string()),
    });
    
    let resource2 = Arc::new(Resource {
        name: "FileSystem".to_string(),  
        level: 2, // Higher level = acquired second
        data: Mutex::new("File Data".to_string()),
    });

    // FIXME: Implement lock_resources function that respects hierarchy
    fn lock_resources(
        thread_name: &str,
        resources: Vec<&Arc<Resource>>
    ) {
        // TODO: Sort resources by level before locking
        println!("{}: Locking resources in hierarchy order", thread_name);
        
        // Sort by level (implement this!)
        let mut sorted_resources = resources;
        // sorted_resources.sort_by_key(|r| r.level); // Implement this!
        
        let mut guards = Vec::new();
        for resource in sorted_resources {
            println!("{}: Locking {} (level {})", thread_name, resource.name, resource.level);
            let guard = resource.data.lock().unwrap();
            guards.push(guard);
        }
        
        println!("{}: ✅ Got all resources in hierarchy order!", thread_name);
        thread::sleep(Duration::from_millis(50));
    }

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        lock_resources("Thread 1", vec![&r1_clone, &r2_clone]);
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        lock_resources("Thread 2", vec![&r2_clone2, &r1_clone2]); // Different order, but hierarchy will sort
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ Hierarchical locking completed!");
    
    // ✅ CHECKPOINT 5: Implement hierarchical lock ordering
    // Progress: [█████░░░░░] 83.3% Complete
}

// Exercise 4.6: Advanced Deadlock Detection
fn advanced_deadlock_detection() {
    println!("\n=== Advanced: Deadlock Detection ===");
    
    // TODO: Implement a simple deadlock detector
    // This is advanced - implement a wait-for graph or timeout detection
    
    println!("🚧 Advanced deadlock detection - implement if you want extra credit!");
    
    // Ideas to implement:
    // 1. Timeout-based detection (if lock takes > X seconds, assume deadlock)
    // 2. Wait-for graph (track which thread is waiting for which resource)
    // 3. Banker's algorithm for deadlock prevention
    
    // ✅ CHECKPOINT 6: Understand advanced deadlock detection concepts
    // Progress: [██████████] 100% Complete
}

// COMPILATION CHALLENGES:
// 1. Fix the lock ordering in lock_ordering_solution()
// 2. Implement try_lock patterns with proper error handling
// 3. Add retry logic for failed lock acquisitions
// 4. Implement hierarchical resource sorting
// 5. Handle lock failures gracefully without panicking
// 6. Understand different deadlock prevention strategies

// LEARNING OBJECTIVES:
// - Identifying classic deadlock patterns (circular wait)
// - Lock ordering as primary deadlock prevention
// - Non-blocking lock attempts with try_lock()
// - Timeout and retry strategies
// - Hierarchical locking for complex systems
// - Performance implications of different approaches

// C# COMPARISON:
// C# Monitor.TryEnter(obj, timeout):
// ```csharp
// if (Monitor.TryEnter(resource1, TimeSpan.FromSeconds(1)))
// {
//     try
//     {
//         if (Monitor.TryEnter(resource2, TimeSpan.FromSeconds(1)))
//         {
//             // Got both locks
//         }
//     }
//     finally { Monitor.Exit(resource1); }
// }
// ```
//
// Rust try_lock():
// ```rust
// if let Ok(guard1) = resource1.try_lock() {
//     if let Ok(guard2) = resource2.try_lock() {
//         // Got both locks, guards auto-release
//     }
// }
// ```

// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Understand deadlock): [ ] Complete
// Checkpoint 2 (Lock ordering): [ ] Complete  
// Checkpoint 3 (Timeout locking): [ ] Complete
// Checkpoint 4 (Try-lock retry): [ ] Complete
// Checkpoint 5 (Hierarchical locking): [ ] Complete
// Checkpoint 6 (Advanced detection): [ ] Complete

// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Deadlock identification and analysis
// ✅ Multiple deadlock prevention strategies
// ✅ Non-blocking lock acquisition patterns
// ✅ Performance vs safety trade-offs
// ✅ Advanced concurrency debugging skills

// 🚀 Ready for the next challenge?
// Move on to ex05-async-tokio.rs to explore async deadlocks!
// Or check your work with: `cargo run --bin ex04-deadlock`
