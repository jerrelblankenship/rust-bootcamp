# Calculator Project Solutions

## 🎯 How to Use This Solutions Directory

**IMPORTANT**: Only look at these solutions after attempting the exercises yourself! The learning happens when you struggle with compilation errors and figure out the fixes.

## 📁 Files in This Directory

- **`main.rs`** - Complete working calculator implementation
- **`README.md`** - This file with guidance and explanations

## 🔧 Running the Complete Solution

To see the finished calculator in action:

```bash
# Copy the solution over the broken starter (backup your work first!)
cp solutions/main.rs src/main.rs

# Build and run
cargo build
cargo run -- 5 + 3
cargo run -- --interactive
cargo test
```

## 🎓 Step-by-Step Solution Guide

### Step 1: Define the Operation Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Subtract, 
    Multiply,
    Divide,
}
```

**Key Learning**: Enums in Rust are powerful! The `#[derive(...)]` attributes automatically implement common traits.

### Step 2: Implement Operation Methods
```rust
impl Operation {
    fn symbol(&self) -> &str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*", 
            Operation::Divide => "/",
        }
    }
    
    fn from_str(s: &str) -> Result<Operation, CalculatorError> {
        match s {
            "+" | "add" => Ok(Operation::Add),
            "-" | "sub" => Ok(Operation::Subtract),
            "*" | "x" | "mul" => Ok(Operation::Multiply),
            "/" | "div" => Ok(Operation::Divide),
            _ => Err(CalculatorError::InvalidOperation(s.to_string())),
        }
    }
}
```

**Key Learning**: Multiple patterns in match arms (`"+" | "add"`), and explicit error handling with `Result`.

### Step 3: Define Expression Struct
```rust
#[derive(Debug, Clone)]
struct Expression {
    left: f64,
    operation: Operation,
    right: f64,
}
```

**Key Learning**: Structs hold related data together. Unlike C# classes, there's no inheritance.

### Step 4: Implement Expression Methods
```rust
impl Expression {
    fn new(left: f64, operation: Operation, right: f64) -> Self {
        Expression { left, operation, right }
    }
    
    fn calculate(&self) -> Result<f64, CalculatorError> {
        match self.operation {
            Operation::Add => Ok(self.left + self.right),
            Operation::Subtract => Ok(self.left - self.right), 
            Operation::Multiply => Ok(self.left * self.right),
            Operation::Divide => {
                if self.right == 0.0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    Ok(self.left / self.right)
                }
            }
        }
    }
}
```

**Key Learning**: Methods take `&self` (like C#'s `this`), and explicit error handling prevents division by zero crashes.

### Step 5: Define Error Types
```rust
#[derive(Debug, Clone, PartialEq)]
enum CalculatorError {
    InvalidInput(String),
    InvalidOperation(String), 
    ParseError(String),
    DivisionByZero,
}
```

**Key Learning**: Enums can carry data! This is more powerful than C# enums.

### Step 6: Implement Error Display
```rust
impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CalculatorError::InvalidOperation(op) => write!(f, "Invalid operation: '{}'", op),
            CalculatorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "Division by zero is not allowed"),
        }
    }
}
```

**Key Learning**: Implementing traits gives your types standard behavior. This enables nice error messages.

## 🔄 C# vs Rust Comparison

| Concept | C# | Rust |
|---------|-----|------|
| **Enums** | `enum { Add, Sub }` | `enum Operation { Add, Subtract }` |
| **Structs** | `class Expression { }` | `struct Expression { }` + `impl Expression { }` |
| **Errors** | `throw new Exception()` | `Err(CalculatorError::ParseError)` |
| **Null Safety** | NullReferenceException risk | `Option<T>` and `Result<T,E>` |
| **Methods** | `public void Method()` | `fn method(&self)` |
| **Constructors** | `new Expression()` | `Expression::new()` or struct literal |

## 🚨 Common Mistakes and Solutions

### Mistake 1: Forgetting `&self` in methods
```rust
// ❌ Wrong
fn display() -> String { ... }

// ✅ Correct  
fn display(&self) -> String { ... }
```

### Mistake 2: Not handling Results
```rust
// ❌ Wrong
let result = calculate(); // Ignores potential errors

// ✅ Correct
match calculate() {
    Ok(result) => println!("Result: {}", result),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Mistake 3: Forgetting to derive Debug
```rust
// ❌ Hard to debug
enum Operation { Add }

// ✅ Can print for debugging
#[derive(Debug)]
enum Operation { Add }
```

## 🎯 Extension Challenges

Once you have the basic calculator working, try these:

1. **Add More Operations**: Power, square root, modulo
2. **Expression Parsing**: Support `"5 + 3 * 2"`  
3. **Variables**: Store and recall values
4. **History Commands**: Show previous calculations
5. **Configuration**: Read settings from a file

## 📚 Key Rust Concepts Demonstrated

1. **Ownership**: Values are moved unless explicitly cloned
2. **Pattern Matching**: Powerful `match` expressions  
3. **Error Handling**: Explicit with `Result<T,E>`
4. **Traits**: Implement standard behavior (`Display`, `Debug`)
5. **Modules**: Organize code with `impl` blocks
6. **Memory Safety**: No null pointers or buffer overflows

## 💡 Pro Tips

1. **Start Small**: Get basic addition working first
2. **Read Compiler Errors**: Rust's error messages are very helpful
3. **Use `cargo check`**: Faster than full compilation for finding errors
4. **Leverage C# Knowledge**: Many concepts translate well
5. **Don't Fight the Borrow Checker**: Learn ownership patterns

## 🔗 Related Resources

- [Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

**Remember**: The goal isn't to memorize syntax, but to understand Rust's approach to safety and performance. Each compilation error you fix teaches you something valuable about Rust's guarantees!
