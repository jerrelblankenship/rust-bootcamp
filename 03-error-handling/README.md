# Module 03: Error Handling

Master Rust's revolutionary approach to error handling without exceptions! This module teaches you to eliminate runtime crashes and null reference exceptions through hands-on debugging and real-world problem solving.

## 🎯 What You'll Learn

By the end of this module, you'll be able to:

- Master Option<T> to eliminate null reference exceptions forever
- Use Result<T, E> for explicit, recoverable error handling
- Apply the ? operator for elegant error propagation
- Design custom error types that guide users to solutions
- Build robust error handling strategies for production applications
- Compare Rust's compile-time safety with C#'s runtime exception model

## 📚 Learning Materials

### **Concept Lessons**
Learn error handling through practical examples and C# comparisons:

1. **[Result and Option Types](01-result-and-option.md)** - From exceptions to values
2. **[Error Propagation](02-error-propagation.md)** - The ? operator and error flow
3. **[Custom Error Types](03-custom-errors.md)** - Designing domain-specific errors

### **Hands-On Practice**
Apply error handling by fixing broken code and building robust systems:

- **ex01-option-basics.rs** - Option handling patterns (broken code to fix)
- **ex02-result-chain.rs** - Result chaining and propagation (broken code to fix)
- **ex03-error-types.rs** - Custom error type design (broken code to fix)
- **ex04-conversions.rs** - Error conversions and advanced patterns (broken code to fix)

### **Major Project**
- **File Processor** - Build a comprehensive file processing tool with robust error handling

## 🔄 For C# Developers

Here's how C#'s exception model compares to Rust's error handling:

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| `null` | `Option<T>` | Impossible to ignore null case |
| `try-catch` | `match` on `Result` | Explicit in function signatures |
| `throw` | `return Err(e)` | Errors are return values |
| `finally` | Drop trait | Automatic, deterministic cleanup |
| Exception propagation | `?` operator | Visible and explicit |
| `NullReferenceException` | Compile error | Caught at compile time |

## 🚀 Getting Started

### **Step 1: Read the First Lesson**
Start with [Result and Option Types](01-result-and-option.md) to understand the foundation.

### **Step 2: Fix the Exercises**
The exercises contain broken code with compilation errors:

```bash
cd 03-error-handling/exercises
rustc ex01-option-basics.rs   # This will show Option handling errors
```

Your job is to fix the errors using Rust's error handling patterns.

### **Step 3: Build the File Processor**
```bash
cd project-file-processor
cargo build  # This will show many compilation errors to fix
```

Start with basic error types and gradually build comprehensive error handling.

## 💡 Learning Approach

### **How the Exercises Work**
- Each exercise contains broken error handling code
- Compilation errors guide you to proper error handling patterns
- Multiple approaches to solving each error handling challenge
- Real-world scenarios that build practical skills

### **How to Succeed**
1. **Embrace the compiler** - Rust's error messages guide you to correct solutions
2. **Think about failure modes** - What can go wrong in each operation?
3. **Make errors explicit** - Function signatures should reveal potential failures
4. **Use pattern matching** - Handle each error case specifically
5. **Compare with C#** - How would exceptions handle this scenario?

## 📈 Your Learning Path

### **Day 1: Master Option and Result Fundamentals**
- Complete the "Result and Option Types" lesson
- Fix compilation errors in exercises 1 and 2
- Understand explicit error handling vs exceptions
- Start the file processor project

### **Day 2: Error Propagation and Custom Types**
- Study "Error Propagation" and "Custom Error Types" lessons
- Fix exercises 3 and 4
- Learn the ? operator and error chaining
- Implement custom error types in your project

### **Day 3: Production-Ready Error Handling**
- Complete all exercises with robust error handling
- Finish the file processor project
- Add comprehensive error recovery strategies
- Compare your solution with exception-based approaches

## 🏆 Success Criteria

You've mastered this module when:
- ✅ All exercises compile and handle errors properly
- ✅ Your file processor gracefully handles all failure modes
- ✅ You can design custom error types for any domain
- ✅ You understand why Rust's approach prevents runtime crashes
- ✅ You can explain the benefits over exception-based error handling

## 📁 Module Structure

```
03-error-handling/
├── README.md                    # This guide
├── 01-result-and-option.md     # Foundation of error handling
├── 02-error-propagation.md     # ? operator and error flow
├── 03-custom-errors.md         # Domain-specific error design
├── exercises/
│   ├── ex01-option-basics.rs   # Option handling (broken code)
│   ├── ex02-result-chain.rs    # Result chaining (broken code)
│   ├── ex03-error-types.rs     # Custom errors (broken code)
│   ├── ex04-conversions.rs     # Error conversions (broken code)
│   └── solutions/
│       └── README.md           # Help when you're stuck
└── project-file-processor/     # Major project
    ├── src/
    │   ├── main.rs             # CLI with error handling
    │   ├── error.rs            # Custom error types
    │   ├── processor.rs        # Core processing logic
    │   └── formats/            # Format-specific processors
    ├── tests/                  # Comprehensive test suite
    ├── solutions/              # Complete working solution
    └── README.md              # Project guide
```

## 🆘 When You Get Stuck

1. **Read the error message carefully** - Rust's compiler explains what's wrong
2. **Check the TODO comments** - They contain specific hints
3. **Think about the error cases** - What can go wrong in this operation?
4. **Review the lesson material** - The concepts build on each other
5. **Look at the solutions** - But only after trying yourself!
6. **Compare with exceptions** - How would C# handle this with try-catch?

## 🔗 Additional Resources

- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [anyhow crate](https://docs.rs/anyhow/) - Flexible error handling
- [thiserror crate](https://docs.rs/thiserror/) - Derive error traits

## ➡️ What's Next?

After completing this module, you'll be ready for [Module 04: Systems Programming](../04-systems-programming/README.md), where you'll learn low-level Rust programming with the safety guarantees you've mastered!

---

**Ready to eliminate runtime crashes?** Begin with [Result and Option Types](01-result-and-option.md) and discover why Rust's error handling is revolutionary! 🦀
