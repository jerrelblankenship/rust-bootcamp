# Exercise 01 - Level 1 Hints: Macro Madness

## 🎯 What's Going Wrong?

Your macros are broken! This is common when learning Rust's macro system. Don't worry - the compiler errors will guide you to the solutions.

## 🔍 First Steps

1. **Try to compile** and see what breaks:
   ```bash
   rustc ex01-macro-madness.rs
   ```

2. **Read the error messages** - Rust's macro errors are very detailed

3. **Identify the error categories**:
   - Syntax errors in macro definitions
   - Pattern matching problems
   - Missing fragment specifiers
   - Hygiene violations

## 🤔 Think About It

- **C# Analogy**: Like debugging Source Generator code that creates invalid syntax
- **Key Question**: What does each macro try to generate, and why is the syntax wrong?
- **Strategy**: Fix one macro at a time, starting with the simplest ones

## 🔧 What to Research

1. **Basic macro syntax** - `macro_rules!` patterns
2. **Fragment specifiers** - `$name:ident`, `$expr:expr`, etc.
3. **Repetition syntax** - `$(...)*` and `$(...)+`
4. **Macro hygiene** - How variables are scoped in macros

## 📚 Resources to Use

- **Rust Book Chapter 19** - Macros
- **`cargo expand`** - See what your macros generate
- **Rust by Example** - Macro examples

## 🎮 Systematic Approach

1. **Start with basic macros** - Fix simple syntax first
2. **Check pattern matching** - Ensure patterns match the calls
3. **Fix repetition syntax** - Handle `$(...)*` correctly
4. **Resolve hygiene issues** - Fix variable capture problems

## ⏰ Time Check

Spent 15 minutes? If you're stuck on syntax, move to Level 2 for specific guidance.

**Hint**: Focus on the missing `!` in `println` and the broken pattern syntax first!