# Ecosystem Debugging Checklist 📦

When working with Rust crates and the ecosystem, use this systematic approach to diagnose and fix issues.

## 🚨 Emergency Quick Fixes

### Common Compilation Errors

**Error:** `no method named 'json' found`
```bash
# Fix: Add json feature to reqwest
cargo add reqwest --features json
```

**Error:** `cannot find derive macro 'Serialize'`
```bash
# Fix: Add derive feature to serde
cargo add serde --features derive
```

**Error:** `could not find 'new_v4' in 'Uuid'`
```bash
# Fix: Add v4 feature to uuid
cargo add uuid --features v4
```

## 1. Dependency Management

### ❌ Version Conflicts
```toml
# Cargo.toml - BROKEN
[dependencies]
serde = "1.0"
some_crate = "2.0" # Uses serde 0.9 - conflict!
```

**Diagnosis:**
```bash
cargo tree              # See dependency tree
cargo tree -d           # Show duplicates only
cargo tree -i serde     # Show what depends on serde
```

**Fix:** Use compatible versions
```toml
[dependencies]
serde = "1.0"
some_crate = "3.0"  # Updated to use serde 1.0
```

### ❌ Feature Flag Issues
```toml
# BROKEN - Missing features
[dependencies]
tokio = "1.0"        # Missing runtime features
reqwest = "0.11"     # Missing json feature  
serde = "1.0"        # Missing derive feature
```

**Fix:** Enable required features
```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
```

## 2. Crate Selection Problems

### ❌ Using Deprecated/Unmaintained Crates
```toml
# BROKEN - Old/deprecated crates
[dependencies]
rustc-serialize = "0.3"  # Use serde instead
time = "0.1"             # Use chrono or time 0.3+
error-chain = "0.12"     # Use anyhow or thiserror
failure = "0.1"          # Use anyhow or thiserror
```

**Diagnosis:**
- Check last commit date on GitHub
- Look for deprecation warnings on crates.io
- Check download trends

**Fix:** Use modern alternatives
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"  # For applications
thiserror = "1.0"  # For libraries
```

### ❌ Duplicate Dependencies
```toml
# BROKEN - Redundant dependencies
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
hyper = "0.14"       # Already included in reqwest!
url = "2.2"          # Already included in reqwest!
tokio = "1.0"        # Already included in reqwest!
```

**Diagnosis:**
```bash
cargo tree --format "{p} {f}"  # Show what's included
```

**Fix:** Remove redundant dependencies
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
# Remove hyper, url, tokio - reqwest includes them
```

## 3. Feature Management

### ❌ Over-Enabled Features (Slow Compiles)
```toml
# BROKEN - Too many features
[dependencies]
tokio = { version = "1.0", features = ["full"] }  # Includes everything!
serde = { version = "1.0", features = ["derive", "rc", "alloc"] }  # Too much!
```

**Diagnosis:**
```bash
cargo build --timings  # See what takes time to compile
cargo bloat            # Check binary size
```

**Fix:** Minimal features only
```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
serde = { version = "1.0", features = ["derive"] }
```

### ❌ Platform-Specific Code Without Guards
```rust
// BROKEN - Will fail on Windows
use std::os::unix::fs::PermissionsExt;
use nix::unistd::getpid;

fn main() {
    let pid = getpid();  // Unix only!
    println!("PID: {}", pid);
}
```

**Fix:** Use conditional compilation
```rust
// FIXED - Platform-specific guards
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
use nix::unistd::getpid;

fn main() {
    #[cfg(unix)]
    {
        let pid = getpid();
        println!("PID: {}", pid);
    }
    
    #[cfg(windows)]
    {
        println!("Windows doesn't expose PID this way");
    }
}
```

## 4. Documentation Issues

### ❌ Broken Doctests
```rust
// BROKEN - This doctest will fail!
/// Parses a user from JSON
/// 
/// # Examples
/// 
/// ```
/// use mycrate::parse_user;  // Wrong import path!
/// 
/// let user = parse_user('{"name": "Alice"}')?;  // Wrong JSON format!
/// assert_eq!(user.name, "Bob");  // Wrong expected value!
/// ```
pub fn parse_user(json: &str) -> Result<User, ParseError> {
    // Implementation
}
```

**Diagnosis:**
```bash
cargo test --doc  # Test all documentation examples
```

**Fix:** Correct and test examples
```rust
/// Parses a user from JSON
/// 
/// # Examples
/// 
/// ```
/// use mycrate::{parse_user, User};
/// 
/// let json = r#"{"name": "Alice", "age": 30}"#;
/// let user = parse_user(json).unwrap();
/// assert_eq!(user.name, "Alice");
/// ```
pub fn parse_user(json: &str) -> Result<User, ParseError> {
    // Implementation
}
```

### ❌ Missing Feature Gate Documentation
```rust
// BROKEN - No docs about when to use this
#[cfg(feature = "advanced")]
pub fn advanced_feature() { /* undocumented! */ }
```

**Fix:** Document feature requirements
```rust
/// Advanced processing function.
/// 
/// This function is only available when the "advanced" feature is enabled.
/// 
/// # Examples
/// 
/// ```toml
/// [dependencies]
/// mycrate = { version = "1.0", features = ["advanced"] }
/// ```
/// 
/// ```
/// # #[cfg(feature = "advanced")]
/// use mycrate::advanced_feature;
/// 
/// # #[cfg(feature = "advanced")]
/// let result = advanced_feature(data);
/// ```
#[cfg(feature = "advanced")]
pub fn advanced_feature() {
    // Implementation
}
```

## 5. Publishing Problems

### ❌ Incomplete Publication Metadata
```toml
# BROKEN - Will fail cargo publish
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
# Missing: description, license, repository, keywords!
```

**Diagnosis:**
```bash
cargo publish --dry-run  # Check what's missing
```

**Fix:** Complete metadata
```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A crate that does awesome things"
license = "MIT OR Apache-2.0"
repository = "https://github.com/you/my-awesome-crate"
homepage = "https://github.com/you/my-awesome-crate"
documentation = "https://docs.rs/my-awesome-crate"
readme = "README.md"
keywords = ["awesome", "utility", "cli"]
categories = ["command-line-utilities"]
```

### ❌ Broken Examples and Tests
```rust
// examples/demo.rs - BROKEN
use my_crate::missing_function;  // Function doesn't exist!
use non_existent_crate::*;       // Crate not in dependencies!

fn main() {
    let result = missing_function();  // Will fail to compile
    println!("{:?}", result);
}
```

**Diagnosis:**
```bash
cargo test --examples    # Test examples compile and run
cargo test --doc         # Test documentation examples
cargo package           # Test packaging includes right files
```

**Fix:** Working examples
```rust
// examples/demo.rs - FIXED
use my_crate::{working_function, User};

fn main() -> anyhow::Result<()> {
    let user = User::new("Alice", 30);
    let result = working_function(&user)?;
    println!("Result: {:?}", result);
    Ok(())
}
```

## 6. API Design Issues

### ❌ Non-Idiomatic Naming
```rust
// BROKEN - C# style naming
pub fn getData() -> String { }           // Should be snake_case
pub fn SetData(data: String) { }         // Should be snake_case
pub fn IsValid(&self) -> bool { }        // Should be snake_case
pub struct userInfo { }                  // Should be PascalCase
```

**Diagnosis:**
```bash
cargo clippy  # Will warn about naming issues
```

**Fix:** Follow Rust conventions
```rust
// FIXED - Rust style naming
pub fn get_data() -> String { }          // snake_case for functions
pub fn set_data(data: String) { }        // snake_case for functions
pub fn is_valid(&self) -> bool { }       // snake_case for methods
pub struct UserInfo { }                  // PascalCase for types
```

### ❌ Poor Error Handling
```rust
// BROKEN - String errors are useless
pub fn parse_config(path: &str) -> Result<Config, String> {
    std::fs::read_to_string(path)
        .map_err(|e| "Failed to read file")?;  // Lost context!
    // ..
}

// BROKEN - Panics instead of errors
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Division by zero!");  // Should return Result!
    }
    a / b
}
```

**Fix:** Proper error types with context
```rust
// FIXED - Custom error types
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Invalid config format: {0}")]
    ParseError(String),
}

pub fn parse_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // Automatic conversion
    // Parse content..
    Ok(config)
}

// FIXED - Return Result instead of panicking
pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}
```

## 🔄 C# to Rust Ecosystem Translation

### Package Management
| C# Concept | Rust Equivalent | Key Differences |
|------------|-----------------|------------------|
| `Install-Package` | `cargo add` | Integrated with build |
| `packages.config` | `Cargo.toml` | Single source of truth |
| `packages.lock.json` | `Cargo.lock` | Always generated |
| PackageReference | Dependencies section | More explicit |
| Conditional packages | Feature flags | More granular |
| Private feeds | Git dependencies | No private registry needed |

### Development Tools
| C# Tool | Rust Equivalent | Usage |
|---------|-----------------|-------|
| `dotnet build` | `cargo build` | Compile project |
| `dotnet run` | `cargo run` | Run project |
| `dotnet test` | `cargo test` | Run tests |
| `dotnet pack` | `cargo package` | Create package |
| `dotnet publish` | `cargo build --release` | Release build |
| Roslyn analyzers | `cargo clippy` | Code linting |
| EditorConfig | `cargo fmt` | Code formatting |

### Error Handling Patterns
| C# Pattern | Rust Pattern | Example |
|------------|--------------|---------|
| `try-catch` | `match` or `?` | Explicit error handling |
| `ArgumentException` | `Result<T, E>` | Type-safe errors |
| `null` checking | `Option<T>` | No null pointer exceptions |
| Custom exceptions | `thiserror` | Structured error types |

## 🔍 Systematic Debugging Workflow

### Step 1: Identify the Issue Type
```bash
# Try to build and categorize the error
cargo build 2>&1 | head -20

# Common error patterns:
# "no method named 'X'" → Missing feature flag
# "cannot find derive macro" → Missing derive feature
# "use of unstable library feature" → Wrong edition/toolchain
# "could not compile" → Dependency issue
```

### Step 2: Dependency Diagnosis
```bash
cargo tree                    # Full dependency tree
cargo tree -d                 # Show duplicates
cargo tree -i crate_name      # What depends on this crate
cargo update --dry-run        # What would be updated
cargo outdated               # Check for newer versions
```

### Step 3: Feature Investigation
```bash
# Check what features are available
cargo metadata --format-version 1 | jq '.packages[] | select(.name=="crate_name") | .features'

# Try with all features
cargo build --all-features

# Try with no features
cargo build --no-default-features
```

### Step 4: Ecosystem Quality Check
```bash
cargo audit                   # Security vulnerabilities
cargo clippy                  # Best practices
cargo fmt --check             # Code formatting
cargo test --doc             # Documentation tests
cargo package --allow-dirty   # Package validation
```

## 🚀 Quick Fixes Reference

### Most Common Issues
```bash
# HTTP client without JSON support
cargo add reqwest --features json

# Serde without derive macros
cargo add serde --features derive

# Tokio without runtime
cargo add tokio --features macros,rt-multi-thread

# UUID without v4 generation
cargo add uuid --features v4,serde

# Chrono without serde support
cargo add chrono --features serde

# CLI parsing without derive
cargo add clap --features derive
```

### Development Dependencies
```bash
# Add development-only dependencies
cargo add --dev criterion     # Benchmarking
cargo add --dev mockall       # Mocking
cargo add --dev proptest      # Property testing
cargo add --dev tempfile      # Temporary files for tests
```

## 📋 Pre-Publication Checklist

### Code Quality
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes all tests
- [ ] `cargo test --doc` passes all doc tests
- [ ] `cargo clippy` has no warnings
- [ ] `cargo fmt --check` passes
- [ ] `cargo audit` shows no vulnerabilities

### Documentation
- [ ] All public items have documentation
- [ ] Documentation includes working examples
- [ ] README.md is comprehensive
- [ ] CHANGELOG.md is updated
- [ ] examples/ directory has working examples

### Metadata
- [ ] Cargo.toml has complete metadata
- [ ] License files are included
- [ ] Keywords and categories are set
- [ ] Repository URL is correct

### Testing
- [ ] Tests pass on multiple platforms
- [ ] Feature flag combinations work
- [ ] Examples compile and run
- [ ] Benchmarks work (if applicable)

### Publication
- [ ] `cargo package` succeeds
- [ ] `cargo publish --dry-run` succeeds
- [ ] Version number follows semver
- [ ] Git tag created for release

## 🎯 Advanced Debugging Tools

```bash
# Compilation performance
cargo build --timings         # See what takes time
cargo +nightly build -Z timings  # More detailed timing

# Binary analysis
cargo bloat                   # What makes binary large
cargo bloat --crates          # Per-crate size breakdown

# Dependency analysis
cargo machete                 # Find unused dependencies
cargo udeps                   # Find unused dependencies (nightly)

# License and security
cargo deny check              # Comprehensive policy checking
cargo geiger                  # Unsafe code detection

# Documentation
cargo doc --open              # Generate and open docs
cargo deadlinks               # Check for broken doc links
```

## 🆘 Getting Help

1. **Read the error message carefully** - Rust errors are usually helpful
2. **Check the crate's documentation** on docs.rs
3. **Search issues** on the crate's GitHub repository
4. **Ask on the Rust forum** at users.rust-lang.org
5. **Join Rust Discord** for real-time help
6. **Use the playground** at play.rust-lang.org for minimal examples