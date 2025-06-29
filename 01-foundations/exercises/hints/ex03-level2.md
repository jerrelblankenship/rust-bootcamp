# Exercise 3 Hints - Level 2 (More Specific Guidance)

## 🎯 Specific Solutions

You've tried Level 1 hints but need more guidance. Here are more specific solutions for common function and control flow issues:

## 🔧 Fix 1: Function Signatures

**Problem**: Missing parameter types or return types
**Solution**: Add type annotations to parameters and return type:
```rust
// BEFORE (broken):
fn add(a, b) {
    a + b
}

// AFTER (fixed):
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Explanation**:
- Parameter syntax: `name: type`
- Return type syntax: `-> type`
- No explicit `return` needed for final expressions

## 🔧 Fix 2: Match Expressions

**Problem**: Non-exhaustive patterns
**Solution**: Handle all cases with `_` wildcard:
```rust
// BEFORE (broken):
match number {
    1 => println!("One"),
    2 => println!("Two"),
}

// AFTER (fixed):
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // Handle all other cases
}
```

**Explanation**:
- `match` must be exhaustive
- `_` catches all unhandled cases
- Each arm uses `=>` (not `:` like C# switch)

## 🔧 Fix 3: Option Type Handling

**Problem**: Using Option values directly
**Solution**: Use pattern matching or methods:
```rust
// BEFORE (broken):
let maybe_number = Some(42);
let doubled = maybe_number * 2;  // Error!

// AFTER (fixed):
let maybe_number = Some(42);
let doubled = match maybe_number {
    Some(n) => Some(n * 2),
    None => None,
};

// OR use map:
let doubled = maybe_number.map(|n| n * 2);
```

**Explanation**:
- `Option<T>` can be `Some(value)` or `None`
- Must handle both cases explicitly
- `.map()` is often more concise

## 🔧 Fix 4: Result Type Handling

**Problem**: Ignoring potential errors
**Solution**: Handle both Ok and Err cases:
```rust
// BEFORE (broken):
let parsed = "42".parse();  // Type inference error

// AFTER (fixed):
let parsed: Result<i32, _> = "42".parse();
match parsed {
    Ok(number) => println!("Got: {}", number),
    Err(e) => println!("Parse error: {}", e),
}

// OR use expect for prototyping:
let number: i32 = "42".parse().expect("Failed to parse");
```

**Explanation**:
- `Result<T, E>` can be `Ok(value)` or `Err(error)`
- Must handle both cases or use methods like `.expect()`
- Type annotations often needed for parsing

## 🔧 Fix 5: Expression vs Statement Returns

**Problem**: Missing return value or incorrect return syntax
**Solution**: Use expressions or explicit return:
```rust
// BEFORE (broken):
fn calculate(x: i32) -> i32 {
    let result = x * 2;
    // Missing return value
}

// AFTER (fixed - expression):
fn calculate(x: i32) -> i32 {
    x * 2  // No semicolon = expression = return value
}

// OR (fixed - explicit return):
fn calculate(x: i32) -> i32 {
    let result = x * 2;
    return result;  // Explicit return
}
```

**Explanation**:
- Expressions don't end with semicolons
- Statements end with semicolons and don't return values
- `return` is optional for final expressions

## 📋 Compilation Checklist

After each fix, run:
```bash
rustc ex03-functions.rs
```

Work through errors one at a time.

## ➡️ Next Level

Still having trouble? Try [Level 3 Hints](ex03-level3.md) for nearly complete solutions.

## 🎓 Key Learning Points

- Function signatures need explicit types
- `match` expressions must be exhaustive
- `Option` and `Result` force explicit error handling
- Expressions return values, statements don't

Keep going - you're learning Rust's safety guarantees! 🦀
