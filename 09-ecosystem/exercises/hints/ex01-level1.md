# Exercise 01 - Level 1 Hints: Dependency Hell

## 🎯 What's the Problem?

You're getting compilation errors because of **dependency version conflicts**. This is similar to .NET's DLL hell, but Rust's cargo handles it better once you understand the rules.

## 🔍 First Steps

1. **Try to compile** and read the error messages:
   ```bash
   cd exercises
   rustc ex01-dependency-hell.rs
   ```

2. **Create a proper Cargo project** instead of using rustc directly:
   ```bash
   cargo new ex01-dependency-hell --bin
   cd ex01-dependency-hell
   # Copy the code into src/main.rs
   ```

3. **Look at the error messages** - Cargo tells you exactly what's wrong with versions

## 🤔 Think About It

- **C# Analogy**: Like NuGet package conflicts, but Cargo is more explicit
- **Key Question**: Which versions of these crates actually work together?
- **Research**: Check the documentation for each crate to see their compatibility

## 🔧 What to Research

1. Visit [crates.io](https://crates.io) and search for:
   - `reqwest` - What tokio version does it require?
   - `serde_json` - What serde version does it need?
   - `tokio` - What features are needed for HTTP clients?

2. Read the **README** and **dependencies** section of each crate

## ⏰ Time Check

Spent 15 minutes? If you're still stuck, move to Level 2 hints.

The key insight is that crates need **compatible versions** of their shared dependencies.