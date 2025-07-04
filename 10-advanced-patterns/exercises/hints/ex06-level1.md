# Exercise 06 - Level 1 Hints: Unsafe Undefined

## 🎯 What's Going Wrong?

Your unsafe code has undefined behavior! This is the most dangerous part of Rust - when you use `unsafe`, you're responsible for maintaining all safety invariants that the compiler usually enforces.

## 🔍 First Steps

1. **Try to compile** and see what breaks:
   ```bash
   rustc ex06-unsafe-undefined.rs
   ```

2. **Read the error messages** - Look for safety violations

3. **Identify the error categories**:
   - Null pointer dereferences
   - Uninitialized memory usage
   - Aliasing violations (multiple mutable references)
   - Use-after-free errors
   - Invalid transmute operations

## 🤔 Think About It

- **C# Analogy**: Like unsafe code blocks but with stricter responsibility
- **Key Question**: What safety contracts are you violating?
- **Strategy**: Understand each unsafe operation's requirements

## 🔧 What to Research

1. **Unsafe operations** - What each `unsafe` block does
2. **Safety invariants** - What you must maintain
3. **Pointer safety** - Valid vs invalid pointer usage
4. **Memory safety** - Initialization, lifetimes, aliasing

## 📚 Resources to Use

- **Rust Nomicon** - The Dark Arts of Unsafe Rust
- **Rust Reference** - Unsafe code guidelines
- **Miri** - Tool for detecting undefined behavior

## 🎮 Systematic Approach

1. **Understand unsafe contracts** - Learn what you're responsible for
2. **Fix null pointer issues** - Add null checks
3. **Fix uninitialized memory** - Use proper initialization
4. **Fix aliasing violations** - Ensure exclusive access
5. **Fix manual memory management** - Proper allocation/deallocation

## ⏰ Time Check

Spent 15 minutes? If you're confused about unsafe responsibilities, move to Level 2 for specific guidance.

**Hint**: Start with the simplest issues - null pointer checks and uninitialized memory. These are immediate safety violations!