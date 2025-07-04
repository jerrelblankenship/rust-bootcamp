# macOS Setup Guide

This guide provides detailed instructions for setting up Rust on macOS, with optimizations for Apple Silicon Macs.

## 🍎 Prerequisites

1. **Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **Homebrew** (if not already installed)
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

## 🦀 Installing Rust

1. **Install rustup** (Rust toolchain manager)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Configure your shell** (add to `~/.zshrc` or `~/.bash_profile`)
   ```bash
   source "$HOME/.cargo/env"
   ```

3. **Verify installation**
   ```bash
   rustc --version
   cargo --version
   ```

## ⚡ Apple Silicon Optimizations

For Apple Silicon Macs (M1, M2, M3, M4):

1. **Ensure native ARM64 compilation**
   ```bash
   rustup default stable-aarch64-apple-darwin
   ```

2. **Configure Cargo for optimal performance**
   Create/edit `~/.cargo/config.toml`:
   ```toml
   [build]
   # Use all available cores (adjust based on your Mac's core count)
   jobs = 0  # 0 = auto-detect available cores
   
   [target.aarch64-apple-darwin]
   # Use Apple's native linker for better performance
   linker = "clang"
   rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/ld"]
   
   [profile.release]
   lto = "thin"  # Link-time optimization
   codegen-units = 1  # Better optimization at cost of compile time
   ```

3. **For Intel Macs**, use the default x86_64 toolchain:
   ```bash
   rustup default stable-x86_64-apple-darwin
   ```

## 🛠️ Additional Tools

Install useful development tools:

```bash
# Install development tools via Homebrew
brew install git podman

# Install cargo extensions
cargo install cargo-watch      # Auto-recompile on changes
cargo install cargo-edit       # Add/remove dependencies from CLI
cargo install cargo-criterion  # Benchmarking tool
cargo install flamegraph      # Performance profiling
cargo install cargo-bloat     # Analyze binary size

# Install performance tools
brew install hyperfine        # Command-line benchmarking
```

## 🐛 Debugging Setup

1. **Install CodeLLDB for VS Code debugging**
   - This will be installed automatically with the VS Code setup

2. **Configure LLDB for better Rust support**
   Create `~/.lldbinit`:
   ```
   command script import ~/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/etc/lldb_lookup.py
   type summary add --no-value --python-function lldb_lookup.summary_lookup ".*" --category Rust
   type category enable Rust
   ```

## 📊 Performance Profiling

1. **Enable debug symbols in release builds**
   Add to `Cargo.toml` for profiling:
   ```toml
   [profile.release]
   debug = true
   ```

2. **Install Instruments** (comes with Xcode)
   - Use for detailed performance analysis
   - Great for memory profiling

3. **Configure cargo for profiling**
   ```bash
   # Install cargo-instruments
   cargo install cargo-instruments
   
   # Profile your application
   cargo instruments -t "Time Profiler" --release
   ```

## 🐳 Podman Setup (Container Development)

1. **Install Podman Desktop**
   ```bash
   brew install podman
   brew install podman-desktop
   ```

2. **Initialize Podman machine**
   ```bash
   # Configure based on your Mac's capabilities
   podman machine init --cpus=4 --memory=8192 --disk-size=50
   podman machine start
   ```

3. **Verify Podman installation**
   ```bash
   podman version
   podman run hello-world
   ```

## 🔍 VS Code Integration

See [VS Code Configuration](vscode-configuration.md) for detailed IDE setup.

Key extensions for macOS development:
- rust-analyzer
- CodeLLDB (for debugging)
- Error Lens
- crates

## ✅ Verification

Run this script to verify your setup:

```bash
#!/bin/bash
echo "🦀 Checking Rust installation..."
rustc --version
cargo --version

echo "\n🏗️ Checking build tools..."
clang --version

echo "\n📦 Checking cargo tools..."
cargo watch --version 2>/dev/null || echo "cargo-watch not installed"
cargo criterion --version 2>/dev/null || echo "cargo-criterion not installed"

echo "\n🐳 Checking Podman..."
podman --version 2>/dev/null || echo "Podman not installed"

echo "\n✅ Creating test project..."
cargo new --bin rust-test
cd rust-test
cargo build --release
cargo run --release

echo "\n🎉 Setup complete!"
```

## 🚀 Performance Tips for Apple Silicon

1. **Use unified memory efficiently**
   - Apple Silicon's unified memory architecture benefits from fewer allocations
   - Prefer stack allocation over heap when possible

2. **Leverage SIMD instructions**
   - Use `std::simd` for vectorized operations (unstable)
   - Consider `packed_simd_2` crate for stable SIMD

3. **Profile-guided optimization**
   ```bash
   # Build with PGO instrumentation
   RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release
   
   # Run your program to generate profile data
   ./target/release/your-program
   
   # Rebuild with profile data
   RUSTFLAGS="-Cprofile-use=/tmp/pgo-data" cargo build --release
   ```

4. **Compilation optimization**
   ```bash
   # Enable parallel LLVM backend
   export RUSTFLAGS="-C codegen-units=16"
   
   # Use LLD linker (faster linking)
   export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
   ```

## 🆘 Troubleshooting

### Common Issues

1. **"xcrun: error: invalid active developer path"**
   - Solution: Install Xcode Command Line Tools
   
2. **Rust commands not found**
   - Solution: Ensure `~/.cargo/bin` is in PATH
   
3. **Slow compilation**
   - Enable sccache: `cargo install sccache`
   - Set `RUSTC_WRAPPER=sccache` in environment

### Apple Silicon Specific Issues

1. **Binary compatibility warnings**
   - Ensure you're using ARM64 versions of all tools
   - Check with: `file $(which rustc)`

2. **Rosetta 2 translation overhead**
   - Avoid x86_64 dependencies when possible
   - Use native ARM64 crates when available

3. **Mixed architecture builds**
   - Explicitly target the correct architecture:
     ```bash
     cargo build --target aarch64-apple-darwin  # For Apple Silicon
     cargo build --target x86_64-apple-darwin   # For Intel Macs
     ```

### Intel Mac Considerations

1. **Ensure correct toolchain**
   ```bash
   rustup default stable-x86_64-apple-darwin
   ```

2. **Homebrew architecture**
   - Intel Macs use `/usr/local/bin`
   - Apple Silicon Macs use `/opt/homebrew/bin`

## 📚 Additional Resources

- [Rust on Apple Silicon](https://github.com/rust-lang/rust/issues/73908)
- [Optimizing for Apple Silicon](https://developer.apple.com/documentation/apple-silicon/optimizing-your-code-for-apple-silicon)
- [Cargo Configuration](https://doc.rust-lang.org/cargo/reference/config.html)
- [Apple Developer Documentation](https://developer.apple.com/documentation/)

---

Next: [VS Code Configuration](vscode-configuration.md) →
