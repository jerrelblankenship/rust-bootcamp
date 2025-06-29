# Troubleshooting Guide

Common issues and solutions for Rust development, specifically tailored for C# developers.

## 🔧 Installation Issues

### Rust Not Found After Installation

**Problem**: `rustc` or `cargo` commands not recognized.

**Solution**:
```bash
# Add to PATH manually
# macOS/Linux:
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Windows PowerShell:
[Environment]::SetEnvironmentVariable("Path", "$env:USERPROFILE\.cargo\bin;" + $env:Path, [EnvironmentVariableTarget]::User)
```

### Wrong Architecture on M4 Mac

**Problem**: Rust installed x86_64 version instead of ARM64.

**Solution**:
```bash
# Reinstall for ARM64
rustup self uninstall
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-host aarch64-apple-darwin
```

## 🐛 Common Compilation Errors

### "cannot borrow `x` as mutable more than once"

**C# Equivalent**: Trying to modify a collection while iterating.

```rust
// Problem
let mut vec = vec![1, 2, 3];
let first = &vec[0];
vec.push(4); // ERROR: cannot borrow as mutable

// Solution 1: Limit scope
let mut vec = vec![1, 2, 3];
{
    let first = &vec[0];
    println!("{}", first);
} // first goes out of scope
vec.push(4); // OK

// Solution 2: Clone if needed
let mut vec = vec![1, 2, 3];
let first = vec[0]; // Copy for primitive types
vec.push(4); // OK
```

### "cannot move out of borrowed content"

**C# Equivalent**: Trying to transfer ownership from a reference.

```rust
// Problem
fn take_ownership(s: String) { }
let s = String::from("hello");
let r = &s;
take_ownership(*r); // ERROR: cannot move

// Solution 1: Clone
take_ownership(r.clone());

// Solution 2: Pass reference
fn take_reference(s: &String) { }
take_reference(r);
```

### "mismatched types"

**Common with Option/Result**:

```rust
// Problem
fn get_value() -> Option<i32> {
    let x = 5;
    x // ERROR: expected Option<i32>, found i32
}

// Solution
fn get_value() -> Option<i32> {
    let x = 5;
    Some(x) // Wrap in Some
}
```

### "use of moved value"

**C# Equivalent**: Using disposed object.

```rust
// Problem
let s = String::from("hello");
let s2 = s; // s moved to s2
println!("{}", s); // ERROR: use of moved value

// Solution 1: Clone
let s = String::from("hello");
let s2 = s.clone();
println!("{}", s); // OK

// Solution 2: Reference
let s = String::from("hello");
let s2 = &s;
println!("{}", s); // OK
```

## 💻 VS Code Issues

### rust-analyzer Not Working

**Try these steps**:

1. Check rust-analyzer status:
   ```bash
   rustup component add rust-analyzer
   ```

2. Restart VS Code

3. Check output panel:
   - View → Output → Select "Rust Analyzer Language Server"

4. Manual installation:
   ```bash
   # Download latest release
   curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > ~/.local/bin/rust-analyzer
   chmod +x ~/.local/bin/rust-analyzer
   ```

### Slow IntelliSense

**Solutions**:

1. Reduce features in settings.json:
   ```json
   {
       "rust-analyzer.cargo.features": [],
       "rust-analyzer.cargo.allFeatures": false
   }
   ```

2. Disable proc-macro:
   ```json
   {
       "rust-analyzer.procMacro.enable": false
   }
   ```

3. Increase memory:
   ```json
   {
       "rust-analyzer.server.extraEnv": {
           "RA_LRU_CAP": "2048"
       }
   }
   ```

## 🚀 Performance Issues

### Slow Compilation

**Solutions**:

1. **Enable incremental compilation**:
   ```toml
   # Cargo.toml
   [profile.dev]
   incremental = true
   ```

2. **Use sccache**:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

3. **Faster linker**:
   ```toml
   # .cargo/config.toml
   [target.x86_64-unknown-linux-gnu]
   linker = "clang"
   rustflags = ["-C", "link-arg=-fuse-ld=lld"]
   ```

4. **Parallel compilation**:
   ```bash
   export CARGO_BUILD_JOBS=8
   ```

### High Memory Usage

**For large projects**:

```toml
# .cargo/config.toml
[build]
jobs = 4  # Reduce parallel jobs

[profile.dev]
opt-level = 0  # Faster compilation
debug = 1      # Less debug info
```

## 🐳 Container Issues

### Podman Permission Denied

**Solution**:
```bash
# SELinux fix
podman run -v $(pwd):/workspace:z ...

# Or disable SELinux for container
podman run --security-opt label=disable ...
```

### Slow File Access on macOS

**Use delegated mounts**:
```yaml
volumes:
  - .:/workspace:delegated
```

## 🔍 Debugging Issues

### Breakpoints Not Working

**Check**:

1. Build with debug symbols:
   ```toml
   [profile.dev]
   debug = true
   ```

2. Use correct launch.json:
   ```json
   {
       "type": "lldb",
       "request": "launch",
       "name": "Debug",
       "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
       "args": [],
       "cwd": "${workspaceFolder}"
   }
   ```

### Can't See Variable Values

**Enable pretty printing**:

```bash
# ~/.lldbinit
command script import ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/lldb_lookup.py
type summary add --no-value --python-function lldb_lookup.summary_lookup ".*" --category Rust
type category enable Rust
```

## 🌐 Network/Async Issues

### "cannot block the current thread"

**Problem**: Blocking in async context.

```rust
// Problem
async fn fetch_data() {
    let data = std::fs::read_to_string("file.txt").unwrap(); // Blocking!
}

// Solution
async fn fetch_data() {
    let data = tokio::fs::read_to_string("file.txt").await.unwrap();
}
```

### Tokio Runtime Panic

**Ensure runtime exists**:

```rust
// Problem: No runtime
async fn main() {
    do_async_work().await;
}

// Solution
#[tokio::main]
async fn main() {
    do_async_work().await;
}
```

## 📦 Dependency Issues

### "failed to select a version"

**Clear cache and retry**:
```bash
cargo clean
rm -rf ~/.cargo/registry/cache
cargo update
cargo build
```

### Conflicting Dependencies

**Use specific versions**:
```toml
[dependencies]
tokio = "=1.28.0"  # Exact version
serde = "~1.0.160" # Compatible updates
```

## 🆘 Getting Help

### Before Asking for Help

1. **Read the error message carefully** - Rust errors are very descriptive
2. **Try `cargo check`** - Faster than full build
3. **Use `cargo explain`**:
   ```bash
   cargo explain E0502  # Explain error code
   ```

### Where to Get Help

1. **Rust Users Forum**: https://users.rust-lang.org/
2. **Discord**: https://discord.gg/rust-lang
3. **Stack Overflow**: Tag with `rust`
4. **Reddit**: r/rust

### Asking Good Questions

Include:
- Rust version: `rustc --version`
- Minimal reproducible example
- Full error message
- What you've already tried

## 📋 Quick Reference

### Common Fixes Cheatsheet

| Error | Quick Fix |
|-------|-----------|
| "cannot borrow as mutable" | Clone or restructure code |
| "doesn't live long enough" | Return owned value or use 'static |
| "no method named" | Import trait or check types |
| "mismatched types" | Check Option/Result wrapping |
| "trait not satisfied" | Implement required trait |

### Useful Commands

```bash
# Clean build
cargo clean && cargo build

# Update dependencies
cargo update

# Check without building
cargo check

# See expanded macros
cargo expand

# Find why dependency included
cargo tree -i <package>

# Fix common issues
cargo fix

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

Remember: Every Rust error is the compiler preventing a potential bug. Embrace the errors—they're making your code safer!
