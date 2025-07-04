// Exercise 08: Zero-Cost Abstractions - High-Level Code with Low-Level Performance
//
// Learning Objectives:
// - Understand zero-cost abstractions in Rust
// - Debug performance issues in high-level code
// - Learn compiler optimization techniques
// - Compare with C# performance characteristics
//
// C# Analogy: Like LINQ and generics that compile to efficient code,
// but with more predictable performance and no garbage collection overhead.
//
// Your Mission: Fix the performance issues in these high-level abstractions
// to ensure they compile to efficient low-level code.

use std::collections::HashMap;
use std::marker::PhantomData;

// ❌ CHECKPOINT 1: Iterator Performance Issues
// This iterator chain should be zero-cost but has performance problems
// C# equivalent: LINQ chain that doesn't optimize properly
fn process_numbers_slow(numbers: Vec<i32>) -> Vec<i32> {
    // ❌ This creates unnecessary allocations
    let mut result = Vec::new();
    for num in numbers {
        if num > 0 {
            result.push(num * 2);
        }
    }
    result
}

fn process_numbers_better(numbers: Vec<i32>) -> Vec<i32> {
    // ❌ This still has performance issues
    numbers
        .iter()
        .map(|&x| x)
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .collect()
}

// ❌ CHECKPOINT 2: Generic Abstraction Issues
// This generic function should be zero-cost but has overhead
// C# equivalent: Generic method that doesn't inline properly
fn generic_operation<T>(value: T) -> T
where
    T: Clone + std::fmt::Debug,
{
    // ❌ This unnecessary clone creates overhead
    let cloned = value.clone();
    println!("Debug: {:?}", cloned);
    cloned.clone()
}

// ❌ CHECKPOINT 3: Trait Object Performance Issues
// This uses trait objects where zero-cost alternatives exist
// C# equivalent: Interface calls that prevent devirtualization
trait Processor {
    fn process(&self, value: i32) -> i32;
}

struct Doubler;
struct Tripler;

impl Processor for Doubler {
    fn process(&self, value: i32) -> i32 {
        value * 2
    }
}

impl Processor for Tripler {
    fn process(&self, value: i32) -> i32 {
        value * 3
    }
}

// ❌ This uses dynamic dispatch when static dispatch would be better
fn process_with_trait_object(processors: Vec<Box<dyn Processor>>, value: i32) -> Vec<i32> {
    processors
        .iter()
        .map(|p| p.process(value))
        .collect()
}

// ❌ CHECKPOINT 4: String Handling Performance Issues
// This string processing has unnecessary allocations
// C# equivalent: String operations that create too many temporary strings
fn format_messages_slow(messages: Vec<String>) -> String {
    // ❌ This creates many temporary strings
    let mut result = String::new();
    for msg in messages {
        result = result + &format!("[{}]", msg);
    }
    result
}

fn format_messages_better(messages: Vec<String>) -> String {
    // ❌ This still has performance issues
    messages
        .iter()
        .map(|msg| format!("[{}]", msg))
        .collect::<Vec<_>>()
        .join("")
}

// ❌ CHECKPOINT 5: Closure Performance Issues
// This closure usage prevents inlining and optimization
// C# equivalent: Lambda expressions that don't optimize well
fn apply_operations_slow<F>(values: Vec<i32>, op: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // ❌ This doesn't optimize as well as it could
    values
        .into_iter()
        .map(|x| op(x))
        .collect()
}

// ❌ CHECKPOINT 6: HashMap Performance Issues
// This HashMap usage has unnecessary overhead
// C# equivalent: Dictionary operations that don't optimize well
fn count_occurrences_slow(items: Vec<String>) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for item in items {
        // ❌ This creates unnecessary string allocations
        match counts.get(&item) {
            Some(count) => {
                counts.insert(item, count + 1);
            }
            None => {
                counts.insert(item, 1);
            }
        }
    }
    counts
}

// ❌ CHECKPOINT 7: Custom Iterator Performance Issues
// This custom iterator has overhead that should be eliminated
// C# equivalent: Custom IEnumerable that doesn't optimize properly
struct NumberRange {
    start: i32,
    end: i32,
    current: i32,
}

impl NumberRange {
    fn new(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            current: start,
        }
    }
}

impl Iterator for NumberRange {
    type Item = i32;
    
    // ❌ This implementation has performance issues
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

// ❌ CHECKPOINT 8: Zero-Cost Wrapper Performance Issues
// This wrapper should be zero-cost but has overhead
// C# equivalent: Wrapper struct that doesn't optimize away
struct SafeInt<T> {
    value: T,
    _phantom: PhantomData<T>,
}

impl<T> SafeInt<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    fn new(value: T) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
    
    // ❌ This method has unnecessary overhead
    fn add(&self, other: &Self) -> Self {
        Self::new(self.value + other.value)
    }
    
    // ❌ This method should be zero-cost but isn't
    fn get(&self) -> T {
        self.value
    }
}

// Performance test functions
fn benchmark_iterator_performance() {
    let numbers = (0..1000000).collect::<Vec<_>>();
    
    let start = std::time::Instant::now();
    let result_slow = process_numbers_slow(numbers.clone());
    let duration_slow = start.elapsed();
    
    let start = std::time::Instant::now();
    let result_better = process_numbers_better(numbers);
    let duration_better = start.elapsed();
    
    println!("Slow version: {:?}, Better version: {:?}", 
             duration_slow, duration_better);
    println!("Results equal: {}", result_slow == result_better);
}

fn benchmark_generic_performance() {
    let value = 42;
    let start = std::time::Instant::now();
    
    for _ in 0..100000 {
        let _ = generic_operation(value);
    }
    
    let duration = start.elapsed();
    println!("Generic operation time: {:?}", duration);
}

fn benchmark_trait_object_performance() {
    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(Doubler),
        Box::new(Tripler),
    ];
    
    let start = std::time::Instant::now();
    
    for _ in 0..100000 {
        let _ = process_with_trait_object(processors.clone(), 42);
    }
    
    let duration = start.elapsed();
    println!("Trait object time: {:?}", duration);
}

fn benchmark_string_performance() {
    let messages = vec![
        "Hello".to_string(),
        "World".to_string(),
        "Rust".to_string(),
    ];
    
    let start = std::time::Instant::now();
    let result_slow = format_messages_slow(messages.clone());
    let duration_slow = start.elapsed();
    
    let start = std::time::Instant::now();
    let result_better = format_messages_better(messages);
    let duration_better = start.elapsed();
    
    println!("String slow: {:?}, better: {:?}", duration_slow, duration_better);
    println!("Results equal: {}", result_slow == result_better);
}

fn benchmark_closure_performance() {
    let values = (0..100000).collect::<Vec<_>>();
    let operation = |x: i32| x * 2 + 1;
    
    let start = std::time::Instant::now();
    let result = apply_operations_slow(values, operation);
    let duration = start.elapsed();
    
    println!("Closure operation time: {:?}", duration);
    println!("Result length: {}", result.len());
}

fn benchmark_hashmap_performance() {
    let items = vec!["apple", "banana", "apple", "cherry", "banana"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    
    let start = std::time::Instant::now();
    let counts = count_occurrences_slow(items);
    let duration = start.elapsed();
    
    println!("HashMap operation time: {:?}", duration);
    println!("Counts: {:?}", counts);
}

fn benchmark_custom_iterator_performance() {
    let range = NumberRange::new(0, 100000);
    
    let start = std::time::Instant::now();
    let sum: i32 = range.sum();
    let duration = start.elapsed();
    
    println!("Custom iterator time: {:?}", duration);
    println!("Sum: {}", sum);
}

fn benchmark_wrapper_performance() {
    let a = SafeInt::new(10);
    let b = SafeInt::new(20);
    
    let start = std::time::Instant::now();
    
    for _ in 0..100000 {
        let _ = a.add(&b);
    }
    
    let duration = start.elapsed();
    println!("Wrapper operation time: {:?}", duration);
}

fn main() {
    println!("=== Zero-Cost Abstractions Exercise ===");
    
    // Test Checkpoint 1: Iterator performance
    println!("\n--- Iterator Performance ---");
    benchmark_iterator_performance();
    
    // Test Checkpoint 2: Generic performance
    println!("\n--- Generic Performance ---");
    benchmark_generic_performance();
    
    // Test Checkpoint 3: Trait object performance
    println!("\n--- Trait Object Performance ---");
    benchmark_trait_object_performance();
    
    // Test Checkpoint 4: String performance
    println!("\n--- String Performance ---");
    benchmark_string_performance();
    
    // Test Checkpoint 5: Closure performance
    println!("\n--- Closure Performance ---");
    benchmark_closure_performance();
    
    // Test Checkpoint 6: HashMap performance
    println!("\n--- HashMap Performance ---");
    benchmark_hashmap_performance();
    
    // Test Checkpoint 7: Custom iterator performance
    println!("\n--- Custom Iterator Performance ---");
    benchmark_custom_iterator_performance();
    
    // Test Checkpoint 8: Wrapper performance
    println!("\n--- Wrapper Performance ---");
    benchmark_wrapper_performance();
    
    println!("\n🎉 Zero-cost abstraction concepts demonstrated!");
}

// C# Comparison Notes:
//
// 1. Iterator chains are like LINQ but with better performance guarantees
// 2. Generic operations are like C# generics but with zero-cost monomorphization
// 3. Trait objects are like interfaces but with explicit dynamic dispatch cost
// 4. String handling is like C# strings but with more control over allocations
// 5. Closures are like lambdas but with better inlining characteristics
// 6. HashMap operations are like Dictionary with more predictable performance
// 7. Custom iterators are like IEnumerable with better optimization
// 8. Zero-cost wrappers are like value types that truly have no overhead

// Key Differences from C#:
// - More predictable performance characteristics
// - No garbage collection overhead
// - Better compiler optimizations for abstractions
// - More explicit control over memory allocations
// - True zero-cost abstractions in many cases