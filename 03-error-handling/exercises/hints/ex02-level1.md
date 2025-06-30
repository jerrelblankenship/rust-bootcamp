# Exercise 2 Hints - Level 1 (Gentle Guidance)

## 🔍 Understanding Result<T, E> for C# Developers

You're working on Result<T, E> patterns, which replace C#'s exception handling model. This is about making errors explicit values rather than exceptions that can crash your program.

## 💡 Core Concept: Errors as Values, Not Exceptions

### What is Result<T, E>?
Result<T, E> represents operations that can succeed (`Ok(value)`) or fail (`Err(error)`). Unlike C# exceptions, errors are values that must be explicitly handled.

**C# Comparison:**
```csharp
// C# - Exceptions can crash the program
try {
    int result = int.Parse(userInput);
    Console.WriteLine($"Parsed: {result}");
} catch (FormatException ex) {
    Console.WriteLine($"Parse error: {ex.Message}");
} catch (Exception ex) {
    Console.WriteLine($"Unexpected error: {ex.Message}");
}
```

**Rust Equivalent:**
```rust
// Rust - Errors are values, cannot be ignored
match user_input.parse::<i32>() {
    Ok(result) => println!("Parsed: {}", result),
    Err(e) => println!("Parse error: {}", e),
}

// Or with ? operator for propagation
fn parse_number(input: &str) -> Result<i32, ParseIntError> {
    let number = input.parse()?;  // Auto-propagate error if parsing fails
    Ok(number)
}
```

## 🎯 Exercise 2.1 Gentle Hints

### Pattern Goal: Basic Result Handling with ? Operator
**What you're trying to achieve:** Parse a string to integer and double it, propagating any parse errors.

**Key Insight:** The `parse()` method returns `Result<i32, ParseIntError>`. You need to handle the potential error and transform the success case.

**Gentle Questions to Ask Yourself:**
- What does `s.parse::<i32>()` return? (Hint: Look at the return type)
- How do you extract the value from `Ok(value)`?
- What does the `?` operator do with errors?
- How do you transform a successful value while keeping error propagation?

**Pattern Approach:**
```rust
// The ? operator pattern:
fn operation_that_can_fail(input: &str) -> Result<ReturnType, ErrorType> {
    let intermediate = input.parse()?;  // If error, return early
    let result = transform(intermediate);  // Only runs if parsing succeeded
    Ok(result)  // Wrap success in Ok()
}
```

## 🎯 Exercise 2.2 Gentle Hints

### Pattern Goal: Chaining Multiple Results
**What you're trying to achieve:** Parse multiple strings and sum them, failing if ANY parsing fails.

**Key Insight:** You want "all or nothing" behavior - if any string fails to parse, the entire operation should fail.

**Gentle Questions:**
- How do you iterate through a collection and handle errors at each step?
- What should happen if the 3rd string in a list of 10 fails to parse?
- How can you accumulate a sum while checking for errors?
- When should the function return early?

**Loop Pattern with ? Operator:**
```rust
fn process_multiple_items(items: &[Item]) -> Result<ProcessedResult, Error> {
    let mut accumulator = initialize();
    
    for item in items {
        let processed = process_item(item)?;  // Return early if any item fails
        accumulator = combine(accumulator, processed);
    }
    
    Ok(accumulator)
}
```

## 🎯 Exercise 2.3 Gentle Hints

### Pattern Goal: Custom Error Types with Conversions
**What you're trying to achieve:** Define your own error type that can wrap different kinds of errors (parsing, math logic, etc.).

**Key Insight:** You want one error type that can represent multiple failure modes, with automatic conversion from library errors.

**Gentle Questions:**
- What different ways can the math operation fail?
- How do you convert a `ParseIntError` into your custom error automatically?
- What makes a good error message?
- How do you chain error types so the original error information isn't lost?

**Custom Error Pattern:**
```rust
#[derive(Debug)]
enum MyError {
    ParseFailed(ParseIntError),    // Wrap the original error
    DivisionByZero,               // Domain-specific error
    NegativeResult,               // Business logic error
}

// Automatic conversion enables ? operator
impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::ParseFailed(err)
    }
}
```

## 🎯 Exercise 2.4 Gentle Hints

### Pattern Goal: File Operations with Multiple Error Types
**What you're trying to achieve:** Read a file and process its contents, handling both I/O errors and processing errors.

**Key Insight:** File operations can fail in multiple ways - file not found, permission denied, parse errors, etc.

**Gentle Questions:**
- What can go wrong when reading a file?
- What can go wrong when processing file contents?
- How do you design an error type that covers all failure modes?
- How do you provide helpful error messages for debugging?

**File Processing Pattern:**
```rust
fn process_file(path: &str) -> Result<ProcessedData, FileError> {
    let content = fs::read_to_string(path)?;  // I/O error auto-converted
    let parsed = parse_content(&content)?;    // Parse error auto-converted
    let processed = validate_data(parsed)?;   // Validation error auto-converted
    Ok(processed)
}
```

## 🔄 General Learning Approach

### Mental Model Shift
**C# Thinking:** "I'll wrap this in try-catch to handle errors"
**Rust Thinking:** "This function signature tells me exactly what can go wrong"

### The ? Operator Magic
```rust
// Without ? operator (verbose)
let result = match operation() {
    Ok(value) => value,
    Err(e) => return Err(e),
};

// With ? operator (concise)
let result = operation()?;
```

### Error Propagation vs Handling
- **Propagate**: Use `?` when the caller should handle the error
- **Handle**: Use `match` when you can recover or provide a meaningful response

### Common Patterns
1. **Immediate handling**: `match result { Ok(val) => ..., Err(e) => ... }`
2. **Propagation**: `let value = operation()?;`
3. **Default values**: `result.unwrap_or(default_value)`
4. **Transformation**: `result.map(|val| transform(val))`
5. **Error mapping**: `result.map_err(|e| MyError::from(e))`

## 💭 Think About It

**Why are Result<T, E> patterns better than exceptions?**
- **Explicit**: Function signatures show exactly what errors can occur
- **No hidden control flow**: No invisible jumps up the call stack
- **Composable**: Easy to chain, transform, and convert between error types
- **Performance**: Zero overhead when no errors occur
- **Exhaustive**: Compiler ensures you handle all error cases

## 🆘 When You See Compilation Errors

- **"expected `Result<T, E>`, found `T`"** → You need to wrap success in `Ok()`
- **"the `?` operator can only be used"** → Your function must return a `Result`
- **"trait bound not satisfied"** → You need to implement `From` for error conversion
- **"cannot find type"** → You need to define your custom error enum

## ➡️ Next Level

Still struggling with specific Result implementations? Try [Level 2 Hints](ex02-level2.md) for concrete code patterns.

Remember: Result<T, E> makes errors explicit and recoverable. The compiler prevents you from ignoring failures! 🦀