# Exercise 06 - Level 3 Hints: Documentation Disaster

## 🎯 Complete Documentation Fix

```rust
//! A simple calculator library for basic arithmetic operations.
//!
//! This library provides a `Calculator` struct that can perform
//! addition, subtraction, multiplication, and division with proper
//! error handling.
//!
//! # Examples
//!
//! ```
//! use ex06_documentation::Calculator;
//!
//! let mut calc = Calculator::new();
//! assert_eq!(calc.add(2.0, 3.0), 5.0);
//! assert_eq!(calc.subtract(10.0, 4.0), 6.0);
//! assert_eq!(calc.multiply(3.0, 4.0), 12.0);
//! 
//! match calc.divide(10.0, 2.0) {
//!     Ok(result) => assert_eq!(result, 5.0),
//!     Err(e) => panic!("Unexpected error: {}"),
//! }
//! ```

/// A calculator that can perform basic arithmetic operations.
/// 
/// The calculator maintains a memory value that can be stored and recalled.
/// All operations are performed with `f64` precision.
///
/// # Examples
///
/// ```
/// use ex06_documentation::Calculator;
///
/// let calc = Calculator::new();
/// let result = calc.add(2.0, 3.0);
/// assert_eq!(result, 5.0);
/// ```
pub struct Calculator {
    memory: f64,
}

#[derive(Debug)]
pub enum CalculatorError {
    DivisionByZero,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

impl std::error::Error for CalculatorError {}

impl Calculator {
    /// Creates a new calculator with memory initialized to 0.0.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// assert_eq!(calc.recall(), 0.0);
    /// ```
    pub fn new() -> Self {
        Calculator { memory: 0.0 }
    }
    
    /// Adds two numbers together.
    /// 
    /// # Arguments
    /// 
    /// * `a` - The first number
    /// * `b` - The second number
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// let result = calc.add(2.0, 3.0);
    /// assert_eq!(result, 5.0);
    /// 
    /// let result = calc.add(10.0, 20.0);
    /// assert_eq!(result, 30.0);
    /// ```
    pub fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }
    
    /// Subtracts the second number from the first.
    /// 
    /// # Arguments
    /// 
    /// * `a` - The number to subtract from
    /// * `b` - The number to subtract
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// let result = calc.subtract(10.0, 3.0);
    /// assert_eq!(result, 7.0);
    /// ```
    pub fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }
    
    /// Multiplies two numbers together.
    /// 
    /// # Arguments
    /// 
    /// * `a` - The first number
    /// * `b` - The second number
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
        a * b
    }
    
    /// Divides the first number by the second.
    /// 
    /// # Arguments
    /// 
    /// * `a` - The dividend
    /// * `b` - The divisor
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(result)` if successful, or `Err(CalculatorError::DivisionByZero)` if the divisor is zero.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// let result = calc.divide(10.0, 2.0).unwrap();
    /// assert_eq!(result, 5.0);
    /// 
    /// let result = calc.divide(10.0, 0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn divide(&self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    /// Stores a value in memory.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to store
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let mut calc = Calculator::new();
    /// calc.store(42.0);
    /// assert_eq!(calc.recall(), 42.0);
    /// ```
    pub fn store(&mut self, value: f64) {
        self.memory = value;
    }
    
    /// Gets the value from memory.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let mut calc = Calculator::new();
    /// calc.store(42.0);
    /// let value = calc.recall();
    /// assert_eq!(value, 42.0);
    /// ```
    pub fn recall(&self) -> f64 {
        self.memory
    }
    
    /// Clears the memory, setting it to 0.0.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let mut calc = Calculator::new();
    /// calc.store(42.0);
    /// calc.clear_memory();
    /// assert_eq!(calc.recall(), 0.0);
    /// ```
    pub fn clear_memory(&mut self) {
        self.memory = 0.0;
    }
}

fn main() {
    let mut calc = Calculator::new();
    
    println!("2 + 3 = {}", calc.add(2.0, 3.0));
    println!("10 - 4 = {}", calc.subtract(10.0, 4.0));
    println!("6 * 7 = {}", calc.multiply(6.0, 7.0));
    
    match calc.divide(15.0, 3.0) {
        Ok(result) => println!("15 / 3 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    calc.store(100.0);
    println!("Memory: {}", calc.recall());
}
```

## 🎯 What Was Fixed

1. **All doctest examples** now compile and pass
2. **Module-level documentation** added
3. **Proper error handling** with custom error type
4. **Consistent behavior** - divide now returns Result as documented
5. **Complete documentation** for all public items
6. **Added missing methods** referenced in examples

## 🏆 Success Criteria

`cargo test --doc` should pass with no failures!