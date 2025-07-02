# 🚀 Image Processor Optimization Challenge

**Your Mission**: Fix catastrophic performance issues in an image processing pipeline!

## 🎯 The Challenge

This image processor should handle **100+ images per second**, but it's currently processing **5-10 images per second** due to terrible performance issues. You need to identify and fix the bottlenecks to achieve professional-grade performance.

## 📊 Current Performance (BROKEN)
- **Throughput**: ~8 images/second (target: 100+/second)  
- **Memory Usage**: ~500MB peak (target: <100MB)
- **CPU Utilization**: ~25% (target: 80%+ on multi-core)
- **Processing Time**: ~125ms per image (target: <10ms)

## 🔧 Quick Start

```bash
# Build and see the terrible performance
cargo run --release -- sample-images/

# Run benchmarks to measure current disaster
cargo bench

# Check for correctness
cargo test
```

## 💥 Known Performance Issues

Your job is to find and fix these performance disasters:

1. **🐌 Allocation Storm**: Massive memory allocations in processing loops
2. **🔄 Cache Misses**: Poor memory access patterns killing performance  
3. **🔒 Synchronization Waste**: Unnecessary locks and contention
4. **⚡ Missing Parallelization**: Single-threaded where it should be parallel
5. **📊 SIMD Opportunities**: Scalar operations that could be vectorized
6. **🔍 Algorithm Inefficiency**: O(n²) algorithms where O(n) exists

## 🎮 How to Play

### Step 1: Measure the Disaster
```bash
cargo run --release -- sample-images/ --verbose
# Watch the terrible performance metrics
```

### Step 2: Profile and Identify
```bash
# Use profiling tools to find bottlenecks
cargo flamegraph --bin image-processor -- sample-images/

# Benchmark specific operations
cargo bench
```

### Step 3: Fix One Issue at a Time
- Start with the biggest bottleneck
- Measure improvement after each fix
- Keep tests passing (correctness matters!)

### Step 4: Achieve Target Performance
- **100+ images/second throughput**
- **<100MB memory usage**  
- **<10ms per image processing time**
- **All tests still passing**

## 🏆 Success Criteria

You've mastered performance optimization when:

- ✅ **Throughput**: 100+ images/second (10x+ improvement)
- ✅ **Memory**: <100MB peak usage (5x+ improvement)  
- ✅ **Latency**: <10ms per image (10x+ improvement)
- ✅ **Tests**: All functionality tests pass
- ✅ **Understanding**: You can explain each optimization

## 🔍 Profiling Tools

```bash
# Memory profiling
cargo run --release -- sample-images/ --memory-profile

# CPU profiling  
cargo flamegraph --bin image-processor -- sample-images/

# Benchmark suite
cargo bench --bench image_processing_benchmarks
```

## 🆘 Getting Unstuck

1. **Start with profiling** - Measure before optimizing
2. **Fix the biggest bottleneck first** - Don't micro-optimize
3. **Use the benchmarks** - Quantify improvements
4. **Check `hints/` directory** - Progressive hints available
5. **Compare with C#** - Think about equivalent optimizations

## 📁 Project Structure

```
project-optimization-challenge/
├── src/
│   ├── main.rs              # CLI application (FIX: Command-line processing)
│   ├── processor.rs         # Core processing (FIX: Multiple performance issues)
│   ├── filters.rs           # Image filters (FIX: SIMD opportunities)
│   ├── pipeline.rs          # Processing pipeline (FIX: Parallelization)
│   └── allocator.rs         # Memory management (FIX: Allocation patterns)
├── sample-images/           # Test images for processing
├── benches/                 # Performance benchmarks
└── tests/                   # Correctness tests
```

## 💡 C# Developer Notes

This optimization challenge mirrors common C# performance issues:

- **Memory allocation** → Like excessive `new` in tight loops
- **Cache misses** → Like poor array access patterns  
- **Threading issues** → Like `lock` contention or missing `Parallel.ForEach`
- **Algorithm choice** → Like using `List.Contains()` instead of `HashSet`
- **SIMD opportunities** → Like missing `System.Numerics.Vector<T>`

The performance gains you'll achieve here translate directly to optimizing C# applications!

---

**Ready to optimize?** Start with: `cargo run --release -- sample-images/ --verbose` 🦀