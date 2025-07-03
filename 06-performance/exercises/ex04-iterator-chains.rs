// Exercise 4: Iterator Chain Performance - Fix the Zero-Cost Abstraction Failures!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix broken iterator patterns that destroy zero-cost abstractions
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex04-iterator-chains.rs`
// 2. Run: `./ex04-iterator-chains`
// 3. Fix the iterator anti-patterns one by one
// 4. Use hints in /hints/ directory if stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Each checkpoint shows a different iterator performance anti-pattern
// - Fix one iterator disaster at a time and measure the improvement
// - Compare with equivalent C# LINQ performance patterns
//
// COMPLETED CONCEPTS:
// [] Iterator fusion and optimization
// [] Collect() vs fold() for performance
// [] Unnecessary heap allocations in chains
// [] Iterator vs for-loop performance
// [] Lazy evaluation vs eager evaluation
// [] Parallel iterator opportunities

use std::time::Instant;

fn main() {
    println!("=== Exercise 4: Iterator Chain Performance (Fix the Zero-Cost Abstraction Failures!) ===\n");
    
    // Exercise 4.1: Collect() in the middle of chain
    println!("🔥 CHECKPOINT 1: Intermediate collect() breaking fusion");
    let start = Instant::now();
    
    let numbers: Vec<i32> = (0..1_000_000).collect();
    
    // PERFORMANCE DISASTER: collect() breaks iterator fusion!
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>()     // FIXME: Unnecessary collect() breaks fusion!
        .iter()
        .filter(|&&x| x % 4 == 0)
        .map(|x| x + 1)
        .collect();
    // In C#, this would be like: ToList() in the middle of LINQ chain
    
    let duration = start.elapsed();
    println!("Processed {} numbers to {} results in {:?}", numbers.len(), result.len(), duration);
    println!("💡 Remove intermediate collect() to enable iterator fusion! Check hints/ex04-level1.md\n");
    
    // ✅ CHECKPOINT 1: Remove intermediate collect() for iterator fusion
    
    // Exercise 4.2: Vec allocation when sum would work
    println!("🔥 CHECKPOINT 2: Unnecessary Vec allocation for reduction");
    let start = Instant::now();
    
    let data = (0..500_000).map(|i| i as f64).collect::<Vec<f64>>();
    
    // PERFORMANCE DISASTER: Collecting into Vec when we only need sum!
    let squares: Vec<f64> = data
        .iter()
        .map(|x| x * x)
        .collect();                // FIXME: Unnecessary Vec allocation!
    
    let sum: f64 = squares.iter().sum();  // FIXME: Could be done in one pass!
    
    let duration = start.elapsed();
    println!("Computed sum of squares: {} in {:?}", sum, duration);
    println!("💡 Use fold() or sum() directly without intermediate Vec!\n");
    
    // ✅ CHECKPOINT 2: Use fold() or sum() instead of collect() + sum()
    
    // Exercise 4.3: Nested loops instead of iterator chain
    println!("🔥 CHECKPOINT 3: Manual loops instead of iterator optimization");
    let start = Instant::now();
    
    let matrix = vec![vec![1, 2, 3, 4, 5]; 1000];
    
    // PERFORMANCE DISASTER: Manual nested loops prevent optimization!
    let mut result = Vec::new();
    for row in &matrix {                    // FIXME: Manual loop!
        for &item in row {                  // FIXME: Nested manual loop!
            if item % 2 == 0 {              // FIXME: Manual filtering!
                result.push(item * item);   // FIXME: Manual mapping!
            }
        }
    }
    
    let duration = start.elapsed();
    println!("Processed matrix to {} results in {:?}", result.len(), duration);
    println!("💡 Use flatten() and iterator chains for better optimization!\n");
    
    // ✅ CHECKPOINT 3: Replace manual loops with iterator chains
    
    // Exercise 4.4: Multiple passes instead of single pass
    println!("🔥 CHECKPOINT 4: Multiple iterator passes");
    let start = Instant::now();
    
    let text = "the quick brown fox jumps over the lazy dog ".repeat(10_000);
    
    // PERFORMANCE DISASTER: Multiple passes over the same data!
    let words: Vec<&str> = text.split_whitespace().collect();  // FIXME: First pass!
    let long_words: Vec<&str> = words.iter()                   // FIXME: Second pass!
        .filter(|word| word.len() > 3)
        .copied()
        .collect();
    let word_lengths: Vec<usize> = long_words.iter()           // FIXME: Third pass!
        .map(|word| word.len())
        .collect();
    let total_length: usize = word_lengths.iter().sum();       // FIXME: Fourth pass!
    
    let duration = start.elapsed();
    println!("Processed text to total length {} in {:?}", total_length, duration);
    println!("💡 Combine operations into single iterator chain!\n");
    
    // ✅ CHECKPOINT 4: Combine multiple passes into single iterator chain
    
    // Exercise 4.5: Unnecessary cloning in iterator
    println!("🔥 CHECKPOINT 5: Excessive cloning in iterator chain");
    let start = Instant::now();
    
    let strings = (0..10_000)
        .map(|i| format!("string_{}", i))
        .collect::<Vec<String>>();
    
    // PERFORMANCE DISASTER: Cloning strings unnecessarily!
    let processed: Vec<String> = strings
        .iter()
        .map(|s| s.clone())              // FIXME: Unnecessary clone!
        .filter(|s| s.len() > 8)
        .map(|s| s.clone().to_uppercase())  // FIXME: Double allocation!
        .collect();
    
    let duration = start.elapsed();
    println!("Processed {} strings to {} results in {:?}", strings.len(), processed.len(), duration);
    println!("💡 Work with references and avoid unnecessary cloning!\n");
    
    // ✅ CHECKPOINT 5: Eliminate unnecessary cloning in iterator chains
    
    // Exercise 4.6: Sequential processing instead of parallel
    println!("🔥 CHECKPOINT 6: Missing parallel processing opportunity");
    let start = Instant::now();
    
    let numbers = (0..100_000).collect::<Vec<i32>>();
    
    // PERFORMANCE DISASTER: Sequential processing of expensive computation!
    let result: Vec<i32> = numbers
        .iter()
        .map(|&x| {
            // Simulate expensive computation
            let mut sum = 0;
            for i in 0..x % 100 + 1 {
                sum += i * i;
            }
            sum
        })
        .collect();
    
    let duration = start.elapsed();
    println!("Computed expensive function for {} numbers in {:?}", result.len(), duration);
    println!("💡 Use rayon for parallel processing of expensive operations!\n");
    
    // ✅ CHECKPOINT 6: Use parallel iteration for expensive computations
    
    println!("🎯 CHALLENGE: Optimize all checkpoints to achieve 5x+ speedup!");
    println!("🔄 C# COMPARISON: These optimizations are like LINQ performance best practices!");
}

// COMPILATION CHALLENGES:
// 1. Remove intermediate collect() calls to enable iterator fusion
// 2. Use fold() or sum() instead of collect() + aggregation
// 3. Replace manual loops with optimized iterator chains
// 4. Combine multiple passes into single iterator chain
// 5. Eliminate unnecessary cloning in iterator operations
// 6. Use parallel iteration for expensive computations
//
// LEARNING OBJECTIVES:
// - Understanding iterator fusion and zero-cost abstractions
// - When to use fold() vs collect() for performance
// - Avoiding unnecessary heap allocations in iterator chains
// - Single-pass vs multi-pass iterator patterns
// - Reference usage vs cloning in iterator chains
// - Parallel processing opportunities with rayon
// - Performance comparison with C# LINQ patterns
//
// C# COMPARISON:
// C#: Avoid ToList() in the middle of LINQ chains
// Rust: Avoid collect() in the middle of iterator chains
//
// C#: Use Aggregate() instead of ToList() + Sum()
// Rust: Use fold() or sum() instead of collect() + sum()
//
// C#: PLINQ with AsParallel() for expensive operations
// Rust: rayon with par_iter() for parallel processing
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Iterator fusion): [ ] Complete
// Checkpoint 2 (Collect vs fold): [ ] Complete
// Checkpoint 3 (Manual loops): [ ] Complete
// Checkpoint 4 (Multiple passes): [ ] Complete
// Checkpoint 5 (Unnecessary cloning): [ ] Complete
// Checkpoint 6 (Parallel processing): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Iterator fusion and zero-cost abstractions
// ✅ Optimal iterator chain patterns
// ✅ Single-pass vs multi-pass performance
// ✅ Reference usage in iterator chains
// ✅ Parallel processing with iterators
// ✅ Performance equivalent to optimized C# LINQ
//
// 🚀 Ready for the next challenge?
// Move on to ex05-bounds-checking.rs to explore bounds checking optimization!
// Or check your work with: `rustc ex04-iterator-chains.rs && ./ex04-iterator-chains`