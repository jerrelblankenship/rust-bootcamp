# Module 01: Foundations

🎯 **Mission**: Fix broken Rust code to learn by doing!

## 🚀 Quick Start

1. **Start coding immediately**:
   ```bash
   cd 01-foundations/exercises
   rustc ex01-hello-world.rs  # Shows compilation errors to fix!
   ```

2. **Fix one error at a time** - Let the compiler guide you
3. **Use hints only when stuck** - Check `hints/` directory
4. **Build the calculator** - Apply what you've learned

## 📝 What You'll Master

Through **fixing broken code**, you'll learn:
- ✅ Basic Rust syntax (macros, variables, functions)
- ✅ Type system fundamentals
- ✅ Pattern matching with `match`
- ✅ Building real applications

## 🔧 Learning Path

### **Step 1: Fix the Exercises**
```bash
# Fix compilation errors one by one
rustc ex01-hello-world.rs  # Basic syntax
rustc ex02-types.rs        # Type system
rustc ex03-functions.rs    # Functions & control flow
rustc ex04-structs.rs      # Custom types
```

### **Step 2: Build the Calculator**
```bash
cd project-calculator
cargo build  # Shows errors to fix
cargo test   # Verify your implementation
cargo run -- 5 + 3  # Test your calculator!
```

## 🆘 When You Get Stuck

1. **Read the error message** - Rust's compiler is incredibly helpful
2. **Check [Debugging Guide](DEBUGGING_CHECKLIST.md)** - Systematic troubleshooting
3. **Use progressive hints** - `hints/ex01-level1.md` → `level2.md` → `level3.md`
4. **Compare with C#** - Most concepts translate directly

## 🏆 Success = Working Code

You've mastered this module when:
- ✅ All exercises compile and run
- ✅ Calculator works: `cargo run -- 5 + 3` outputs `8`
- ✅ You can explain key Rust vs C# differences

## 📚 Need More Detail?

- 📖 **[Detailed Concepts](reference/)** - In-depth explanations
- 🔄 **[C# vs Rust Guide](reference/csharp-comparisons.md)** - Translation guide
- 📋 **[Troubleshooting](DEBUGGING_CHECKLIST.md)** - When things go wrong

---

**Ready?** Start with: `cd exercises && rustc ex01-hello-world.rs` 🦀

**Next Module**: [02 - Ownership](../02-ownership-and-borrowing/README.md) →
