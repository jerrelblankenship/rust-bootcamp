// Exercise 3: Shared State with Arc and Mutex - Fix the Broken Code!
// 
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (8 checkpoints to fix)
// 
// Your task: Make this shared state code work by fixing
// all the compilation errors and race conditions.
//
// INSTRUCTIONS:
// 1. Fix ONE error at a time - don't try to fix everything at once!
// 2. Compile after each fix: `rustc ex03-shared-state.rs`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (Arc creation)
// - Think about C# lock statements and shared objects
// - Each checkpoint builds concurrent state management understanding
//
// COMPLETED CONCEPTS:
// [] Arc (Atomically Reference Counted) for shared ownership
// [] Mutex (Mutual Exclusion) for thread synchronization
// [] Lock guard automatic releasing and scope management
// [] Lock poisoning detection and recovery
// [] Performance implications of shared state

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 3: Shared State with Arc and Mutex (Fix the Code!) ===\n");

    // Exercise 3.1: Fix shared counter creation
    // FIXME: Wrong type - this won't be shareable between threads
    let counter = Mutex::new(0); // COMPILE ERROR: Can't share Mutex directly!
    let counter_clone = counter; // COMPILE ERROR: Can't move Mutex directly
    
    println!("Initial counter value: {}", *counter.lock().unwrap());
    
    // ✅ CHECKPOINT 1: Fix counter to be shareable with Arc
    // Progress: [█░░░░░░░░░] 12.5% Complete

    // Exercise 3.2: Fix thread creation with shared state
    let mut handles = vec![];
    
    for i in 1..=5 {
        // FIXME: How do we share the counter between threads?
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // Exercise 3.3: Fix the mutex access
                // FIXME: counter not available in this scope
                let mut count = counter.lock().unwrap(); // COMPILE ERROR: counter not in scope
                *count += 1;
                println!("Thread {} incremented counter to: {}", i, *count);
                // FIXME: What if this lock is held too long?
                thread::sleep(Duration::from_millis(1)); // BAD: Holding lock during sleep!
            }
        });
        handles.push(handle);
    }
    
    // ✅ CHECKPOINT 2: Fix thread spawning with cloned Arc
    // Progress: [██░░░░░░░░] 25% Complete
    
    // ✅ CHECKPOINT 3: Fix mutex access and lock duration
    // Progress: [███░░░░░░░] 37.5% Complete

    // Exercise 3.4: Fix thread joining and final result
    for handle in handles {
        handle.join().unwrap();
    }
    
    // FIXME: How do we access the final counter value?
    println!("Final counter value: {}", *counter.lock().unwrap()); // COMPILE ERROR: counter moved
    
    // ✅ CHECKPOINT 4: Fix final value access
    // Progress: [████░░░░░░] 50% Complete

    // Exercise 3.5: RwLock challenge
    // TODO: Implement reader-writer lock pattern
    println!("\n--- RwLock Challenge ---");
    
    // FIXME: Import RwLock and create shared data for reading/writing
    use std::sync::RwLock; // TODO: Add this import at the top
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Spawn 3 reader threads
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // FIXME: Use read lock to access data
            let reader = data_clone.read().unwrap();
            println!("Reader {}: data length = {}", i, reader.len());
            // Simulate read work
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // Spawn 1 writer thread
    let data_clone = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        // FIXME: Use write lock to modify data
        let mut writer = data_clone.write().unwrap();
        writer.push(6);
        println!("Writer: added element, new length = {}", writer.len());
    });
    handles.push(writer_handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // ✅ CHECKPOINT 5: RwLock should allow concurrent reads
    // Progress: [█████░░░░░] 62.5% Complete

    // Exercise 3.6: Handle lock poisoning
    println!("\n--- Lock Poisoning Challenge ---");
    
    let poisoned_mutex = Arc::new(Mutex::new(0));
    let poisoned_clone = Arc::clone(&poisoned_mutex);
    
    let panic_handle = thread::spawn(move || {
        let _guard = poisoned_clone.lock().unwrap();
        panic!("Oops! This will poison the mutex!");
    });
    
    // Wait for the panic to happen
    let _ = panic_handle.join(); // This will be Err because thread panicked
    
    // FIXME: How do we handle poisoned mutex?
    match poisoned_mutex.lock() {
        Ok(guard) => println!("Got clean lock: {}", *guard),
        Err(poisoned) => {
            // TODO: Handle the poisoned error properly
            println!("Mutex was poisoned! Need to recover..."); // COMPILE ERROR: Don't panic, recover!
        }
    }
    
    // ✅ CHECKPOINT 6: Lock poisoning should be handled gracefully
    // Progress: [██████░░░░] 75% Complete

    // Exercise 3.7: Performance comparison
    println!("\n--- Performance Comparison ---");
    
    // Compare atomic operations vs mutex for simple counter
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mutex_counter = Arc::new(Mutex::new(0usize));
    
    // Test atomic performance
    let start = std::time::Instant::now();
    let mut atomic_handles = vec![];
    
    for _ in 0..4 {
        let counter_clone = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..25000 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        atomic_handles.push(handle);
    }
    
    for handle in atomic_handles {
        handle.join().unwrap();
    }
    
    let atomic_duration = start.elapsed();
    println!("Atomic operations: 100,000 increments in {:?}", atomic_duration);
    
    // Test mutex performance
    let start = std::time::Instant::now();
    let mut mutex_handles = vec![];
    
    for _ in 0..4 {
        let counter_clone = Arc::clone(&mutex_counter);
        let handle = thread::spawn(move || {
            for _ in 0..25000 {
                *counter_clone.lock().unwrap() += 1;
            }
        });
        mutex_handles.push(handle);
    }
    
    for handle in mutex_handles {
        handle.join().unwrap();
    }
    
    let mutex_duration = start.elapsed();
    println!("Mutex operations: 100,000 increments in {:?}", mutex_duration);
    
    println!("Atomic result: {}", atomic_counter.load(Ordering::SeqCst));
    println!("Mutex result: {}", *mutex_counter.lock().unwrap());
    
    // ✅ CHECKPOINT 7: Performance comparison should show atomic advantage
    // Progress: [███████░░░] 87.5% Complete

    // Exercise 3.8: Deadlock prevention
    println!("\n--- Deadlock Prevention ---");
    
    // TODO: Create a scenario that would deadlock and fix it
    let resource1 = Arc::new(Mutex::new("Resource 1"));
    let resource2 = Arc::new(Mutex::new("Resource 2"));
    
    let res1_clone = Arc::clone(&resource1);
    let res2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        // FIXME: This order could cause deadlock
        let _lock1 = res1_clone.lock().unwrap();
        println!("Thread 1: Got resource 1");
        thread::sleep(Duration::from_millis(50));
        let _lock2 = res2_clone.lock().unwrap();
        println!("Thread 1: Got resource 2");
    });
    
    let res1_clone = Arc::clone(&resource1);
    let res2_clone = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        // FIXME: Different order - potential deadlock!
        let _lock2 = res2_clone.lock().unwrap();
        println!("Thread 2: Got resource 2");
        thread::sleep(Duration::from_millis(50));
        let _lock1 = res1_clone.lock().unwrap();
        println!("Thread 2: Got resource 1");
    });
    
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    // ✅ CHECKPOINT 8: Deadlock should be prevented with consistent ordering
    // Progress: [██████████] 100% Complete
    
    println!("\n🎉 Exercise 3 completed! You've mastered shared state management!");
}

// COMPILATION CHALLENGES:
// 1. Wrap Mutex in Arc for shared ownership across threads
// 2. Clone Arc references for each thread
// 3. Use proper lock scope to avoid holding locks too long
// 4. Access final values through Arc after threads complete
// 5. Implement RwLock for reader-writer patterns
// 6. Handle lock poisoning without panicking
// 7. Compare atomic vs mutex performance
// 8. Prevent deadlocks with consistent lock ordering

// LEARNING OBJECTIVES:
// - Understanding Arc vs Rc for thread-safe reference counting
// - Mutex vs RwLock for different access patterns
// - Lock guard scope management and automatic release
// - Lock poisoning scenarios and recovery
// - Performance implications of different synchronization primitives
// - Deadlock prevention strategies

// C# COMPARISON:
// C#: private static readonly object _lock = new object();
// Rust: let data = Arc::new(Mutex::new(value));
//
// C#: lock (_lock) { /* critical section */ }
// Rust: { let guard = mutex.lock().unwrap(); /* critical section */ }
//
// C#: ReaderWriterLockSlim rwLock = new();
// Rust: let rw_lock = Arc::new(RwLock::new(value));
//
// C#: Interlocked.Increment(ref counter);
// Rust: counter.fetch_add(1, Ordering::SeqCst);

// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Arc creation): [ ] Complete
// Checkpoint 2 (Thread spawning): [ ] Complete  
// Checkpoint 3 (Mutex access scope): [ ] Complete
// Checkpoint 4 (Final value access): [ ] Complete
// Checkpoint 5 (RwLock implementation): [ ] Complete
// Checkpoint 6 (Lock poisoning): [ ] Complete
// Checkpoint 7 (Performance comparison): [ ] Complete
// Checkpoint 8 (Deadlock prevention): [ ] Complete

// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Shared ownership patterns with Arc
// ✅ Mutual exclusion with Mutex and RwLock
// ✅ Lock scope management and performance
// ✅ Error handling and lock poisoning
// ✅ Deadlock prevention strategies

// 🚀 Ready for the next challenge?
// Move on to ex04-deadlock.rs to debug real deadlock scenarios!
// Or check your work with: `rustc ex03-shared-state.rs && ./ex03-shared-state`
