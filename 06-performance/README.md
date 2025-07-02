# Module 06: Performance Optimization

🎯 **Mission**: Fix broken performance code to master Rust optimization!

## 🚀 Quick Start

1. **Start fixing immediately**:
   ```bash
   cd 06-performance/exercises
   rustc ex01-allocation-storm.rs && ./ex01-allocation-storm  # Shows terrible performance to fix!
   ```

2. **Fix one performance issue at a time** - Profile, identify, optimize
3. **Use hints only when stuck** - Check `hints/` directory
4. **Build the image processor** - Apply your optimization skills

## 📝 What You'll Master

Through **fixing broken performance code**, you'll learn:
- ✅ Memory allocation optimization and pool patterns
- ✅ CPU cache optimization and data layout
- ✅ Iterator chains and zero-cost abstractions
- ✅ SIMD vectorization for computation speedup
- ✅ Parallel processing without overhead
- ✅ Async performance and avoiding blocking
- ✅ Bounds checking elimination strategies
- ✅ String processing optimization techniques

## 🔧 Learning Path

### **Step 1: Fix the Exercises**
```bash
# Fix performance disasters one by one
rustc ex01-allocation-storm.rs && ./ex01-allocation-storm    # Memory allocation issues
rustc ex02-string-builder.rs && ./ex02-string-builder      # String concatenation hell
rustc ex03-cache-misses.rs && ./ex03-cache-misses        # CPU cache disasters
rustc ex04-iterator-chains.rs && ./ex04-iterator-chains     # Slow iterator patterns
rustc ex05-bounds-checking.rs && ./ex05-bounds-checking     # Safety vs performance
rustc ex06-parallel-waste.rs && ./ex06-parallel-waste      # Bad threading patterns
rustc ex07-async-blocking.rs && ./ex07-async-blocking      # Blocking patterns
rustc ex08-simd-opportunity.rs && ./ex08-simd-opportunity    # Missing vectorization
```

### **Step 2: Build the Image Processor**
```bash
cd project-optimization-challenge
cargo bench   # Shows current terrible performance
cargo test     # Verify correctness while optimizing
cargo run      # Process images at blazing speed!
```

## 🆘 When You Get Stuck

1. **Profile first** - Use `cargo flamegraph` or built-in profiling
2. **Check [Performance Debugging Guide](DEBUGGING_CHECKLIST.md)** - Systematic approach
3. **Use progressive hints** - `hints/ex01-level1.md` → `level2.md` → `level3.md`
4. **Compare with C#** - Think about equivalent optimizations you'd make

## 🏆 Success = Blazing Fast Code

You've mastered this module when:
- ✅ All exercises meet their performance targets (shown in benchmarks)
- ✅ Image processor handles 100+ images per second
- ✅ You can identify performance bottlenecks by reading code
- ✅ You write fast Rust code from the start

## 📚 Need More Detail?

- 📖 **[Performance Concepts](reference/)** - In-depth optimization guides
- 🔄 **[C# vs Rust Performance](reference/csharp-performance.md)** - Speed comparisons
- 🔧 **[Profiling Tools Guide](reference/profiling-guide.md)** - How to measure performance

---

**Ready?** Start with: `cd exercises && rustc ex01-allocation-storm.rs && ./ex01-allocation-storm` 🦀

**Next Module**: [07 - CLI Tools](../07-cli-tools/README.md) →