# Calculator Project Hints - Level 2 (More Specific Guidance)

## 🎯 Specific Implementation Solutions

You've tried Level 1 hints but need more concrete guidance. Here are specific solutions for each part of the calculator implementation.

## 🔧 Fix 1: Parse Number Function

**Problem**: Function doesn't exist and needs to handle string-to-number conversion

**Specific Solution**:
```rust
fn parse_number(s: &str) -> f64 {
    s.parse().expect("Failed to parse number")
}
```

**Explanation**:
- `.parse()` method attempts to convert string to target type
- Rust infers the type from the return type annotation (f64)
- `.expect()` panics with helpful message if parsing fails
- This handles both integer and floating-point inputs

## 🔧 Fix 2: Calculate Function Implementation

**Problem**: Match arms contain `todo!()` macros

**Specific Solutions**:
```rust
fn calculate(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => {
            if right == 0.0 {
                panic!("Cannot divide by zero");
            }
            left / right
        },
        _ => panic!("Unknown operator: {}", operator),
    }
}
```

**Key Points**:
- Each match arm performs the corresponding arithmetic operation
- Division includes zero-check for safety
- Panic with descriptive messages for error cases
- `_` wildcard catches unknown operators

## 🔧 Fix 3: Helper Functions (Optional)

**Problem**: Helper functions contain `todo!()` macros

**Specific Solutions**:
```rust
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
        panic!("Division by zero");
    }
    a / b
}
```

**Alternative**: Use these in the calculate function:
```rust
fn calculate(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => add(left, right),
        "-" => subtract(left, right),
        "*" => multiply(left, right),
        "/" => divide(left, right),
        _ => panic!("Unknown operator: {}", operator),
    }
}
```

## 🔧 Error Handling Strategies

### Strategy 1: Panic on Error (Simple)
```rust
fn parse_number(s: &str) -> f64 {
    s.parse().expect("Invalid number format")
}

fn calculate(left: f64, operator: &str, right: f64) -> f64 {
    // ... with panic! for errors
}
```

### Strategy 2: Better Error Messages
```rust
fn parse_number(s: &str) -> f64 {
    s.parse().unwrap_or_else(|_| {
        eprintln!("Error: '{}' is not a valid number", s);
        std::process::exit(1);
    })
}
```

### Strategy 3: Early Validation in Main
```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        eprintln!("Usage: calculator <number> <operation> <number>");
        eprintln!("Example: calculator 5 + 3");
        std::process::exit(1);
    }
    
    // Continue with parsing...
}
```

## 🔧 Testing Your Implementation

### Manual Testing Commands
```bash
# Basic operations
cargo run 5 + 3        # Should output: 5 + 3 = 8
cargo run 10 - 4       # Should output: 10 - 4 = 6
cargo run 6 * 7        # Should output: 6 * 7 = 42
cargo run 15 / 3       # Should output: 15 / 3 = 5

# Edge cases
cargo run 3.14 + 2.86  # Should work with decimals
cargo run -5 + 3       # Should work with negatives
cargo run 10 / 0       # Should panic/error gracefully
```

### Running Tests
```bash
cargo test              # Run unit tests
cargo test --tests     # Run integration tests
```

## 🔧 Common Implementation Issues

### Issue 1: Type Confusion
```rust
// ❌ Wrong: Integer parsing
let left: i32 = args[1].parse().expect("Invalid number");

// ✅ Correct: Float parsing for all numbers
let left: f64 = args[1].parse().expect("Invalid number");
```

### Issue 2: String vs &str
```rust
// ❌ Wrong: Taking ownership
fn calculate(left: f64, operator: String, right: f64) -> f64 {

// ✅ Correct: Borrowing
fn calculate(left: f64, operator: &str, right: f64) -> f64 {
```

### Issue 3: Printing Format
```rust
// ❌ Wrong: No context
println!("{}", result);

// ✅ Correct: Show the calculation
println!("{} {} {} = {}", left, operator, right, result);
```

## 🎓 Understanding the Flow

### Program Structure
1. **Parse arguments** → Convert command line to usable values
2. **Validate input** → Ensure we have correct number of arguments
3. **Convert strings** → Parse strings to numbers
4. **Calculate result** → Perform the arithmetic operation
5. **Display output** → Show the calculation and result

### Data Flow
```
Command Line: ["calculator", "5", "+", "3"]
       ↓
args[1] = "5" → parse_number() → 5.0
args[2] = "+" → operator
args[3] = "3" → parse_number() → 3.0
       ↓
calculate(5.0, "+", 3.0) → 8.0
       ↓
println!("5 + 3 = 8")
```

## 🔍 Debugging Tips

### Check Arguments
```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args); // Debug print
    
    // Continue with your code...
}
```

### Test Parsing Separately
```rust
fn main() {
    let test_num = parse_number("42.5");
    println!("Parsed: {}", test_num);
}
```

### Test Calculation Separately
```rust
fn main() {
    let result = calculate(10.0, "+", 5.0);
    println!("Result: {}", result);
}
```

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](calculator-level3.md) for full solutions.

## 💡 Key Learning Points

- **Function signatures matter**: Parameter types must be correct
- **Error handling**: Use `expect()` for descriptive panics
- **Pattern matching**: `match` is powerful for operation selection
- **String parsing**: `.parse()` method with type inference
- **Command line args**: `env::args()` provides program arguments

Keep going - you're building a real CLI application! 🦀