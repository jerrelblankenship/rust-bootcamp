# Module 05: Concurrency and Parallelism

🎯 **Mission**: Fix broken concurrent code to master fearless concurrency!

## 🚀 Quick Start

1. **Start coding immediately**:
   ```bash
   cd 05-concurrency/exercises
   rustc ex01-threading-basics.rs  # Shows compilation errors to fix!
   ```

2. **Fix one error at a time** - Let the compiler guide you
3. **Use hints only when stuck** - Check `hints/` directory
4. **Build the web scraper** - Apply what you've learned

## 📝 What You'll Master

Through **fixing broken concurrent code**, you'll learn:
- ✅ Thread spawning and ownership transfer (`move` semantics)
- ✅ Channel communication patterns (MPSC)
- ✅ Shared state with `Arc` and `Mutex`
- ✅ Deadlock identification and prevention
- ✅ Async/await and Tokio task spawning

## 🔧 Learning Path

### **Step 1: Fix the Exercises**
```bash
# Fix compilation errors one by one
rustc ex01-threading-basics.rs  # Thread spawning & ownership
rustc ex02-channels.rs          # Producer-consumer patterns
rustc ex03-shared-state.rs      # Arc/Mutex for shared data
rustc ex04-deadlock.rs          # Deadlock debugging
cargo check ex05-async-tokio.rs # Async/await with Tokio
```

### **Step 2: Build the Web Scraper**
```bash
cd project-web-scraper
cargo build  # Shows errors to fix
cargo test   # Verify your implementation
cargo run -- --urls https://example.com  # Test your scraper!
```

## 🆘 When You Get Stuck

1. **Read the error message** - Rust prevents data races at compile time!
2. **Check [Debugging Guide](DEBUGGING_CHECKLIST.md)** - Systematic concurrency troubleshooting
3. **Use progressive hints** - `hints/ex01-level1.md` → `level2.md` → `level3.md`
4. **Compare with C#** - `Task.Run()` vs `thread::spawn()`, `lock` vs `Mutex`

## 🏆 Success = Working Concurrent Code

You've mastered this module when:
- ✅ All exercises compile and run without data races
- ✅ Web scraper works: `cargo run -- --urls https://example.com`
- ✅ You can explain why Rust prevents data races at compile time
- ✅ You can convert C# concurrent code to Rust

## 📚 Need More Detail?

- 📖 **[Detailed Concepts](reference/)** - In-depth explanations
- 🔄 **[C# vs Rust Guide](reference/csharp-comparison.md)** - Task vs Thread patterns
- 📋 **[Troubleshooting](DEBUGGING_CHECKLIST.md)** - When concurrent code goes wrong



---

**Ready?** Start with: `cd exercises && rustc ex01-threading-basics.rs` 🦀

**Next Module**: [06 - Performance](../06-performance/README.md) →
