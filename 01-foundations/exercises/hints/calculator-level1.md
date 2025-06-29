# Calculator Project Hints - Level 1 (Gentle Nudges)

## 🔍 Understanding the Calculator Project

You're working on a real CLI calculator that takes command line arguments like `cargo run 5 + 3`. This project applies everything from Module 01 in a practical context.

## 💡 General Approach

1. **Follow the compilation errors** - Fix one at a time
2. **Start simple** - Get basic functionality working first
3. **Use the tests** - They show exactly what each function should do
4. **Think C# first** - Then translate to Rust syntax

## 🎯 Phase-by-Phase Hints

### Phase 1: Parse Number Function

**Error**: `cannot find function 'parse_number'`

**C# Context**: In C#, you'd use `double.Parse(str)` or `double.TryParse()`.

**Rust Difference**: Rust uses `.parse()` method on strings.

**Gentle Hints**:
- Look at the test: `assert_eq!(parse_number("42"), 42.0);`
- The function should convert a string to f64
- Use `.parse()` method on the string parameter
- Use `.expect()` to handle parsing failures with a helpful message

**Questions to guide you**:
- What type should this function return?
- What happens if the string isn't a valid number?
- How can you provide a helpful error message?

### Phase 2: Calculate Function

**Error**: Missing match arm implementations

**C# Context**: You'd use a switch statement with cases for each operator.

**Rust Difference**: `match` is more powerful and must be exhaustive.

**Gentle Hints**:
- The function signature is already correct: `fn calculate(left: f64, operator: &str, right: f64) -> f64`
- Replace each `todo!()` with the actual arithmetic operation
- Addition in Rust is the same as C#: `left + right`
- The `_` case handles unknown operators (already implemented)

**Questions to guide you**:
- What should `"+" => todo!()` become?
- How do you add two f64 numbers in Rust?
- What about subtraction, multiplication, and division?

### Phase 3: Helper Functions (Optional)

**Error**: Functions contain `todo!()` macros

**C# Context**: Simple arithmetic methods like `public double Add(double a, double b)`.

**Rust Equivalent**: `fn add(a: f64, b: f64) -> f64`

**Gentle Hints**:
- These are straightforward arithmetic operations
- Think about what each function name suggests
- Consider edge cases (like division by zero)

## 🔧 Step-by-Step Debugging

### Step 1: Implement parse_number
```bash
cargo build
# Look for: error: cannot find function `parse_number`
```

**What to fix**:
- Replace `todo!("Parse string to number")` with actual parsing
- Use the pattern from the hint comments

### Step 2: Implement calculate operations
```bash
cargo build
# Look for: error: not yet implemented: Implement addition
```

**What to fix**:
- Replace each `todo!()` in the match arms
- Use basic arithmetic operators: `+`, `-`, `*`, `/`

### Step 3: Test your work
```bash
cargo test
# See which tests pass/fail
```

### Step 4: Try the CLI
```bash
cargo run 5 + 3
# Should output: 5 + 3 = 8
```

## 🎓 Key Learning Concepts

### Function Definition Pattern
**C# vs Rust comparison**:
```csharp
// C#
public double Add(double a, double b) {
    return a + b;
}
```

```rust
// Rust
fn add(a: f64, b: f64) -> f64 {
    a + b  // No semicolon = return value
}
```

### Pattern Matching vs Switch
**C# switch**:
```csharp
switch (operator) {
    case "+":
        return left + right;
    case "-":
        return left - right;
    default:
        throw new Exception("Unknown operator");
}
```

**Rust match**:
```rust
match operator {
    "+" => left + right,
    "-" => left - right,
    _ => panic!("Unknown operator: {}", operator),
}
```

### String Parsing
**C# parsing**:
```csharp
try {
    return double.Parse(input);
} catch {
    throw new Exception("Failed to parse");
}
```

**Rust parsing**:
```rust
input.parse().expect("Failed to parse number")
```

## 🚨 Common Pitfalls to Avoid

### Pitfall 1: Forgetting Return Type
```rust
// ❌ Wrong:
fn add(a: f64, b: f64) {
    a + b
}

// ✅ Correct:
fn add(a: f64, b: f64) -> f64 {
    a + b
}
```

### Pitfall 2: Adding Semicolon to Return Value
```rust
// ❌ Wrong:
fn add(a: f64, b: f64) -> f64 {
    a + b;  // Semicolon makes this a statement, not expression
}

// ✅ Correct:
fn add(a: f64, b: f64) -> f64 {
    a + b   // No semicolon = return value
}
```

### Pitfall 3: Wrong Parse Syntax
```rust
// ❌ Wrong:
fn parse_number(s: &str) -> f64 {
    s.parse()  // Returns Result<f64, _>, not f64
}

// ✅ Correct:
fn parse_number(s: &str) -> f64 {
    s.parse().expect("Failed to parse number")
}
```

## 🏆 Success Indicators

You're on the right track when:
- ✅ `cargo build` shows fewer compilation errors with each fix
- ✅ Tests start passing: `cargo test`
- ✅ The CLI works: `cargo run 5 + 3` outputs `5 + 3 = 8`
- ✅ You understand what each function does

## 💭 Think About It

**Why this project is valuable**:
- Shows how functions compose into complete programs
- Demonstrates pattern matching in real usage
- Introduces command-line argument handling
- Provides experience with Rust's testing framework

**Enterprise relevance**:
- Mathematical operations are common in business logic
- Command-line tools are standard in DevOps
- Pattern matching scales to complex decision trees
- Testing practices transfer directly to production code

## ➡️ Next Level

Still stuck on specific implementations? Try [Level 2 Hints](calculator-level2.md) for more specific code guidance.

Remember: Every enterprise developer needs to parse input, handle operations, and provide output. This calculator teaches those fundamentals! 🦀