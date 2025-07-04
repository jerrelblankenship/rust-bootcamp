# Module 04: Systems Programming

🎯 **Mission**: Fix broken systems code to master low-level Rust!

## 🚀 Quick Start

1. **Start fixing immediately**:
   ```bash
   cd 04-systems-programming/exercises
   rustc ex01-memory-layout.rs  # Shows 6 errors to fix!
   ```

2. **Fix one error at a time** - Learn memory layout by doing
3. **Use hints only when stuck** - Check `hints/` directory
4. **Build the system monitor** - Apply your systems knowledge

## 📝 What You'll Master

Through **fixing broken code**, you'll learn:
- ✅ Memory layout control (`#[repr(C)]`, alignment, packing)
- ✅ Safe unsafe code patterns
- ✅ Manual memory management
- ✅ Foreign Function Interface (FFI)

## 🔧 Learning Path

### **Step 1: Fix the Exercises**
```bash
# Fix compilation errors to learn systems concepts
rustc ex01-memory-layout.rs     # Memory representation
rustc ex02-unsafe-operations.rs # Unsafe blocks & pointers
rustc ex03-manual-memory.rs     # Allocation/deallocation
rustc ex04-safe-abstractions.rs # Safe wrappers
rustc ex05-ffi-interop.rs       # C interoperability
```

### **Step 2: Build the System Monitor**
```bash
cd project-system-monitor
cargo build  # Shows errors to fix
cargo test   # Verify your implementation
cargo run    # Monitor real system resources!
```

## 🆘 When You Get Stuck

1. **Read the error message** - Unsafe errors are precisely located
2. **Check [Debugging Guide](DEBUGGING_CHECKLIST.md)** - Systems-specific tips
3. **Use progressive hints** - `hints/ex01-level1.md` → `level2.md` → `level3.md`
4. **Compare with C#** - Think P/Invoke but more powerful

## 🏆 Success = Working Systems Code

You've mastered this module when:
- ✅ All exercises compile and run
- ✅ System monitor shows CPU, memory, and disk usage
- ✅ You understand when and why to use unsafe code

## 📚 Need More Detail?

- 📖 **[Systems Concepts](reference/)** - In-depth explanations
- 🔄 **[C# Unsafe vs Rust](reference/csharp-comparisons.md)** - Direct comparisons
- 📋 **[FFI Patterns](reference/ffi-detailed.md)** - C integration guide

---

**Prerequisites**: Complete [Module 03 - Error Handling](../03-error-handling/README.md) first - systems programming requires solid error handling.

**Ready?** Start with: `cd exercises && rustc ex01-memory-layout.rs` 🦀

**Next Module**: [05 - Concurrency](../05-concurrency/README.md) → Master fearless concurrency with threads and async!
