// Exercise 1: Allocation Storm - Fix the Broken Performance!
//
// Your task: Fix catastrophic memory allocation performance disasters
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex01-allocation-storm.rs`
// 2. Run: `./ex01-allocation-storm`
// 3. Use hints in /hints/ directory if stuck

use std::time::Instant;

fn main() {
    println!("=== Exercise 1: Allocation Storm (Fix the Performance!) ===");
    
    let numbers: Vec<i32> = (0..100_000).collect();
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: Creating new Vec in every iteration!
    let mut results = Vec::new();
    for chunk in numbers.chunks(100) {
        let mut temp = Vec::new();  // FIXME: Allocation storm!
        for &num in chunk {
            temp.push(num * 2);
        }
        results.extend(temp);
    }
    
    let duration = start.elapsed();
    println!("Processed {} numbers in {:?}", results.len(), duration);
    println!("💡 Check hints/ex01-level1.md for optimization guidance!");
}