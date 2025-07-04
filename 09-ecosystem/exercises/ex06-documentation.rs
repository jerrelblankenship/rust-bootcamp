// Exercise 06: Documentation Disaster - Fix the broken docs!
//
// This exercise has poor documentation that doesn't compile,
// missing examples, and unclear API documentation.
//
// Your mission: Fix the docs and make them shine!

// Expected: Comprehensive docs with working examples
// Currently: Broken doctests, missing examples, unclear documentation

/// A calculator that can perform basic arithmetic operations.
/// 
/// This calculator is designed to be simple and easy to use.
/// It supports addition, subtraction, multiplication, and division.
///
/// # Examples
///
/// ```
/// use ex06_documentation::Calculator;
///
/// let calc = Calculator::new();
/// let result = calc.add(2, 3);
/// assert_eq!(result, 5);
///
/// // This example is broken - method doesn't exist!
/// let result = calc.complex_operation(vec![1, 2, 3]);
/// assert_eq!(result, 6);
/// ```
///
/// # Panics
///
/// This calculator will panic if you try to divide by zero.
/// Actually, it returns an error, but the documentation is wrong.
///
/// # Safety
///
/// This calculator is safe to use. No unsafe code here.
/// Wait, actually there is unsafe code below, but it's not documented.
pub struct Calculator {
    memory: f64,
}

impl Calculator {
    /// Creates a new calculator.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let calc = Calculator::new();
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
    /// let result = calc.add(2, 3);
    /// assert_eq!(result, 5);
    /// 
    /// // This example is broken - wrong expected result!
    /// let result = calc.add(10, 20);
    /// assert_eq!(result, 35);
    /// ```
    pub fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }
    
    /// Subtracts the second number from the first.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // This example doesn't compile - wrong import!
    /// use calculator::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// let result = calc.subtract(10, 3);
    /// assert_eq!(result, 7);
    /// ```
    pub fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }
    
    /// Divides the first number by the second.
    /// 
    /// Returns an error if the divisor is zero.
    /// Wait, actually it panics. The documentation is inconsistent.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ex06_documentation::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// let result = calc.divide(10.0, 2.0);
    /// assert_eq!(result, 5.0);
    /// 
    /// // This example will panic but documentation says it returns an error!
    /// let result = calc.divide(10.0, 0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn divide(&self, a: f64, b: f64) -> f64 {
        if b == 0.0 {
            panic!("Division by zero!");
        }
        a / b
    }
    
    // Missing documentation entirely!
    pub fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }
    
    /// Stores a value in memory.
    /// 
    /// # Safety
    /// 
    /// This function is safe to use.
    /// Actually, it contains unsafe code but doesn't document why.
    pub fn store(&mut self, value: f64) {
        // Why is this unsafe? The documentation doesn't explain!
        unsafe {
            std::ptr::write(&mut self.memory, value);
        }
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
    /// 
    /// // This example uses a method that doesn't exist!
    /// calc.clear_memory();
    /// let value = calc.recall();
    /// assert_eq!(value, 0.0);
    /// ```
    pub fn recall(&self) -> f64 {
        self.memory
    }
}

// Missing module-level documentation!
// No examples showing how to use the entire module!
// No links to related functionality!

fn main() {
    let mut calc = Calculator::new();
    
    println!("2 + 3 = {}", calc.add(2.0, 3.0));
    println!("10 - 4 = {}", calc.subtract(10.0, 4.0));
    println!("6 * 7 = {}", calc.multiply(6.0, 7.0));
    println!("15 / 3 = {}", calc.divide(15.0, 3.0));
    
    calc.store(100.0);
    println!("Memory: {}", calc.recall());
}

// What you need to fix:
// 1. Fix all broken doctests
// 2. Add proper module-level documentation
// 3. Fix inconsistent documentation (panic vs error)
// 4. Document the unsafe code properly
// 5. Add missing documentation for all public items
// 6. Add comprehensive examples
// 7. Fix import paths in examples
// 8. Add proper error handling where documented