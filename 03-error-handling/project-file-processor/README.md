# File Processor Project - Fix the Broken Code! 🔧

**Module 03: Error Handling - Discovery-Based Learning Project**

> ⚠️ **This project contains broken starter code that you must fix!** This is intentional - you'll learn Rust error handling by debugging and implementing real solutions.

## 🎯 Your Mission

Transform this broken file processor into a production-ready CLI tool by fixing compilation errors and implementing missing functionality. You'll learn advanced error handling patterns while building something practical and useful.

## 🚨 Current Status: BROKEN BY DESIGN

```bash
$ cargo build
error[E0412]: cannot find type `FileProcessorError` in this scope
error[E0425]: cannot find function `todo` in this scope
error: failed to resolve: could not find `error` in `file_processor`
... (many more errors - your job to fix them!)
```

**This is exactly what you want to see!** These compilation errors are your learning roadmap.

## 🎓 Learning Objectives

By fixing this broken code, you will master:

- ✅ **Custom error types** with proper Display and Error trait implementations
- ✅ **Error propagation** with the `?` operator across modules
- ✅ **Error conversion** with `From` traits for seamless error handling
- ✅ **Error recovery** strategies and retry patterns
- ✅ **Production error handling** in a real CLI application
- ✅ **Module organization** for large Rust projects
- ✅ **File I/O errors** and robust file processing
- ✅ **Configuration management** with multiple sources

## 📁 Project Structure (All Broken - Fix Required!)

```
src/
├── main.rs              🔧 CLI interface with compilation errors
├── lib.rs               🔧 Library structure with todo!() implementations
├── error.rs             🔧 Custom error types to implement
├── config.rs            🔧 Configuration management to implement
├── processor.rs         🔧 Core processing engine to implement
├── reporting.rs         🔧 Report generation to implement
└── formats/             🔧 Format-specific processors to implement
    ├── mod.rs          🔧 Module organization
    ├── json.rs         🔧 JSON processor implementation
    ├── csv.rs          🔧 CSV processor implementation
    └── text.rs         🔧 Text processor implementation
```

Every file contains:
- **Real compilation errors** that prevent building
- **TODO comments** guiding your implementation
- **FIXME markers** highlighting specific problems
- **Hints** pointing you toward solutions
- **Complete solution available** in `solutions/` directory if you get stuck

## 🚀 Getting Started

### 1. Witness the Broken State
```bash
cd 03-error-handling/project-file-processor
cargo build
# Expect LOTS of compilation errors - this is your starting point!
```

### 2. Start with Basic Structure
```bash
# Begin by examining the main files
cat src/lib.rs        # See the module structure you need to implement
cat src/error.rs      # Start with error type definitions
cat src/main.rs       # Understand the CLI interface you're building
```

### 3. Fix Compilation Errors Incrementally
```bash
# Work through errors systematically
cargo build 2>&1 | head -20   # See first 20 errors
# Fix one error at a time
# Recompile frequently to track progress
```

### 4. Use Tests to Guide Development
```bash
# Run tests to see what needs implementing
cargo test 2>&1 | grep "todo"   # Find all unimplemented functions
```

## 🔧 Implementation Roadmap

### Phase 1: Foundation (Days 1-2)
1. **Define error types** in `src/error.rs`
   - Implement `FileProcessorError` enum variants
   - Add `Display` and `Error` trait implementations
   - Create `From` trait conversions

2. **Basic configuration** in `src/config.rs`
   - Implement `Config` struct and loading logic
   - Add environment variable support

### Phase 2: Core Functionality (Days 3-4)
3. **Processing engine** in `src/processor.rs`
   - Implement `FileProcessorEngine` with batch processing
   - Add file format detection
   - Create processing statistics tracking

4. **Format processors** in `src/formats/`
   - Implement JSON, CSV, and Text processors
   - Add comprehensive validation and error handling

### Phase 3: CLI and Reporting (Day 5)
5. **CLI interface** in `src/main.rs`
   - Fix argument parsing and option handling
   - Implement error chain printing
   - Add comprehensive error recovery

6. **Report generation** in `src/reporting.rs`
   - Implement multiple output formats
   - Add performance metrics and statistics

## 🏃 Quick Compilation Guide

### Start Here - Make Basic Types Compile:
```rust
// In src/error.rs - fix this first
#[derive(Debug)]
pub enum FileProcessorError {
    FileNotFound { path: String },
    IoError(std::io::Error),
    // TODO: Add more variants
}

// Implement Display trait
impl std::fmt::Display for FileProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileProcessorError::FileNotFound { path } => {
                write!(f, "File not found: {}", path)
            }
            // TODO: Handle other variants
        }
    }
}
```

### Key Implementation Challenges:

1. **Error Type Design**
   ```rust
   // Challenge: Design errors that are both specific and actionable
   FileProcessorError::ProcessingFailed { 
       path: String, 
       reason: String 
   }
   ```

2. **Error Conversion**
   ```rust
   // Challenge: Automatic error conversion
   impl From<std::io::Error> for FileProcessorError {
       fn from(err: std::io::Error) -> Self {
           // TODO: Implement conversion
       }
   }
   ```

3. **Error Propagation**
   ```rust
   // Challenge: Use ? operator throughout
   fn process_file(&self, path: &str) -> Result<ProcessingResult> {
       let content = fs::read_to_string(path)?;  // Automatic conversion
       let result = self.analyze_content(&content)?;
       Ok(result)
   }
   ```

## 🧪 Testing Your Implementation

```bash
# Test as you implement
cargo test error_types          # Test error handling
cargo test config_loading       # Test configuration
cargo test file_processing      # Test core functionality
cargo test cli_integration      # Test CLI interface

# Test with real files (create test data)
cargo run -- test.json --output results/ --verbose
cargo run -- *.csv --format summary --validate
```

## 🆘 When You Get Stuck

### Debugging Strategy:
1. **Read error messages carefully** - Rust's compiler gives excellent guidance
2. **Fix one error at a time** - Don't try to fix everything at once
3. **Use `todo!()` temporarily** - Replace implementations incrementally
4. **Check the solutions** - Complete working code in `solutions/` directory

### Common Issues:
- **"cannot find type"** → Implement the missing type definition
- **"todo! is not yet implemented"** → Replace `todo!()` with actual implementation
- **"trait bounds not satisfied"** → Implement required traits (Display, Error, etc.)
- **"mismatched types"** → Add error conversion with `From` traits

## 📊 Success Metrics

You'll know you're succeeding when:

- ✅ `cargo build` completes without errors
- ✅ `cargo test` shows passing tests
- ✅ CLI runs and processes files correctly
- ✅ Error messages are helpful and actionable
- ✅ Configuration loading works from multiple sources
- ✅ Reports generate in multiple formats

## 🎯 Final Goal

Transform broken starter code into a production-ready file processor:

```bash
# This should work when you're done:
$ file-processor *.json *.csv --output results/ --format summary --validate
📊 Processing Summary
==================
✅ Files processed: 15/15
📈 Success rate: 100%
📝 Total records: 1,247
⏱️  Processing time: 1.2s
⚡ Records per second: 1,039
📁 Output directory: results/
```

## 🏆 Advanced Challenges (Bonus)

After completing the basic implementation:

1. **Add retry logic** for network-like failures
2. **Implement configuration file discovery** across multiple locations
3. **Add progress bars** for long-running operations
4. **Create custom derive macros** for error types
5. **Add comprehensive benchmarks** with criterion
6. **Implement streaming processing** for very large files

## 📚 C# Developer Notes

This project demonstrates patterns you know from C#, but implemented the Rust way:

| C# Pattern | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| `try-catch` | `match result` | Errors are values, not exceptions |
| `Exception` types | `Error` enum variants | Structured errors with data |
| `ConfigurationBuilder` | `Config::from_file()` | Explicit error handling |
| `IDisposable` | Automatic `Drop` | Automatic resource cleanup |
| `async/await` | `Result<T, E>` | Error handling, not async |

## 🔗 Resources

- **Error Handling Guide**: `03-error-handling/01-result-and-option.md`
- **Project Solutions**: `solutions/` directory (use when stuck)
- **Rust Book**: [Error Handling Chapter](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- **Error Handling Blog**: [Rust Error Handling](https://blog.burntsushi.net/rust-error-handling/)

---

**Ready to start debugging and implementing?** 

Begin with `src/error.rs` and work through the compilation errors systematically. Remember: every error you fix teaches you something valuable about Rust's error handling!

**🦀 Good luck, and embrace the compilation errors - they're your teachers!**
