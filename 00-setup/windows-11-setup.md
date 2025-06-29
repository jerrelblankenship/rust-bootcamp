# Windows 11 Setup Guide

This guide provides detailed instructions for setting up Rust on Windows 11, with optimizations for native Windows development.

## 🪟 Prerequisites

1. **Visual Studio Build Tools 2022** (Required for linking)
   - Download from [Visual Studio Downloads](https://visualstudio.microsoft.com/downloads/)
   - Select "Desktop development with C++" workload
   - Or use the minimal installer:
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

2. **Windows Terminal** (Recommended)
   ```powershell
   winget install Microsoft.WindowsTerminal
   ```

## 🦀 Installing Rust

1. **Install rustup** (Rust toolchain manager)
   - Download and run [rustup-init.exe](https://win.rustup.rs/)
   - Or use PowerShell:
   ```powershell
   Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
   .\rustup-init.exe
   ```

2. **Configure installation**
   - Choose "1) Proceed with installation (default)"
   - Rust will be installed to `%USERPROFILE%\.cargo`

3. **Verify installation** (open new terminal)
   ```powershell
   rustc --version
   cargo --version
   ```

## ⚡ Windows Optimizations

1. **Configure Cargo for Windows performance**
   Create/edit `%USERPROFILE%\.cargo\config.toml`:
   ```toml
   [build]
   # Use all available cores
   jobs = 16  # Adjust based on your CPU
   
   [target.x86_64-pc-windows-msvc]
   # Use lld linker for faster linking
   linker = "rust-lld.exe"
   rustflags = ["-C", "link-arg=/DEBUG:NONE"]  # Faster release builds
   
   [profile.release]
   lto = "thin"
   codegen-units = 1
   
   [net]
   git-fetch-with-cli = true  # Use native git for better performance
   ```

2. **Enable Developer Mode** (for symlink support)
   - Settings → Update & Security → For developers
   - Enable "Developer Mode"

3. **Configure Windows Defender exclusions**
   ```powershell
   # Run as Administrator
   Add-MpPreference -ExclusionPath "$env:USERPROFILE\Source"
   Add-MpPreference -ExclusionPath "$env:USERPROFILE\.cargo"
   Add-MpPreference -ExclusionPath "$env:USERPROFILE\.rustup"
   Add-MpPreference -ExclusionProcess "cargo.exe"
   Add-MpPreference -ExclusionProcess "rustc.exe"
   ```

## 🛠️ Additional Tools

Install useful development tools:

```powershell
# Install Scoop package manager (optional but recommended)
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# Install development tools
scoop install git
winget install RedHat.Podman

# Install cargo extensions
cargo install cargo-watch      # Auto-recompile on changes
cargo install cargo-edit       # Add/remove dependencies from CLI
cargo install cargo-criterion  # Benchmarking tool
cargo install cargo-bloat     # Analyze binary size

# Install Windows-specific tools
cargo install cargo-wix        # MSI installer creation
```

## 🐛 Debugging Setup

1. **Install debugging tools**
   ```powershell
   # Windows Debugging Tools (part of Windows SDK)
   winget install Microsoft.WindowsSDK
   ```

2. **Configure VS Code for Windows debugging**
   - CodeLLDB extension works on Windows
   - Alternative: Use Microsoft C++ extension

3. **Enable Windows debugging features**
   Create `%USERPROFILE%\.cargo\config.toml` entry:
   ```toml
   [profile.dev]
   # Better debugging experience on Windows
   split-debuginfo = "packed"
   ```

## 📊 Performance Profiling

1. **Windows Performance Toolkit**
   ```powershell
   # Install WPA (Windows Performance Analyzer)
   winget install Microsoft.WindowsPerformanceToolkit
   ```

2. **Configure symbols**
   ```powershell
   # Set symbol path for Windows debugging
   setx _NT_SYMBOL_PATH "srv*c:\symbols*https://msdl.microsoft.com/download/symbols"
   ```

3. **Use PerfView for .NET-style profiling**
   ```powershell
   # Download PerfView
   Invoke-WebRequest https://github.com/Microsoft/perfview/releases/latest/download/PerfView.exe -OutFile PerfView.exe
   ```

## 🐳 Podman Setup (Container Development)

1. **Install Podman Desktop**
   ```powershell
   winget install RedHat.Podman-Desktop
   ```

2. **Initialize Podman machine**
   ```powershell
   podman machine init
   podman machine start
   ```

3. **Configure Podman for Windows**
   ```powershell
   # Set Podman to use WSL2 backend
   podman machine set --rootful
   ```

4. **Verify Podman installation**
   ```powershell
   podman version
   podman run hello-world
   ```

## 🔧 PowerShell Profile Setup

Add to your PowerShell profile (`$PROFILE`):

```powershell
# Rust environment
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Aliases for common cargo commands
function cb { cargo build $args }
function cr { cargo run $args }
function ct { cargo test $args }
function cc { cargo check $args }
function cf { cargo fmt $args }
function ccl { cargo clippy $args }

# Function to create new Rust project with gitignore
function New-RustProject {
    param($name)
    cargo new $name
    cd $name
    @"
# Rust
/target/
**/*.rs.bk
*.pdb

# Cargo
Cargo.lock

# IDE
.idea/
.vscode/
*.swp
*.swo
"@ | Out-File -Encoding utf8 .gitignore
    git init
    git add .
    git commit -m "Initial commit"
}
```

## ✅ Verification

Run this PowerShell script to verify your setup:

```powershell
Write-Host "🦀 Checking Rust installation..." -ForegroundColor Green
rustc --version
cargo --version

Write-Host "`n🏗️ Checking build tools..." -ForegroundColor Green
where cl.exe

Write-Host "`n📦 Checking cargo tools..." -ForegroundColor Green
$tools = @("cargo-watch", "cargo-edit", "cargo-criterion")
foreach ($tool in $tools) {
    if (Get-Command $tool -ErrorAction SilentlyContinue) {
        & $tool --version
    } else {
        Write-Host "$tool not installed" -ForegroundColor Yellow
    }
}

Write-Host "`n🐳 Checking Podman..." -ForegroundColor Green
if (Get-Command podman -ErrorAction SilentlyContinue) {
    podman --version
} else {
    Write-Host "Podman not installed" -ForegroundColor Yellow
}

Write-Host "`n✅ Creating test project..." -ForegroundColor Green
$testDir = "rust-test-$(Get-Random)"
cargo new --bin $testDir
cd $testDir
cargo build --release
cargo run --release
cd ..
Remove-Item -Recurse -Force $testDir

Write-Host "`n🎉 Setup complete!" -ForegroundColor Green
```

## 🚀 Performance Tips for Windows

1. **Use native Windows APIs**
   ```toml
   # In Cargo.toml
   [dependencies.windows]
   version = "0.48"
   features = ["Win32_Foundation", "Win32_System_Threading"]
   ```

2. **Optimize for Windows filesystem**
   - Place projects on fast SSD
   - Avoid deep directory structures
   - Consider RAMDisk for temporary build artifacts

3. **Parallel compilation settings**
   ```powershell
   # Set environment variables for better performance
   [Environment]::SetEnvironmentVariable("CARGO_BUILD_JOBS", "16", "User")
   [Environment]::SetEnvironmentVariable("CARGO_INCREMENTAL", "1", "User")
   ```

## 🆘 Troubleshooting

### Common Issues

1. **"error: linker `link.exe` not found"**
   - Solution: Install Visual Studio Build Tools
   - Ensure VS tools are in PATH

2. **"could not compile `xxx` due to previous error"**
   - Solution: Clear build cache: `cargo clean`
   - Check Windows Defender isn't blocking

3. **Slow compilation times**
   - Enable Windows Developer Mode
   - Add exclusions to antivirus
   - Use `rust-lld` linker

### Windows-Specific Issues

1. **Path length limitations**
   - Enable long path support in Windows
   - Use shorter project paths

2. **Permission errors**
   ```powershell
   # Run as Administrator if needed
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope LocalMachine
   ```

3. **WSL2 Integration**
   - Consider using WSL2 for Linux-like environment
   - Install Rust in both Windows and WSL2 if needed

## 📚 Additional Resources

- [Rust on Windows](https://rust-lang.github.io/rustup/installation/windows.html)
- [Windows-rs Documentation](https://microsoft.github.io/windows-docs-rs/)
- [Optimizing Rust for Windows](https://blog.rust-lang.org/2021/01/08/Rust-1.49.0.html#windows-support)

---

Next: [VS Code Configuration](vscode-configuration.md) →
