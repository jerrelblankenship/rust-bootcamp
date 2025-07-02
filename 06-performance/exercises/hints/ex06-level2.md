# Exercise 6 Hints - Level 2 (Specific Guidance)

## 🎯 More specific threading optimization hints...

### Checkpoint 1: Right-size Thread Work
- Small data (1K items) ÷ 4 threads = 250 items per thread
- Thread creation overhead probably > computation time
- **Solution**: Use threading only when work > overhead cost

### Checkpoint 2: Eliminate Lock Contention
```rust
// PROBLEM: Shared lock in hot loop
let mut counter = counter_clone.lock().unwrap();
*counter += result as u64;  // All threads wait here

// HINT: Local aggregation + final combine
let mut local_sum = 0u64;
for j in start_idx..end_idx {
    local_sum += expensive_computation(data_clone[j]) as u64;
}
// Return local_sum, combine results in main thread
```

### Checkpoint 3: Better Work Distribution
```rust
// PROBLEM: Uneven chunks
if x % 100 == 0 {
    very_expensive_computation(x)  // Some threads get all expensive work
} else {
    cheap_computation(x)           // Others get all cheap work
}

// HINT: Distribute expensive items across threads evenly
// Or use work-stealing pattern
```

### Checkpoint 4: Reuse Allocations
```rust
// PROBLEM: Allocation in hot path
let mut temp_strings = Vec::new();  // New allocation every iteration

// HINT: Reuse buffer
let mut reusable_buffer = Vec::with_capacity(5);
// ... use buffer ...
reusable_buffer.clear();  // Reuse for next iteration
```

## 🚀 Threading Best Practices
- **Profile first**: Is the problem actually thread-related?
- **Compare serial vs parallel**: Threading isn't always faster
- **Local then global**: Aggregate locally, combine globally
- **Pool resources**: Reuse expensive-to-create objects

Ready for complete solutions? Try Level 3! 🚀