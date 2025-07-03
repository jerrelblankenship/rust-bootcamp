# Performance Principles

Fundamental principles of performance optimization in Rust.

## 🎯 Core Performance Philosophy

### The Performance Hierarchy

1. **Algorithm** - O(n) vs O(n²) matters more than micro-optimizations
2. **Data Structures** - Right data structure for the access pattern
3. **Memory Layout** - Cache-friendly data organization
4. **Allocation Patterns** - Minimize heap allocations
5. **Parallelization** - Use multiple cores when beneficial
6. **Micro-optimizations** - SIMD, bounds checking, etc.

### Measurement-Driven Optimization

> "In God we trust. All others must bring data." - W. Edwards Deming

```rust
// Always measure before optimizing
use std::time::Instant;

fn benchmark_approach() {
    let start = Instant::now();
    
    // Your code here
    expensive_operation();
    
    let duration = start.elapsed();
    println!("Operation took: {:?}", duration);
}
```

## 📊 Performance Mental Models

### Cost Hierarchy (Approximate CPU Cycles)

| Operation | Cost | Example |
|-----------|------|---------|
| CPU Register | 1 | Local variable access |
| L1 Cache | 3-4 | Recent memory access |
| L2 Cache | 10-20 | Nearby memory access |
| L3 Cache | 40-45 | Shared cache access |
| RAM | 100-300 | Main memory access |
| SSD | 100,000+ | File system access |
| Network | 10,000,000+ | Remote API call |

**Key Insight**: The gap between cache levels is enormous. Keep hot data close!

### Memory Access Patterns

```rust
// ✅ Good: Sequential access (cache-friendly)
fn process_sequential(data: &[i32]) -> i32 {
    data.iter().sum()  // Prefetcher loves this
}

// ❌ Bad: Random access (cache-hostile)
fn process_random(data: &[i32], indices: &[usize]) -> i32 {
    indices.iter().map(|&i| data[i]).sum()  // Cache misses galore
}

// ✅ Better: Sort indices first if possible
fn process_random_optimized(data: &[i32], mut indices: Vec<usize>) -> i32 {
    indices.sort_unstable();  // Make access more sequential
    indices.iter().map(|&i| data[i]).sum()
}
```

## 🚀 Algorithm-Level Optimization

### Choose the Right Big-O

```rust
// ❌ O(n²) - Quadratic
fn find_duplicates_slow(items: &[String]) -> Vec<String> {
    let mut duplicates = Vec::new();
    for i in 0..items.len() {
        for j in i+1..items.len() {
            if items[i] == items[j] {
                duplicates.push(items[i].clone());
            }
        }
    }
    duplicates
}

// ✅ O(n) - Linear with HashSet
use std::collections::HashSet;

fn find_duplicates_fast(items: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    
    for item in items {
        if !seen.insert(item) {
            duplicates.insert(item.clone());
        }
    }
    
    duplicates.into_iter().collect()
}
```

### Data Structure Selection

| Use Case | Optimal Structure | Why |
|----------|------------------|-----|
| Sequential access | `Vec<T>` | Cache-friendly, minimal overhead |
| Random access by index | `Vec<T>` | O(1) indexing |
| Frequent insertions/deletions | `VecDeque<T>` | O(1) at both ends |
| Key-value lookup | `HashMap<K,V>` | O(1) average lookup |
| Sorted data | `BTreeMap<K,V>` | O(log n) lookup, maintains order |
| Unique items | `HashSet<T>` | O(1) membership testing |
| Priority queue | `BinaryHeap<T>` | O(log n) push/pop |

## 🧠 Memory Optimization Principles

### Allocation Hierarchy

```rust
// ✅ Best: Stack allocation (free!)
fn stack_allocation() {
    let data = [0; 1000];  // Stack allocated
    process_data(&data);
}

// ✅ Good: Pre-allocated Vec
fn preallocated_vec(capacity: usize) {
    let mut data = Vec::with_capacity(capacity);  // No reallocations
    for i in 0..capacity {
        data.push(i);
    }
}

// ❌ Bad: Growing Vec
fn growing_vec(size: usize) {
    let mut data = Vec::new();  // Will reallocate many times
    for i in 0..size {
        data.push(i);  // Triggers reallocations
    }
}
```

### Memory Layout Optimization

```rust
// ❌ Bad: Array of Structs (AoS) - poor cache utilization
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

fn process_aos(points: &[Point]) -> f32 {
    points.iter().map(|p| p.x).sum()  // Only need x, but load x,y,z
}

// ✅ Good: Struct of Arrays (SoA) - better cache utilization
struct Points {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

impl Points {
    fn sum_x(&self) -> f32 {
        self.x.iter().sum()  // Only load x values
    }
}
```

## ⚡ Iterator and Closure Optimization

### Zero-Cost Abstractions

```rust
// These compile to identical assembly:

// Imperative style
fn sum_imperative(data: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..data.len() {
        if data[i] % 2 == 0 {
            sum += data[i] * data[i];
        }
    }
    sum
}

// Functional style (zero-cost!)
fn sum_functional(data: &[i32]) -> i32 {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}
```

### Iterator Fusion

```rust
// ✅ Iterator chain - fuses into single loop
let result: Vec<_> = data
    .iter()
    .filter(|&&x| x > 0)      // No intermediate allocation
    .map(|&x| x * 2)          // Fused with filter
    .take(100)                // Fused with map
    .collect();               // Single allocation

// ❌ Eager evaluation - multiple passes and allocations
let filtered: Vec<_> = data.iter().filter(|&&x| x > 0).cloned().collect();
let mapped: Vec<_> = filtered.iter().map(|&x| x * 2).collect();
let taken: Vec<_> = mapped.iter().take(100).cloned().collect();
```

## 🔄 Parallelization Principles

### When to Parallelize

```rust
use rayon::prelude::*;

// ✅ Good: CPU-bound work with minimal coordination
fn parallel_computation(data: &[f64]) -> Vec<f64> {
    data.par_iter()
        .map(|&x| expensive_math_function(x))  // Independent work
        .collect()
}

// ❌ Bad: Coordination overhead exceeds benefit
fn bad_parallel_sum(data: &[i32]) -> i32 {
    data.par_iter().sum()  // Sequential sum is faster for simple operations
}
```

### Parallelization Patterns

```rust
// Embarrassingly parallel - perfect for rayon
let results: Vec<_> = large_dataset
    .par_iter()
    .map(|item| process_independently(item))
    .collect();

// Reduction pattern
let total = large_dataset
    .par_iter()
    .map(|item| compute_value(item))
    .reduce(|| 0, |a, b| a + b);

// Parallel sorting
let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
data.par_sort_unstable();  // Often faster than sequential for large data
```

## 🎛️ Compiler Optimization Principles

### Release Mode vs Debug

```toml
# Cargo.toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization, slower compile
panic = "abort"       # Smaller binary, faster execution
```

### Helping the Optimizer

```rust
// ✅ Help inlining decisions
#[inline]
fn small_hot_function(x: i32) -> i32 {
    x * x + 1
}

#[inline(never)]
fn large_cold_function() {
    // Large function that shouldn't be inlined
}

// ✅ Help bounds checking elimination
fn safe_access(data: &[i32], index: usize) -> i32 {
    if index < data.len() {
        unsafe { *data.get_unchecked(index) }  // Bounds check eliminated
    } else {
        0
    }
}

// ✅ Use likely/unlikely for branch prediction
fn process_with_hint(value: i32) -> i32 {
    if std::intrinsics::likely(value > 0) {
        expensive_positive_case(value)
    } else {
        simple_negative_case(value)
    }
}
```

## 📏 Measurement and Profiling

### Micro-benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    let data = vec![1; 1000];
    
    c.bench_function("my_function", |b| {
        b.iter(|| {
            // black_box prevents over-optimization
            my_function(black_box(&data))
        })
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

### Profiling Tools

```bash
# CPU profiling with perf
perf record --call-graph dwarf ./my_program
perf report

# Memory profiling with valgrind
valgrind --tool=massif ./my_program

# Heap profiling with heaptrack
heaptrack ./my_program

# Rust-specific profiling
cargo install flamegraph
cargo flamegraph --bin my_program
```

## 🎯 Optimization Decision Framework

### When to Optimize

1. **Measure first** - Is there actually a performance problem?
2. **Profile** - Where is the time actually spent?
3. **Prioritize** - Focus on the biggest bottlenecks
4. **Algorithm first** - Better algorithm beats micro-optimizations
5. **Data structures** - Right structure for access patterns
6. **Memory patterns** - Allocation and cache optimization
7. **Parallelization** - Use multiple cores if beneficial
8. **Micro-optimizations** - Only after everything else

### When NOT to Optimize

- **Premature optimization** - Before you have a performance problem
- **Unmeasured optimization** - Without profiling data
- **Micro-optimizing hot paths** - When algorithm is the real problem
- **Over-engineering** - Trading maintainability for marginal gains
- **Cargo cult optimization** - Copying optimizations without understanding

## 📋 Performance Checklist

### Before You Start
- [ ] Do you have a measurable performance problem?
- [ ] Have you profiled to find the actual bottleneck?
- [ ] Is the algorithm fundamentally efficient?

### Optimization Steps
- [ ] Choose optimal data structures for access patterns
- [ ] Minimize allocations (pre-allocate, reuse)
- [ ] Optimize memory layout for cache efficiency
- [ ] Use iterator fusion for zero-cost abstractions
- [ ] Consider parallelization for CPU-bound work
- [ ] Apply micro-optimizations selectively

### After Optimization
- [ ] Measure the improvement
- [ ] Ensure correctness is maintained
- [ ] Document the optimization rationale
- [ ] Consider maintainability impact

## 🎓 Key Takeaways

1. **Measure, don't guess** - Performance intuition is often wrong
2. **Algorithm first** - Big-O improvements beat micro-optimizations
3. **Memory matters** - Cache behavior dominates modern performance
4. **Rust helps** - Zero-cost abstractions make fast code readable
5. **Profile everything** - The bottleneck is rarely where you think
6. **Stop when good enough** - Perfect is the enemy of good enough

Remember: The fastest code is the code that doesn't run. Sometimes the best optimization is eliminating work entirely!