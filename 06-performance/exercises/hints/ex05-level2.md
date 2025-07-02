# Exercise 5 Hints - Level 2 (Specific Guidance)

## 🎯 More specific hints for bounds checking optimization...

### Checkpoint 1: Eliminate Array Indexing
```rust
// PROBLEM: Manual indexing with bounds checking
for i in 0..data.len() {
    sum += data[i] as i64;  // Bounds check every time
}

// HINT: What iterator method gives you values directly?
for value in data.iter() {
    sum += *value as i64;  // No bounds checking!
}
// Or even better: data.iter().map(|&x| x as i64).sum()
```

### Checkpoint 2: Use windows() for Sliding Operations
```rust
// PROBLEM: Manual slicing
for i in 0..data.len().saturating_sub(3) {
    let window = &data[i..i+3];  // Bounds check for slice
    let avg = (window[0] + window[1] + window[2]) / 3.0;  // More bounds checks
}

// HINT: The windows() method does this safely and efficiently
for window in data.windows(3) {
    let avg = (window[0] + window[1] + window[2]) / 3.0;
}
```

### Checkpoint 3: Direct Array Iteration
```rust
// PROBLEM: 2D indexing with calculation
for row in 0..SIZE {
    for col in 0..SIZE {
        let index = row * SIZE + col;
        sum += data[index];  // Bounds check
    }
}

// HINT: Process the flat array directly
for &value in data.iter() {
    sum += value;  // No bounds checking, cache friendly
}
```

### Checkpoint 4: Strategic unsafe for Hot Paths
```rust
// PROBLEM: Multiple bounds checks in tight loop
for i in 1..data.len()-1 {
    let diff = data[i+1] - 2.0 * data[i] + data[i-1];  // 3 bounds checks
    result += diff * diff;
}

// HINT: When you can prove safety, unsafe eliminates checks
for i in 1..data.len()-1 {
    let diff = unsafe {
        *data.get_unchecked(i+1) - 2.0 * *data.get_unchecked(i) + *data.get_unchecked(i-1)
    };
    result += diff * diff;
}
// CAUTION: Only use unsafe when you can prove the indices are always valid!
```

## 🔍 When to Optimize Bounds Checking
- **Profile first**: Is it actually the bottleneck?
- **Try iterators**: Often eliminate bounds checking safely
- **Consider unsafe**: Only for proven hot paths where safety is guaranteed
- **Measure impact**: Bounds checking overhead varies by use case

Ready for complete solutions? Try Level 3! 🚀