# Exercise 02 - Level 2 Hints: Feature Flag Fiasco

## 🎯 Specific Feature Requirements

Here's what each crate needs:

## 🔧 Required Features

1. **serde**: `derive` feature for `#[derive(Serialize, Deserialize)]`
2. **reqwest**: `json` feature for `.json()` method
3. **chrono**: `serde` feature to serialize `DateTime<Utc>`
4. **uuid**: `v4` and `serde` features for `Uuid::new_v4()` and serialization

## 📋 Cargo.toml Pattern

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

## 🤔 C# Comparison

This is like:
- **NuGet sub-packages**: `Microsoft.AspNetCore.Mvc` vs `Microsoft.AspNetCore.Mvc.Core`
- **Conditional compilation**: `#if FEATURE_X` but managed by the build system
- **Assembly references**: Only including what you need

## 🔍 Debug Strategy

1. **Read each compilation error** carefully
2. **Match the error** to the missing feature:
   - `derive` macro not found → `serde` needs `derive` feature
   - `json` method not found → `reqwest` needs `json` feature
   - Serialization errors → crates need `serde` feature

## ⏰ Time Check

Still struggling after 30 minutes? Move to Level 3 for the complete solution.

**Hint**: Some crates need multiple features - read the docs for the complete list!