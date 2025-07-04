# Exercise 03 - Level 2 Hints: Crate Selection Confusion

## 🎯 Better Crate Alternatives

Here are the modern replacements for the outdated crates:

## 🔧 Recommended Replacements

1. **JSON**: `serde + serde_json` instead of `rustc_serialize`
2. **HTTP Client**: `reqwest` instead of raw `hyper`
3. **Error Handling**: `anyhow` or `thiserror` instead of `failure`

## 📋 Why These Are Better

1. **serde + serde_json**:
   - More performant
   - Better error messages
   - Actively maintained
   - Derive macros for easy serialization

2. **reqwest**:
   - High-level, ergonomic API
   - Built on top of hyper
   - Async/await support
   - JSON support built-in

3. **anyhow**:
   - Modern error handling
   - Great for applications (not libraries)
   - Easy error propagation
   - Good error messages

## 🤔 C# Comparison

This is like:
- **Newtonsoft.Json** → **System.Text.Json** (newer, faster)
- **HttpClient** → **HttpClient** (but using modern patterns)
- **Exception** → **Result<T, E>** (more explicit error handling)

## 🔍 Code Structure

Your new code should be much simpler:
```rust
// Simple HTTP request with reqwest
let response = reqwest::get(url).await?.json::<User>().await?;

// Easy JSON parsing with serde
#[derive(Deserialize)]
struct User { ... }
```

## ⏰ Time Check

Still struggling after 30 minutes? Move to Level 3 for the complete rewrite.

**Hint**: The new version should be about 1/3 the length of the original!