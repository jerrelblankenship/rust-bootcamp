# Exercise 01 - Level 3 Hints 🔴

## Complete Solutions

### Checkpoint 1: Basic Threading
```rust
let data = vec![1, 2, 3, 4, 5];
let handle = thread::spawn(move || {  // 'move' transfers ownership
    println!("Thread data: {:?}", data);
});
handle.join().unwrap();
```

### Checkpoint 2: Multiple Threads
```rust
let numbers = vec![10, 20, 30];
let handles: Vec<_> = (0..3).map(|i| {
    let numbers_clone = numbers.clone();
    thread::spawn(move || {
        println!("Thread {}: {:?}", i, numbers_clone);
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

### Checkpoint 3: Thread Communication
```rust
let handle = thread::spawn(|| {
    42  // Return value from thread
});
let result = handle.join().unwrap();  // Get the return value
println!("Result: {}", result);
```

### Checkpoint 4: Scoped Threads
```rust
use std::thread;

let local_data = "Hello from main thread!";
thread::scope(|s| {  // Scoped threads can borrow!
    s.spawn(|| {
        println!("Borrowed: {}", local_data);
    });
});  // All scoped threads are joined here
```

### Checkpoint 5: Panic Handling
```rust
let handle = thread::spawn(|| {
    panic!("Thread panicked!");
});

match handle.join() {
    Ok(_) => println!("Thread completed successfully"),
    Err(_) => println!("Thread panicked, but we handled it!"),
}
```

## Key Insights

- **`move`** transfers ownership to the thread
- **Clone** when multiple threads need the same data
- **Scoped threads** can borrow from parent scope
- **`join()`** returns `Result<T, Box<dyn Any + Send>>`

You've learned the foundation of safe threading in Rust!