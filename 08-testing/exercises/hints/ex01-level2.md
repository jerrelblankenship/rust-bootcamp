# Exercise 1 Hints - Level 2 (Rust-Specific Patterns)

## 🟡 Ready for more specific Rust testing guidance? Here's what you need...

### Checkpoint 1: Test Attribute Syntax
```rust
// WRONG:
fn test_add_positive_numbers() {

// RIGHT:
#[test]
fn test_add_positive_numbers() {
```
The `#[test]` attribute tells Rust this function should be run as a test.

### Checkpoint 2: Fix the Math
```rust
// WRONG:
assert_eq!(result, 1);  // -2 + -3 ≠ 1

// RIGHT:
assert_eq!(result, -5); // -2 + -3 = -5
```

### Checkpoint 3: Handling Result Types
```rust
// WRONG:
assert_eq!(result, 5.0);  // Can't compare Result<f64, String> with f64

// RIGHT - Option 1:
assert_eq!(result.unwrap(), 5.0);

// RIGHT - Option 2:
assert!(result.is_ok());
assert_eq!(result.unwrap(), 5.0);

// RIGHT - Option 3:
match result {
    Ok(value) => assert_eq!(value, 5.0),
    Err(e) => panic!("Expected success, got error: {}", e),
}
```

### Checkpoint 4: Testing Error Cases
```rust
// WRONG:
assert!(result.is_ok());  // Division by zero should fail!

// RIGHT:
assert!(result.is_err());
// Optionally check the error message:
assert!(result.unwrap_err().contains("divide by zero"));
```

### Checkpoint 5: Better Assertion Macros
```rust
// WRONG - Poor error messages:
assert!(factorial(0) == 1);
assert!(factorial(1) == 1);

// RIGHT - Clear error messages:
assert_eq!(factorial(0), 1);
assert_eq!(factorial(1), 1);
```

When `assert_eq!` fails, it shows both expected and actual values.

### Checkpoint 6: Fix Boolean Logic
```rust
// WRONG:
assert_eq!(is_even(2), false);  // 2 IS even!
assert_eq!(is_even(3), true);   // 3 is NOT even!
assert_eq!(is_even(0), false);  // 0 IS even!

// RIGHT:
assert_eq!(is_even(2), true);   // 2 is even
assert_eq!(is_even(3), false);  // 3 is odd  
assert_eq!(is_even(0), true);   // 0 is even (divisible by 2)
```

### Checkpoint 7: Option Type Handling
```rust
// WRONG:
assert_eq!(result, 0);  // Can't compare Option<i32> with i32

// RIGHT:
assert_eq!(result, None);  // Empty slice should return None

// Or if you want to be explicit:
assert!(result.is_none());
```

### Checkpoint 8: Fix Value and Add Message
```rust
// WRONG:
assert_eq!(result, Some(8));  // Max of [1,5,3,9,2] is 9, not 8!

// RIGHT:
assert_eq!(result, Some(9), "find_max should return the largest value in the slice");
```

## 🔧 Testing Pattern Summary

### Rust Test Assertions:
- `assert!(condition)` - Basic boolean assertion
- `assert_eq!(left, right)` - Equality with good error messages  
- `assert_ne!(left, right)` - Inequality assertion
- `assert!(condition, "message")` - Custom failure message

### Handling Rust Types in Tests:
- `Result<T, E>`: Use `.unwrap()`, `.is_ok()`, `.is_err()`, or pattern matching
- `Option<T>`: Use `.unwrap()`, `.is_some()`, `.is_none()`, or pattern matching
- Always prefer `assert_eq!` over `assert!` for value comparisons

Still stuck? Try Level 3 hints for complete solutions! 🚀