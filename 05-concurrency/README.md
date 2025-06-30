# Module 05: Concurrency and Parallelism

Master Rust's fearless concurrency by fixing race conditions, deadlocks, and thread safety issues. Learn by debugging real concurrent code!

## 🎯 Learning Objectives

By fixing broken concurrent code, you will:
- Master Rust's `Send` and `Sync` traits through compiler errors
- Fix thread safety issues with `Arc`, `Mutex`, and `RwLock`
- Debug deadlocks and race conditions
- Repair broken async/await code
- Convert C# Task-based code to Rust futures
- Build a high-performance web scraper

## 🚀 Quick Start

```bash
# Start with the first exercise
cd exercises
rustc ex01-thread-spawn.rs

# When you see compilation errors, that's your learning opportunity!
# Fix them one by one, using hints only after 15+ minutes of trying
```

## 📚 Module Overview

**Your C# Experience**: You've used `Task<T>`, `async/await`, and `lock` statements.
**What's Different**: Rust prevents data races at compile time! No more debugging mysterious concurrency bugs in production.

## 💪 Exercises - Fix These Broken Programs!

Each exercise contains intentionally broken concurrent code. Your job: make it compile and run correctly!

1. **ex01-thread-spawn.rs** - Fix basic thread spawning errors
2. **ex02-move-semantics.rs** - Resolve ownership issues with threads  
3. **ex03-channels-mpsc.rs** - Repair broken channel communication
4. **ex04-shared-state.rs** - Fix Arc/Mutex compilation errors
5. **ex05-deadlock.rs** - Identify and fix a deadlock
6. **ex06-async-basic.rs** - Debug async/await issues
7. **ex07-tokio-spawn.rs** - Fix tokio runtime errors
8. **ex08-parallel-iterator.rs** - Repair rayon parallel iterator code

## 🏗️ Project: Multi-threaded Web Scraper

Fix a broken web scraper that should:
- Spawn multiple threads to fetch URLs concurrently
- Use channels to distribute work
- Share results with Arc<Mutex<T>>
- Implement async I/O with tokio
- Handle errors gracefully

**Starting State**: Compiles with 15+ errors!
**Your Goal**: A working concurrent web scraper faster than any C# equivalent!

## 🧰 Debugging Toolkit

- **[DEBUGGING_CHECKLIST.md](DEBUGGING_CHECKLIST.md)** - Systematic approach to concurrency errors
- **Hint System** - Progressive hints in `exercises/hints/`
- **Reference Docs** - Detailed explanations in `reference/`

## 🎮 Learning Path

1. **Start here**: `exercises/ex01-thread-spawn.rs`
2. **Stuck?** Wait 15 minutes, then check `hints/ex01-level1.md`
3. **Still stuck?** Progress through hint levels
4. **Success?** Move to the next exercise
5. **All exercises done?** Tackle the web scraper project!

## 🏆 Victory Conditions

You've mastered this module when you can:
- [ ] Fix all 8 exercises without Level 3 hints
- [ ] Complete the web scraper project
- [ ] Explain why Rust prevents data races
- [ ] Convert any C# concurrent code to Rust
- [ ] Debug deadlocks systematically

## 📂 Module Structure

```
05-concurrency/
├── README.md                          # You are here!
├── DEBUGGING_CHECKLIST.md             # Concurrency debugging guide
├── exercises/
│   ├── ex01-thread-spawn.rs           # Basic thread errors
│   ├── ex02-move-semantics.rs         # Ownership with threads
│   ├── ex03-channels-mpsc.rs          # Channel communication
│   ├── ex04-shared-state.rs           # Arc/Mutex errors
│   ├── ex05-deadlock.rs               # Deadlock to fix
│   ├── ex06-async-basic.rs            # Async/await issues
│   ├── ex07-tokio-spawn.rs            # Runtime errors
│   ├── ex08-parallel-iterator.rs      # Rayon parallelism
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

Ready to embrace fearless concurrency? Start with `exercises/ex01-thread-spawn.rs`! 🚀
