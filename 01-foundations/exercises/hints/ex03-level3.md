# Exercise 3 Hints - Level 3 (Nearly Complete Solutions)

## ⚠️ Last Resort Guidance

**Use these hints only if Level 1 and Level 2 didn't help!** These contain nearly complete solutions.

## 🛠️ Complete Function Examples

### Basic Function with Parameters and Return Type
```rust
// Complete function signature and implementation:
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b  // Expression return (no semicolon)
}

// Alternative with explicit return:
fn multiply_numbers(a: i32, b: i32) -> i32 {
    let result = a * b;
    return result;  // Explicit return
}
```

### Pattern Matching with Match
```rust
// Complete match expression:
fn describe_number(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        n if n < 0 => "negative".to_string(),
        _ => "other".to_string(),  // Catch-all case
    }
}

// Match with multiple values:
fn categorize_grade(grade: char) -> String {
    match grade {
        'A' | 'B' => "Good".to_string(),
        'C' => "Average".to_string(),
        'D' | 'F' => "Poor".to_string(),
        _ => "Invalid grade".to_string(),
    }
}
```

### Option Type Handling
```rust
// Complete Option handling:
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn use_optional_value(maybe_value: Option<i32>) -> i32 {
    match maybe_value {
        Some(value) => value * 2,
        None => 0,  // Default value
    }
}

// Using Option methods:
fn double_if_some(maybe_number: Option<i32>) -> Option<i32> {
    maybe_number.map(|n| n * 2)
}
```

### Result Type Handling
```rust
// Complete Result handling:
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

fn use_parse_result(input: &str) -> String {
    match parse_number(input) {
        Ok(number) => format!("Parsed: {}", number),
        Err(e) => format!("Error: {}", e),
    }
}

// Using ? operator for error propagation:
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let number = s.parse()?;  // ? propagates errors automatically
    Ok(number * 2)
}
```

### Control Flow Examples
```rust
// Complete if-let pattern:
fn check_optional(maybe_value: Option<i32>) {
    if let Some(value) = maybe_value {
        println!("Got value: {}", value);
    } else {
        println!("No value");
    }
}

// Complete loop with break:
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &number in numbers {
        if number % 2 == 0 {
            return Some(number);
        }
    }
    None
}

// Complete while loop:
fn countdown(mut n: i32) {
    while n > 0 {
        println!("{}", n);
        n -= 1;
    }
    println!("Done!");
}
```

## 🔍 Complete Working Example

Here's how a comprehensive function file might look:

```rust
fn main() {
    // Test basic functions
    let sum = add_numbers(5, 3);
    println!("Sum: {}", sum);
    
    // Test pattern matching
    let description = describe_number(42);
    println!("Description: {}", description);
    
    // Test Option handling
    let division = safe_divide(10.0, 2.0);
    match division {
        Some(result) => println!("Division result: {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    // Test Result handling
    let parse_result = use_parse_result("123");
    println!("{}", parse_result);
    
    let bad_parse = use_parse_result("abc");
    println!("{}", bad_parse);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn describe_number(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        1..=10 => "small".to_string(),
        11..=100 => "medium".to_string(),
        _ => "large".to_string(),
    }
}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn use_parse_result(input: &str) -> String {
    match input.parse::<i32>() {
        Ok(number) => format!("Successfully parsed: {}", number),
        Err(_) => format!("Failed to parse: {}", input),
    }
}
```

## 🎯 Understanding Check

Before moving on, make sure you understand:

1. **Function signature syntax** - Parameter types and return types
2. **Expression vs statement** - When to use semicolons
3. **Pattern matching** - Exhaustive matching with `_`
4. **Option handling** - `Some` and `None` cases
5. **Result handling** - `Ok` and `Err` cases
6. **Error propagation** - When to use `?` operator

## 🚀 Next Steps

- Compile and run: `rustc ex03-functions.rs && ./ex03-functions`
- Move to [Exercise 4](ex04-level1.md)
- If you used these hints, review the concepts in the lesson material

## 🎓 Reflection Questions

1. How does Rust's pattern matching compare to C#'s switch statements?
2. Why does Rust force you to handle Option and Result types explicitly?
3. What are the benefits of expression-based returns?

Remember: Rust's function system prevents many runtime errors that are common in other languages! 🦀
