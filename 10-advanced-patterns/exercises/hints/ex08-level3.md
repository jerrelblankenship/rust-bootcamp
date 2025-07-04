# Exercise 08 - Level 3 Hints: Zero-Cost Abstractions

## 🎯 Complete Solutions

Here are the optimized implementations for each checkpoint:

## 🔧 Checkpoint 1: Iterator Performance Fix

```rust
fn process_numbers_optimized(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .collect()
}

// Even more optimized with pre-allocation
fn process_numbers_zero_cost(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(numbers.len());
    for num in numbers {
        if num > 0 {
            result.push(num * 2);
        }
    }
    result
}
```

## 🔧 Checkpoint 2: Generic Operations Fix

```rust
// Add inline hint for aggressive optimization
#[inline]
fn generic_operation<T>(value: T) -> T
where
    T: std::fmt::Debug,
{
    // Remove unnecessary clone
    println!("Debug: {:?}", value);
    value  // Return original, no cloning
}

// For cases where you need a copy
#[inline]
fn generic_operation_with_copy<T>(value: &T) -> T
where
    T: Clone + std::fmt::Debug,
{
    println!("Debug: {:?}", value);
    value.clone()
}
```

## 🔧 Checkpoint 3: Static Dispatch Fix

```rust
// Use generics instead of trait objects
fn process_with_static_dispatch<P: Processor>(processors: &[P], value: i32) -> Vec<i32> {
    processors
        .iter()
        .map(|p| p.process(value))
        .collect()
}

// For mixed processor types, use enum dispatch
enum ProcessorType {
    Doubler(Doubler),
    Tripler(Tripler),
}

impl Processor for ProcessorType {
    #[inline]
    fn process(&self, value: i32) -> i32 {
        match self {
            ProcessorType::Doubler(d) => d.process(value),
            ProcessorType::Tripler(t) => t.process(value),
        }
    }
}

fn process_mixed_optimized(processors: Vec<ProcessorType>, value: i32) -> Vec<i32> {
    processors
        .iter()
        .map(|p| p.process(value))
        .collect()
}
```

## 🔧 Checkpoint 4: String Operations Fix

```rust
fn format_messages_optimized(messages: Vec<String>) -> String {
    // Calculate exact capacity needed
    let total_len = messages.iter().map(|m| m.len() + 2).sum();
    let mut result = String::with_capacity(total_len);
    
    for msg in messages {
        result.push('[');
        result.push_str(&msg);
        result.push(']');
    }
    
    result
}

// Alternative: using itertools join with formatting
fn format_messages_functional(messages: Vec<String>) -> String {
    messages
        .into_iter()
        .map(|msg| format!("[{}]", msg))
        .collect::<String>()
}
```

## 🔧 Checkpoint 5: Closure Optimization Fix

```rust
#[inline]
fn apply_operations_optimized<F>(values: Vec<i32>, op: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    values
        .into_iter()
        .map(op)
        .collect()
}

// For maximum performance, monomorphization
fn apply_doubling_optimized(values: Vec<i32>) -> Vec<i32> {
    values
        .into_iter()
        .map(|x| x * 2)
        .collect()
}

// Generic with inline operation
fn apply_inline_operation<T, F>(values: Vec<T>, op: F) -> Vec<T>
where
    F: Fn(T) -> T,
{
    values.into_iter().map(op).collect()
}
```

## 🔧 Checkpoint 6: HashMap Optimization Fix

```rust
fn count_occurrences_optimized(items: Vec<String>) -> HashMap<String, usize> {
    let mut counts = HashMap::with_capacity(items.len());
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

// Using fold for functional style
fn count_occurrences_functional(items: Vec<String>) -> HashMap<String, usize> {
    items.into_iter().fold(
        HashMap::with_capacity(items.len()),
        |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        }
    )
}
```

## 🔧 Checkpoint 7: Custom Iterator Fix

```rust
// Add size hint for better optimization
impl Iterator for NumberRange {
    type Item = i32;
    
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
    
    // Provide size hint for better collection optimization
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end - self.current).max(0) as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for NumberRange {}

// Alternative: use built-in range
fn create_optimized_range(start: i32, end: i32) -> impl Iterator<Item = i32> {
    start..end
}
```

## 🔧 Checkpoint 8: Zero-Cost Wrapper Fix

```rust
#[repr(transparent)]  // Ensure zero-cost representation
struct SafeInt<T> {
    value: T,
}

impl<T> SafeInt<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    #[inline(always)]
    fn new(value: T) -> Self {
        Self { value }
    }
    
    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        Self::new(self.value + other.value)
    }
    
    #[inline(always)]
    fn get(&self) -> T {
        self.value
    }
    
    #[inline(always)]
    fn into_inner(self) -> T {
        self.value
    }
}

// Manual implementation of common operations
impl<T> std::ops::Add for SafeInt<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}
```

## 🎮 Optimized Benchmark Functions

```rust
fn benchmark_iterator_performance_optimized() {
    let numbers = (0..1000000).collect::<Vec<_>>();
    
    let start = std::time::Instant::now();
    let result_optimized = process_numbers_optimized(numbers.clone());
    let duration_optimized = start.elapsed();
    
    let start = std::time::Instant::now();
    let result_zero_cost = process_numbers_zero_cost(numbers);
    let duration_zero_cost = start.elapsed();
    
    println!("Optimized: {:?}, Zero-cost: {:?}", 
             duration_optimized, duration_zero_cost);
    println!("Results equal: {}", result_optimized == result_zero_cost);
}

fn benchmark_static_dispatch() {
    let processors = vec![Doubler, Tripler, Doubler];
    
    let start = std::time::Instant::now();
    for _ in 0..100000 {
        let _ = process_with_static_dispatch(&processors, 42);
    }
    let duration = start.elapsed();
    
    println!("Static dispatch time: {:?}", duration);
}
```

## 🎮 Complete Optimized Main

```rust
fn main() {
    println!("=== Zero-Cost Abstractions Exercise (Optimized) ===");
    
    // Test optimized iterator performance
    println!("\n--- Optimized Iterator Performance ---");
    benchmark_iterator_performance_optimized();
    
    // Test static dispatch
    println!("\n--- Static Dispatch Performance ---");
    benchmark_static_dispatch();
    
    // Test optimized string operations
    println!("\n--- Optimized String Performance ---");
    let messages = vec!["Hello", "World", "Rust"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    
    let start = std::time::Instant::now();
    let result = format_messages_optimized(messages);
    let duration = start.elapsed();
    
    println!("Optimized string time: {:?}", duration);
    println!("Result: {}", result);
    
    println!("\n🎉 Zero-cost abstractions achieved!");
}
```

## 🔍 Key Optimization Techniques

1. **Use `into_iter()` instead of `iter().map(|&x| x)`** - avoid unnecessary copies
2. **Pre-allocate collections** - use `with_capacity()` when size is known
3. **Static dispatch over dynamic** - use generics instead of trait objects
4. **Inline critical functions** - use `#[inline]` or `#[inline(always)]`
5. **Efficient string operations** - use push/push_str instead of format!
6. **HashMap entry API** - use `entry().or_insert()` pattern
7. **Size hints for iterators** - help collection allocation
8. **`#[repr(transparent)]`** - ensure wrapper types are truly zero-cost

## 🎯 Verification

Compile with optimizations and check assembly:
```bash
rustc -O ex08-zero-cost-abstractions.rs
```

Use tools like `cargo asm` or Godbolt to verify the generated assembly is optimal. Your high-level code should compile to the same efficient machine code as hand-optimized loops!