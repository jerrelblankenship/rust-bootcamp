# Exercise 01 - Level 2 Hints 🟡

## Specific Solutions

### Checkpoint 1: Basic Thread Spawning
```rust
// Problem: data is borrowed by the closure
let data = vec![1, 2, 3, 4, 5];
let handle = thread::spawn(|| {
    println!("Thread data: {:?}", data); // ERROR
});
```

**Fix**: Use `move` to transfer ownership:
```rust
let data = vec![1, 2, 3, 4, 5];
let handle = thread::spawn(move || {  // Add 'move'
    println!("Thread data: {:?}", data);
});
```

### Checkpoint 2: Multiple Threads
```rust
// Problem: Can't move data into multiple closures
let numbers = vec![10, 20, 30];
let handles: Vec<_> = (0..3).map(|i| {
    thread::spawn(|| {
        println!("Thread {}: {:?}", i, numbers); // ERROR
    })
}).collect();
```

**Fix**: Clone the data for each thread:
```rust
let numbers = vec![10, 20, 30];
let handles: Vec<_> = (0..3).map(|i| {
    let numbers_clone = numbers.clone();  // Clone for each thread
    thread::spawn(move || {  // Move the clone
        println!("Thread {}: {:?}", i, numbers_clone);
    })
}).collect();
```

### Checkpoint 3: Return Values
Threads can't directly modify variables in the parent scope. Use channels or return values from `join()`.

## C# Comparison

In C#: Variables are captured by reference (dangerous but convenient)
In Rust: You must explicitly choose move or clone (safe but requires thought)

Need the complete solution? Check Level 3 hints!