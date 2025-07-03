# CLI Tools Reference Materials

Comprehensive reference documentation for building professional CLI tools in Rust.

## 📚 Available References

### [Cargo Basics - From rustc to Real Projects](cargo-basics.md) 🆕
A comprehensive guide to transitioning from `rustc` to Cargo, Rust's build tool and package manager. Essential reading for Module 7!

**Topics Covered:**
- Why we switch from rustc to Cargo in Module 7
- Cargo.toml structure and dependencies
- Common Cargo commands and workflows
- C# dotnet CLI comparisons
- Troubleshooting Cargo issues

### [C# to Rust CLI Development Guide](csharp-console.md)
A comprehensive comparison mapping C# console application patterns to their Rust equivalents. Perfect for leveraging your existing C# knowledge while learning Rust CLI development.

**Topics Covered:**
- Command line parsing (CommandLineParser → clap)
- Configuration management (appsettings.json → TOML + serde)
- Error handling (Exceptions → Result types)
- Console output and colors
- File operations and async patterns
- Testing strategies
- Dependency management

### [CLI Patterns and Best Practices](cli-patterns.md)
Essential patterns for building professional-grade CLI tools that users love.

### [Error Design Guidelines](error-design.md)
How to create helpful, actionable error messages that guide users to success.

### [Configuration Hierarchies](config-hierarchy.md)
Best practices for configuration management with proper precedence and validation.

## 🎯 Quick Reference Cards

### Command Structure Patterns

```rust
// Git-style: git remote add origin url
#[derive(Subcommand)]
enum Commands {
    Remote {
        #[clap(subcommand)]
        action: RemoteCommands,
    },
}

#[derive(Subcommand)]  
enum RemoteCommands {
    Add { name: String, url: String },
    Remove { name: String },
}
```

### Error Handling Pattern

```rust
#[derive(Error, Debug)]
enum CliError {
    #[error("File '{path}' not found")]
    FileNotFound { path: String },
    
    #[error("Invalid format: {message}")]
    InvalidFormat { message: String },
}

// Usage
fn main() {
    if let Err(e) = run() {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    }
}
```

### Configuration Pattern

```rust
#[derive(Deserialize, Default)]
struct Config {
    editor: EditorConfig,
    server: ServerConfig,
}

// Load with precedence: CLI > env > file > defaults
let config = ConfigBuilder::new()
    .add_source(File::with_name("config"))
    .add_source(Environment::with_prefix("APP"))
    .add_source(cli_overrides)
    .build()?;
```

### Progress Indication

```rust
use indicatif::{ProgressBar, ProgressStyle};

let pb = ProgressBar::new(total);
pb.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap());

for item in items {
    pb.set_message(format!("Processing {}", item.name));
    process_item(item)?;
    pb.inc(1);
}
pb.finish_with_message("Complete!");
```

## 🛠️ Essential Crates

### Core CLI Building
- **clap** - Command line argument parsing
- **serde** - Configuration serialization/deserialization  
- **toml** / **yaml-rust** - Configuration file formats
- **anyhow** / **thiserror** - Error handling

### User Experience
- **colored** - Terminal colors
- **indicatif** - Progress bars and spinners
- **console** - Advanced terminal features
- **dialoguer** - Interactive prompts

### File Operations
- **walkdir** - Recursive directory traversal
- **glob** - File pattern matching
- **tempfile** - Temporary file management
- **directories** - Platform-specific directories

### Cross-Platform
- **which** - Finding executables in PATH
- **ctrlc** - Graceful shutdown handling
- **terminal_size** - Terminal dimensions

### Testing
- **assert_cmd** - CLI integration testing
- **predicates** - Test assertions
- **tempfile** - Temporary test files

## 📋 CLI Design Checklist

### Basic Functionality
- [ ] Proper argument parsing with help text
- [ ] Meaningful error messages with suggestions
- [ ] Appropriate exit codes (0=success, 1=error)
- [ ] Version information (`--version`)
- [ ] Comprehensive help (`--help`, `command --help`)

### User Experience  
- [ ] Progress indication for long operations
- [ ] Colored output (with terminal detection)
- [ ] Quiet mode for scripting (`-q/--quiet`)
- [ ] Verbose mode for debugging (`-v/--verbose`)
- [ ] Confirmation prompts for destructive operations

### Configuration
- [ ] Multiple configuration sources (file, env, CLI)
- [ ] Proper precedence (CLI > env > file > defaults)
- [ ] Configuration validation with helpful errors
- [ ] Default configuration that "just works"

### Cross-Platform
- [ ] Proper path handling (PathBuf, not string concat)
- [ ] Platform-appropriate config directories
- [ ] Terminal capability detection
- [ ] Signal handling (Ctrl+C, SIGPIPE)

### Pipeline Friendly
- [ ] Reads from stdin when no file specified
- [ ] Silent operation in non-interactive mode
- [ ] Proper stdout/stderr separation
- [ ] Efficient streaming for large inputs

### Professional Polish
- [ ] Shell completion scripts
- [ ] Man page or comprehensive documentation
- [ ] Integration tests covering edge cases
- [ ] Performance optimization for common use cases
- [ ] Semantic versioning and changelog

## 🎓 Learning Path

1. **Start with Argument Parsing** - Master clap derive macros
2. **Add Error Handling** - Create helpful, actionable errors
3. **Implement Configuration** - Support multiple config sources
4. **Enhance User Experience** - Add colors, progress, interactivity
5. **Ensure Cross-Platform** - Test on Windows, macOS, Linux
6. **Polish and Test** - Add edge case handling and comprehensive tests

## 🔗 External Resources

### Documentation
- [Rust CLI Book](https://rust-cli.github.io/book/) - Comprehensive CLI development guide
- [clap Documentation](https://docs.rs/clap/) - Argument parsing library
- [serde Documentation](https://serde.rs/) - Serialization framework

### Examples and Inspiration
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast grep alternative
- [fd](https://github.com/sharkdp/fd) - Find command alternative  
- [bat](https://github.com/sharkdp/bat) - Cat with syntax highlighting
- [exa](https://github.com/ogham/exa) - Modern ls replacement

### Tools
- [cargo-edit](https://github.com/killercup/cargo-edit) - Cargo subcommands for editing Cargo.toml
- [cargo-watch](https://github.com/watchexec/cargo-watch) - Watch for changes and rebuild
- [hyperfine](https://github.com/sharkdp/hyperfine) - Command-line benchmarking

Remember: Great CLI tools are invisible when they work and helpful when they don't!