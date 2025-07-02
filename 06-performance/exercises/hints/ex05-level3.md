# Exercise 5 Hints - Level 3 (Complete Solutions)

## 🔧 Complete solutions for bounds checking optimization...

### Checkpoint 1: Iterator Instead of Indexing
```rust
// COMPLETE SOLUTION:
let sum: i64 = data.iter().map(|&x| x as i64).sum();
// Single line, no bounds checking, compiler optimized
```

### Checkpoint 2: windows() Method
```rust
// COMPLETE SOLUTION:
let results: Vec<f64> = data
    .windows(3)
    .map(|window| (window[0] + window[1] + window[2]) / 3.0)
    .collect();
// Safe, efficient, no manual bounds checking
```

### Checkpoint 3: Direct Array Iteration
```rust
// COMPLETE SOLUTION:
let sum: f64 = data.iter().sum();
// Processes flat array directly, maximum cache efficiency
```

### Checkpoint 4: Strategic unsafe Optimization
```rust
// COMPLETE SOLUTION (when performance is critical):
let mut result = 0.0;
for i in 1..data.len()-1 {
    let diff = unsafe {
        *data.get_unchecked(i+1) - 2.0 * *data.get_unchecked(i) + *data.get_unchecked(i-1)
    };
    result += diff * diff;
}

// SAFER ALTERNATIVE (often just as fast):
let result: f64 = data
    .windows(3)
    .map(|w| {
        let diff = w[2] - 2.0 * w[1] + w[0];
        diff * diff
    })
    .sum();
```

## 🎯 Expected Performance Improvements
- Checkpoint 1: 10ms → 3ms (3x faster, no bounds checking)
- Checkpoint 2: 15ms → 6ms (2.5x faster, efficient windows)
- Checkpoint 3: 8ms → 2ms (4x faster, direct iteration)
- Checkpoint 4: 12ms → 3ms (4x faster, eliminated bounds checks)

## ⚖️ Safety vs Performance Trade-offs

### When to Use Each Approach:
1. **Iterator methods (preferred)**: Safe, fast, clear code
2. **windows() and chunks()**: Safe alternatives to manual slicing
3. **unsafe blocks**: Only when:
   - Profiling shows bounds checking is the bottleneck
   - You can mathematically prove safety
   - The performance gain is significant
   - It's in a well-tested hot path

Remember: Profile first, optimize second. Most code doesn't need unsafe!