# Publishing Guide: From Development to crates.io

## 🎯 Overview

Publishing a Rust crate is like publishing a NuGet package, but with some key differences. This guide walks you through the entire process.

## 🔄 NuGet vs crates.io Comparison

| Aspect | NuGet / .NET | crates.io / Rust | Key Difference |
|--------|--------------|------------------|----------------|
| **Registry** | nuget.org | crates.io | Single official registry |
| **Versioning** | Assembly versioning | Semantic versioning | Stricter semver enforcement |
| **Metadata** | .nuspec file | Cargo.toml | Integrated with build |
| **Documentation** | Separate hosting | docs.rs automatic | Auto-generated docs |
| **Publishing** | nuget push | cargo publish | Immutable once published |
| **Dependencies** | packages.config | Cargo.lock | More precise resolution |

## 📋 Pre-Publication Checklist

### 1. Complete Cargo.toml Metadata

```toml
[package]
name = "my-awesome-crate"           # Must be unique on crates.io
version = "0.1.0"                   # Follow semantic versioning
edition = "2021"                    # Rust edition
authors = ["Your Name <you@example.com>"]
description = "A brief, clear description of what this crate does"
license = "MIT OR Apache-2.0"      # Standard Rust licensing
repository = "https://github.com/yourusername/my-awesome-crate"
homepage = "https://github.com/yourusername/my-awesome-crate"
documentation = "https://docs.rs/my-awesome-crate"
readme = "README.md"
keywords = ["cli", "tool", "utility"]  # Max 5 keywords
categories = ["command-line-utilities"] # From crates.io categories

# Optional but recommended
include = [
    "src/**/*",
    "Cargo.toml",
    "README.md",
    "LICENSE*",
    "CHANGELOG.md",
]
exclude = [
    "target/",
    ".git/",
    "*.tmp",
]

[lib]
name = "my_awesome_crate"  # Snake case for library name
path = "src/lib.rs"

[[bin]]
name = "my-tool"           # Binary name (if applicable)
path = "src/main.rs"
```

**C# Comparison:**
```xml
<!-- C# .nuspec equivalent -->
<package>
  <metadata>
    <id>MyAwesomeCrate</id>
    <version>0.1.0</version>
    <authors>Your Name</authors>
    <description>A brief, clear description</description>
    <license type="expression">MIT</license>
    <projectUrl>https://github.com/yourusername/my-awesome-crate</projectUrl>
    <repository type="git" url="https://github.com/yourusername/my-awesome-crate" />
    <tags>cli tool utility</tags>
  </metadata>
</package>
```

### 2. Required Files

**README.md** - Your crate's front page:
```markdown
# My Awesome Crate

Brief description of what it does.

## Installation

```toml
[dependencies]
my-awesome-crate = "0.1"
```

## Quick Start

```rust
use my_awesome_crate::*;

fn main() {
    // Show a simple example
}
```

## Features

- Feature 1
- Feature 2
- Feature 3

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
```

**LICENSE files** - Standard Rust dual licensing:
- `LICENSE-APACHE` (Apache License 2.0)
- `LICENSE-MIT` (MIT License)

**CHANGELOG.md** - Version history:
```markdown
# Changelog

## [0.1.0] - 2024-01-01

### Added
- Initial release
- Basic functionality
- Documentation

### Changed
- Nothing yet

### Fixed
- Nothing yet
```

### 3. Documentation Requirements

**Module-level documentation:**
```rust
//! # My Awesome Crate
//!
//! This crate provides utilities for doing awesome things.
//!
//! ## Quick Start
//!
//! ```
//! use my_awesome_crate::do_something;
//!
//! let result = do_something("input");
//! assert_eq!(result, "expected output");
//! ```

/// Does something awesome with the input.
///
/// # Examples
///
/// ```
/// use my_awesome_crate::do_something;
///
/// let result = do_something("hello");
/// assert_eq!(result, "Hello, World!");
/// ```
///
/// # Errors
///
/// This function will return an error if the input is empty.
pub fn do_something(input: &str) -> Result<String, MyError> {
    // Implementation
}
```

**Test your documentation:**
```bash
cargo test --doc  # Test all doc examples
cargo doc --open  # Generate and view docs locally
```

## 🔧 Development Workflow

### 1. Version Management

**Semantic Versioning Rules:**
- `0.x.y` - Pre-1.0, breaking changes allowed in minor versions
- `x.y.z` - Post-1.0, breaking changes require major version bump
- `x.y.z-alpha.1` - Pre-release versions

```bash
# Bump version (use cargo-edit)
cargo install cargo-edit
cargo set-version --bump patch    # 0.1.0 -> 0.1.1
cargo set-version --bump minor    # 0.1.1 -> 0.2.0
cargo set-version --bump major    # 0.2.0 -> 1.0.0
```

### 2. Testing and Quality

```bash
# Complete test suite
cargo test                        # Unit and integration tests
cargo test --doc                  # Documentation tests
cargo bench                       # Benchmarks

# Code quality
cargo fmt                         # Format code
cargo clippy -- -D warnings       # Lint with warnings as errors
cargo audit                       # Security audit

# Build verification
cargo build --release             # Release build
cargo package                     # Test packaging
cargo publish --dry-run           # Simulate publishing
```

### 3. Feature Management

```toml
[features]
default = ["std"]
std = []                          # Standard library support
serde = ["dep:serde"]             # Optional serde support
async = ["dep:tokio"]             # Optional async support

[dependencies]
serde = { version = "1.0", optional = true }
tokio = { version = "1.0", optional = true }
```

**Test all feature combinations:**
```bash
# Test with different feature combinations
cargo test --no-default-features  # Minimal features
cargo test --all-features         # All features
cargo test --features "serde"     # Specific features
```

## 🚀 Publishing Process

### 1. Authentication

```bash
# Get API token from crates.io
# Go to https://crates.io/me and create a token

# Configure cargo
cargo login <your-api-token>
```

### 2. Pre-Publication Verification

```bash
# Final checks before publishing
cargo package                     # Create package
cargo publish --dry-run           # Simulate publishing

# Check the generated package
cd target/package/my-awesome-crate-0.1.0
cargo test                        # Test the packaged version
```

### 3. Actual Publishing

```bash
# Publish to crates.io (irreversible!)
cargo publish

# The crate is now available at:
# https://crates.io/crates/my-awesome-crate
# https://docs.rs/my-awesome-crate
```

## 📊 Post-Publication

### 1. Monitoring

**Check your crate:**
- **Downloads** - Track adoption on crates.io
- **Documentation** - Verify docs.rs built correctly
- **Dependencies** - Monitor reverse dependencies
- **Issues** - Respond to bug reports and feature requests

### 2. Maintenance

**Regular updates:**
```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Security audits
cargo audit
```

**Version releases:**
```bash
# Create git tag for release
git tag v0.1.0
git push origin v0.1.0

# Update CHANGELOG.md
# Bump version in Cargo.toml
# Publish new version
cargo publish
```

## 🚨 Common Pitfalls

### 1. Naming Conflicts

**Problem:**
```bash
$ cargo publish
error: crate name `http` is already taken
```

**Solution:**
- Choose a unique name
- Check availability on crates.io first
- Consider namespacing: `mycompany-toolname`

### 2. Missing Documentation

**Problem:**
```bash
warning: missing documentation for a public function
```

**Solution:**
```rust
// Add documentation to all public items
#![warn(missing_docs)]

/// This function is now documented.
pub fn my_function() {}
```

### 3. Large Package Size

**Problem:**
```bash
warning: package size (15.2 MB) is too large
```

**Solution:**
```toml
# Exclude unnecessary files
[package]
exclude = [
    "target/",
    ".git/",
    "*.tmp",
    "test_data/",
    "examples/large_files/",
]
```

### 4. Broken Documentation Links

**Problem:**
```bash
warning: broken doc link
```

**Solution:**
```rust
// Use proper doc links
/// See [`other_function`] for details.
/// 
/// [`other_function`]: crate::other_function
pub fn my_function() {}
```

## 🎯 Best Practices

### 1. API Stability

**Pre-1.0 (breaking changes allowed):**
```rust
// Feel free to break APIs before 1.0
pub fn old_api(x: i32) -> String { ... }

// In next version, you can change this
pub fn old_api(x: i32, y: bool) -> Result<String, Error> { ... }
```

**Post-1.0 (semver compatibility required):**
```rust
// Must maintain backward compatibility
pub fn stable_api(x: i32) -> String { ... }

// To add parameters, create a new function
pub fn stable_api_v2(x: i32, y: bool) -> Result<String, Error> { ... }

// Or use a config struct
pub fn stable_api_with_config(config: ApiConfig) -> Result<String, Error> { ... }
```

### 2. Feature Design

```toml
# Good: Optional dependencies
[dependencies]
serde = { version = "1.0", optional = true }
tokio = { version = "1.0", optional = true }

[features]
default = []
serde = ["dep:serde"]
async = ["dep:tokio"]
full = ["serde", "async"]
```

### 3. Documentation Examples

```rust
/// Parse a configuration file.
///
/// # Examples
///
/// ```
/// use my_crate::parse_config;
///
/// let config = parse_config("database_url = \"postgres://localhost\"")?;
/// assert_eq!(config.database_url, "postgres://localhost");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// Returns an error if the configuration is malformed.
pub fn parse_config(input: &str) -> Result<Config, ParseError> {
    // Implementation
}
```

## 🔄 Migration from NuGet

| NuGet Concept | Rust Equivalent | Migration Notes |
|---------------|-----------------|-----------------|
| Package ID | Crate name | Must be globally unique |
| Target framework | Rust edition | Less frequent changes |
| Dependencies | Dependencies | More precise versioning |
| Package restore | Cargo build | Automatic, no restore step |
| Package cache | Cargo cache | Global, shared cache |
| Local packages | Path dependencies | For development |
| Pre-release | Pre-release versions | Alpha, beta, rc suffixes |

## 🏆 Success Metrics

**Healthy crate indicators:**
- ✅ Regular downloads (>100/month for niche, >1000/month for popular)
- ✅ Recent updates (within 6 months)
- ✅ Good documentation (examples, clear API)
- ✅ Responsive maintainer (issues addressed)
- ✅ Stable API (semantic versioning followed)
- ✅ Active community (contributions, discussions)

**Growing your crate:**
1. **Write excellent documentation** with real examples
2. **Respond quickly** to issues and PRs
3. **Follow semver** religiously for trust
4. **Publish regularly** with useful features
5. **Engage with community** in forums and Discord
6. **Write blog posts** about your crate
7. **Present at meetups** or conferences

---

**Next:** Compare with the [.NET Ecosystem](dotnet-ecosystem.md) to understand the differences!