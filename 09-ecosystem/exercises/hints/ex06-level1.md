# Exercise 06 - Level 1 Hints: Documentation Disaster

## 🎯 What's the Problem?

The documentation has broken doctests, missing examples, and inconsistent information. This will fail `cargo test` and confuse users.

## 🔍 Test the Documentation

```bash
cargo test --doc
```

You'll see multiple failures showing what's broken.

## 🤔 C# Comparison

This is like:
- **XML documentation** with wrong `<example>` tags
- **Unit tests** that don't compile
- **Inconsistent documentation** vs actual behavior

## 🔧 What to Fix

1. **Broken import paths** in examples
2. **Wrong expected results** in assertions
3. **Missing documentation** for public items
4. **Inconsistent behavior** descriptions
5. **Unsafe code** without proper documentation

## ⏰ Time Check

Spent 15 minutes? Move to Level 2 for specific fixes.