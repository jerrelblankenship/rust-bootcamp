# Exercise 02 - Level 1 Hints: Feature Flag Fiasco

## 🎯 What's the Problem?

You're getting compilation errors because **feature flags** are missing. Rust crates use features to keep compile times fast and binaries small - but you need to enable the features you use!

## 🔍 First Steps

1. **Try to compile** and read the error messages:
   ```bash
   cargo build
   ```

2. **Look for messages** like:
   - "no method named `json` found"
   - "cannot find derive macro `Serialize`"
   - "no associated function `new_v4` found"

3. **Each error** points to a missing feature flag

## 🤔 Think About It

- **C# Analogy**: Like conditional compilation (`#if DEBUG`), but at the package level
- **Key Question**: Which features does each crate need for your use case?
- **Research**: Check each crate's documentation for feature flags

## 🔧 What to Research

1. **serde** - needs `derive` feature for `#[derive(Serialize, Deserialize)]`
2. **reqwest** - needs `json` feature for `.json()` method
3. **chrono** - needs `serde` feature for serialization
4. **uuid** - needs `v4` feature for `Uuid::new_v4()`

## 📚 How to Find Features

1. Visit [docs.rs](https://docs.rs) for each crate
2. Look for **"Feature flags"** or **"Cargo features"** section
3. Check the **Cargo.toml** examples in documentation

## ⏰ Time Check

Spent 15 minutes? If you're still stuck, move to Level 2 hints.

**Hint**: Each compilation error tells you exactly what feature is missing!