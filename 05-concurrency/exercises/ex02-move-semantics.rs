// Exercise 02: Move Semantics with Threads
// BROKEN: Understanding ownership in concurrent contexts

use std::thread;
use std::sync::Arc;

fn main() {
    println!("📦 Move Semantics Exercise");
    println!("Progress: [░░░░░░░░░░] 0% Complete");
    
    // CHECKPOINT 1: The move keyword
    // FIXME: Missing move keyword
    let message = String::from("Hello from thread!");
    let handle = thread::spawn(|| {
        println!("{}", message); // ERROR: Closure may outlive current function
    });
    handle.join().unwrap();
    
    println!("✅ Checkpoint 1 complete!");
    println!("Progress: [██░░░░░░░░] 20% Complete");
    
    // CHECKPOINT 2: Clone vs Move
    // FIXME: Choose between cloning and moving
    let expensive_data = vec![1; 1000000]; // Large vector
    let handle = thread::spawn(|| {
        let sum: usize = expensive_data.iter().sum(); // ERROR: Value moved
        println!("Sum: {}", sum);
    });
    
    // We want to use expensive_data here too!
    println!("Original length: {}", expensive_data.len()); // ERROR: Value moved
    handle.join().unwrap();
    
    println!("✅ Checkpoint 2 complete!");
    println!("Progress: [████░░░░░░] 40% Complete");
    
    // CHECKPOINT 3: Shared ownership with Arc
    // FIXME: Multiple threads need access to the same data
    let shared_data = vec!["Rust", "is", "awesome"];
    
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            println!("Thread {}: {:?}", i, shared_data); // ERROR: Each closure needs ownership
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✅ Checkpoint 3 complete!");
    println!("Progress: [██████░░░░] 60% Complete");
    
    // CHECKPOINT 4: Reference counting
    // FIXME: Implement proper reference counting
    let reference_counted = vec!["Share", "me", "safely"];
    let mut handles = vec![];
    
    for i in 0..5 {
        // Each thread needs a reference to the same data
        handles.push(thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(100));
            println!("Thread {} accessing: {:?}", i, reference_counted); // ERROR: Move occurs
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✅ All checkpoints complete!");
    println!("Progress: [██████████] 100% Complete");
    
    println!("\n🎉 Exercise 2 completed! You understand move semantics in threads!");
}

/*
C# COMPARISON:
In C# all reference types are automatically shared:
```csharp
var data = new List<string> {"Rust", "is", "awesome"};
var tasks = Enumerable.Range(0, 3).Select(i => 
    Task.Run(() => Console.WriteLine($"Task {i}: {string.Join(" ", data)}")));
await Task.WhenAll(tasks);
```

In Rust, you must be explicit about sharing!

KEY CONCEPTS:
- `move` keyword transfers ownership to the closure
- `Clone` creates a copy (expensive for large data)  
- `Arc` provides shared ownership (like shared_ptr in C++)
- Each thread gets its own Arc pointing to the same data

HINTS:
- Level 1: When do you move vs clone vs share?
- Level 2: Arc stands for "Atomically Reference Counted"
- Level 3: Use Arc::clone() to create new references
*/