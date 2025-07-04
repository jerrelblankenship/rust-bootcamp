# Exercise 06 - Level 2 Hints: Documentation Disaster

## 🎯 Specific Documentation Fixes

### 🔧 Fix Import Paths
```rust
// Wrong:
use calculator::Calculator;

// Correct:
use ex06_documentation::Calculator;
```

### 📊 Fix Expected Results
```rust
// Wrong:
let result = calc.add(10, 20);
assert_eq!(result, 35);  // 10 + 20 = 30, not 35!

// Correct:
assert_eq!(result, 30);
```

### 🔒 Fix Return Types
```rust
// Documentation says it returns Result, but code panics
// Either change the code to return Result or fix the docs
```

### 📝 Add Missing Documentation
```rust
/// Multiplies two numbers together.
/// 
/// # Examples
/// 
/// ```
/// use ex06_documentation::Calculator;
/// 
/// let calc = Calculator::new();
/// let result = calc.multiply(4.0, 5.0);
/// assert_eq!(result, 20.0);
/// ```
pub fn multiply(&self, a: f64, b: f64) -> f64 {
```

## ⏰ Time Check

Still stuck after 30 minutes? Move to Level 3 for the complete solution.