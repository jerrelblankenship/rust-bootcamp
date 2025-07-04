# Crate Selection Guide

## 🎯 Overview

Choosing the right crates is like selecting NuGet packages in .NET, but with some important differences. This guide helps you make informed decisions.

## 🤔 C# vs Rust Package Selection

| Aspect | C# / NuGet | Rust / Cargo | Key Difference |
|--------|------------|--------------|----------------|
| **Granularity** | Large packages | Focused crates | More specific purpose |
| **Dependencies** | Often heavy | Minimal by default | Explicit feature flags |
| **Versioning** | Assembly versioning | Semantic versioning | More predictable |
| **Compilation** | Runtime linking | Compile-time inclusion | No runtime dependencies |
| **Features** | Conditional compilation | Feature flags | More granular |

## 🔍 How to Evaluate Crates

### 1. Popularity Metrics

```bash
# Search for HTTP client crates
cargo search http client
```

**Look for:**
- **Download count** - More downloads = more tested
- **Recent versions** - Active development
- **GitHub stars** - Community adoption

**C# Analogy:** Like checking NuGet download statistics and package popularity.

### 2. Maintenance Status

**Green Flags:**
- Recent commits (within 6 months)
- Regular releases
- Responsive issue handling
- Clear changelogs

**Red Flags:**
- No commits for 1+ years
- Many open critical issues
- Deprecated notices
- No documentation updates

### 3. Documentation Quality

**Excellent Documentation:**
- Working examples in README
- Comprehensive API docs
- Doctests that compile
- Migration guides

**C# Comparison:**
```csharp
// C# - XML documentation
/// <summary>
/// Gets the user by ID
/// </summary>
/// <param name="id">User identifier</param>
/// <returns>User object or null</returns>
public User GetUser(int id) { ... }
```

```rust
// Rust - executable documentation
/// Gets the user by ID
/// 
/// # Examples
/// 
/// ```
/// use mylib::get_user;
/// 
/// let user = get_user(123).await?;
/// assert_eq!(user.name, "Alice");
/// ```
pub async fn get_user(id: u32) -> Result<User> { ... }
```

## 📋 Category-Specific Recommendations

### HTTP Clients

| Crate | Use Case | C# Equivalent |
|-------|----------|---------------|
| `reqwest` | High-level HTTP client | `HttpClient` |
| `hyper` | Low-level HTTP | `HttpListener` |
| `ureq` | Synchronous, lightweight | `WebClient` |

**Example Decision Matrix:**
```rust
// For most applications - use reqwest
reqwest = { version = "0.11", features = ["json"] }

// For low-level control - use hyper
hyper = { version = "0.14", features = ["full"] }

// For simple sync operations - use ureq
ureq = { version = "2.0", features = ["json"] }
```

### JSON Handling

| Crate | Use Case | C# Equivalent |
|-------|----------|---------------|
| `serde_json` | Standard JSON (with serde) | `System.Text.Json` |
| `json` | Simple JSON parsing | `JObject` |
| `simd-json` | High-performance parsing | High-perf JSON libs |

**Recommendation:** Always use `serde_json` with `serde` derive macros.

### Database Access

| Crate | Use Case | C# Equivalent |
|-------|----------|---------------|
| `sqlx` | Async SQL with compile-time checks | Entity Framework Core |
| `diesel` | Sync ORM with strong typing | Entity Framework |
| `rusqlite` | SQLite only | `System.Data.SQLite` |
| `mongodb` | MongoDB driver | MongoDB .NET Driver |

### Error Handling

| Crate | Use Case | C# Equivalent |
|-------|----------|---------------|
| `anyhow` | Application error handling | `Exception` + context |
| `thiserror` | Library error types | Custom exceptions |
| `eyre` | Better error reporting | Exception with stack trace |

### CLI Development

| Crate | Use Case | C# Equivalent |
|-------|----------|---------------|
| `clap` | Full-featured CLI | `System.CommandLine` |
| `structopt` | Derive-based CLI | Attribute-based parsing |
| `argh` | Lightweight alternative | Simple arg parsing |

## 🚨 Common Pitfalls

### 1. Feature Flag Confusion

**Wrong:**
```toml
serde = "1.0"  # Missing derive feature
reqwest = "0.11"  # Missing json feature
```

**Right:**
```toml
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
```

### 2. Version Conflicts

**Problem:**
```toml
# These might have conflicting dependencies
old-crate = "0.1"  # Uses old version of shared dependency
new-crate = "2.0"  # Uses new version of shared dependency
```

**Solution:** Use `cargo tree` to identify conflicts and choose compatible versions.

### 3. Overengineering

**C# Developer Trap:**
```toml
# Don't add everything at once!
serde = { version = "1.0", features = ["derive", "rc"] }
reqwest = { version = "0.11", features = ["json", "stream", "multipart", "cookies", "gzip"] }
tokio = { version = "1.0", features = ["full"] }
```

**Better Approach:**
```toml
# Start minimal, add features as needed
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## 🎯 Decision Framework

### 1. Define Requirements
- **Performance needs** - High throughput? Low latency?
- **Feature requirements** - What functionality do you need?
- **Platform targets** - Cross-platform? Specific OS?
- **Async/sync** - Does it fit your architecture?

### 2. Research Options
```bash
# Search for crates
cargo search "keyword"

# Check details
cargo info crate-name

# View dependencies
cargo tree
```

### 3. Prototype and Test
```rust
// Create a small test project
cargo new --bin test-crate
cd test-crate

// Add the crate
cargo add potential-crate

// Write a small test
// src/main.rs
use potential_crate::*;

fn main() {
    // Test basic functionality
}
```

### 4. Evaluate Integration
- **Compilation time** - Does it slow down builds?
- **Binary size** - Does it bloat your executable?
- **API ergonomics** - Is it pleasant to use?
- **Error messages** - Are errors helpful?

## 📊 Real-World Examples

### Web API Server

```toml
[dependencies]
# Web framework - like ASP.NET Core
axum = "0.6"

# Async runtime - like Task.Run infrastructure
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

# Serialization - like System.Text.Json
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database - like Entity Framework
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "macros"] }

# Error handling - like custom exceptions
anyhow = "1.0"

# Logging - like ILogger
tracing = "0.1"
tracing-subscriber = "0.3"
```

### CLI Tool

```toml
[dependencies]
# Argument parsing - like System.CommandLine
clap = { version = "4.0", features = ["derive"] }

# File operations - like System.IO
std::fs  # Built-in, no crate needed

# Progress bars - like custom progress reporting
indicatif = "0.17"

# Colors - like Console.ForegroundColor
colored = "2.0"

# Error handling
anyhow = "1.0"
```

### Data Processing

```toml
[dependencies]
# CSV - like CsvHelper
csv = "1.2"

# JSON - like System.Text.Json
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Parallel processing - like Parallel.ForEach
rayon = "1.7"

# Progress tracking
indicatif = "0.17"
```

## 🏆 Best Practices

1. **Start with popular crates** - They're usually well-tested
2. **Read the entire README** - Understand the design philosophy
3. **Check the examples** - See if the API feels natural
4. **Test compilation time** - Some crates are compile-time heavy
5. **Monitor dependencies** - Use `cargo audit` for security
6. **Plan for updates** - Choose actively maintained crates
7. **Document your choices** - Help future maintainers understand

## 🔄 Migration from .NET

| .NET Library | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| System.Text.Json | serde_json | More explicit, better performance |
| HttpClient | reqwest | Async by default, better error handling |
| Entity Framework | sqlx or diesel | More explicit, compile-time checks |
| ILogger | tracing | Structured logging, better performance |
| System.CommandLine | clap | More powerful, derive-based |
| Newtonsoft.Json | serde_json | Similar API, better performance |
| FluentValidation | validator | Derive-based validation |
| AutoMapper | Manual or custom macros | More explicit mapping |

---

**Next:** Learn about [API Design Patterns](api-design.md) to create great Rust APIs!