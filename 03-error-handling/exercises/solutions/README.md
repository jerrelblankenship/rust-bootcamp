# Module 03 Error Handling - Solutions

## 🎯 How to Use This Solutions Directory

**IMPORTANT**: Only look at these solutions after attempting the broken code exercises yourself! The learning happens when you struggle with compilation errors and figure out how to fix them.

## 📁 Files in This Directory

- **`ex01-option-basics.rs`** - Complete Option<T> exercise solutions
- **`ex02-result-chain.rs`** - Complete Result chaining and ? operator solutions
- **`ex03-error-types.rs`** - Complete custom error type solutions  
- **`ex04-conversions.rs`** - Complete error conversion solutions
- **`README.md`** - This file with guidance and explanations

## 🔧 Running the Complete Solutions

To see the finished exercises in action:

```bash
# Copy a solution over the broken starter (backup your work first!)
cp solutions/ex01-option-basics.rs ex01-option-basics.rs

# Compile and run
rustc ex01-option-basics.rs && ./ex01-option-basics
rustc ex02-result-chain.rs && ./ex02-result-chain
```

## 🎓 Key Learning Points

### Exercise 1: Option<T> Basics
**C# Comparison**: Option<T> replaces nullable types and eliminates NullReferenceException

```rust
// C# - NullReferenceException risk
string name = user?.Name ?? "Unknown";

// Rust - Compile-time null safety
let name = user.map(|u| u.name).unwrap_or_else(|| "Unknown".to_string());
```

**Key Methods**:
- `.map()` - Transform the value inside Some
- `.and_then()` - Chain operations that return Option
- `.unwrap_or()` - Provide default value
- `.filter_map()` - Filter and transform in iterators

### Exercise 2: Result<T, E> and ? Operator
**C# Comparison**: Result<T, E> replaces exception handling

```rust
// C# - Exception handling
try {
    var result = int.Parse(input);
    return result * 2;
} catch (FormatException e) {
    return -1; // or throw
}

// Rust - Error as values
fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
    let num = input.parse::<i32>()?;  // ? operator for early return
    Ok(num * 2)
}
```

**Key Concepts**:
- `?` operator propagates errors up the call stack
- `.map_err()` transforms error types
- `.and_then()` chains fallible operations
- Errors are explicit in function signatures

### Exercise 3: Custom Error Types
**Best Practices**:
```rust
#[derive(Debug)]
enum MyError {
    InvalidInput { field: String },
    NetworkError { url: String, status: u16 },
    ParseError(ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::InvalidInput { field } => write!(f, "Invalid field: {}", field),
            MyError::NetworkError { url, status } => write!(f, "Network error: {} ({})", url, status),
            MyError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for MyError {}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::ParseError(err)
    }
}
```

## 🔄 C# vs Rust Error Handling Comparison

| Aspect | C# | Rust |
|--------|-----|------|
| **Null Safety** | `string?` (nullable) | `Option<String>` |
| **Error Handling** | `try-catch` blocks | `Result<T, E>` types |
| **Error Propagation** | `throw` keyword | `?` operator |
| **Multiple Error Types** | Exception hierarchy | `From` trait conversions |
| **Performance** | Exception overhead | Zero-cost abstractions |
| **Compile-time Safety** | Runtime null checks | Compile-time guarantees |

## 🚨 Common Mistakes and Solutions

### Mistake 1: Fighting the type system
```rust
// ❌ Wrong - unwrap() can panic
let value = might_fail().unwrap();

// ✅ Correct - handle both cases
match might_fail() {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}
```

### Mistake 2: Not leveraging ? operator
```rust
// ❌ Verbose nested matching
fn parse_two_numbers(a: &str, b: &str) -> Result<(i32, i32), ParseIntError> {
    match a.parse::<i32>() {
        Ok(num_a) => {
            match b.parse::<i32>() {
                Ok(num_b) => Ok((num_a, num_b)),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// ✅ Clean with ? operator
fn parse_two_numbers(a: &str, b: &str) -> Result<(i32, i32), ParseIntError> {
    let num_a = a.parse::<i32>()?;
    let num_b = b.parse::<i32>()?;
    Ok((num_a, num_b))
}
```

### Mistake 3: Not using From trait for error conversion
```rust
// ❌ Manual error conversion
fn process(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(format!("Parse error: {}", e)),
    }
}

// ✅ Automatic conversion with From trait
#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

fn process(input: &str) -> Result<i32, MyError> {
    let n = input.parse::<i32>()?;  // Auto-converts ParseIntError
    Ok(n * 2)
}
```

## 💡 Pro Tips

1. **Start with Option and Result**: Master these before custom error types
2. **Use ? operator liberally**: It makes error handling much cleaner
3. **Implement From traits**: Enable automatic error conversions
4. **Design error hierarchies**: Group related errors in enums
5. **Provide context**: Use error messages that help debugging
6. **Don't fear errors**: They're just values, not exceptions

## 🎯 Advanced Patterns

### Error Context Chain
```rust
#[derive(Debug)]
struct ErrorWithContext {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ErrorWithContext {
    fn with_context<E>(source: E, message: String) -> Self 
    where E: std::error::Error + Send + Sync + 'static 
    {
        Self {
            message,
            source: Some(Box::new(source)),
        }
    }
}
```

### Retry Logic
```rust
fn retry_with_backoff<F, T, E>(mut operation: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for attempt in 1..=max_attempts {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts => return Err(e),
            Err(_) => {
                // Wait before retry (exponential backoff)
                std::thread::sleep(std::time::Duration::from_millis(100 * attempt as u64));
            }
        }
    }
    unreachable!()
}
```

## 🔗 Related Resources

- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [anyhow crate](https://docs.rs/anyhow/) - Flexible error handling
- [thiserror crate](https://docs.rs/thiserror/) - Derive error traits

---

**Remember**: The goal isn't to avoid errors, but to handle them gracefully and explicitly. Rust's error handling makes bugs visible at compile time rather than runtime crashes!
