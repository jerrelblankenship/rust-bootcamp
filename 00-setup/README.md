# Module 00: Environment Setup

This module will guide you through setting up a complete Rust development environment tailored for your specific platforms and preferences.

## 🎯 Module Objectives

By the end of this setup, you will have:

- ✅ Rust toolchain installed and configured
- ✅ VS Code with rust-analyzer extension configured
- ✅ Platform-specific optimizations applied
- ✅ Container development environment (optional)
- ✅ All necessary tools for profiling and debugging

## 📋 Setup Guides

Choose the guide for your platform:

1. **[macOS Setup](macos-setup.md)** - Optimized for Apple Silicon
2. **[Windows 11 Setup](windows-11-setup.md)** - PowerShell and native tools
3. **[VS Code Configuration](vscode-configuration.md)** - Universal IDE setup
4. **[Container Setup](container-setup.md)** - Podman-based development

## 🔧 Required Tools

### Core Tools
- **Rust**: Latest stable version via rustup
- **Cargo**: Rust's package manager (included with Rust)
- **VS Code**: Primary development environment
- **Git**: Version control

### Platform-Specific Tools

#### macOS
- Xcode Command Line Tools
- Homebrew (for additional tools)

#### Windows
- Visual Studio Build Tools 2022
- Windows Terminal (recommended)

### Optional but Recommended
- **Podman**: Container runtime (Docker alternative)
- **cargo-watch**: Auto-recompilation on file changes
- **cargo-edit**: Cargo subcommands for managing dependencies
- **sccache**: Compilation cache for faster builds

## 🏃 Quick Verification

After setup, verify your installation:

```bash
# Check Rust installation
rustc --version
cargo --version

# Check VS Code extensions
code --list-extensions | grep rust

# Create and run a test project
cargo new hello-rust
cd hello-rust
cargo run
```

## 🚀 Next Steps

Once your environment is set up:

1. Test the setup with a simple "Hello, World!" program
2. Configure your preferred debugging setup
3. Familiarize yourself with cargo commands
4. Proceed to [Module 01: Foundations](../01-foundations/README.md)

## ⚡ Performance Tips

- Enable parallel compilation: `export CARGO_BUILD_JOBS=8`
- Use `lld` linker for faster builds (especially on Windows)
- Configure sccache for build caching
- Set up cargo target directory on fast storage (SSD)

## 🆘 Troubleshooting

Common issues and solutions:

- **Rust not found**: Ensure `~/.cargo/bin` is in your PATH
- **VS Code IntelliSense slow**: Check rust-analyzer settings
- **Compilation errors**: Update Rust toolchain with `rustup update`

For additional help, see the [Troubleshooting Guide](../resources/troubleshooting.md).
