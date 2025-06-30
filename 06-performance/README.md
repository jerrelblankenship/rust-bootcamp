# Module 06: Performance Optimization

Transform slow Rust code into blazingly fast implementations by fixing performance bottlenecks. Learn optimization through hands-on debugging!

## 🎯 Learning Objectives

By fixing performance issues, you will:
- Master profiling tools by finding real bottlenecks
- Fix memory allocation problems and cache misses
- Debug slow iterators and optimize algorithms
- Repair broken benchmarks and interpret results
- Convert slow C#-style code to idiomatic fast Rust
- Build a high-performance image processor

## 🚀 Quick Start

```bash
# Start with the first exercise
cd exercises
cargo build --release  # Always benchmark in release mode!
cargo run --release --bin ex01-allocation-storm

# When you see terrible performance, that's your cue!
# Profile, identify bottlenecks, and fix them!
```

## 📚 Module Overview

**Your C# Experience**: You've used BenchmarkDotNet, profilers, and `Span<T>`.
**What's Different**: Rust gives you precise control over every allocation and CPU cycle. No GC pauses!

## 💪 Exercises - Fix These Performance Disasters!

Each exercise contains working but terribly slow Rust code. Your mission: make it fast!

1. **ex01-allocation-storm.rs** - Fix excessive allocations in loops
2. **ex02-string-builder.rs** - Optimize string concatenation (10x slower than C#!)
3. **ex03-cache-misses.rs** - Fix data layout for CPU cache efficiency
4. **ex04-iterator-chains.rs** - Repair slow iterator usage
5. **ex05-bounds-checking.rs** - Eliminate unnecessary safety checks
6. **ex06-parallel-waste.rs** - Fix inefficient parallelization
7. **ex07-async-blocking.rs** - Remove blocking calls in async code
8. **ex08-simd-opportunity.rs** - Vectorize computation for 4x speedup

## 🏗️ Project: Image Processing Pipeline

Fix a catastrophically slow image processor that should:
- Load and decode images efficiently
- Apply filters without excessive allocations
- Use SIMD for pixel operations
- Process in parallel without overhead
- Beat equivalent C# implementation by 2x

**Starting State**: Takes 30 seconds to process one image!
**Your Goal**: Process 100 images per second!

## 🧰 Debugging Toolkit

- **[DEBUGGING_CHECKLIST.md](DEBUGGING_CHECKLIST.md)** - Systematic performance debugging
- **Hint System** - Progressive optimization hints in `exercises/hints/`
- **Reference Docs** - Deep dives on performance in `reference/`

## 🎮 Learning Path

1. **Start here**: `exercises/ex01-allocation-storm.rs`
2. **Profile first**: Use `cargo flamegraph` or `perf record`
3. **Stuck?** Wait 15 minutes, then check `hints/ex01-level1.md`
4. **Benchmark**: Prove your fix with measurements
5. **All exercises done?** Tackle the image processor!

## 🏆 Victory Conditions

You've mastered this module when you can:
- [ ] Fix all 8 exercises achieving target performance
- [ ] Complete the image processor project
- [ ] Explain your optimizations with profiler data
- [ ] Write efficient Rust code from the start
- [ ] Identify performance anti-patterns immediately

## 📂 Module Structure

```
06-performance/
├── README.md                          # You are here!
├── DEBUGGING_CHECKLIST.md             # Performance debugging guide
├── exercises/
│   ├── ex01-allocation-storm.rs       # Memory allocation issues
│   ├── ex02-string-builder.rs         # String performance
│   ├── ex03-cache-misses.rs           # CPU cache optimization
│   ├── ex04-iterator-chains.rs        # Iterator performance
│   ├── ex05-bounds-checking.rs        # Safety vs speed
│   ├── ex06-parallel-waste.rs         # Parallelization issues
│   ├── ex07-async-blocking.rs         # Async performance
│   ├── ex08-simd-opportunity.rs       # SIMD vectorization
│   └── hints/
│       ├── README.md                  # How to use hints
│       ├── ex01-level1.md             # Profiling guidance
│       ├── ex01-level2.md             # Specific bottlenecks
│       ├── ex01-level3.md             # Solution approach
│       └── ... (3 levels per exercise)
├── project-optimization-challenge/
│   ├── Cargo.toml                     # Dependencies ready
│   ├── README.md                      # Project instructions
│   ├── src/
│   │   ├── main.rs                    # Slow entry point
│   │   └── lib.rs                     # Inefficient algorithms
│   ├── benches/
│   │   └── benchmark.rs               # Performance targets
│   └── tests/
│       └── correctness.rs             # Don't break functionality!
└── reference/
    ├── README.md                      # Additional resources
    ├── profiling-guide.md             # Tool tutorials
    ├── memory-optimization.md         # Allocation strategies
    ├── cpu-optimization.md            # Cache and SIMD
    └── csharp-performance.md          # Performance comparison
```

---

Ready to make your code blazingly fast? Start profiling `exercises/ex01-allocation-storm.rs`! 🏎️