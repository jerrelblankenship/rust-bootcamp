# Exercise 01 - Level 2 Hints: Dependency Hell

## 🎯 Specific Guidance

The issue is that **reqwest** and **tokio** need to use compatible versions. Here's what you need to know:

## 🔧 Version Compatibility

1. **reqwest 0.11** requires **tokio 1.0** with specific features
2. **serde_json 1.0** is compatible with both
3. The problem is in the **feature flags** and **version alignment**

## 📋 Steps to Fix

1. **Check reqwest documentation** for its tokio requirements:
   ```toml
   reqwest = { version = "0.11", features = ["json"] }
   ```

2. **Match tokio version** to what reqwest expects:
   ```toml
   tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
   ```

3. **Verify serde_json** is compatible:
   ```toml
   serde_json = "1.0"
   ```

## 🤔 C# Comparison

This is like ensuring your **Microsoft.AspNetCore.Http** package matches your **Microsoft.Extensions.Hosting** version in .NET. The versions must be compatible.

## 🔍 Debug Strategy

1. **Use `cargo tree`** to see dependency tree
2. **Check for duplicate versions** of the same crate
3. **Read the error messages** - they often suggest the right versions

## ⏰ Time Check

Still struggling after 30 minutes? Move to Level 3 for the solution.

**Hint**: The exact versions matter, and features need to be enabled properly.