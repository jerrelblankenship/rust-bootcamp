# Exercise 02 - Level 3 Hints: Feature Flag Fiasco

## 🎯 Complete Solution

Here's the working `Cargo.toml` with all required features:

```toml
[package]
name = "ex02-feature-flags"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## 🔧 Why Each Feature is Needed

1. **serde = { features = ["derive"] }**
   - Enables `#[derive(Serialize, Deserialize)]` macros
   - Without this: "cannot find derive macro `Serialize`"

2. **reqwest = { features = ["json"] }**
   - Enables `.json()` method for HTTP requests/responses
   - Without this: "no method named `json` found"

3. **chrono = { features = ["serde"] }**
   - Enables serialization for `DateTime<Utc>`
   - Without this: "DateTime<Utc> doesn't implement Serialize"

4. **uuid = { features = ["v4", "serde"] }**
   - `v4`: Enables `Uuid::new_v4()` for random UUIDs
   - `serde`: Enables serialization for UUID
   - Without these: "no associated function `new_v4` found"

## 📋 Complete Fix Process

1. **Update Cargo.toml** with the features above
2. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

## 🎓 What You Learned

- **Feature flags** control what code gets compiled
- **Compilation errors** tell you exactly what features are missing
- **Multiple features** can be specified in arrays: `["feature1", "feature2"]`
- **Feature flags** reduce binary size and compile time

## 🤔 C# Comparison

This is like:
- **Conditional compilation** with `#if` directives
- **NuGet package variants** (Core vs Full Framework)
- **Assembly references** - only including what you need
- **Preprocessor directives** but managed by the build system

## 🏆 Success Criteria

Your program should compile and run, creating a user with UUID, serializing to JSON, and making HTTP requests successfully!