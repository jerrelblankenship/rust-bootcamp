# Exercise 8 Hints - Level 2 (Specific Guidance)

## 🎯 More specific hints for SIMD optimization...

### Checkpoint 1: Vectorized Arithmetic
```rust
// PROBLEM: Scalar operations
for i in 0..a.len() {
    result[i] = a[i] + b[i];  // One addition at a time
}

// HINT: Use iterator methods that auto-vectorize
let result: Vec<f32> = a.iter()
    .zip(b.iter())
    .map(|(&x, &y)| x + y)
    .collect();
// Compiler often vectorizes this automatically
```

### Checkpoint 2: Iterator-Based Math
```rust
// PROBLEM: Scalar mathematical operations
for i in 0..data.len() {
    results[i] = data[i].sin() + data[i].cos();
}

// HINT: Use iterator methods for potential auto-vectorization
let results: Vec<f32> = data.iter()
    .map(|&x| x.sin() + x.cos())
    .collect();
// Some compilers/libraries can vectorize math functions
```

### Checkpoint 3: Better Search Algorithms
```rust
// PROBLEM: Linear search O(n) for each needle
for &needle in &needles {
    for (pos, &value) in haystack.iter().enumerate() {
        if value == needle { break; }
    }
}

// HINT: Use binary search O(log n) since data is sorted
for &needle in &needles {
    if let Ok(pos) = haystack.binary_search(&needle) {
        found_positions.push(pos);
    }
}
// Or use vectorized search if available
```

### Checkpoint 4: Built-in Aggregation Methods
```rust
// PROBLEM: Manual aggregation
let mut sum = 0.0;
let mut min_val = f64::INFINITY;
let mut max_val = f64::NEG_INFINITY;
for &value in &data {
    sum += value;
    if value < min_val { min_val = value; }
    if value > max_val { max_val = value; }
}

// HINT: Use iterator methods that are often vectorized
let sum: f64 = data.iter().sum();
let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
// Or: let min_val = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
```

## 🔧 SIMD Optimization Strategies
1. **Use iterator methods**: Often auto-vectorized by compiler
2. **Prefer bulk operations**: sum(), min(), max() over manual loops  
3. **Consider data layout**: Structure of Arrays for better vectorization
4. **Algorithm choice**: Sometimes better algorithm > SIMD scalar algorithm

## 📊 Auto-vectorization Tips
- **Simple loops**: More likely to be vectorized
- **No branches**: Conditional logic prevents vectorization
- **Aligned data**: Better performance with aligned memory
- **Primitive types**: f32, f64, i32 vectorize better than complex types

Ready for complete solutions? Try Level 3! 🚀