// Exercise 5: Bounds Checking Performance - Fix the Safety vs Speed Balance!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 checkpoints to fix)
//
// Your task: Fix performance disasters caused by unnecessary bounds checking
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex05-bounds-checking.rs`
// 2. Run: `./ex05-bounds-checking`
// 3. Fix the bounds checking overhead one by one
// 4. Use hints in /hints/ directory if stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Each checkpoint shows a different bounds checking anti-pattern
// - Fix one bounds checking disaster at a time and measure improvement
// - Learn when safety can be relaxed for performance gains
//
// COMPLETED CONCEPTS:
// [] Unnecessary bounds checks in loops
// [] get_unchecked() for verified safe access
// [] Iterator vs indexed access performance
// [] Vec capacity and bounds checking
// [] Slice patterns vs indexing

use std::time::Instant;

fn main() {
    println!("=== Exercise 5: Bounds Checking Performance (Fix the Safety vs Speed Balance!) ===\n");
    
    // Exercise 5.1: Redundant bounds checks in loop
    println!("🔥 CHECKPOINT 1: Redundant bounds checks in verified loop");
    let start = Instant::now();
    
    let data = (0..1_000_000).collect::<Vec<i32>>();
    let mut result = 0i64;
    
    // PERFORMANCE DISASTER: Bounds check on every iteration when length is known!
    for i in 0..data.len() {
        result += data[i] as i64;  // FIXME: Unnecessary bounds check every iteration!
    }
    // In C#, this would be array[i] in a for loop - also bounds checked
    
    let duration = start.elapsed();
    println!("Summed {} elements: {} in {:?}", data.len(), result, duration);
    println!("💡 Use iterators or get_unchecked() for verified safe access! Check hints/ex05-level1.md\n");
    
    // ✅ CHECKPOINT 1: Eliminate redundant bounds checks in verified loops
    
    // Exercise 5.2: Matrix access with double bounds checking
    println!("🔥 CHECKPOINT 2: Double bounds checking in matrix operations");
    let start = Instant::now();
    
    const SIZE: usize = 1000;
    let matrix = vec![vec![1i32; SIZE]; SIZE];
    let mut sum = 0i64;
    
    // PERFORMANCE DISASTER: Bounds checking on both dimensions!
    for i in 0..SIZE {
        for j in 0..SIZE {
            sum += matrix[i][j] as i64;  // FIXME: Double bounds check (row + column)!
        }
    }
    
    let duration = start.elapsed();
    println!("Matrix sum: {} in {:?}", sum, duration);
    println!("💡 Use get_unchecked() or flattened access for known-safe indices!\n");
    
    // ✅ CHECKPOINT 2: Eliminate double bounds checking in matrix access
    
    // Exercise 5.3: Vec push with unnecessary capacity checks
    println!("🔥 CHECKPOINT 3: Vec growth and capacity checking overhead");
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: Vec grows and checks capacity on every push!
    let mut dynamic_vec = Vec::new();  // FIXME: No initial capacity!
    for i in 0..100_000 {
        dynamic_vec.push(i);  // FIXME: Capacity check and potential reallocation!
    }
    
    let duration = start.elapsed();
    println!("Built vec with {} elements in {:?}", dynamic_vec.len(), duration);
    println!("💡 Pre-allocate capacity to eliminate capacity checks!\n");
    
    // ✅ CHECKPOINT 3: Pre-allocate Vec capacity to avoid capacity checks
    
    // Exercise 5.4: String indexing with bounds checks
    println!("🔥 CHECKPOINT 4: String byte indexing with bounds checking");
    let start = Instant::now();
    
    let text = "Hello, World! ".repeat(50_000);
    let mut vowel_count = 0;
    
    // PERFORMANCE DISASTER: Bounds checking every byte access!
    let bytes = text.as_bytes();
    for i in 0..bytes.len() {
        match bytes[i] {  // FIXME: Bounds check on every byte!
            b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => {
                vowel_count += 1;
            }
            _ => {}
        }
    }
    
    let duration = start.elapsed();
    println!("Counted {} vowels in {:?}", vowel_count, duration);
    println!("💡 Use iterator or unsafe indexing for byte-level operations!\n");
    
    // ✅ CHECKPOINT 4: Eliminate bounds checks in byte-level string processing
    
    // Exercise 5.5: Slice pattern matching vs indexing
    println!("🔥 CHECKPOINT 5: Manual indexing vs pattern matching");
    let start = Instant::now();
    
    let data = (0..500_000).map(|i| i % 256).collect::<Vec<u8>>();
    let mut patterns_found = 0;
    
    // PERFORMANCE DISASTER: Manual indexing with bounds checks for pattern matching!
    for i in 0..data.len().saturating_sub(3) {
        if data[i] == 0x12     // FIXME: Bounds check!
            && data[i + 1] == 0x34  // FIXME: Bounds check!
            && data[i + 2] == 0x56  // FIXME: Bounds check!
            && data[i + 3] == 0x78  // FIXME: Bounds check!
        {
            patterns_found += 1;
        }
    }
    
    let duration = start.elapsed();
    println!("Found {} patterns in {:?}", patterns_found, duration);
    println!("💡 Use slice windows() or pattern matching for safer and faster code!\n");
    
    // ✅ CHECKPOINT 5: Use slice patterns instead of manual indexing
    
    println!("🎯 CHALLENGE: Optimize all checkpoints while maintaining safety!");
    println!("🔄 C# COMPARISON: These are like optimizing array bounds checking in tight loops!");
}

// COMPILATION CHALLENGES:
// 1. Use iterators instead of indexed loops to eliminate bounds checks
// 2. Use get_unchecked() for verified safe access in critical sections
// 3. Pre-allocate Vec capacity to avoid growth-related checks
// 4. Use iterator methods for string/byte processing
// 5. Use slice patterns and windows() for multi-element access
//
// LEARNING OBJECTIVES:
// - Understanding when bounds checking impacts performance
// - Safe techniques to eliminate unnecessary bounds checks
// - Using iterators vs indexed access for performance
// - Vec capacity management and performance implications
// - Slice patterns vs manual indexing for safety and speed
// - When and how to use unsafe code responsibly
//
// C# COMPARISON:
// C#: Array bounds checking in for loops
// Rust: Vec bounds checking in indexed loops
//
// C#: unsafe code with pointers for performance-critical sections
// Rust: get_unchecked() for verified safe access
//
// C#: List<T> capacity management for performance
// Rust: Vec::with_capacity() for allocation optimization
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Loop bounds checks): [ ] Complete
// Checkpoint 2 (Matrix bounds checks): [ ] Complete
// Checkpoint 3 (Vec capacity checks): [ ] Complete
// Checkpoint 4 (String indexing): [ ] Complete
// Checkpoint 5 (Slice patterns): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Eliminating unnecessary bounds checking overhead
// ✅ Safe techniques for performance-critical code
// ✅ Iterator vs indexed access trade-offs
// ✅ Vec capacity and performance optimization
// ✅ Slice patterns for safe multi-element access
// ✅ Responsible use of unsafe code for performance
//
// 🚀 Ready for the next challenge?
// Move on to ex06-parallel-waste.rs to explore threading performance!
// Or check your work with: `rustc ex05-bounds-checking.rs && ./ex05-bounds-checking`