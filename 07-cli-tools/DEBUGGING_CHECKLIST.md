# CLI Tools Debugging Checklist 🛠️

When building CLI tools in Rust, work through these common issues:

## 1. Argument Parsing Errors

### ❌ Manual String Parsing
```rust
let args: Vec<String> = env::args().collect();
if args[1] == "--help" { // Panics if no args!
    // ...
}
```
**Fix:** Use `clap` or `structopt` for robust parsing

### ❌ Missing Required Arguments
```rust
// User forgets to provide required arg
let filename = &args[1]; // Index out of bounds!
```
**Fix:** Validate args and provide helpful error messages

## 2. Error Handling in CLI

### ❌ Panicking on User Errors
```rust
let file = File::open(path).unwrap(); // Ugly panic message!
```
**Fix:** Use `anyhow` or custom error types with context

### ❌ Poor Error Messages
```rust
Err("failed") // Not helpful!
```
**Fix:** Provide context: what failed, why, and how to fix

## 3. Input/Output Issues

### ❌ Not Handling Piped Input
```rust
// Only reads from files, not stdin
let content = fs::read_to_string(filename)?;
```
**Fix:** Support both file input and stdin

### ❌ Breaking Unix Pipes
```rust
println!("Processing..."); // Pollutes stdout!
```
**Fix:** Use `eprintln!` for status messages

## 4. Configuration Problems

### ❌ Hardcoded Paths
```rust
let config_path = "/home/user/.config/app.toml"; // Won't work on Windows!
```
**Fix:** Use `dirs` crate for platform-specific paths

### ❌ No Config File Flexibility
```rust
// Only looks in one place
let config = read_config("./.app.toml")?;
```
**Fix:** Check multiple locations, support env vars

## 5. Terminal Output

### ❌ No Color Support Detection
```rust
// Always outputs color codes
println!("\x1b[31mError!\x1b[0m"); // Breaks in pipes!
```
**Fix:** Use `termcolor` or check `isatty()`

### ❌ Poor Progress Indication
```rust
for item in large_list {
    process(item); // User sees nothing!
}
```
**Fix:** Use `indicatif` for progress bars

## 6. Cross-Platform Issues

### ❌ Unix-Only Code
```rust
use std::os::unix::fs::PermissionsExt; // Won't compile on Windows!
```
**Fix:** Use `cfg` attributes for platform-specific code

### ❌ Path Separator Issues
```rust
let path = format!("{}/{}", dir, file); // Wrong on Windows!
```
**Fix:** Use `PathBuf` and `join()`

## C# to Rust CLI Patterns

| C# Pattern | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| `CommandLineParser` | `clap`/`structopt` | Derive-based parsing |
| `Console.WriteLine` | `println!` | Buffered by default |
| `Environment.Exit` | `std::process::exit` | Or return from main |
| `Console.ReadLine` | `std::io::stdin().read_line()` | Handle EOF |
| `Path.Combine` | `PathBuf::join` | Cross-platform |
| `app.config` | TOML/YAML with `serde` | Type-safe deserialization |

## Quick CLI Best Practices

1. **Exit Codes**: Return `Result` from main or use proper exit codes
2. **Help Text**: Provide comprehensive `--help` with examples
3. **Version Info**: Include `--version` flag
4. **Quiet Mode**: Support `-q/--quiet` for scripting
5. **Verbose Mode**: Support `-v/-vv/-vvv` for debugging
6. **Config Priority**: CLI args > env vars > config file > defaults

## Testing CLI Tools

- [ ] Test with missing arguments
- [ ] Test with invalid arguments  
- [ ] Test piped input/output
- [ ] Test on Windows, macOS, and Linux
- [ ] Test with different terminal types
- [ ] Test error scenarios

## Pro Tips

- Use `#[derive(Parser)]` with clap for type-safe parsing
- Support both JSON and human-readable output
- Implement shell completions
- Use `ctrlc` crate for graceful shutdown
- Consider `duct` for testing CLI tools
- Add `--dry-run` for dangerous operations