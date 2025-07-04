# Exercise 01 - Level 3 Hints: Dependency Hell

## 🎯 Complete Solution

Here's the working `Cargo.toml` that resolves all version conflicts:

```toml
[package]
name = "ex01-dependency-hell"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## 🔧 Why This Works

1. **reqwest 0.11** is compatible with **tokio 1.0**
2. **Feature flags** enable the needed functionality:
   - `reqwest` needs `"json"` feature for `.json()` method
   - `tokio` needs `"macros"` for `#[tokio::main]` and `"rt-multi-thread"` for async runtime

3. **serde_json 1.0** works with both

## 📋 Complete Fix Process

1. **Create new Cargo project**:
   ```bash
   cargo new ex01-dependency-hell --bin
   cd ex01-dependency-hell
   ```

2. **Copy the code** into `src/main.rs` (remove the commented Cargo.toml)

3. **Use the working Cargo.toml** above

4. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

## 🎓 What You Learned

- **Version compatibility** is crucial in Rust ecosystems
- **Feature flags** control what functionality is compiled
- **Cargo.toml** is more explicit than NuGet's packages.config
- **cargo tree** helps debug dependency conflicts

## 🤔 C# Comparison

This is like resolving NuGet package conflicts in .NET, but:
- **More explicit**: Cargo forces you to specify compatible versions
- **Better error messages**: Rust tells you exactly what's wrong
- **Feature flags**: Like conditional compilation, but more granular
- **No runtime conflicts**: Issues are caught at compile time

## 🏆 Success Criteria

Your program should compile and run, fetching data from GitHub's API and parsing JSON successfully!