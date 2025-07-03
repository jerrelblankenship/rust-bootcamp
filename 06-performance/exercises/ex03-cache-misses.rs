// Exercise 3: Cache Miss Performance - Fix the CPU Cache Disasters!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 checkpoints to fix)
//
// Your task: Fix catastrophic CPU cache performance disasters
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex03-cache-misses.rs`
// 2. Run: `./ex03-cache-misses`
// 3. Fix the cache-unfriendly patterns one by one
// 4. Use hints in /hints/ directory if stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Each checkpoint shows a different cache performance anti-pattern
// - Fix one cache disaster at a time and measure the improvement
// - Compare with equivalent C# memory access optimizations
//
// COMPLETED CONCEPTS:
// [] Memory access patterns (row-major vs column-major)
// [] Data structure layout optimization (AoS vs SoA)
// [] Cache line alignment and utilization
// [] Prefetching and memory bandwidth
// [] False sharing in multi-threaded scenarios

use std::time::Instant;

fn main() {
    println!("=== Exercise 3: Cache Miss Performance (Fix the CPU Cache Disasters!) ===\n");
    
    // Exercise 3.1: Matrix traversal disaster
    println!("🔥 CHECKPOINT 1: Matrix traversal cache misses");
    let start = Instant::now();
    
    const SIZE: usize = 1000;
    let mut matrix = vec![vec![0i32; SIZE]; SIZE];
    
    // Initialize matrix
    for i in 0..SIZE {
        for j in 0..SIZE {
            matrix[i][j] = (i * SIZE + j) as i32;
        }
    }
    
    // PERFORMANCE DISASTER: Column-major access pattern causes cache misses!
    let mut sum = 0i64;
    for j in 0..SIZE {          // FIXME: Wrong outer loop!
        for i in 0..SIZE {      // FIXME: Wrong inner loop!
            sum += matrix[i][j] as i64;  // FIXME: Cache-unfriendly access!
        }
    }
    // In C#, this would be: matrix[i,j] with wrong loop order
    
    let duration = start.elapsed();
    println!("Matrix sum: {} (computed in {:?})", sum, duration);
    println!("💡 This should be 3-5x faster with proper cache-friendly access! Check hints/ex03-level1.md\n");
    
    // ✅ CHECKPOINT 1: Change to row-major access pattern
    
    // Exercise 3.2: Array of Structures (AoS) disaster
    println!("🔥 CHECKPOINT 2: Array of Structures cache inefficiency");
    let start = Instant::now();
    
    #[derive(Clone)]
    struct Particle {
        x: f64,
        y: f64,
        z: f64,
        vx: f64,
        vy: f64,
        vz: f64,
        mass: f64,
        charge: f64,
    }
    
    let mut particles = vec![Particle {
        x: 0.0, y: 0.0, z: 0.0,
        vx: 1.0, vy: 1.0, vz: 1.0,
        mass: 1.0, charge: 1.0,
    }; 100_000];
    
    // PERFORMANCE DISASTER: Only accessing position, but loading entire struct!
    for i in 0..particles.len() {
        particles[i].x += particles[i].vx * 0.1;  // FIXME: Cache-unfriendly layout!
        particles[i].y += particles[i].vy * 0.1;  // FIXME: Loading unused fields!
        particles[i].z += particles[i].vz * 0.1;  // FIXME: Poor cache utilization!
    }
    
    let duration = start.elapsed();
    println!("Updated {} particles in {:?}", particles.len(), duration);
    println!("💡 Consider Structure of Arrays (SoA) for better cache usage!\n");
    
    // ✅ CHECKPOINT 2: Restructure to SoA or process related fields together
    
    // Exercise 3.3: Strided memory access disaster
    println!("🔥 CHECKPOINT 3: Strided memory access pattern");
    let start = Instant::now();
    
    let data = (0..1_000_000).map(|i| i as f64).collect::<Vec<f64>>();
    let mut result = Vec::new();
    
    // PERFORMANCE DISASTER: Accessing every 8th element causes cache misses!
    let stride = 8;  // FIXME: Large stride kills cache performance!
    for i in (0..data.len()).step_by(stride) {
        result.push(data[i] * 2.0);  // FIXME: Cache-unfriendly access pattern!
    }
    
    let duration = start.elapsed();
    println!("Processed {} elements with stride {} in {:?}", result.len(), stride, duration);
    println!("💡 Consider data restructuring or prefetching for strided access!\n");
    
    // ✅ CHECKPOINT 3: Optimize strided access or restructure data
    
    // Exercise 3.4: Linked list traversal disaster
    println!("🔥 CHECKPOINT 4: Linked list pointer chasing");
    let start = Instant::now();
    
    #[derive(Clone)]
    struct Node {
        data: i32,
        next: Option<Box<Node>>,
    }
    
    // Create a linked list
    let mut head = Some(Box::new(Node { data: 0, next: None }));
    let mut current = &mut head;
    
    for i in 1..10_000 {
        if let Some(ref mut node) = current {
            node.next = Some(Box::new(Node { data: i, next: None }));
            current = &mut node.next;
        }
    }
    
    // PERFORMANCE DISASTER: Pointer chasing through linked list!
    let mut sum = 0i64;
    let mut current = &head;
    while let Some(node) = current {
        sum += node.data as i64;  // FIXME: Cache-unfriendly pointer chasing!
        current = &node.next;     // FIXME: Each access likely a cache miss!
    }
    
    let duration = start.elapsed();
    println!("Linked list sum: {} (computed in {:?})", sum, duration);
    println!("💡 Arrays have much better cache locality than linked lists!\n");
    
    // ✅ CHECKPOINT 4: Consider array-based data structure for better cache usage
    
    // Exercise 3.5: Hash table access disaster
    println!("🔥 CHECKPOINT 5: Hash table cache thrashing");
    let start = Instant::now();
    
    use std::collections::HashMap;
    
    // Create a large hash map
    let mut map = HashMap::new();
    for i in 0..50_000 {
        map.insert(i, i * i);
    }
    
    // PERFORMANCE DISASTER: Random access pattern causes cache thrashing!
    let mut sum = 0i64;
    for i in (0..50_000).rev() {  // FIXME: Reverse order access!
        if let Some(&value) = map.get(&(i * 17 % 50_000)) {  // FIXME: Random access pattern!
            sum += value as i64;
        }
    }
    
    let duration = start.elapsed();
    println!("HashMap random access sum: {} (computed in {:?})", sum, duration);
    println!("💡 Sequential access patterns are much more cache-friendly!\n");
    
    // ✅ CHECKPOINT 5: Optimize hash table access patterns
    
    println!("🎯 CHALLENGE: Optimize all checkpoints to achieve 5x+ speedup!");
    println!("🔄 C# COMPARISON: These cache optimizations apply to C# arrays, collections, and memory access patterns!");
}

// COMPILATION CHALLENGES:
// 1. Change matrix traversal to row-major order
// 2. Restructure data layout for better cache utilization
// 3. Optimize strided memory access patterns
// 4. Consider array-based alternatives to linked structures
// 5. Implement cache-friendly hash table access
//
// LEARNING OBJECTIVES:
// - Understanding CPU cache behavior and memory hierarchy
// - Row-major vs column-major memory access patterns
// - Array of Structures (AoS) vs Structure of Arrays (SoA)
// - Cache line utilization and data layout optimization
// - Avoiding pointer chasing and random access patterns
// - Performance comparison with C# memory access patterns
//
// C# COMPARISON:
// C#: Multi-dimensional arrays [,] vs jagged arrays [][]
// Rust: Vec<Vec<T>> vs flattened Vec<T> with manual indexing
//
// C#: struct vs class layout and memory locality
// Rust: struct layout and #[repr(C)] for predictable layout
//
// C#: LinkedList<T> vs List<T> for cache performance
// Rust: LinkedList vs Vec for cache performance
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Matrix traversal): [ ] Complete
// Checkpoint 2 (AoS vs SoA): [ ] Complete
// Checkpoint 3 (Strided access): [ ] Complete
// Checkpoint 4 (Linked list): [ ] Complete
// Checkpoint 5 (Hash table): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Cache-friendly memory access patterns
// ✅ Data structure layout optimization
// ✅ Understanding CPU cache behavior
// ✅ Memory hierarchy performance principles
// ✅ Avoiding cache thrashing patterns
// ✅ Performance equivalent to optimized C# memory access
//
// 🚀 Ready for the next challenge?
// Move on to ex04-iterator-chains.rs to explore iterator performance!
// Or check your work with: `rustc ex03-cache-misses.rs && ./ex03-cache-misses`