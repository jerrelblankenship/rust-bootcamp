# File Processor Project Hints - Level 1 (Gentle Guidance)

## 🔍 Understanding Production Error Handling

You're working on the file processor project, which demonstrates real-world error handling in a complete CLI application. This is about applying all the error handling concepts you've learned in a production-style codebase.

## 💡 Core Concept: Comprehensive Error Handling Architecture

### What is Production Error Handling?
Production error handling means designing error systems that are robust, debuggable, recoverable, and user-friendly. Every possible failure mode should be handled gracefully.

**Key Production Requirements:**
- **Robustness**: Handle all possible failure modes without crashing
- **Observability**: Provide detailed error information for debugging
- **Recovery**: Offer retry and fallback strategies where appropriate
- **User Experience**: Give actionable error messages to users

## 🎯 Project Structure Understanding

### What You're Building
A file processing CLI tool that demonstrates production error handling patterns:

```
file-processor/
├── src/
│   ├── main.rs          # CLI interface with error handling
│   ├── lib.rs           # Library organization
│   ├── error.rs         # Comprehensive error types
│   ├── config.rs        # Configuration loading
│   ├── processor.rs     # Core processing engine
│   ├── reporting.rs     # Report generation
│   └── formats/         # Format-specific processors
│       ├── mod.rs      # Module organization
│       ├── json.rs     # JSON processing
│       ├── csv.rs      # CSV processing
│       └── text.rs     # Text processing
```

## 🎯 Getting Started Hints

### Hint 1: Start with Error Type Design
**Pattern Goal**: Create a comprehensive error hierarchy for the entire application.

**Key Insight**: Your error types should cover every possible failure mode while providing actionable information.

**Gentle Questions to Ask Yourself:**
- What can go wrong when processing files?
- What can go wrong when reading configuration?
- What can go wrong with different file formats?
- How do you provide helpful error messages to users?

**Error Design Approach:**
```rust
#[derive(Debug)]
pub enum FileProcessorError {
    // File system errors
    FileNotFound { path: String },
    PermissionDenied { path: String, operation: String },
    
    // Processing errors
    FormatNotSupported { format: String, file: String },
    ProcessingFailed { file: String, reason: String },
    
    // Configuration errors
    ConfigurationError { message: String },
    
    // Validation errors
    ValidationFailed { field: String, value: String, reason: String },
}
```

### Hint 2: Implement Basic Module Structure
**Pattern Goal**: Organize code into logical modules with proper error propagation.

**Key Insight**: Each module should have a clear responsibility and consistent error handling patterns.

**Module Organization:**
- `error.rs`: Central error type definitions
- `config.rs`: Configuration loading with validation
- `processor.rs`: Core file processing logic
- `formats/`: Format-specific processing implementations

### Hint 3: CLI Interface Design
**Pattern Goal**: Create user-friendly CLI that handles errors gracefully.

**Key Insight**: CLI tools should never crash with panic - always provide helpful error messages.

**CLI Error Handling Pattern:**
```rust
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        
        // Print error chain for debugging
        let mut source = e.source();
        while let Some(err) = source {
            eprintln!("  Caused by: {}", err);
            source = err.source();
        }
        
        std::process::exit(1);
    }
}

fn run() -> Result<(), FileProcessorError> {
    // Main application logic
}
```

## 🔧 Implementation Strategy

### Phase 1: Foundation (Start Here)
1. **Define Error Types**: Create comprehensive error enums in `error.rs`
2. **Implement Display**: User-friendly error messages
3. **Add From Conversions**: Automatic error type conversions
4. **Basic Module Structure**: Stub out main modules

### Phase 2: Core Functionality
1. **Configuration Loading**: Implement config parsing with validation
2. **File Processing Engine**: Core processing logic with error handling
3. **Format Processors**: JSON, CSV, and text format handling
4. **Error Recovery**: Retry logic and fallback strategies

### Phase 3: Polish and Production Readiness
1. **CLI Interface**: Complete command-line interface
2. **Report Generation**: Output formatting and statistics
3. **Testing**: Comprehensive error scenario testing
4. **Documentation**: Error handling patterns and usage

## 🎯 Specific Implementation Hints

### Error Type Implementation
**Problem**: You need to fix compilation errors in `error.rs`.

**Gentle Approach:**
```rust
// Start with basic error variants
#[derive(Debug)]
pub enum FileProcessorError {
    FileNotFound { path: String },
    IoError(std::io::Error),
    // TODO: Add more variants as you encounter them
}

// Implement Display trait
impl std::fmt::Display for FileProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileProcessorError::FileNotFound { path } => {
                write!(f, "File not found: {}", path)
            }
            FileProcessorError::IoError(e) => {
                write!(f, "I/O error: {}", e)
            }
        }
    }
}

// Implement Error trait
impl std::error::Error for FileProcessorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileProcessorError::IoError(e) => Some(e),
            _ => None,
        }
    }
}
```

### Configuration Loading Pattern
**Problem**: Need to load and validate configuration from multiple sources.

**Gentle Approach:**
```rust
#[derive(Debug)]
pub struct Config {
    pub input_patterns: Vec<String>,
    pub output_dir: String,
    pub max_file_size: usize,
    pub parallel_processing: bool,
}

impl Config {
    pub fn load() -> Result<Self, FileProcessorError> {
        // Try loading from command line args
        // Fall back to config file
        // Fall back to environment variables
        // Use defaults if nothing else works
        todo!("Implement config loading")
    }
}
```

### File Processing Pattern
**Problem**: Need to process different file types with unified error handling.

**Gentle Approach:**
```rust
pub struct FileProcessorEngine {
    config: Config,
}

impl FileProcessorEngine {
    pub fn new(config: Config) -> Self {
        FileProcessorEngine { config }
    }
    
    pub fn process_file(&self, path: &str) -> Result<ProcessingResult, FileProcessorError> {
        // Detect file format
        // Route to appropriate processor
        // Handle errors consistently
        todo!("Implement file processing")
    }
}
```

## 💭 Think About It

**Production Error Handling Principles:**
- **Fail Fast**: Detect errors as early as possible
- **Fail Safe**: When errors occur, leave the system in a safe state
- **Provide Context**: Include enough information to diagnose and fix issues
- **Be Consistent**: Handle similar errors in similar ways
- **Be Actionable**: Tell users what they can do to fix the problem

**Error Categories in File Processing:**
- **User Errors**: Wrong command line arguments, missing files
- **System Errors**: Disk full, permission denied, network issues
- **Data Errors**: Malformed JSON, encoding issues, corrupt files
- **Logic Errors**: Configuration conflicts, validation failures

## 🆘 Common Issues and Solutions

### "Cannot find type 'FileProcessorError'"
**Solution**: Implement the error type in `error.rs` and export it from `lib.rs`

### "todo! is not yet implemented"
**Solution**: Replace `todo!()` macros with actual implementations step by step

### "trait bounds not satisfied"
**Solution**: Implement required traits (Display, Error, Debug) for your error types

### "mismatched types"
**Solution**: Add `From` trait implementations for automatic error conversion

## 🔧 Testing Your Implementation

### Basic Compilation Test
```bash
cd project-file-processor
cargo build
# Should compile without errors once basic types are implemented
```

### Functionality Test
```bash
# Test with different scenarios
cargo run -- --help
cargo run -- *.json --output results/
cargo run -- nonexistent_file.txt
```

### Error Handling Test
```bash
# Test error scenarios
cargo run -- /root/protected_file.txt  # Permission denied
cargo run -- *.xyz                     # Unsupported format
cargo run -- --invalid-flag            # Invalid arguments
```

## ➡️ Next Level

Ready for specific implementation details? Try [Level 2 Hints](file-processor-level2.md) for concrete code solutions.

Remember: This project demonstrates how all the error handling concepts work together in a real application! 🦀