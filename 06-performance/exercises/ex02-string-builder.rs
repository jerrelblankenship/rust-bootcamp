// Exercise 2: String Building Performance - Fix the Catastrophic Performance!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix catastrophic string building performance disasters
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex02-string-builder.rs`
// 2. Run: `./ex02-string-builder`
// 3. Fix the performance disasters one by one
// 4. Use hints in /hints/ directory if stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Each checkpoint shows a different string performance anti-pattern
// - Fix one disaster at a time and measure the improvement
// - Compare with equivalent C# StringBuilder optimizations
//
// COMPLETED CONCEPTS:
// [] String concatenation in loops (String::push_str vs +)
// [] String capacity pre-allocation
// [] format! macro vs direct construction
// [] Heap allocation avoidance
// [] String interning patterns
// [] StringBuilder equivalent patterns

use std::time::Instant;

fn main() {
    println!("=== Exercise 2: String Building Performance (Fix the Disasters!) ===\n");
    
    // Exercise 2.1: String concatenation disaster
    println!("🔥 CHECKPOINT 1: String concatenation in loop");
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: String concatenation in loop creates new strings every time!
    let mut result = String::new();
    for i in 0..10_000 {
        result = result + &format!("Item {}: Processing data\n", i);  // FIXME: Catastrophic performance!
    }
    // In C#, this would be like: result += $"Item {i}: Processing data\n"; in a loop
    
    let duration = start.elapsed();
    println!("Built string with {} characters in {:?}", result.len(), duration);
    println!("💡 This should be < 1ms when optimized! Check hints/ex02-level1.md\n");
    
    // ✅ CHECKPOINT 1: Replace + concatenation with push_str or String::with_capacity
    
    // Exercise 2.2: Format! macro overuse
    println!("🔥 CHECKPOINT 2: Excessive format! macro usage");
    let start = Instant::now();
    
    let mut log_entries = Vec::new();
    for i in 0..5_000 {
        // PERFORMANCE DISASTER: format! creates new String allocation every time!
        let entry = format!("LOG[{}]: {}: {}", 
                           i, 
                           format!("User{}", i % 100),  // FIXME: Nested format! calls!
                           format!("Action{}", i % 10)   // FIXME: More allocations!
                          );
        log_entries.push(entry);
    }
    
    let duration = start.elapsed();
    println!("Created {} log entries in {:?}", log_entries.len(), duration);
    println!("💡 Avoid nested format! calls - build strings directly!\n");
    
    // ✅ CHECKPOINT 2: Eliminate nested format! calls and reduce allocations
    
    // Exercise 2.3: No capacity pre-allocation
    println!("🔥 CHECKPOINT 3: Missing capacity pre-allocation");
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: String grows and reallocates multiple times!
    let mut large_string = String::new();  // FIXME: No initial capacity!
    for i in 0..1_000 {
        large_string.push_str(&format!("This is a long line of text that will cause many reallocations as the string grows beyond its current capacity. Line number: {}\n", i));
    }
    
    let duration = start.elapsed();
    println!("Built large string ({} bytes) in {:?}", large_string.len(), duration);
    println!("💡 Use String::with_capacity() to pre-allocate space!\n");
    
    // ✅ CHECKPOINT 3: Pre-allocate string capacity to avoid reallocations
    
    // Exercise 2.4: Unnecessary heap allocations
    println!("🔥 CHECKPOINT 4: Unnecessary heap allocations");
    let start = Instant::now();
    
    let mut words = Vec::new();
    for i in 0..1_000 {
        // PERFORMANCE DISASTER: Creating owned strings when references would work!
        let word = format!("word{}", i);  // FIXME: Heap allocation for every word!
        words.push(word);
    }
    
    // More disaster: Join with heap-allocated separator
    let separator = format!(" | ");  // FIXME: Heap allocation for simple separator!
    let result = words.join(&separator);
    
    let duration = start.elapsed();
    println!("Created and joined {} words in {:?}", words.len(), duration);
    println!("💡 Use string literals and avoid unnecessary format! calls!\n");
    
    // ✅ CHECKPOINT 4: Use string literals and avoid format! for simple cases
    
    // Exercise 2.5: Cloning strings unnecessarily
    println!("🔥 CHECKPOINT 5: Excessive string cloning");
    let start = Instant::now();
    
    let template = "Hello, {}! Welcome to our service.".to_string();
    let mut personalized_messages = Vec::new();
    
    for i in 0..2_000 {
        // PERFORMANCE DISASTER: Cloning the template string every time!
        let mut message = template.clone();  // FIXME: Unnecessary clone!
        message = message.replace("{}", &format!("User{}", i));  // FIXME: More allocations!
        personalized_messages.push(message);
    }
    
    let duration = start.elapsed();
    println!("Generated {} personalized messages in {:?}", personalized_messages.len(), duration);
    println!("💡 Use references and direct formatting instead of cloning!\n");
    
    // ✅ CHECKPOINT 5: Eliminate unnecessary string cloning
    
    // Exercise 2.6: String interning disaster
    println!("🔥 CHECKPOINT 6: Missing string interning optimization");
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: Creating same strings over and over!
    let mut status_messages = Vec::new();
    for i in 0..5_000 {
        let status = match i % 4 {
            0 => format!("SUCCESS"),      // FIXME: Same string allocated 1250 times!
            1 => format!("ERROR"),        // FIXME: Same string allocated 1250 times!
            2 => format!("WARNING"),      // FIXME: Same string allocated 1250 times!
            _ => format!("PENDING"),      // FIXME: Same string allocated 1250 times!
        };
        status_messages.push(format!("Status {}: {}", i, status));  // FIXME: More format! calls!
    }
    
    let duration = start.elapsed();
    println!("Created {} status messages in {:?}", status_messages.len(), duration);
    println!("💡 Use string constants for repeated values!\n");
    
    // ✅ CHECKPOINT 6: Use string constants instead of repeated format! calls
    
    println!("🎯 CHALLENGE: Optimize all checkpoints to achieve < 5ms total runtime!");
    println!("🔄 C# COMPARISON: These are equivalent to StringBuilder, string.Format, and string interning optimizations!");
}

// COMPILATION CHALLENGES:
// 1. Replace + concatenation with push_str or String::with_capacity
// 2. Eliminate nested format! macro calls
// 3. Pre-allocate string capacity using String::with_capacity
// 4. Use string literals instead of format! for simple cases
// 5. Avoid unnecessary string cloning
// 6. Use string constants for repeated values
//
// LEARNING OBJECTIVES:
// - String concatenation performance patterns
// - Memory allocation patterns in string building
// - When to use format! vs direct string construction
// - String capacity management and pre-allocation
// - String interning and constant reuse patterns
// - Performance comparison with C# StringBuilder patterns
//
// C# COMPARISON:
// C#: StringBuilder sb = new StringBuilder(capacity); sb.Append(value);
// Rust: let mut s = String::with_capacity(capacity); s.push_str(value);
//
// C#: string.Format("Hello {0}", name) vs string interpolation $"Hello {name}"
// Rust: format!("Hello {}", name) vs direct string construction when possible
//
// C#: const string STATUS_SUCCESS = "SUCCESS"; (string interning)
// Rust: const STATUS_SUCCESS: &str = "SUCCESS"; (string literals)
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (String concatenation): [ ] Complete
// Checkpoint 2 (Format! overuse): [ ] Complete
// Checkpoint 3 (Capacity pre-allocation): [ ] Complete
// Checkpoint 4 (Heap allocations): [ ] Complete
// Checkpoint 5 (String cloning): [ ] Complete
// Checkpoint 6 (String interning): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ String building performance patterns
// ✅ Memory allocation optimization
// ✅ Format! macro vs direct construction
// ✅ String capacity management
// ✅ String interning patterns
// ✅ Performance equivalent to C# StringBuilder
//
// 🚀 Ready for the next challenge?
// Move on to ex03-cache-misses.rs to explore CPU cache performance!
// Or check your work with: `rustc ex02-string-builder.rs && ./ex02-string-builder`