# Exercise 7 Hints - Level 2 (Specific Guidance)

## 🎯 More specific hints for blocking optimization...

### Checkpoint 1: Concurrent Execution
```rust
// PROBLEM: Sequential blocking (10 × 20ms = 200ms)
for i in 0..10 {
    let result = simulate_blocking_work(i);  // Blocks everything
    results.push(result);
}

// HINT: Use threads to run concurrently
let mut handles = vec![];
for i in 0..10 {
    let handle = thread::spawn(move || simulate_blocking_work(i));
    handles.push(handle);
}
let results: Vec<u64> = handles.into_iter().map(|h| h.join().unwrap()).collect();
// Should take ~20ms instead of 200ms
```

### Checkpoint 2: Batch Small Operations
```rust
// PROBLEM: 100 threads for tiny operations
for i in 0..100 {
    let handle = thread::spawn(move || tiny_operation(i));
    handles.push(handle);
}

// HINT: Group operations to reduce thread overhead
let chunk_size = 25;  // 4 threads instead of 100
for chunk_start in (0..100).step_by(chunk_size) {
    let handle = thread::spawn(move || {
        (chunk_start..chunk_start + chunk_size)
            .map(|i| tiny_operation(i))
            .sum()
    });
    handles.push(handle);
}
```

### Checkpoint 3: Concurrent I/O
```rust
// PROBLEM: Sequential I/O (20 × 25ms = 500ms)
for i in 0..20 {
    let result = simulate_blocking_io(i);  // Each blocks
    results.push(result);
}

// HINT: Overlap I/O operations
let handles: Vec<_> = (0..20)
    .map(|i| thread::spawn(move || simulate_blocking_io(i)))
    .collect();
let results: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();
// Should take ~25ms instead of 500ms
```

### Checkpoint 4: Pipeline Processing
```rust
// PROBLEM: Sequential producer-consumer
for _ in 0..50 {
    let value = rx.recv().unwrap();           // Wait for producer
    let processed = expensive_processing(value);  // Then process
    results.push(processed);
}

// HINT: Use multiple consumer threads
let handles: Vec<_> = (0..4).map(|_| {
    let rx = rx.clone();
    thread::spawn(move || {
        let mut local_results = Vec::new();
        while let Ok(value) = rx.recv() {
            local_results.push(expensive_processing(value));
        }
        local_results
    })
}).collect();
```

## 🚀 Performance Patterns
- **I/O Bound**: Use concurrency to overlap waits
- **CPU Bound**: Use parallelism for multiple cores  
- **Mixed**: Pipeline pattern to keep both busy
- **Small Tasks**: Batch to reduce overhead

Ready for complete solutions? Try Level 3! 🚀