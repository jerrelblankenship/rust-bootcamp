# Cargo: Rust's Build Tool and Package Manager

## Transitioning from rustc to Cargo

In previous modules, you compiled Rust files directly:
```bash
rustc ex01-hello-world.rs
./ex01-hello-world
```

Starting with Module 7, we use Cargo because real-world Rust development requires:
- External dependencies (crates)
- Build configuration
- Test management
- Documentation generation

## C# Developer's Guide to Cargo

| C#/.NET Tool | Cargo Equivalent | Purpose |
|--------------|------------------|---------|
| NuGet | crates.io | Package repository |
| .csproj | Cargo.toml | Project configuration |
| dotnet restore | cargo fetch | Download dependencies |
| dotnet build | cargo build | Compile project |
| dotnet run | cargo run | Build & execute |
| dotnet test | cargo test | Run unit tests |
| dotnet publish | cargo build --release | Create optimized binary |

## Understanding Cargo.toml

Think of `Cargo.toml` as your `.csproj` file:

```toml
[package]
name = "my-cli-tool"        # Like <AssemblyName>
version = "0.1.0"           # Like <Version>
edition = "2021"            # Rust edition (like C# language version)

[dependencies]              # Like <PackageReference> items
clap = "4.5"               # CommandLineParser equivalent
serde = "1.0"              # Newtonsoft.Json equivalent
```

## Key Cargo Commands for CLI Development

### 1. Running Exercises
```bash
# Run a specific exercise (notice the --bin flag)
cargo run --bin ex01-arg-parser

# Pass arguments to your program after --
cargo run --bin ex01-arg-parser -- --help
cargo run --bin ex01-arg-parser -- --input file.txt --verbose
```

### 2. Checking Your Code
```bash
# Fast type checking (doesn't produce binary)
cargo check

# See all compiler warnings
cargo clippy
```

### 3. Building
```bash
# Debug build (fast compile, slow runtime)
cargo build

# Release build (slow compile, fast runtime)
cargo build --release
```

### 4. Managing Dependencies
```bash
# Add a new dependency
cargo add colored

# Update dependencies
cargo update

# See dependency tree
cargo tree
```

## Common Cargo Patterns for CLI Tools

### Binary vs Library Crates
- **Binary crate**: Has a `main()` function, creates executable
- **Library crate**: Provides reusable code for other crates

Our exercises are all binary crates (notice the `[[bin]]` sections in Cargo.toml).

### Workspace Structure
```
07-cli-tools/
├── exercises/
│   ├── Cargo.toml         # Defines all exercise binaries
│   ├── ex01-arg-parser.rs # Each file is a separate binary
│   └── ex02-error-handling.rs
└── project-dev-tools/
    ├── Cargo.toml         # Separate project with its own deps
    └── src/
        └── main.rs
```

### Debug vs Release Builds

**Debug** (default):
- Fast compilation
- Includes debug symbols
- No optimizations
- Larger binary size
- Better error messages

**Release** (`--release` flag):
- Slow compilation  
- Optimized for speed
- Smaller binary size
- Stripped debug info

## Troubleshooting Cargo

### "Could not compile dependency"
```bash
# Clean build artifacts and try again
cargo clean
cargo build
```

### "Package not found"
```bash
# Update the crates.io index
cargo update
```

### "Multiple packages with same name"
```bash
# Be specific about which binary to run
cargo run --bin ex01-arg-parser
```

## Cargo vs dotnet CLI Comparison

```bash
# C#: dotnet new console -n MyApp
cargo new my-app --bin

# C#: dotnet add package Newtonsoft.Json
cargo add serde_json

# C#: dotnet run -- arg1 arg2
cargo run -- arg1 arg2

# C#: dotnet test
cargo test

# C#: dotnet publish -c Release
cargo build --release
```

## Why This Matters for CLI Tools

1. **Dependency Management**: CLI tools often need many crates:
   - `clap` for argument parsing
   - `tokio` for async operations
   - `reqwest` for HTTP requests
   - `serde` for serialization

2. **Binary Distribution**: Cargo creates single, standalone executables
   - No runtime required (unlike .NET)
   - Copy one file to deploy

3. **Cross-Platform Builds**: Cargo handles platform differences
   - Same code compiles for Windows, macOS, Linux
   - Platform-specific dependencies handled automatically

## Next Steps

Now that you understand Cargo, you're ready to build real CLI tools that use the full Rust ecosystem. The exercises will feel more like actual Rust development!

Remember: From now on, always use `cargo run --bin <exercise-name>` instead of `rustc`.