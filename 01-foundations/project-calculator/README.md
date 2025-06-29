# CLI Calculator Project - Fix the Broken Code!

🔧 **Your Mission**: Make this calculator compile and work correctly by fixing all the compilation errors and implementing the missing functionality.

## 🎯 Learning Through Discovery

This project demonstrates the **discovery-based learning approach**:
- ✅ **Start with broken code** that won't compile
- ✅ **Fix compilation errors** one by one
- ✅ **Learn from real Rust compiler messages**
- ✅ **Build understanding through debugging**

## 🚨 Current Status: BROKEN! 

```bash
cd project-calculator
cargo build  # ❌ WILL SHOW COMPILATION ERRORS
```

Your job is to make it work by fixing the broken starter code in `src/main.rs`.

## 🎯 Project Goals

Through fixing the broken code, you'll learn to create a calculator that:
- ✅ Parses command-line arguments correctly
- ✅ Supports basic arithmetic operations (+, -, *, /)
- ✅ Handles errors gracefully (no crashes!)
- ✅ Uses structs for data organization
- ✅ Implements enums for operation types
- ✅ Demonstrates Rust best practices

## 🔧 How to Approach This Project

### **Step 1: Start Simple - Fix Compilation**
```bash
cargo build  # Read the first error message carefully
```

The Rust compiler will tell you exactly what's wrong. Start with the first error and fix them one by one.

### **Step 2: Implement TODO Items**
Look for comments like:
- `TODO: Add operation variants here`
- `FIXME: This enum is missing - add the variants!`
- `todo!("Implement calculation logic")`

### **Step 3: Run Tests Frequently**
```bash
cargo test  # See what still needs to be implemented
```

Tests will guide you to what needs to be implemented next.

### **Step 4: Test Your Implementation**
```bash
cargo run -- 5 + 3        # Test basic calculation
cargo run -- 10 / 0       # Test error handling
cargo run -- --help       # Test help functionality
```

## 🧩 What You'll Need to Implement

### **Core Types** (Follow the TODO comments in the code)
- `Operation` enum with arithmetic variants
- `Expression` struct to hold calculation data  
- `CalculatorError` enum for error handling
- `Calculator` struct for state management

### **Key Methods** (Replace the `todo!()` macros)
- `Operation::from_str()` - Parse operation from string
- `Expression::calculate()` - Perform the math
- `parse_args()` - Handle command-line arguments
- `parse_number()` - Convert strings to numbers

### **Error Handling** (Critical for Rust!)
- Division by zero detection
- Invalid input handling
- Graceful error messages

## 🎓 Learning Objectives

By fixing this broken code, you'll master:

1. **Enum Definition**: Representing operations as variants
2. **Struct Implementation**: Organizing calculation data
3. **Pattern Matching**: Handling different operations with `match`
4. **Error Handling**: Using `Result<T, E>` properly
5. **Method vs Function**: Understanding `&self` syntax
6. **String Parsing**: Converting user input to data types
7. **Command-Line Args**: Processing `env::args()`

## 🔄 C# Developer Notes

| Concept | C# Way | Rust Way |
|---------|--------|----------|
| **Enums** | `enum { Add, Sub }` | `enum Operation { Add, Subtract }` |
| **Classes** | `class Calculator { }` | `struct Calculator { }` + `impl` block |
| **Exceptions** | `throw new Exception()` | `Err(CalculatorError::DivisionByZero)` |
| **Null Checks** | `if (value == null)` | `match result { Some(val) => ..., None => ... }` |
| **Method Calls** | `obj.Method()` | `obj.method()` (lowercase, snake_case) |

## 🚨 Common Compilation Errors You'll Fix

### **1. Missing Enum Variants**
```rust
// ❌ This will error
enum Operation {
    // Empty - add variants here!
}

// ✅ Fix by adding variants  
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
```

### **2. Missing Struct Fields**
```rust
// ❌ This will error
struct Expression {
    // Missing fields!
}

// ✅ Fix by adding fields
struct Expression {
    left: f64,
    operation: Operation,
    right: f64,
}
```

### **3. Unimplemented Methods**
```rust
// ❌ This will error
fn calculate(&self) -> Result<f64, CalculatorError> {
    todo!("Implement calculation logic")
}

// ✅ Fix by implementing logic
fn calculate(&self) -> Result<f64, CalculatorError> {
    match self.operation {
        Operation::Add => Ok(self.left + self.right),
        // ... more operations
    }
}
```

## 💡 Debugging Tips

### **Read Error Messages Carefully**
Rust's compiler gives excellent hints:
```
error[E0412]: cannot find type `Operation` in this scope
   --> src/main.rs:15:20
    |
15  |     operation: Operation,
    |                ^^^^^^^^^ not found in this scope
```

### **Fix One Error at a Time**
Don't try to fix everything at once. Fix the first error, then run `cargo build` again.

### **Use `cargo check` for Faster Feedback**
```bash
cargo check  # Faster than full compilation
```

### **Ask Yourself: "What Would This Be in C#?"**
Then translate that concept to Rust patterns.

## 🧪 Testing Your Progress

The project includes tests that will help guide your implementation:

```bash
cargo test                    # Run all tests
cargo test test_operation     # Run specific test
cargo test -- --nocapture    # See println! output
```

Tests start failing and gradually pass as you implement functionality.

## 🏆 Success Criteria

Your calculator is complete when:

1. ✅ `cargo build` succeeds (no compilation errors)
2. ✅ `cargo test` passes (all tests green)  
3. ✅ `cargo run -- 5 + 3` outputs `5 + 3 = 8`
4. ✅ `cargo run -- 10 / 0` shows error message (doesn't crash)
5. ✅ `cargo run -- --help` shows usage information

## 🆘 When You Get Stuck

1. **Read the compiler error** - it's usually very helpful
2. **Look at the TODO comments** - they contain hints
3. **Check the test cases** - they show expected behavior
4. **Think in C# first** - then translate to Rust patterns
5. **Consult the solutions/** directory - but only as a last resort!

## 🚀 Extension Challenges (After Basic Version Works)

Once you have the basic calculator working:

1. **Add more operations**: Power (^), modulo (%), square root
2. **Support parentheses**: `calc "(5 + 3) * 2"`
3. **Interactive mode**: `calc -i` for REPL interface
4. **Variable storage**: `calc "x = 5" "x + 3"`
5. **Calculation history**: Remember previous results

## 📚 What This Project Teaches

By the end, you'll understand:
- How Rust's ownership system prevents common bugs
- Why explicit error handling is powerful
- How pattern matching makes code clear and safe
- The difference between `panic!` and graceful error handling
- How Rust's type system catches mistakes at compile time

---

**Remember**: Every compilation error you fix teaches you something valuable about Rust's safety guarantees. Embrace the struggle - it's where the learning happens!

**🎯 Ready to start?** Open `src/main.rs` and begin fixing the broken code!
