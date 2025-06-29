# Module 02: Ownership and Borrowing

🎯 **Mission**: Master Rust's revolutionary memory model by fixing ownership errors!

## 🚀 Start Debugging Now (2 minutes)

```bash
cd 02-ownership-and-borrowing/exercises
rustc ex01-ownership.rs  # Shows ownership violations to fix!
```

**Golden Rule**: Fix ONE ownership error at a time. Let the compiler teach you!

## 💡 The Big Idea

**C# has garbage collection** - multiple variables can reference the same object  
**Rust has ownership** - only ONE variable owns data at a time

This prevents crashes, memory leaks, and data races **at compile time**!

## 🔧 Your Learning Path

### **Step 1: Fix Ownership Violations** (45 minutes)
```bash
# Each exercise builds your ownership intuition
rustc ex01-ownership.rs    # Move vs copy patterns
rustc ex02-borrowing.rs    # Reference rules  
rustc ex03-lifetimes.rs    # Reference validity
rustc ex04-smart-pointers.rs  # Shared ownership
```

### **Step 2: Build Memory Visualizer** (60 minutes)
```bash
cd project-memory-visualizer
cargo build  # Fix ownership errors in a real project
cargo run --demo ownership  # See your learning in action!
```

## 🔍 Key Breakthroughs You'll Have

### **"Aha!" Moment 1: Ownership Transfer**
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2
println!("{}", s1);  // ❌ Error: s1 no longer valid!
```

### **"Aha!" Moment 2: Borrowing is Safe**
```rust
let s1 = String::from("hello");
let s2 = &s1;  // s2 BORROWS from s1  
println!("{}", s1);  // ✅ Works: s1 still owns the data!
```

### **"Aha!" Moment 3: Compiler = Your Friend**
Every error message teaches you safe memory patterns!

## 🔄 C# vs Rust Mental Model

| Scenario | C# (GC) | Rust (Ownership) |
|----------|---------|------------------|
| **Create data** | `var s = "hello";` | `let s = String::from("hello");` |
| **Share data** | `var s2 = s;` (both valid) | `let s2 = s;` (s moved, invalid) |
| **Safe sharing** | Multiple refs allowed | `let s2 = &s;` (borrow, don't move) |
| **Cleanup** | GC decides when | Automatic when owner goes out of scope |

## 🏆 Success = Ownership Intuition

You've mastered this module when:
- ✅ You can predict which operations move vs borrow
- ✅ You fix ownership errors quickly using compiler hints  
- ✅ Your memory visualizer compiles and demonstrates concepts
- ✅ You understand why Rust doesn't need garbage collection

## 🆘 When Ownership Errors Strike

1. **Read the error carefully** - Rust pinpoints the exact problem
2. **Use [Debugging Checklist](DEBUGGING_CHECKLIST.md)** - Ownership-specific help
3. **Ask**: "Who owns this data?" and "Where does ownership transfer?"
4. **Try borrowing first** - Use `&` before moving or cloning

## 📚 Need Deeper Understanding?

- 📖 **[Ownership Deep Dive](reference/)** - Detailed explanations
- 🔧 **[Debugging Guide](DEBUGGING_CHECKLIST.md)** - Systematic troubleshooting
- 💡 **[Progressive Hints](exercises/hints/)** - Guided discovery

---

**Start now**: `cd exercises && rustc ex01-ownership.rs` 🦀

**Next Module**: [03 - Error Handling](../03-error-handling/README.md) →
