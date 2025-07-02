# Performance Reference Materials

This directory contains detailed reference materials for Module 06 - Performance Optimization.

## 📚 Available References

### Core Performance Concepts
- **[C# Performance Comparisons](csharp-comparisons.md)** - Side-by-side comparison of C# vs Rust performance patterns
- **[Performance Principles](performance-principles.md)** - Fundamental principles of performance optimization
- **[Profiling Guide](profiling-guide.md)** - How to measure and profile Rust performance

### Detailed Guides
- **[Memory Optimization](memory-optimization.md)** - Allocation patterns, capacity, and reuse strategies
- **[Cache Performance](cache-performance.md)** - CPU cache optimization and data layout
- **[Iterator Performance](iterator-performance.md)** - Zero-cost abstractions and iterator fusion
- **[Threading Performance](threading-performance.md)** - When and how to use threads effectively
- **[SIMD and Vectorization](simd-vectorization.md)** - Data parallelism and vector operations

## 🎯 How to Use These References

### While Working on Exercises
1. **Start with the exercise** - Try to fix the broken code first
2. **Use hints progressively** - Level 1 → Level 2 → Level 3
3. **Consult references for deep understanding** - Come here when you want to learn more

### For Real-World Application
- **[C# Performance Comparisons](csharp-comparisons.md)** - Translate your C# performance knowledge to Rust
- **[Performance Principles](performance-principles.md)** - Apply these concepts to your own projects
- **[Profiling Guide](profiling-guide.md)** - Essential for performance work in any project

## 🔧 Quick Performance Checklist

Before optimizing any code:
1. **Measure first** - Profile to find actual bottlenecks
2. **Algorithm** - Is there a fundamentally better approach?
3. **Memory** - Are you allocating unnecessarily?
4. **Data access** - Are you cache-friendly?
5. **Parallelization** - Can independent work run concurrently?
6. **Micro-optimizations** - SIMD, bounds checking, etc.

## 🚀 Performance Mindset

> "Premature optimization is the root of all evil" - Donald Knuth

But also:

> "Premature pessimization is the root of all evil" - Andrei Alexandrescu

The key is **measure-driven optimization**:
- Write clear, correct code first
- Profile to find real bottlenecks
- Optimize systematically
- Measure improvements
- Stop when good enough

## 📖 External Resources

For deeper performance learning:
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rust Compiler Optimization Guide](https://doc.rust-lang.org/rustc/codegen-options/index.html)
- [CPU Cache Effects](https://people.freebsd.org/~lstewart/articles/cpumemory.pdf) (Classic paper)
- [SIMD Programming](https://doc.rust-lang.org/core/arch/index.html) (Rust docs)