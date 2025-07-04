# Exercise 1 Hints - Level 1 (Gentle Nudges)

## 🤔 Stuck on Basic Test Fundamentals? Here are some gentle hints...

### Checkpoint 1: Missing Test Attribute
- Look at line 34: The function `test_add_positive_numbers()` 
- **Question**: How does Rust know this function is a test?
- **C# Comparison**: Like `[Test]` in NUnit or `[Fact]` in xUnit
- **Hint**: Tests need a special attribute to be discovered by the test runner

### Checkpoint 2: Logic Error in Assertion
- Line 43: `assert_eq!(result, 1)` for `add(-2, -3)`
- **Question**: What is -2 + -3 actually equal to?
- **Hint**: Check your math! The expected value is wrong.

### Checkpoint 3: Result Type Handling
- Line 52: `assert_eq!(result, 5.0)` but `result` is `Result<f64, String>`
- **Question**: How do you compare a `Result` with a plain value?
- **C# Comparison**: Like checking a method that doesn't throw vs checking the return value
- **Hint**: You need to handle the `Result` wrapper first

### Checkpoint 4: Wrong Error Expectation
- Line 58: `assert!(result.is_ok())` for divide by zero
- **Question**: Should dividing by zero succeed or fail?
- **Hint**: Division by zero should return an error, not success

### Checkpoint 5: Poor Assertion Messages
- Lines 65-66: `assert!(factorial(0) == 1)`
- **Question**: What happens when this assertion fails? How helpful is the error message?
- **C# Comparison**: `Assert.AreEqual()` vs `Assert.IsTrue()` for better error messages
- **Hint**: Use `assert_eq!` for better failure output

### Checkpoint 6: Logic Errors in Even/Odd Tests
- Lines 72-74: Check your expectations carefully
- **Question**: Is 2 even or odd? Is 3 even or odd? Is 0 even or odd?
- **Hint**: Review the basic definition of even and odd numbers

### Checkpoint 7: Option Type Handling
- Line 82: `assert_eq!(result, 0)` but `result` is `Option<i32>`
- **Question**: What should `find_max` return for an empty slice?
- **Hint**: Empty collections typically return `None`, not a default value

### Checkpoint 8: Wrong Expected Value and Missing Message
- Line 89: `assert_eq!(result, Some(8))` but max of `[1, 5, 3, 9, 2]` is not 8
- **Question**: What's the actual maximum value in that array?
- **Hint**: Also add a custom message to help debug failures

## 💡 General Strategy
1. **Read compiler errors carefully** - Rust gives excellent error messages
2. **Think about types** - `Result<T, E>` and `Option<T>` need special handling
3. **Check your logic** - Make sure expected values are correct
4. **Use descriptive assertions** - `assert_eq!` over `assert!` for better errors

Still stuck? Try Level 2 hints! 🚀