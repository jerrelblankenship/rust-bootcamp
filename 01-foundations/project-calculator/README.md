# CLI Calculator Project

Build a command-line calculator that demonstrates all the concepts learned in Module 01: Foundations.

## 🎯 Project Goals

Create a calculator that:
- Parses command-line arguments
- Supports basic arithmetic operations (+, -, *, /)
- Handles errors gracefully
- Uses structs for data organization
- Implements enums for operation types
- Demonstrates Rust best practices

## 📋 Requirements

### Core Features
1. **Command-line parsing**: Accept expressions like `calc 5 + 3`
2. **Operations**: Addition, subtraction, multiplication, division
3. **Error handling**: Invalid input, division by zero
4. **Floating-point**: Support decimal numbers
5. **Help text**: Display usage when requested

### Stretch Goals
1. **Parentheses**: Support `(5 + 3) * 2`
2. **Variables**: Store and recall values
3. **History**: Show previous calculations
4. **Advanced operations**: Power, square root, modulo

## 🏗️ Project Structure

```
project-calculator/
├── Cargo.toml
├── src/
│   ├── main.rs         # Entry point
│   ├── parser.rs       # Input parsing
│   ├── calculator.rs   # Calculation logic
│   └── error.rs        # Error types
└── tests/
    └── integration.rs  # Integration tests
```

## 🚀 Implementation Guide

### Step 1: Set Up Project

```bash
cargo new calculator --bin
cd calculator
```

### Step 2: Define Data Types

```rust
// src/main.rs

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Expression {
    left: f64,
    operation: Operation,
    right: f64,
}

#[derive(Debug)]
enum CalculatorError {
    InvalidInput(String),
    DivisionByZero,
    ParseError(String),
}

type Result<T> = std::result::Result<T, CalculatorError>;
```

### Step 3: Implement Parser

```rust
// src/parser.rs

use crate::{Expression, Operation, CalculatorError, Result};

pub fn parse_args(args: &[String]) -> Result<Expression> {
    if args.len() != 4 {
        return Err(CalculatorError::InvalidInput(
            "Usage: calc <number> <operation> <number>".to_string()
        ));
    }
    
    let left = parse_number(&args[1])?;
    let operation = parse_operation(&args[2])?;
    let right = parse_number(&args[3])?;
    
    Ok(Expression { left, operation, right })
}

fn parse_number(s: &str) -> Result<f64> {
    s.parse::<f64>()
        .map_err(|_| CalculatorError::ParseError(
            format!("'{}' is not a valid number", s)
        ))
}

fn parse_operation(s: &str) -> Result<Operation> {
    match s {
        "+" => Ok(Operation::Add),
        "-" => Ok(Operation::Subtract),
        "*" | "x" => Ok(Operation::Multiply),
        "/" => Ok(Operation::Divide),
        _ => Err(CalculatorError::InvalidInput(
            format!("'{}' is not a valid operation", s)
        )),
    }
}
```

### Step 4: Implement Calculator

```rust
// src/calculator.rs

use crate::{Expression, Operation, CalculatorError, Result};

pub fn calculate(expr: Expression) -> Result<f64> {
    match expr.operation {
        Operation::Add => Ok(expr.left + expr.right),
        Operation::Subtract => Ok(expr.left - expr.right),
        Operation::Multiply => Ok(expr.left * expr.right),
        Operation::Divide => {
            if expr.right == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(expr.left / expr.right)
            }
        }
    }
}

impl Operation {
    fn symbol(&self) -> &str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        }
    }
}

impl Expression {
    pub fn display(&self) -> String {
        format!("{} {} {} = ", 
            self.left, 
            self.operation.symbol(), 
            self.right
        )
    }
}
```

### Step 5: Main Application

```rust
// src/main.rs

mod parser;
mod calculator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }
    
    match parser::parse_args(&args) {
        Ok(expression) => {
            match calculator::calculate(expression.clone()) {
                Ok(result) => {
                    println!("{}{}", expression.display(), result);
                }
                Err(e) => {
                    eprintln!("Calculation error: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            eprintln!("\nUse 'calc --help' for usage information");
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("Calculator - A simple command-line calculator");
    println!("\nUsage:");
    println!("  calc <number> <operation> <number>");
    println!("\nOperations:");
    println!("  +  Addition");
    println!("  -  Subtraction");
    println!("  *  Multiplication (also 'x')");
    println!("  /  Division");
    println!("\nExamples:");
    println!("  calc 5 + 3");
    println!("  calc 10.5 - 2.5");
    println!("  calc 4 x 5");
    println!("  calc 20 / 4");
}
```

### Step 6: Error Display

```rust
// src/error.rs

use std::fmt;

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
            CalculatorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for CalculatorError {}
```

## 🧪 Testing

Create integration tests:

```rust
// tests/integration.rs

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_addition() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["5", "+", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5 + 3 = 8"));
}

#[test]
fn test_division_by_zero() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["10", "/", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Division by zero"));
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}
```

Add to Cargo.toml:
```toml
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
```

## 🚀 Running the Calculator

```bash
# Build the project
cargo build --release

# Run with cargo
cargo run -- 5 + 3

# Or use the binary directly
./target/release/calculator 10 / 2

# Get help
./target/release/calculator --help
```

## 🌟 Extension Ideas

### 1. Expression Parser
Support full expressions: `calc "5 + 3 * 2"`

### 2. Interactive Mode
```bash
calc -i
> 5 + 3
8
> ans * 2
16
> exit
```

### 3. History Feature
```rust
struct Calculator {
    history: Vec<(Expression, f64)>,
}
```

### 4. Configuration File
```toml
# ~/.calc/config.toml
[display]
precision = 2
scientific_notation = false

[constants]
pi = 3.14159
e = 2.71828
```

### 5. Advanced Operations
```rust
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    SquareRoot,
    Modulo,
}
```

## 📚 What You've Learned

Through this project, you've applied:

1. **Project Structure**: Organizing code into modules
2. **Enums**: Representing operations as variants
3. **Structs**: Organizing expression data
4. **Error Handling**: Custom error types with Result
5. **Pattern Matching**: Handling different operations
6. **Command-Line Parsing**: Working with env::args()
7. **Testing**: Integration tests for CLI applications

## 🎯 Challenge Yourself

1. Add support for multiple operations: `calc 5 + 3 - 1`
2. Implement a tokenizer and parser for complex expressions
3. Add unit conversions: `calc 5 km to miles`
4. Create a GUI version using a Rust GUI framework
5. Add mathematical functions: sin, cos, log, etc.

---

Congratulations! You've built a functional CLI calculator in Rust. This project demonstrates fundamental Rust concepts and provides a foundation for more complex applications.

Next Module: [02 - Ownership and Borrowing](../../02-ownership-and-borrowing/README.md) →
