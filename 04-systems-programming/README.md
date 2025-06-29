# Module 04: Systems Programming

Master low-level Rust programming through hands-on debugging and real-world problem solving. This module teaches systems concepts by fixing broken code, implementing unsafe abstractions safely, and building high-performance applications.

## 🎯 What You'll Learn

By the end of this module, you'll be able to:

- Control memory layout and choose between stack/heap allocation
- Write safe abstractions over unsafe code blocks  
- Interface with C libraries using Foreign Function Interface (FFI)
- Build zero-copy, high-performance data structures
- Debug memory-related compilation errors with confidence
- Create system-level applications that rival C/C++ performance

## 📚 Learning Materials

### **Concept Lessons**
Learn systems programming through practical examples and C# comparisons:

1. **[Memory Layout and Control](01-memory-layout.md)** - From managed memory to manual control
2. **[Unsafe Rust](02-unsafe-rust.md)** - Safe abstractions over dangerous operations  
3. **[Foreign Function Interface](03-ffi.md)** - Calling C from Rust and vice versa

### **Hands-On Practice**
Master systems programming by fixing broken code and debugging real errors:

- **ex01-memory-layout.rs** - Memory layout bugs and padding errors (broken code to fix)
- **ex02-unsafe-debugging.rs** - Unsafe code compilation errors (broken code to fix)
- **ex03-c-interop.rs** - FFI integration problems (broken code to fix)

### **Major Project**
- **System Monitor** - Build a real-time system resource monitor from partially working starter code

## 🔄 For C# Developers

Here's how C#'s managed memory model compares to Rust's systems programming:

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| Garbage Collector | Manual memory management | Compile-time ownership |
| `unsafe` blocks | `unsafe` blocks | More restricted, safer |
| P/Invoke | FFI | Type-safe by default |
| `fixed` statement | Raw pointers | Must be in unsafe blocks |
| `Marshal` class | `std::mem` utilities | Zero-cost abstractions |
| Memory profilers | Built-in layout control | Compile-time optimization |

## 🚀 Getting Started

### **Step 1: Read the First Lesson**
Start with [Memory Layout and Control](01-memory-layout.md) to understand the foundation.

### **Step 2: Fix the Exercises**
The exercises contain broken code with real compilation errors:

```bash
cd 04-systems-programming/exercises
rustc ex01-memory-layout.rs   # This will show memory layout errors
```

Your job is to fix the errors using Rust's powerful compiler messages.

### **Step 3: Build the System Monitor**
```bash
cd project-system-monitor
cargo build  # This will show compilation errors to fix
```

Start with basic memory operations and build up to a complete monitoring system.

## 💡 Learning Approach

### **How the Exercises Work**
- Each exercise contains broken systems programming code
- Compilation errors guide you to correct memory management patterns
- Multiple approaches to solving each low-level challenge
- Real-world scenarios that build practical systems skills

### **How to Succeed**
1. **Trust the compiler** - Rust's error messages are exceptionally detailed for systems code
2. **Think about safety** - Even in unsafe blocks, maintain invariants
3. **Compare with C#** - How would P/Invoke or unsafe code handle this?
4. **Start simple** - Fix one memory error at a time
5. **Understand the why** - Every unsafe block should have a clear safety justification

## 📈 Your Learning Path

### **Day 1: Master Memory Layout Through Debugging**
- Complete the "Memory Layout and Control" lesson
- Fix compilation errors in exercise 1
- Understand struct padding and alignment through trial and error
- Start the system monitor project

### **Day 2: Conquer Unsafe Code Safely**
- Study "Unsafe Rust" lesson
- Debug exercises 2 and 3
- Learn to write safe abstractions over unsafe operations
- Implement core monitoring functionality

### **Day 3: FFI and Production Systems**
- Learn "Foreign Function Interface" lesson
- Complete all exercises with robust error handling
- Finish the system monitor project
- Integrate with system libraries for real data

## 🏆 Success Criteria

You've mastered this module when:
- ✅ All exercises compile and run without errors
- ✅ You can explain when and why to use unsafe code
- ✅ Your system monitor displays real CPU and memory data
- ✅ You understand memory layout optimization techniques
- ✅ You can safely interface with C libraries

## 📁 Module Structure

```
04-systems-programming/
├── README.md                     # This guide
├── 01-memory-layout.md           # Memory control fundamentals
├── 02-unsafe-rust.md             # Safe abstractions over unsafe
├── 03-ffi.md                     # C interoperability
├── exercises/
│   ├── ex01-memory-layout.rs     # Memory bugs to fix
│   ├── ex02-unsafe-debugging.rs  # Unsafe compilation errors
│   ├── ex03-c-interop.rs         # FFI integration problems
│   └── hints/
│       └── README.md            # Help when you're stuck
└── project-system-monitor/       # Major project
    ├── Cargo.toml               # Dependencies for system access
    ├── src/
    │   ├── main.rs              # Partially working CLI interface
    │   ├── memory.rs            # Memory monitoring (partially working)
    │   ├── cpu.rs               # CPU monitoring (partially working)
    │   └── process.rs           # Process tracking (partially working)
    ├── tests/                   # Integration tests
    └── README.md               # Project guide with step-by-step fixes
```

## 🆘 When You Get Stuck

**Follow this systematic approach:**

1. **Read the error message carefully** - Rust's compiler is incredibly helpful for systems code
2. **Check the [Debugging Checklist](DEBUGGING_CHECKLIST.md)** - Systems programming troubleshooting guide
3. **Use the progressive hints system** - Check `exercises/hints/` directory
   - Level 1: Gentle systems programming concept guidance
   - Level 2: Specific unsafe code and FFI solutions
   - Level 3: Complete systems programming pattern examples
4. **Think about memory safety** - What are the safety invariants?
5. **Review the lesson material** - Systems concepts build on each other
6. **Ask for help** - After working through systems programming patterns

**Remember:** Systems programming requires careful attention to safety. Unsafe code needs extra scrutiny!

## 🔗 Additional Resources

### **Built-in Learning Support:**
- [Debugging Checklist](DEBUGGING_CHECKLIST.md) - Systems programming troubleshooting
- [Progressive Hints](exercises/hints/README.md) - Guided systems programming discovery
- [Exercise Solutions](exercises/instructor-only/README.md) - For instructors only

### **External Resources:**
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - The Dark Arts of Unsafe Rust
- [std::mem documentation](https://doc.rust-lang.org/std/mem/) - Memory utilities
- [libc crate](https://docs.rs/libc/) - C library bindings

## ➡️ What's Next?

After completing this module, you'll be ready for [Module 05: Concurrency](../05-concurrency/README.md), where you'll apply your systems programming skills to build high-performance concurrent applications!

---

**Ready to master systems programming?** Begin with [Memory Layout and Control](01-memory-layout.md) and discover the power of manual memory management! 🦀