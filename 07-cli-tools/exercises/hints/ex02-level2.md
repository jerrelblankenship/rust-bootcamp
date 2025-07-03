# Exercise 2 Hints - Level 2: Specific Guidance 🟡

## Concrete Error Handling Patterns

**Ready for specific error handling techniques?** Here's how to transform terrible errors into helpful ones.

### 🏗️ Error Enum Structure

Define specific error types for different problems:
```rust
#[derive(Debug)]
enum CliError {
    InvalidArgs(String),
    FileNotFound(String),
    PermissionDenied(String),
    ParseError(String),
    EmptyFile(String),
}
```

### 🎨 Error Display Implementation

Make errors user-friendly:
```rust
impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::FileNotFound(filename) => {
                write!(f, "File '{}' not found", filename)
            },
            CliError::PermissionDenied(filename) => {
                write!(f, "Cannot read '{}': Permission denied", filename)
            },
            CliError::ParseError(details) => {
                write!(f, "Cannot parse input: {}", details)
            },
            // ... other variants
        }
    }
}
```

### 🔧 Replace Panics with Results

Transform the terrible patterns:
```rust
// BEFORE (terrible)
if args.len() != 2 {
    panic!("Wrong number of arguments!");
}

// AFTER (helpful)
fn validate_args(args: &[String]) -> Result<String, CliError> {
    if args.len() == 1 {
        return Err(CliError::InvalidArgs(
            "Expected exactly 1 filename, got 0 arguments.\n💡 Try: program myfile.txt".to_string()
        ));
    }
    if args.len() > 2 {
        return Err(CliError::InvalidArgs(
            format!("Expected exactly 1 filename, got {} arguments.\n💡 Try: program myfile.txt", args.len() - 1)
        ));
    }
    Ok(args[1].clone())
}
```

### 📁 Better File Operations

```rust
// BEFORE (terrible)
let content = fs::read_to_string(filename).unwrap();

// AFTER (helpful)
fn read_file_safely(filename: &str) -> Result<String, CliError> {
    if !Path::new(filename).exists() {
        return Err(CliError::FileNotFound(format!(
            "{}\n💡 Check the filename and path:\n  • Make sure the file exists\n  • Check for typos in the filename\n  • Use absolute path if in different directory", 
            filename
        )));
    }
    
    fs::read_to_string(filename).map_err(|e| {
        match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                CliError::PermissionDenied(format!(
                    "{}\n💡 Check file permissions:\n  • Ensure you have read access\n  • Try running with appropriate permissions", 
                    filename
                ))
            },
            _ => CliError::FileNotFound(format!("Cannot read '{}': {}", filename, e))
        }
    })
}
```

### 🔢 Better Parsing with Context

```rust
// BEFORE (terrible)
let number: i32 = first_line.parse().unwrap();

// AFTER (helpful)
fn parse_first_line(content: &str) -> Result<i32, CliError> {
    let lines: Vec<&str> = content.lines().collect();
    
    if lines.is_empty() {
        return Err(CliError::EmptyFile(
            "File is empty.\n💡 Please provide a file with at least one line containing a number.".to_string()
        ));
    }
    
    let first_line = lines[0].trim();
    
    first_line.parse::<i32>().map_err(|_| {
        CliError::ParseError(format!(
            "Cannot parse '{}' as a number.\n💡 Expected an integer like:\n  • 42\n  • -17\n  • 0", 
            first_line
        ))
    })
}
```

### 🎨 Colored Error Output

```rust
fn print_error(error: &CliError) {
    eprintln!("❌ {}", error);
    
    // Add specific suggestions based on error type
    match error {
        CliError::FileNotFound(_) => {
            eprintln!("📋 Common solutions:");
            eprintln!("  • Check spelling of filename");
            eprintln!("  • Verify file is in current directory");
            eprintln!("  • Use 'ls' or 'dir' to list available files");
        },
        CliError::ParseError(_) => {
            eprintln!("📋 Valid number formats:");
            eprintln!("  • Whole numbers: 42, -17, 0");
            eprintln!("  • No decimals or text allowed");
        },
        _ => {}
    }
}
```

### 🏗️ Main Function Pattern

```rust
fn main() {
    if let Err(error) = run() {
        print_error(&error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let args: Vec<String> = std::env::args().collect();
    let filename = validate_args(&args)?;
    let content = read_file_safely(&filename)?;
    let number = parse_first_line(&content)?;
    
    println!("✅ Successfully parsed number: {}", number);
    Ok(())
}
```

### 🧪 Testing Your Improvements

Try these scenarios to test your error messages:
```bash
# Test with no arguments
cargo run

# Test with missing file  
cargo run missing.txt

# Test with empty file
touch empty.txt && cargo run empty.txt

# Test with invalid content
echo "not a number" > invalid.txt && cargo run invalid.txt
```

### 🎯 Error Message Quality Check

Good error messages should:
- ✅ Start with what went wrong
- ✅ Include the problematic input
- ✅ Suggest how to fix it
- ✅ Provide examples when helpful
- ✅ Use appropriate emoji/symbols for visual scanning

### ⏱️ Still Struggling?

If you're still having trouble after **30+ minutes**, check Level 3 hints for complete implementations.

The goal is transforming cryptic panics into helpful guidance!