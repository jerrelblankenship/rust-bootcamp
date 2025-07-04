# Exercise 03 - Level 1 Hints: Crate Selection Confusion

## 🎯 What's the Problem?

You're using **outdated, deprecated, or suboptimal crates**. The Rust ecosystem has evolved, and there are better alternatives for common tasks.

## 🔍 First Steps

1. **Try to compile** and see what happens:
   ```bash
   cargo build
   ```

2. **Look for warning messages** about deprecated crates

3. **Notice the verbose, complex code** - this should be simpler!

## 🤔 Think About It

- **C# Analogy**: Like using old .NET Framework libraries instead of modern .NET alternatives
- **Key Question**: What are the current best practices for these tasks?
- **Research**: Find modern alternatives for each outdated crate

## 🔧 What to Research

1. **JSON parsing**: What replaced `rustc_serialize`?
2. **HTTP clients**: What's easier than raw `hyper` for simple requests?
3. **Error handling**: What replaced `failure`?

## 📚 How to Find Better Crates

1. **Search [crates.io](https://crates.io)** for:
   - "json" - look for most downloaded/maintained
   - "http client" - find high-level alternatives
   - "error handling" - modern error libraries

2. **Check [lib.rs](https://lib.rs)** for curated recommendations

3. **Look at GitHub stars** and **recent commits** for activity

## ⏰ Time Check

Spent 15 minutes? If you're still stuck, move to Level 2 hints.

**Hint**: The most popular crates are usually the best choice for common tasks!