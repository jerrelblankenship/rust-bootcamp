# Module 03 Debugging Checklist - Error Handling

## 🔍 Error Handling Debugging Approach

Error handling in Rust is fundamentally different from C#'s exception model. Follow this systematic approach:

### **Step 1: Identify Error Handling Pattern**

**Common error handling patterns:**
- [ ] "cannot find type `Option`" → Missing Option handling
- [ ] "cannot find type `Result`" → Missing Result handling
- [ ] "expected `()`, found enum `Option`" → Need to handle Option value
- [ ] "the `?` operator can only be used" → Incorrect ? operator usage
- [ ] "trait bound not satisfied" → Missing error conversion

### **Step 2: C# Exception Model vs Rust Error Values**

**Mental model translation:**
```csharp
// C# - Exceptions thrown at runtime
try {
    var result = RiskyOperation();
    Console.WriteLine($"Success: {result}");
} catch (ArgumentException ex) {
    Console.WriteLine($"Argument error: {ex.Message}");
} catch (Exception ex) {
    Console.WriteLine($"General error: {ex.Message}");
}
```

```rust
// Rust - Errors are values, handled at compile time
match risky_operation() {
    Ok(result) => println!("Success: {}", result),
    Err(e) => match e {
        MyError::ArgumentError(msg) => println!("Argument error: {}", msg),
        MyError::GeneralError(msg) => println!("General error: {}", msg),
    }
}

// Or using ? operator for propagation
fn caller() -> Result<(), MyError> {
    let result = risky_operation()?;  // Auto-propagates errors
    println!("Success: {}", result);
    Ok(())
}
```

### **Step 3: Option<T> Debugging Patterns**

**"expected `T`, found enum `Option<T>`"**
```rust
// Problem:
let maybe_number = Some(42);
let doubled = maybe_number * 2;  // ERROR: Can't multiply Option

// Solutions:
// Solution 1: Pattern matching
let doubled = match maybe_number {
    Some(n) => n * 2,
    None => 0,  // Default value
};

// Solution 2: unwrap (risky - panics if None)
let doubled = maybe_number.unwrap() * 2;

// Solution 3: unwrap_or (safe with default)
let doubled = maybe_number.unwrap_or(0) * 2;

// Solution 4: map (functional approach)
let doubled = maybe_number.map(|n| n * 2).unwrap_or(0);
```

### **Step 4: Result<T, E> Debugging Patterns**

**"expected `T`, found enum `Result<T, E>`"**
```rust
// Problem:
let file_content = std::fs::read_to_string("file.txt");
let length = file_content.len();  // ERROR: Result doesn't have len()

// Solutions:
// Solution 1: Pattern matching
let length = match std::fs::read_to_string("file.txt") {
    Ok(content) => content.len(),
    Err(e) => {
        eprintln!("Error reading file: {}", e);
        0
    }
};

// Solution 2: unwrap (panics on error - use carefully)
let content = std::fs::read_to_string("file.txt").unwrap();
let length = content.len();

// Solution 3: ? operator (propagates error)
fn read_file_length() -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content.len())
}
```

### **Step 5: Error Propagation with ? Operator**

**"the `?` operator can only be used in a function that returns `Result`"**
```rust
// Problem:
fn main() {
    let content = std::fs::read_to_string("file.txt")?;  // ERROR
}

// Solutions:
// Solution 1: Change main to return Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    println!("{}", content);
    Ok(())
}

// Solution 2: Handle error in main
fn main() {
    match std::fs::read_to_string("file.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Solution 3: Extract to separate function
fn read_and_print() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    println!("{}", content);
    Ok(())
}

fn main() {
    if let Err(e) = read_and_print() {
        eprintln!("Error: {}", e);
    }
}
```

### **Step 6: Custom Error Types**

**"trait bound `Error` is not satisfied"**
```rust
// Problem: Custom error doesn't implement required traits
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(String),
}

// Solution: Implement required traits
use std::fmt;

#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}

// Automatic conversion from other error types
impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError(error)
    }
}
```

## 🚨 Error Handling Debugging Workflow

### **1. Identify the Error Category**

**Check the error message pattern:**
- [ ] **Option errors**: "expected `T`, found `Option<T>`"
- [ ] **Result errors**: "expected `T`, found `Result<T, E>`"
- [ ] **? operator errors**: "can only be used in functions that return `Result`"
- [ ] **Trait errors**: "trait bound not satisfied"
- [ ] **Conversion errors**: "cannot convert"

### **2. Choose Handling Strategy**

**For Option<T>:**
- [ ] **Default value**: `option.unwrap_or(default)`
- [ ] **Transform**: `option.map(|x| transform(x))`
- [ ] **Chain operations**: `option.and_then(|x| other_option(x))`
- [ ] **Pattern match**: Full control over Some/None cases

**For Result<T, E>:**
- [ ] **Propagate**: Use `?` operator to bubble up errors
- [ ] **Handle locally**: Use `match` or `if let`
- [ ] **Provide fallback**: Use `unwrap_or_else`
- [ ] **Transform errors**: Use `map_err` to convert error types

### **3. Error Design Checklist**

**When creating custom errors:**
- [ ] Implement `Debug` trait (usually derive)
- [ ] Implement `Display` trait for user-friendly messages
- [ ] Implement `std::error::Error` trait
- [ ] Implement `From` traits for automatic conversions
- [ ] Consider using `thiserror` crate for easier implementation

### **4. Testing Error Handling**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_case() {
        let result = my_function(valid_input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_case() {
        let result = my_function(invalid_input);
        assert!(result.is_err());
        // Test specific error type
        match result {
            Err(MyError::ParseError(_)) => (), // Expected
            _ => panic!("Wrong error type"),
        }
    }
}
```

## 🎯 Error Handling Best Practices

### **Decision Tree for Error Handling:**

**1. Can this operation fail?**
- Yes → Use `Result<T, E>`
- Unlikely but possible → Use `Option<T>`
- Never → Use plain type `T`

**2. Should errors propagate up?**
- Yes → Use `?` operator or return `Result`
- No → Handle with `match` or `unwrap_or`

**3. What should happen on error?**
- Retry → Implement retry logic
- Default value → Use `unwrap_or`
- Different algorithm → Use `or_else`
- Abort → Use `expect` with good message

### **Error Message Quality:**
```rust
// Bad: Generic panic
let file = File::open("config.txt").unwrap();

// Good: Descriptive expect
let file = File::open("config.txt")
    .expect("Failed to open config.txt - ensure file exists and is readable");

// Better: Proper error handling
let file = File::open("config.txt")
    .map_err(|e| format!("Cannot open config file 'config.txt': {}", e))?;
```

## 🔧 Debugging Tools

```bash
# Check error handling without full compilation
cargo check

# Run tests with error output
cargo test -- --nocapture

# Show error chain with backtrace
RUST_BACKTRACE=1 cargo run

# Use clippy for error handling suggestions
cargo clippy
```

## 📚 Learning Mindset

**Remember:**
- **Errors are data, not exceptions** - They must be explicitly handled
- **Compiler enforces error handling** - Can't ignore like C# exceptions
- **? operator is powerful** - Learn to use it for clean error propagation
- **Start simple** - Use `unwrap()` during prototyping, handle properly later

**Mental model shift from C#:**
- In C#: "Try operation, catch if it fails"
- In Rust: "Operation returns success or error, decide what to do"

---

**Ready to build robust error handling?** Work through the exercises and build production-quality error management!
