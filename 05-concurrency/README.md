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

## 💪 Exercises - Fix These Broken Programs!

Each exercise contains intentionally broken concurrent code. Your job: make it compile and run correctly!

1. **ex01-threading-basics.rs** - Fix thread spawning and ownership errors (10 checkpoints)
2. **ex02-channels.rs** - Repair producer-consumer communication (8 checkpoints)  
3. **ex03-shared-state.rs** - Fix Arc/Mutex compilation errors (8 checkpoints)
4. **ex04-deadlock.rs** - Identify and fix a real deadlock (6 checkpoints)
5. **ex05-async-tokio.rs** - Debug async/await and task spawning (10 checkpoints)

## 🏗️ Project: Multi-threaded Web Scraper

Fix a broken web scraper that should:
- Spawn multiple threads to fetch URLs concurrently
- Use channels to distribute work
- Share results with `Arc<Mutex<T>>`
- Implement async I/O with tokio
- Handle errors gracefully

**Starting State**: Compiles with 15+ errors!
**Your Goal**: A working concurrent web scraper faster than any C# equivalent!

## 📂 Module Structure

```
05-concurrency/
├── README.md                          # You are here!
├── DEBUGGING_CHECKLIST.md             # Concurrency debugging guide
├── exercises/
│   ├── ex01-threading-basics.rs       # Thread spawning & ownership
│   ├── ex02-channels.rs               # Producer-consumer patterns
│   ├── ex03-shared-state.rs           # Arc/Mutex shared state
│   ├── ex04-deadlock.rs               # Deadlock identification/fixing
│   ├── ex05-async-tokio.rs            # Async/await and Tokio
│   └── hints/
│       ├── README.md                  # How to use hints
│       ├── ex01-level1.md             # Gentle nudges
│       ├── ex01-level2.md             # Specific guidance
│       ├── ex01-level3.md             # Near-solution
│       └── ... (3 levels per exercise)
├── project-web-scraper/
│   ├── Cargo.toml                     # Dependencies set up
│   ├── README.md                      # Project instructions
│   ├── src/
│   │   └── main.rs                    # Broken scraper code
│   └── tests/
│       └── integration.rs             # Tests to pass
└── reference/
    ├── README.md                      # Additional resources
    ├── send-sync-traits.md            # Deep dive: Send/Sync
    ├── arc-mutex-patterns.md          # Sharing state safely
    ├── async-ecosystem.md             # Tokio and futures
    └── csharp-comparison.md           # Task vs Future
```

---

**Ready?** Start with: `cd exercises && rustc ex01-threading-basics.rs` 🦀

**Next Module**: [06 - Performance](../06-performance/README.md) →
