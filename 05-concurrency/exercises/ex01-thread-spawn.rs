// Exercise 01: Thread Spawning Basics
// BROKEN: Multiple thread spawning issues to fix!

use std::thread;
use std::time::Duration;

fn main() {
    println!("🧵 Thread Spawning Exercise");
    println!("Progress: [░░░░░░░░░░] 0% Complete");
    
    // TODO: Fix all compilation errors to make this work!
    
    // CHECKPOINT 1: Basic thread spawning
    // FIXME: This won't compile - ownership issue!
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(|| {
        println!("Thread data: {:?}", data); // ERROR: Capture of moved value
    });
    handle.join().unwrap();
    
    println!("✅ Checkpoint 1 complete!");
    println!("Progress: [██░░░░░░░░] 20% Complete");
    
    // CHECKPOINT 2: Multiple threads with shared data
    // FIXME: Can't use the same data in multiple threads!
    let numbers = vec![10, 20, 30];
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(|| {
            println!("Thread {}: {:?}", i, numbers); // ERROR: Multiple ownership
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✅ Checkpoint 2 complete!");
    println!("Progress: [████░░░░░░] 40% Complete");
    
    // CHECKPOINT 3: Thread with return value
    // FIXME: Can't capture mutable reference!
    let mut result = 0;
    let handle = thread::spawn(|| {
        result = 42; // ERROR: Can't capture mutable reference
    });
    handle.join().unwrap();
    println!("Result: {}", result);
    
    println!("✅ Checkpoint 3 complete!");
    println!("Progress: [██████░░░░] 60% Complete");
    
    // CHECKPOINT 4: Scoped threads (advanced)
    // FIXME: Regular threads can't borrow local data!
    let local_data = "Hello from main thread!";
    thread::spawn(|| {
        println!("Borrowed: {}", local_data); // ERROR: Lifetime issue
    }).join().unwrap();
    
    println!("✅ Checkpoint 4 complete!");
    println!("Progress: [████████░░] 80% Complete");
    
    // CHECKPOINT 5: Thread panic handling
    // FIXME: How do we handle thread panics gracefully?
    let handle = thread::spawn(|| {
        panic!("Thread panicked!"); // This will crash the thread
    });
    
    // Handle the panic without crashing main thread
    match handle.join() {
        Ok(_) => println!("Thread completed successfully"),
        Err(_) => println!("Thread panicked, but we handled it!"),
    }
    
    println!("✅ All checkpoints complete!");
    println!("Progress: [██████████] 100% Complete");
    
    println!("\n🎉 Exercise 1 completed! You've mastered basic thread spawning!");
}

/*
COMPILATION CHALLENGES:
1. error[E0382]: borrow of moved value: `data`
2. error[E0382]: borrow of moved value: `numbers`
3. error[E0596]: cannot borrow `result` as mutable
4. error[E0373]: closure may outlive the current function

C# COMPARISON:
In C# you might write:
```csharp
var data = new List<int> {1, 2, 3, 4, 5};
var task = Task.Run(() => {
    Console.WriteLine($"Task data: {string.Join(", ", data)}");
});
await task;
```

In Rust, you need to be explicit about ownership!

HINTS:
- Level 1: Think about ownership - who owns the data?
- Level 2: Consider using `move` keyword and cloning
- Level 3: Look into `Arc` for shared ownership
*/