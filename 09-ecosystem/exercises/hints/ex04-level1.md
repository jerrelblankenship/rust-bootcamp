# Exercise 04 - Level 1 Hints: API Design Disaster

## 🎯 What's the Problem?

This API violates Rust idioms and is painful to use. It's verbose, error-prone, and doesn't leverage Rust's type system.

## 🔍 First Steps

1. **Try to use the API** - notice how painful it is
2. **Look for these problems**:
   - Too many required parameters
   - String-based errors
   - Mutation-based operations
   - Poor method chaining

## 🤔 Think About It

- **C# Analogy**: Like old .NET 1.0 APIs before fluent interfaces
- **Key Question**: How can we make this more ergonomic?
- **Research**: Look at well-designed Rust APIs for patterns

## 🔧 What to Research

1. **Builder pattern** in Rust
2. **Method chaining** patterns
3. **Error types** vs string errors
4. **Immutable by default** design

## 📚 Study These Examples

- `reqwest::Client::builder()`
- `clap::Command::new()`
- `serde_json::Value` API

## ⏰ Time Check

Spent 15 minutes? Move to Level 2 for specific design patterns.