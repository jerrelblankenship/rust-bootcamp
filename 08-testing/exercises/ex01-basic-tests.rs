// Exercise 1: Basic Test Fundamentals - Fix the Broken Tests!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (8 checkpoints to fix)
//
// Your task: Fix broken test code to learn Rust testing fundamentals
//
// INSTRUCTIONS:
// 1. Fix ONE test error at a time - don't try to fix everything at once!
// 2. Run tests: `cargo test --bin ex01-basic-tests`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (missing test attribute)
// - Each checkpoint builds on the previous one
// - Compare with C# xUnit/NUnit patterns you know
//
// COMPLETED CONCEPTS:
// [] Test attributes and organization
// [] Basic assertions (assert!, assert_eq!)
// [] Descriptive test names
// [] Test failure messages
// [] Testing return values vs exceptions
// [] Custom assertion messages
// [] Testing edge cases
// [] Test documentation

// Simple calculator functions to test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn find_max(numbers: &[i32]) -> Option<i32> {
    numbers.iter().max().copied()
}

// BROKEN TESTS TO FIX!

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1.1: Fix the missing test attribute
    // FIXME: This test won't run - what's missing?
    fn test_add_positive_numbers() {  // COMPILE ERROR: Missing test attribute!
        let result = add(2, 3);
        assert_eq!(result, 5);
    }
    // In C#, this would be [Test] or [Fact]
    
    // ✅ CHECKPOINT 1: Add #[test] attribute so the test runs

    #[test]
    fn test_add_negative_numbers() {
        let result = add(-2, -3);
        // FIXME: Wrong assertion - this will fail!
        assert_eq!(result, 1);  // LOGIC ERROR: -2 + -3 ≠ 1
    }
    
    // ✅ CHECKPOINT 2: Fix the assertion to expect the correct result

    #[test]
    fn test_divide_valid_numbers() {
        let result = divide(10.0, 2.0);
        // FIXME: Not handling Result type properly!
        assert_eq!(result, 5.0);  // COMPILE ERROR: Can't compare Result with f64
    }
    // In C#, you might use Assert.DoesNotThrow() or just call the method
    
    // ✅ CHECKPOINT 3: Handle Result type properly with .unwrap() or pattern matching

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        // FIXME: Should test for error case!
        assert!(result.is_ok());  // LOGIC ERROR: Should be an error!
    }
    
    // ✅ CHECKPOINT 4: Fix assertion to check for error case

    #[test]
    fn test_factorial_base_cases() {
        // FIXME: Using wrong assertion macro!
        assert!(factorial(0) == 1);  // STYLE ERROR: Poor error messages
        assert!(factorial(1) == 1);  // STYLE ERROR: Poor error messages
    }
    // In C#, Assert.AreEqual gives better error messages than Assert.IsTrue
    
    // ✅ CHECKPOINT 5: Use assert_eq! for better failure messages

    #[test]
    fn test_is_even_numbers() {
        // FIXME: Logic errors in test expectations!
        assert_eq!(is_even(2), false);  // LOGIC ERROR: 2 is even!
        assert_eq!(is_even(3), true);   // LOGIC ERROR: 3 is odd!
        assert_eq!(is_even(0), false);  // LOGIC ERROR: 0 is even!
    }
    
    // ✅ CHECKPOINT 6: Fix the logic errors in test expectations

    #[test]
    fn test_find_max_empty_slice() {
        let numbers = [];
        let result = find_max(&numbers);
        // FIXME: Wrong assertion for Option type!
        assert_eq!(result, 0);  // COMPILE ERROR: Can't compare Option<i32> with i32
    }
    
    // ✅ CHECKPOINT 7: Handle Option type properly (should be None for empty slice)

    #[test]
    fn test_find_max_with_custom_message() {
        let numbers = [1, 5, 3, 9, 2];
        let result = find_max(&numbers);
        // FIXME: No custom error message!
        assert_eq!(result, Some(8));  // LOGIC ERROR: Max is 9, not 8, and no helpful message
    }
    
    // ✅ CHECKPOINT 8: Fix expected value and add custom assertion message
}

// COMPILATION CHALLENGES:
// 1. Add missing #[test] attributes
// 2. Fix Result type handling in assertions
// 3. Fix Option type handling in assertions
// 4. Use assert_eq! instead of assert! for better error messages
// 5. Fix logic errors in test expectations
// 6. Add custom assertion messages for better debugging
//
// LEARNING OBJECTIVES:
// - Understanding Rust test attributes and organization
// - Proper assertion macros for different data types
// - Handling Result and Option types in tests
// - Writing descriptive test names and error messages
// - Comparing Rust testing with C# xUnit/NUnit patterns
//
// C# COMPARISON:
// C#: [Test] / [Fact] attribute
// Rust: #[test] attribute
//
// C#: Assert.AreEqual(expected, actual)
// Rust: assert_eq!(actual, expected)  // Note: order is different!
//
// C#: Assert.IsTrue(condition)
// Rust: assert!(condition)
//
// C#: Assert.Throws<Exception>(() => method())
// Rust: assert!(result.is_err()) or use should_panic attribute
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Test attribute): [ ] Complete
// Checkpoint 2 (Logic error): [ ] Complete
// Checkpoint 3 (Result handling): [ ] Complete
// Checkpoint 4 (Error case testing): [ ] Complete
// Checkpoint 5 (Better assertions): [ ] Complete
// Checkpoint 6 (Logic fixes): [ ] Complete
// Checkpoint 7 (Option handling): [ ] Complete
// Checkpoint 8 (Custom messages): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Rust test attributes and organization
// ✅ Proper assertion macros for different types
// ✅ Result and Option type testing
// ✅ Writing clear, descriptive test code
// ✅ Rust testing patterns vs C# patterns
//
// 🚀 Ready for the next challenge?
// Move on to ex02-test-organization.rs to learn test module structure!
// Or check your work with: `cargo test --bin ex01-basic-tests`