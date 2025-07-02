# Exercise 1 Hints - Level 2 (Specific Guidance)

## 🎯 More specific hints for Allocation Storm optimization...

### Checkpoint 1: Eliminate Vec Allocation in Loop

**Problem**: Creating 1000 new Vec objects causes allocation storm
```rust
// BAD: Creates new Vec each iteration
for chunk in numbers.chunks(1000) {
    let mut temp = Vec::new();  // 1000 allocations!
    for &num in chunk {
        temp.push(num * 2);     // Plus reallocation as it grows
    }
    results.extend(temp);       // Plus copying
}
```

**Solution approaches**:
1. **Pre-allocate**: `Vec::with_capacity(chunk.len())`
2. **Reuse buffer**: Create one Vec outside the loop, clear it each iteration
3. **Direct extend**: Use `results.extend(chunk.iter().map(|&num| num * 2))`

### Checkpoint 2: Fix String Concatenation

**Problem**: `result = result + &str` creates new String each time
```rust
// BAD: O(n²) complexity like C# string +=
result = result + &format!("Number: {} | ", i);
```

**Solutions**:
- Use `push_str()`: `result.push_str(&format!("Number: {} | ", i))`
- Or avoid format!: `result.push_str("Number: "); result.push_str(&i.to_string()); result.push_str(" | ");`

### Checkpoint 3: Remove Unnecessary Boxing

**Problem**: `Vec<Box<i32>>` puts 4-byte integers on heap
```rust
// BAD: Each integer gets heap allocation
let mut values: Vec<Box<i32>> = Vec::new();
for i in 0..100_000 {
    values.push(Box::new(i));  // Heap allocation per integer!
}
```

**Solution**: Use `Vec<i32>` directly - integers don't need boxing!

### Checkpoint 4: Eliminate Excessive Cloning

**Problem**: Multiple clones create unnecessary allocations
```rust
// BAD: Three allocations per item
let cloned = item.clone();           // Clone 1
let upper = cloned.to_uppercase();   // Clone 2 (new String)
let final_result = upper.clone();    // Clone 3
```

**Solution**: Chain operations without intermediate clones:
```rust
let final_result = item.to_uppercase();  // Just one allocation
```

### Checkpoint 5: HashMap with Capacity

**Problem**: HashMap resizes multiple times as it grows
```rust
// BAD: Will rehash multiple times
let mut word_count = HashMap::new();
```

**Solution**: Pre-allocate capacity:
```rust
let mut word_count = HashMap::with_capacity(1000);  // Estimate size
```

### Checkpoint 6: Reuse Work Buffer

**Problem**: Allocating buffer in hot path
```rust
// BAD: New allocation every iteration
for chunk in data.chunks(100) {
    let mut work_buffer = Vec::new();  // Hot path allocation!
    // ... process
}
```

**Solution**: Reuse buffer:
```rust
let mut work_buffer = Vec::with_capacity(100);
for chunk in data.chunks(100) {
    work_buffer.clear();              // Reuse existing allocation
    work_buffer.extend_from_slice(chunk);
    // ... process
}
```

## 🔧 Implementation Tips
- **Measure each fix**: Run `cargo run --release --bin ex01-allocation-storm` after each change
- **Expect big improvements**: Each fix should cut time by 50%+ 
- **Target times**: Aim for <200ms total (from ~3000ms)

Need more help? Check Level 3 hints! 🎯