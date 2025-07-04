# Exercise 1 Hints - Level 3 (Complete Solutions)

## 🔴 Here are the complete fixes for all checkpoints:

### Checkpoint 1: Add Missing Test Attribute
```rust
// BEFORE:
fn test_add_positive_numbers() {

// AFTER:
#[test]
fn test_add_positive_numbers() {
```

### Checkpoint 2: Fix Logic Error
```rust
// BEFORE:
assert_eq!(result, 1);  // Wrong!

// AFTER:
assert_eq!(result, -5); // -2 + -3 = -5
```

### Checkpoint 3: Handle Result Type
```rust
// BEFORE:
assert_eq!(result, 5.0);

// AFTER:
assert!(result.is_ok());
assert_eq!(result.unwrap(), 5.0);
```

### Checkpoint 4: Test Error Case
```rust
// BEFORE:
assert!(result.is_ok());

// AFTER:
assert!(result.is_err());
assert!(result.unwrap_err().contains("Cannot divide by zero"));
```

### Checkpoint 5: Use Better Assertions
```rust
// BEFORE:
assert!(factorial(0) == 1);
assert!(factorial(1) == 1);

// AFTER:
assert_eq!(factorial(0), 1);
assert_eq!(factorial(1), 1);
```

### Checkpoint 6: Fix Boolean Logic
```rust
// BEFORE:
assert_eq!(is_even(2), false);
assert_eq!(is_even(3), true);
assert_eq!(is_even(0), false);

// AFTER:
assert_eq!(is_even(2), true);   // 2 is even
assert_eq!(is_even(3), false);  // 3 is odd
assert_eq!(is_even(0), true);   // 0 is even
```

### Checkpoint 7: Handle Option Type
```rust
// BEFORE:
assert_eq!(result, 0);

// AFTER:
assert_eq!(result, None);
```

### Checkpoint 8: Fix Value and Add Message
```rust
// BEFORE:
assert_eq!(result, Some(8));

// AFTER:
assert_eq!(result, Some(9), "find_max should return the largest value in the slice");
```

## 🎯 Complete Fixed Test Module

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]  // ✅ Added missing attribute
    fn test_add_positive_numbers() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        let result = add(-2, -3);
        assert_eq!(result, -5);  // ✅ Fixed logic error
    }

    #[test]
    fn test_divide_valid_numbers() {
        let result = divide(10.0, 2.0);
        assert!(result.is_ok());  // ✅ Handle Result type
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());  // ✅ Test error case
    }

    #[test]
    fn test_factorial_base_cases() {
        assert_eq!(factorial(0), 1);  // ✅ Better assertions
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_is_even_numbers() {
        assert_eq!(is_even(2), true);   // ✅ Fixed logic
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
    }

    #[test]
    fn test_find_max_empty_slice() {
        let numbers = [];
        let result = find_max(&numbers);
        assert_eq!(result, None);  // ✅ Handle Option type
    }

    #[test]
    fn test_find_max_with_custom_message() {
        let numbers = [1, 5, 3, 9, 2];
        let result = find_max(&numbers);
        assert_eq!(result, Some(9), "find_max should return the largest value");  // ✅ Fixed value and added message
    }
}
```

## 🚀 Key Learning Points

### 1. Test Attributes
- Always use `#[test]` to mark test functions
- Tests run with `cargo test`

### 2. Assertion Macros
- `assert_eq!(actual, expected)` for value equality
- `assert!(condition)` for boolean conditions
- Add custom messages for better debugging

### 3. Rust Type Handling
- `Result<T, E>`: Check with `.is_ok()` / `.is_err()`, extract with `.unwrap()`
- `Option<T>`: Check with `.is_some()` / `.is_none()`, extract with `.unwrap()`

### 4. C# Comparison
| C# | Rust |
|---|---|
| `[Test]` / `[Fact]` | `#[test]` |
| `Assert.AreEqual(expected, actual)` | `assert_eq!(actual, expected)` |
| `Assert.IsTrue(condition)` | `assert!(condition)` |
| `Assert.Throws<Exception>()` | `assert!(result.is_err())` |

🎉 **Congratulations!** You've mastered Rust test fundamentals. Move on to the next exercise!