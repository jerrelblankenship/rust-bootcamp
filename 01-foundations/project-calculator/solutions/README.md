# Calculator Project Solutions

## 🛟 When You're Really Stuck

**First, make sure you've tried!** This directory exists to help when you've genuinely hit a wall, not as a shortcut. The real learning happens when you struggle with the problems yourself.

## 📋 What's Available Here

- **`main.rs`** - Complete working calculator (last resort!)
- **`README.md`** - This debugging and concept guide

## 🚨 Before Looking at Code Solutions

Try these debugging strategies first:

### **Read the Compiler Error Carefully**
```
error[E0412]: cannot find type `Operation` in this scope
```
This tells you exactly what's missing - you need to define an `Operation` type.

### **Start with the Simplest Fix**
Don't try to implement everything at once. If you see "cannot find type `Operation`", just create an empty enum first:
```rust
enum Operation {
    // Start here - what operations does a calculator need?
}
```

### **Use the TODO Comments as Hints**
The broken starter code has hints like:
```rust
// TODO: Add operation variants here
// What operations do calculators typically support?
```

## 🔧 Debugging Strategy Guide

### **When You See: "cannot find type 'X'"**
**What it means**: You need to define that type (enum, struct, etc.)
**How to fix**: Look at how the type is being used, then define it
**C# parallel**: Like getting "Type 'X' not found" - you need to create the class/enum

### **When You See: "this function takes 0 parameters but 3 were supplied"**
**What it means**: Your function signature doesn't match how it's being called
**How to fix**: Look at the function call, then fix the function definition
**C# parallel**: Method signature mismatch - same concept

### **When You See: "mismatched types"**
**What it means**: You're trying to use a value as the wrong type
**How to fix**: Either convert the type or fix the expected type
**C# parallel**: Like trying to assign `int` to `string` without conversion

## 💡 Conceptual Guidance (Without Spoilers)

### **For the Operation Enum**
Think about: What mathematical operations does a calculator need?
- How would you represent these in C# enum?
- Rust enums work similarly, but with more power
- You'll need methods to convert strings to operations

### **For the Expression Struct**
Think about: What data represents "5 + 3"?
- What are the components of this expression?
- How would you model this in a C# class?
- Rust structs are like C# classes but simpler

### **For Error Handling**
Think about: What can go wrong in a calculator?
- Invalid input
- Math errors (like division by zero)
- Parsing failures
- How does this compare to C# exceptions?

## 🔄 Key Rust Concepts You'll Discover

### **Enums Are More Powerful**
```rust
// C# enum: enum { Add, Subtract }
// Rust enum: can hold data and have methods!
```

### **Pattern Matching Is Your Friend**
```rust
// Instead of C#'s switch, use match:
match operation {
    Operation::Add => /* handle addition */,
    Operation::Divide => /* check for zero! */,
    // ...
}
```

### **Results Instead of Exceptions**
```rust
// C#: throw new Exception("error");
// Rust: Err(CalculatorError::DivisionByZero)
```

## 🚨 Common Pitfalls for C# Developers

### **Forgetting `&self` in Methods**
```rust
// ❌ This won't compile
fn display() -> String { ... }

// ✅ Methods need &self (like 'this' in C#)
fn display(&self) -> String { ... }
```

### **Not Handling Results**
```rust
// ❌ Ignoring potential errors
let result = calculate();

// ✅ Rust forces you to handle errors
match calculate() {
    Ok(result) => println!("Answer: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

## 🆘 If You're Completely Stuck

### **Check One Thing at a Time**
1. Does your code compile? Fix compilation errors first
2. Do the tests pass? Run `cargo test` to see what's still broken
3. Does it handle basic cases? Try `cargo run -- 5 + 3`
4. Does it handle errors? Try `cargo run -- 5 / 0`

### **Compare with C# Thinking**
1. How would I solve this in C#?
2. What's the Rust equivalent of that C# concept?
3. What's different about error handling in Rust?

### **Last Resort: Look at Complete Solution**
If you've tried everything above and are still stuck:
1. Look at `main.rs` in this directory
2. Don't copy-paste! Read and understand each part
3. Then go back and implement it yourself
4. The goal is understanding, not just getting it to work

## 🎯 Success Indicators

You're succeeding when:
- You can read a Rust error message and know what to fix
- You think "this is like X in C#, but safer" 
- You fix problems without looking at solutions
- You understand WHY the solution works, not just WHAT it is

## 📚 Helpful Resources

- [Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

**Remember**: Every moment you spend struggling with a problem is learning time. The compiler errors that seem frustrating now are teaching you to write safer, more reliable code!

**🎯 Goal**: Understand Rust's approach to safety and performance through hands-on problem solving, not just get the code to work.
