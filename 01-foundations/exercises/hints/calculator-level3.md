# Calculator Project Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through the earlier levels but need to see the complete, working calculator. Here's the full implementation with detailed explanations.

## 📝 Complete main.rs Implementation

```rust
// Calculator - A simple command-line calculator in Rust
//
// Usage: calculator <number> <operation> <number>
// Example: calculator 5 + 3

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check if we have the right number of arguments
    if args.len() != 4 {
        eprintln!("Usage: calculator <number> <operation> <number>");
        eprintln!("Example: calculator 5 + 3");
        eprintln!("Supported operations: +, -, *, /");
        std::process::exit(1);
    }
    
    // Parse the arguments
    let left = parse_number(&args[1]);
    let operator = &args[2];
    let right = parse_number(&args[3]);
    
    // Calculate the result
    let result = calculate(left, operator, right);
    
    // Print the result nicely
    println!("{} {} {} = {}", left, operator, right, result);
}

/// Parse a string to f64 with helpful error handling
fn parse_number(s: &str) -> f64 {
    s.parse().unwrap_or_else(|_| {
        eprintln!("Error: '{}' is not a valid number", s);
        eprintln!("Please enter a valid number (e.g., 42, 3.14, -5)");
        std::process::exit(1);
    })
}

/// Calculate the result of an arithmetic operation
fn calculate(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => {
            if right == 0.0 {
                eprintln!("Error: Division by zero is not allowed");
                std::process::exit(1);
            }
            left / right
        },
        _ => {
            eprintln!("Error: Unknown operator '{}'", operator);
            eprintln!("Supported operations: +, -, *, /");
            std::process::exit(1);
        }
    }
}

// Optional: Helper functions for each operation
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        eprintln!("Error: Division by zero is not allowed");
        std::process::exit(1);
    }
    a / b
}

// Alternative implementation using helper functions
fn calculate_with_helpers(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => add(left, right),
        "-" => subtract(left, right),
        "*" => multiply(left, right),
        "/" => divide(left, right),
        _ => {
            eprintln!("Error: Unknown operator '{}'", operator);
            eprintln!("Supported operations: +, -, *, /");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        let result = calculate(5.0, "+", 3.0);
        assert_eq!(result, 8.0);
    }
    
    #[test]
    fn test_subtraction() {
        let result = calculate(10.0, "-", 4.0);
        assert_eq!(result, 6.0);
    }
    
    #[test]
    fn test_multiplication() {
        let result = calculate(6.0, "*", 7.0);
        assert_eq!(result, 42.0);
    }
    
    #[test]
    fn test_division() {
        let result = calculate(15.0, "/", 3.0);
        assert_eq!(result, 5.0);
    }
    
    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), 42.0);
        assert_eq!(parse_number("3.14"), 3.14);
        assert_eq!(parse_number("-5"), -5.0);
    }
    
    #[test]
    fn test_helper_functions() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(subtract(10.0, 4.0), 6.0);
        assert_eq!(multiply(3.0, 4.0), 12.0);
        assert_eq!(divide(8.0, 2.0), 4.0);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero() {
        divide(10.0, 0.0);
    }
}
```

## 🔧 Enhanced Version with Better Error Handling

For a more robust implementation, here's a version with Result types:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Validate arguments
    if args.len() != 4 {
        eprintln!("Error: Expected 3 arguments, got {}", args.len() - 1);
        eprintln!("Usage: calculator <number> <operation> <number>");
        eprintln!("Example: calculator 5 + 3");
        std::process::exit(1);
    }
    
    // Parse and calculate with error handling
    match run_calculator(&args[1], &args[2], &args[3]) {
        Ok(result) => println!("{} {} {} = {}", args[1], args[2], args[3], result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Main calculator logic with Result error handling
fn run_calculator(left_str: &str, operator: &str, right_str: &str) -> Result<f64, String> {
    let left = parse_number_safe(left_str)?;
    let right = parse_number_safe(right_str)?;
    let result = calculate_safe(left, operator, right)?;
    Ok(result)
}

/// Safe number parsing that returns Result
fn parse_number_safe(s: &str) -> Result<f64, String> {
    s.parse()
        .map_err(|_| format!("'{}' is not a valid number", s))
}

/// Safe calculation that returns Result
fn calculate_safe(left: f64, operator: &str, right: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0.0 {
                Err("Division by zero is not allowed".to_string())
            } else {
                Ok(left / right)
            }
        },
        _ => Err(format!("Unknown operator '{}'. Supported: +, -, *, /", operator))
    }
}
```

## 🎓 Complete Code Walkthrough

### 1. Argument Processing
```rust
let args: Vec<String> = env::args().collect();
```
- `env::args()` returns an iterator over command line arguments
- `.collect()` converts iterator to `Vec<String>`
- First argument (`args[0]`) is always the program name

### 2. Input Validation
```rust
if args.len() != 4 {
    eprintln!("Usage: calculator <number> <operation> <number>");
    std::process::exit(1);
}
```
- Checks for exactly 4 arguments (program + 3 inputs)
- `eprintln!` prints to stderr instead of stdout
- `std::process::exit(1)` terminates with error code

### 3. String to Number Conversion
```rust
fn parse_number(s: &str) -> f64 {
    s.parse().unwrap_or_else(|_| {
        eprintln!("Error: '{}' is not a valid number", s);
        std::process::exit(1);
    })
}
```
- `.parse()` attempts conversion based on target type
- `unwrap_or_else()` handles parse errors gracefully
- Exits program with helpful error message on failure

### 4. Operation Logic
```rust
fn calculate(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        // ... other operations
        _ => {
            // Handle unknown operators
        }
    }
}
```
- Pattern matching on string slices
- Each arm performs the corresponding operation
- Wildcard `_` catches invalid operators

### 5. Output Formatting
```rust
println!("{} {} {} = {}", left, operator, right, result);
```
- Shows the complete calculation for clarity
- Uses positional formatting with `{}`

## 🚀 Testing Your Implementation

### Manual Testing
```bash
# Build the project
cargo build

# Test basic operations
cargo run 5 + 3
cargo run 10.5 - 2.3
cargo run 6 * 7
cargo run 15 / 3

# Test edge cases
cargo run -5 + 3      # Negative numbers
cargo run 10 / 0      # Division by zero
cargo run 5 ^ 3       # Invalid operator
cargo run abc + 3     # Invalid number
cargo run 5 +         # Missing argument
```

### Automated Testing
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --tests

# Run specific test
cargo test test_addition

# Run tests with output
cargo test -- --nocapture
```

## 🔧 Possible Extensions

### 1. Additional Operations
```rust
match operator {
    "+" => left + right,
    "-" => left - right,
    "*" => left * right,
    "/" => divide_safe(left, right),
    "%" => left % right,           // Modulo
    "^" | "**" => left.powf(right), // Power
    _ => handle_unknown_operator(operator)
}
```

### 2. Multiple Number Support
```rust
// Support: calculator 1 + 2 + 3 + 4
fn calculate_multiple(args: &[String]) -> Result<f64, String> {
    // Parse alternating numbers and operators
    // Apply operations left-to-right
}
```

### 3. Parentheses Support
```rust
// Support: calculator "( 2 + 3 ) * 4"
fn parse_expression(expr: &str) -> Result<f64, String> {
    // Implement expression parser
    // Handle operator precedence
}
```

## 🎯 Key Learning Achievements

By completing this calculator, you've mastered:

### ✅ **Rust Fundamentals**
- Function definitions with parameters and return types
- Pattern matching with `match` expressions
- String parsing and type conversion
- Error handling with `panic!` and `Result`

### ✅ **Command Line Programming**
- Accessing command line arguments
- Input validation and user feedback
- Exit codes and error reporting
- Integration testing with external tools

### ✅ **Real-World Patterns**
- Separation of concerns (parsing, calculation, output)
- Error handling strategies
- User-friendly error messages
- Testing both units and integration

### ✅ **C# to Rust Translation**
You've seen how C# concepts map to Rust:
- Method parameters → Function parameters with explicit types
- Exception handling → `Result` types and `panic!`
- String operations → `&str` borrowing and `String` ownership
- Console applications → `println!` and `eprintln!`

## 🏆 Next Steps

**Congratulations!** You've built a complete, working Rust application. You've mastered:
- Variables and types
- Functions and control flow
- Pattern matching
- Basic error handling
- Real-world application structure

**You're ready for Module 02: Ownership and Borrowing!** 🦀

The foundation concepts you've learned here will be essential as you dive deeper into Rust's unique ownership system.