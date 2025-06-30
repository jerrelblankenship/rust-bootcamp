# Ecosystem Debugging Checklist 📦

When working with Rust crates and the ecosystem, watch for these issues:

## 1. Dependency Management

### ❌ Version Conflicts
```toml
# Cargo.toml
[dependencies]
serde = "1.0"
some_crate = "2.0" # Uses serde 0.9 - conflict!
```
**Fix:** Use `cargo tree` to diagnose, upgrade or downgrade versions

### ❌ Feature Flag Issues
```toml
[dependencies]
tokio = "1.0" # Missing required features!
```
**Fix:** Enable needed features: `tokio = { version = "1.0", features = ["full"] }`

## 2. Crate Selection Problems

### ❌ Using Deprecated Crates
```toml
[dependencies]
time = "0.1" # Deprecated, use chrono or time 0.3+
error-chain = "0.12" # Use anyhow or thiserror instead
```
**Fix:** Check crates.io for maintenance status and alternatives

### ❌ Over-Dependencies
```toml
[dependencies]
reqwest = "0.11"     # HTTP client
hyper = "0.14"       # Already included in reqwest!
url = "2.2"          # Already included in reqwest!
```
**Fix:** Let high-level crates pull in their own dependencies

## 3. Feature Management

### ❌ All Features Enabled
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] } # Bloated!
```
**Fix:** Only enable needed features for faster compile times

### ❌ Missing Conditional Compilation
```rust
use std::os::unix::fs::PermissionsExt; // Breaks on Windows!
```
**Fix:** Use `#[cfg(unix)]` guards

## 4. Documentation Issues

### ❌ No Examples in Docs
```rust
/// This function does something important
pub fn important_function() { /* ... */ }
```
**Fix:** Add examples with `/// # Examples`

### ❌ Missing Feature Documentation
```rust
#[cfg(feature = "advanced")]
pub fn advanced_feature() { /* undocumented! */ }
```
**Fix:** Document feature gates and when to use them

## 5. Publishing Problems

### ❌ Missing Metadata
```toml
[package]
name = "my-crate"
version = "0.1.0"
# Missing description, license, repository!
```
**Fix:** Add complete metadata for crates.io

### ❌ Broken Examples
```rust
// examples/demo.rs
use my_crate::missing_function; // Doesn't exist!
```
**Fix:** Test examples with `cargo test --examples`

## 6. API Design Issues

### ❌ Inconsistent Naming
```rust
pub fn getData() -> String { } // Not snake_case!
pub fn set_data(data: String) { } // Inconsistent with above
```
**Fix:** Follow Rust naming conventions

### ❌ Poor Error Types
```rust
pub fn parse_config() -> Result<Config, String> { // String error!
    // ...
}
```
**Fix:** Use proper error types with context

## C# to Rust Ecosystem Patterns

| C# Concept | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| NuGet packages | Crates.io crates | More granular |
| project.json/packages.config | Cargo.toml | Built-in tool |
| Assembly binding | Cargo lock file | Automatic |
| .NET Framework/.NET Core | Edition (2018/2021) | Language evolution |
| Strong naming | Semantic versioning | Community standard |
| GAC | Registry | No global cache |

## Quick Ecosystem Navigation

1. **Find Crates**: Use crates.io, lib.rs, or awesome-rust
2. **Check Quality**: Downloads, recent updates, GitHub stars
3. **Read Docs**: docs.rs has comprehensive documentation
4. **Check Dependencies**: Use `cargo tree` to understand deps
5. **Security**: Use `cargo audit` to check vulnerabilities

## Publishing Checklist

- [ ] Complete Cargo.toml metadata
- [ ] Comprehensive documentation with examples
- [ ] All examples and tests pass
- [ ] Semantic version bump appropriate
- [ ] CHANGELOG.md updated
- [ ] Security audit clean
- [ ] Cross-platform testing done

## Pro Tips

- Use `cargo expand` to see macro expansions
- Enable clippy for ecosystem best practices: `cargo clippy`
- Use `cargo outdated` to check for newer versions
- Consider `cargo deny` for license and security policies
- Browse source on docs.rs to understand implementations
- Join the Rust community Discord for ecosystem questions!