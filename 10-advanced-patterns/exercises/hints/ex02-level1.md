# Exercise 02 - Level 1 Hints: Procedural Macro Panic

## 🎯 What's the Problem?

Procedural macros are more complex than declarative macros - they work at the Abstract Syntax Tree level and require separate crates. This exercise simulates the common issues.

## 🔍 First Steps

1. **Try to compile** and see what breaks:
   ```bash
   rustc ex02-proc-macro-panic.rs
   ```

2. **Read the error messages** - Focus on missing dependencies and trait implementations

3. **Identify the error categories**:
   - Missing dependencies in Cargo.toml
   - Incorrect procedural macro syntax
   - Broken token parsing logic
   - Invalid generated code

## 🤔 Think About It

- **C# Analogy**: Like Source Generators that analyze syntax trees and generate code
- **Key Question**: What would real proc macros need to work correctly?
- **Strategy**: Fix the simulated proc macro implementations step by step

## 🔧 What to Research

1. **Proc macro dependencies** - What crates are needed
2. **Token parsing** - How to parse Rust syntax
3. **Code generation** - How to generate valid Rust code
4. **Feature flags** - What features need to be enabled

## 📚 Resources to Use

- **syn crate documentation** - Parsing Rust syntax
- **quote crate documentation** - Generating Rust code
- **proc-macro2 crate** - Procedural macro primitives

## 🎮 Systematic Approach

1. **Start with dependencies** - Fix Cargo.toml issues
2. **Fix token parsing** - Make parsing logic work correctly
3. **Fix code generation** - Ensure generated code is valid
4. **Test macro expansion** - Verify the macros work as expected

## ⏰ Time Check

Spent 15 minutes? If you're overwhelmed by proc macro complexity, move to Level 2.

**Hint**: This exercise simulates proc macros - focus on the parsing and generation logic!