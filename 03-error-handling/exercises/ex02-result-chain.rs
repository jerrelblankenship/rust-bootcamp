// Exercise 2: Result Chaining and Error Propagation - Fix the Broken Code!
//
// Your mission: Make this code compile and work correctly by implementing
// the missing functionality and fixing all compilation errors.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Understand how the ? operator works for error propagation  
// 3. Learn to chain Result operations
// 4. Run `rustc ex02-result-chain.rs` to check your progress
// 5. Check solutions/ex02-result-chain.rs if you get completely stuck

use std::fs;
use std::num::ParseIntError;

fn main() {
    println!("=== Exercise 2: Result Chaining (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each exercise one by one and fix the errors
    exercise_2_1();
    // exercise_2_2();
    // exercise_2_3();
    // exercise_2_4();
    // exercise_2_5();
    // exercise_2_6();
    
    println!("\n🎉 All exercises completed successfully!");
}

// Exercise 2.1: Basic Result handling with ? operator - BROKEN CODE TO FIX!
fn exercise_2_1() {
    println!("Exercise 2.1: Basic Result handling with ?");
    
    // FIXME: This function doesn't compile - implement it!
    fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
        // TODO: Parse the string to i32, then double it using ? operator
        // HINT: Use s.parse()? to handle the potential error
        // QUESTION: What does the ? operator do?
        todo!("Parse string and double the result")
    }
    
    // Test cases
    let test_inputs = ["42", "100", "abc", "-15", ""];
    
    for input in test_inputs {
        match parse_and_double(input) {
            Ok(result) => println!("'{}' -> {}", input, result),
            Err(e) => println!("'{}' -> Error: {}", input, e),
        }
    }
    
    println!("✅ Exercise 2.1 complete\n");
}

// Exercise 2.2: Chaining multiple Results - BROKEN CODE TO FIX!
fn exercise_2_2() {
    println!("Exercise 2.2: Chaining multiple Results");
    
    // FIXME: This function has compilation errors
    fn calculate_sum_from_strings(nums: &[&str]) -> Result<i32, ParseIntError> {
        // TODO: Parse each string to i32 and sum them all
        // HINT: Use a loop with ? operator, or try the iterator approach
        // QUESTION: What happens if ANY parsing fails?
        
        let mut sum = 0;
        // TODO: Implement the loop
        // for num_str in nums {
        //     // FIXME: Parse and add to sum, handling errors with ?
        // }
        
        todo!("Calculate sum from string array")
    }
    
    // Test cases
    let good_input = vec!["10", "20", "30"];
    let bad_input = vec!["10", "abc", "30"];
    let empty_input = vec![];
    
    match calculate_sum_from_strings(&good_input) {
        Ok(sum) => println!("Sum of {:?}: {}", good_input, sum),
        Err(e) => println!("Error with {:?}: {}", good_input, e),
    }
    
    match calculate_sum_from_strings(&bad_input) {
        Ok(sum) => println!("Sum of {:?}: {}", bad_input, sum),
        Err(e) => println!("Error with {:?}: {}", bad_input, e),
    }
    
    println!("✅ Exercise 2.2 complete\n");
}

// Exercise 2.3: Custom error types with conversions - BROKEN CODE TO FIX!
fn exercise_2_3() {
    println!("Exercise 2.3: Custom error types with conversions");
    
    // TODO: Define MathError enum
    // HINT: What kinds of math errors can happen?
    #[derive(Debug)]
    enum MathError {
        // TODO: Add error variants
        // - ParseError for wrapping ParseIntError
        // - DivisionByZero
        // - NegativeResult
    }
    
    // FIXME: This implementation is missing
    impl From<ParseIntError> for MathError {
        fn from(err: ParseIntError) -> Self {
            // TODO: Convert ParseIntError to MathError
            todo!("Convert ParseIntError to MathError")
        }
    }
    
    // FIXME: Display trait is missing
    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // TODO: Implement nice error messages for each variant
            todo!("Implement Display for MathError")
        }
    }
    
    // TODO: Add Error trait implementation
    // impl std::error::Error for MathError {}
    
    // FIXME: This function doesn't compile
    fn safe_divide_strings(a_str: &str, b_str: &str) -> Result<f64, MathError> {
        // TODO: Parse both strings using ? operator (auto-converts ParseIntError)
        // TODO: Check for division by zero
        // TODO: Check for negative results
        // HINT: ? operator will automatically convert ParseIntError to MathError
        
        todo!("Implement safe division with custom error handling")
    }
    
    // Test cases
    let test_cases = [
        ("10", "2"),   // Should work
        ("15", "3"),   // Should work
        ("20", "0"),   // Division by zero
        ("abc", "5"),  // Parse error
        ("8", "def"),  // Parse error
        ("-10", "2"),  // Negative result
    ];
    
    for (a, b) in test_cases {
        match safe_divide_strings(a, b) {
            Ok(result) => println!("{} / {} = {:.2}", a, b, result),
            Err(e) => println!("{} / {} -> Error: {}", a, b, e),
        }
    }
    
    println!("✅ Exercise 2.3 complete\n");
}

// Exercise 2.4: Working with file operations - BROKEN CODE TO FIX!
fn exercise_2_4() {
    println!("Exercise 2.4: Working with file operations");
    
    // First, let's create a test file
    let test_content = "42\n17\n88\n";
    fs::write("test_numbers.txt", test_content).expect("Failed to create test file");
    
    // TODO: Define FileProcessingError enum
    #[derive(Debug)]
    enum FileProcessingError {
        // TODO: Add error variants
        // - IoError for wrapping std::io::Error
        // - ParseError for wrapping ParseIntError  
        // - EmptyFile
    }
    
    // FIXME: From trait implementations are missing
    // impl From<std::io::Error> for FileProcessingError { ... }
    // impl From<ParseIntError> for FileProcessingError { ... }
    
    // FIXME: Display trait is missing
    // impl std::fmt::Display for FileProcessingError { ... }
    
    // TODO: Error trait implementation
    // impl std::error::Error for FileProcessingError {}
    
    // FIXME: This function doesn't compile
    fn sum_numbers_from_file(path: &str) -> Result<i32, FileProcessingError> {
        // TODO: Read file content using fs::read_to_string
        // TODO: Check if file is empty
        // TODO: Parse each line and sum them
        // HINT: Use ? operator for automatic error conversion
        
        todo!("Read file and sum all numbers in it")
    }
    
    // Test with our created file
    match sum_numbers_from_file("test_numbers.txt") {
        Ok(sum) => println!("Sum from file: {}", sum),
        Err(e) => println!("Error reading file: {}", e),
    }
    
    // Test with non-existent file
    match sum_numbers_from_file("nonexistent.txt") {
        Ok(sum) => println!("Sum from nonexistent file: {}", sum),
        Err(e) => println!("Error reading nonexistent file: {}", e),
    }
    
    // Clean up
    let _ = fs::remove_file("test_numbers.txt");
    
    println!("✅ Exercise 2.4 complete\n");
}

// Exercise 2.5: Result combinators - BROKEN CODE TO FIX!
fn exercise_2_5() {
    println!("Exercise 2.5: Result combinators");
    
    // FIXME: This function is missing implementation
    fn process_user_input(input: &str) -> Result<String, String> {
        // TODO: Chain operations using Result methods
        // Step 1: Parse input to i32
        // Step 2: Convert parse error to string error (use .map_err())
        // Step 3: Check if number is positive (use .and_then())
        // Step 4: Calculate square (use .map())
        // Step 5: Format result (use .map())
        
        // HINT: Result has methods like .map(), .map_err(), .and_then()
        // These are similar to Option methods but for error handling
        
        todo!("Chain Result operations to process user input")
    }
    
    let test_inputs = ["25", "-10", "abc", "0", "100"];
    
    for input in test_inputs {
        match process_user_input(input) {
            Ok(result) => println!("'{}' -> {}", input, result),
            Err(e) => println!("'{}' -> Error: {}", input, e),
        }
    }
    
    println!("✅ Exercise 2.5 complete\n");
}

// Exercise 2.6: Collecting Results - BROKEN CODE TO FIX!
fn exercise_2_6() {
    println!("Exercise 2.6: Collecting Results");
    
    // FIXME: These functions are missing implementations
    fn try_parse_all(strings: &[&str]) -> Result<Vec<i32>, ParseIntError> {
        // TODO: Return Ok(Vec) if all operations succeed, or first Err if any fail
        // HINT: Use .map() on the slice, then .collect() on the iterator
        // QUESTION: What happens when collect() encounters an error?
        
        todo!("Parse all strings, fail on first error")
    }
    
    fn parse_with_partition(strings: &[&str]) -> (Vec<i32>, Vec<ParseIntError>) {
        // TODO: Separate successful and failed parsing attempts
        // HINT: Parse all, then separate successes and failures
        // This function should never fail - it collects both successes and errors
        
        todo!("Parse all and separate successes from failures")
    }
    
    fn sum_valid_numbers(strings: &[&str]) -> i32 {
        // TODO: Parse and sum only the valid numbers, ignoring errors
        // HINT: Use .filter_map() with .parse().ok()
        // QUESTION: How is this different from the C# approach with try-catch?
        
        todo!("Sum only the valid numbers, ignore invalid ones")
    }
    
    let mixed_input = vec!["10", "abc", "20", "def", "30"];
    
    // Test try_parse_all (should fail on first error)
    match try_parse_all(&mixed_input) {
        Ok(numbers) => println!("All parsed: {:?}", numbers),
        Err(e) => println!("Parse failed: {}", e),
    }
    
    // Test parse_with_partition
    let (successes, failures) = parse_with_partition(&mixed_input);
    println!("Successes: {:?}", successes);
    println!("Failures: {} errors", failures.len());
    
    // Test sum_valid_numbers
    let sum = sum_valid_numbers(&mixed_input);
    println!("Sum of valid numbers: {}", sum);
    
    println!("✅ Exercise 2.6 complete\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // FIXME: These tests won't compile until you implement the functions
    #[test]
    fn test_parse_and_double() {
        // TODO: Uncomment when you implement parse_and_double
        // assert_eq!(parse_and_double("21").unwrap(), 42);
        // assert!(parse_and_double("abc").is_err());
    }
    
    #[test]
    fn test_calculate_sum() {
        // TODO: Uncomment when you implement calculate_sum_from_strings
        // assert_eq!(calculate_sum_from_strings(&["10", "20", "30"]).unwrap(), 60);
        // assert!(calculate_sum_from_strings(&["10", "abc", "30"]).is_err());
    }
    
    #[test]
    fn test_try_parse_all() {
        // TODO: Uncomment when you implement try_parse_all
        // assert_eq!(try_parse_all(&["10", "20", "30"]).unwrap(), vec![10, 20, 30]);
        // assert!(try_parse_all(&["10", "abc", "30"]).is_err());
    }
    
    // TODO: Add more tests as you implement functionality
}

// COMPILATION CHALLENGES:
// 1. Start with exercise_2_1 - implement parse_and_double with ? operator
// 2. Understand how ? operator propagates errors
// 3. Move to exercise_2_2 - sum multiple parsed strings
// 4. Exercise_2_3 - create custom error types with From conversions
// 5. Exercise_2_4 - handle file I/O errors with multiple error types
// 6. Exercise_2_5 - use Result combinators for chaining operations
// 7. Exercise_2_6 - collect results in different ways
//
// KEY CONCEPTS FOR C# DEVELOPERS:
// - ? operator is like C#'s null-conditional operator but for Result
// - Result<T, E> is like Either type - success or error, never both
// - From trait enables automatic error conversions
// - .map() transforms success values
// - .map_err() transforms error values
// - .and_then() chains operations that might fail
// - collect() can transform Vec<Result<T,E>> to Result<Vec<T>,E>
//
// Remember: In Rust, errors are values, not exceptions!
// This makes error handling explicit and recoverable.
