// Exercise 8: SIMD Vectorization Opportunities - Fix the Scalar Performance Disasters!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 checkpoints to fix)
//
// Your task: Fix catastrophic scalar processing that could be vectorized
//
// INSTRUCTIONS:
// 1. Compile: `rustc ex08-simd-opportunity.rs`
// 2. Run: `./ex08-simd-opportunity`
// 3. Fix the scalar processing disasters one by one
// 4. Use hints in /hints/ directory if stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Each checkpoint shows a different vectorization opportunity
// - Fix one scalar disaster at a time and measure improvement
// - Compare with equivalent C# System.Numerics.Vector optimizations
//
// COMPLETED CONCEPTS:
// [] Basic SIMD operations for arithmetic
// [] Vector operations for data transformations
// [] Parallel reductions with SIMD
// [] Memory alignment for optimal SIMD performance
// [] Auto-vectorization vs explicit SIMD

use std::time::Instant;

fn main() {
    println!("=== Exercise 8: SIMD Vectorization Opportunities (Fix the Scalar Performance Disasters!) ===\n");
    
    // Exercise 8.1: Scalar arithmetic on large arrays
    println!("🔥 CHECKPOINT 1: Scalar arithmetic missing vectorization");
    let start = Instant::now();
    
    let a = vec![1.0f32; 1_000_000];
    let b = vec![2.0f32; 1_000_000];
    let mut result = vec![0.0f32; 1_000_000];
    
    // PERFORMANCE DISASTER: Scalar operations on vectorizable data!
    for i in 0..a.len() {
        result[i] = a[i] * b[i] + 3.0;  // FIXME: Scalar multiply-add operation!
    }
    // In C#, this could use System.Numerics.Vector<float> for SIMD
    
    let duration = start.elapsed();
    let sum: f32 = result.iter().sum();
    println!("Scalar multiply-add result sum: {} in {:?}", sum, duration);
    println!("💡 Use SIMD intrinsics or auto-vectorization for 4x+ speedup! Check hints/ex08-level1.md\n");
    
    // ✅ CHECKPOINT 1: Vectorize basic arithmetic operations
    
    // Exercise 8.2: Scalar reduction operations
    println!("🔥 CHECKPOINT 2: Scalar reduction missing SIMD acceleration");
    let start = Instant::now();
    
    let data = (0..2_000_000).map(|i| (i % 1000) as f32).collect::<Vec<f32>>();
    
    // PERFORMANCE DISASTER: Scalar dot product computation!
    let mut dot_product = 0.0f32;
    for i in 0..data.len() / 2 {
        dot_product += data[i * 2] * data[i * 2 + 1];  // FIXME: Scalar multiply-accumulate!
    }
    
    let duration = start.elapsed();
    println!("Scalar dot product: {} in {:?}", dot_product, duration);
    println!("💡 Use SIMD horizontal add operations for reductions!\n");
    
    // ✅ CHECKPOINT 2: Vectorize reduction operations
    
    // Exercise 8.3: Scalar data transformations
    println!("🔥 CHECKPOINT 3: Scalar data transformation pipeline");
    let start = Instant::now();
    
    let input = (0..1_000_000).map(|i| i as f32 / 1000.0).collect::<Vec<f32>>();
    let mut output = vec![0.0f32; input.len()];
    
    // PERFORMANCE DISASTER: Scalar math operations on bulk data!
    for i in 0..input.len() {
        let x = input[i];
        output[i] = (x * x + 2.0 * x + 1.0).sqrt();  // FIXME: Scalar polynomial + sqrt!
    }
    
    let duration = start.elapsed();
    let result_sum: f32 = output.iter().sum();
    println!("Scalar transformation sum: {} in {:?}", result_sum, duration);
    println!("💡 Vectorize polynomial evaluation and math functions!\n");
    
    // ✅ CHECKPOINT 3: Vectorize mathematical transformations
    
    // Exercise 8.4: Unaligned memory access pattern
    println!("🔥 CHECKPOINT 4: Unaligned memory access hurting SIMD");
    let start = Instant::now();
    
    // PERFORMANCE DISASTER: Unaligned data layout prevents optimal SIMD!
    struct UnalignedData {
        flag: bool,        // FIXME: Breaks alignment!
        values: [f32; 4],  // FIXME: No longer 16-byte aligned!
    }
    
    let data = vec![UnalignedData { 
        flag: true, 
        values: [1.0, 2.0, 3.0, 4.0] 
    }; 250_000];
    
    let mut sum = 0.0f32;
    for item in &data {
        // FIXME: Unaligned access prevents vectorization!
        for &val in &item.values {
            sum += val * val;
        }
    }
    
    let duration = start.elapsed();
    println!("Unaligned processing sum: {} in {:?}", sum, duration);
    println!("💡 Use aligned data structures and proper memory layout!\n");
    
    // ✅ CHECKPOINT 4: Optimize memory alignment for SIMD
    
    // Exercise 8.5: Conditional processing preventing vectorization
    println!("🔥 CHECKPOINT 5: Branching preventing auto-vectorization");
    let start = Instant::now();
    
    let data = (0..1_000_000).map(|i| i as f32).collect::<Vec<f32>>();
    let mut positive_sum = 0.0f32;
    let mut negative_sum = 0.0f32;
    
    // PERFORMANCE DISASTER: Branching prevents vectorization!
    for &value in &data {
        if value > 500_000.0 {      // FIXME: Branch prevents vectorization!
            positive_sum += value * 2.0;
        } else {
            negative_sum += value * 0.5;
        }
    }
    
    let duration = start.elapsed();
    println!("Branched processing: pos={}, neg={} in {:?}", positive_sum, negative_sum, duration);
    println!("💡 Use branchless operations or separate the hot paths!\n");
    
    // ✅ CHECKPOINT 5: Eliminate branching for better vectorization
    
    println!("🎯 CHALLENGE: Optimize all checkpoints to achieve 4-8x SIMD speedup!");
    println!("🔄 C# COMPARISON: These are like System.Numerics.Vector<T> optimizations!");
}

// Example of how SIMD operations could look (requires std::arch)
#[cfg(target_arch = "x86_64")]
mod simd_examples {
    use std::arch::x86_64::*;
    
    // Example of vectorized multiply-add
    unsafe fn vectorized_multiply_add(a: &[f32], b: &[f32], result: &mut [f32], scalar: f32) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());
        assert_eq!(a.len() % 8, 0); // Assume multiple of 8 for simplicity
        
        let scalar_vec = _mm256_set1_ps(scalar);
        
        for i in (0..a.len()).step_by(8) {
            let a_vec = _mm256_load_ps(a.as_ptr().add(i));
            let b_vec = _mm256_load_ps(b.as_ptr().add(i));
            let result_vec = _mm256_fmadd_ps(a_vec, b_vec, scalar_vec);
            _mm256_store_ps(result.as_mut_ptr().add(i), result_vec);
        }
    }
    
    // Example of vectorized dot product
    unsafe fn vectorized_dot_product(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len() % 8, 0);
        
        let mut acc = _mm256_setzero_ps();
        
        for i in (0..a.len()).step_by(8) {
            let a_vec = _mm256_load_ps(a.as_ptr().add(i));
            let b_vec = _mm256_load_ps(b.as_ptr().add(i));
            acc = _mm256_fmadd_ps(a_vec, b_vec, acc);
        }
        
        // Horizontal add to get final result
        let hi = _mm256_extractf128_ps(acc, 1);
        let lo = _mm256_castps256_ps128(acc);
        let sum_quad = _mm_add_ps(hi, lo);
        let sum_dual = _mm_hadd_ps(sum_quad, sum_quad);
        let sum = _mm_hadd_ps(sum_dual, sum_dual);
        
        _mm_cvtss_f32(sum)
    }
}

// COMPILATION CHALLENGES:
// 1. Use SIMD intrinsics or enable auto-vectorization for arithmetic
// 2. Implement vectorized reduction operations
// 3. Vectorize mathematical transformation pipelines
// 4. Ensure proper memory alignment for SIMD operations
// 5. Eliminate branching or use branchless SIMD operations
//
// LEARNING OBJECTIVES:
// - Understanding SIMD/vector processing opportunities
// - When and how to use explicit SIMD intrinsics
// - Memory alignment requirements for optimal SIMD performance
// - Auto-vectorization vs manual vectorization trade-offs
// - Branching effects on vectorization
// - Performance comparison with C# System.Numerics.Vector patterns
//
// C# COMPARISON:
// C#: System.Numerics.Vector<T> for cross-platform SIMD
// Rust: std::arch intrinsics for explicit SIMD control
//
// C#: Vector.Dot() for vectorized reductions
// Rust: Manual horizontal add operations with SIMD intrinsics
//
// C#: Vector.ConditionalSelect() for branchless operations
// Rust: SIMD blend and mask operations for conditional processing
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Basic arithmetic): [ ] Complete
// Checkpoint 2 (Reduction operations): [ ] Complete
// Checkpoint 3 (Data transformations): [ ] Complete
// Checkpoint 4 (Memory alignment): [ ] Complete
// Checkpoint 5 (Branching elimination): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ SIMD vectorization opportunities identification
// ✅ Explicit SIMD intrinsics usage
// ✅ Memory alignment optimization for SIMD
// ✅ Auto-vectorization vs manual control
// ✅ Branchless SIMD operation patterns
// ✅ Performance equivalent to optimized C# vector operations
//
// 🚀 Ready for the final challenge?
// Move on to project-optimization-challenge to apply all your skills!
// Or check your work with: `rustc ex08-simd-opportunity.rs && ./ex08-simd-opportunity`