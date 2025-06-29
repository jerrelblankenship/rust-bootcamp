// Baby Calculator - Fix the Broken Code!
//
// Your mission: Make this calculator work by fixing the compilation errors
// and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Run `cargo build` to see compilation errors
// 2. Fix ONE error at a time
// 3. Run `cargo test` to verify your implementations
// 4. Test with: `cargo run 5 + 3`

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check if we have the right number of arguments
    if args.len() != 4 {
        println!("Usage: calculator <number> <operation> <number>");
        println!("Example: calculator 5 + 3");
        return;
    }
    
    // FIXME: Parse the arguments - these don't compile yet!
    let left = parse_number(&args[1]);
    let operator = &args[2];
    let right = parse_number(&args[3]);
    
    // FIXME: Call the calculate function 
    let result = calculate(left, operator, right);
    
    // Print the result nicely
    println!("{} {} {} = {}", left, operator, right, result);
}

// FIXME: This function doesn't exist - implement it!
// HINT: Use .parse() method on strings to convert to f64
// TODO: What should this function return if parsing fails?
fn parse_number(s: &str) -> f64 {
    // TODO: Parse the string to f64 or panic with a helpful message
    // HINT: s.parse().expect("Failed to parse number")
    todo!("Parse string to number")
}

// FIXME: This function doesn't compile - fix the signature and implementation!
fn calculate(left: f64, operator: &str, right: f64) -> f64 {
    // TODO: Use a match statement to handle different operators
    // HINT: match operator { "+" => ..., "-" => ..., "*" => ..., "/" => ... }
    
    match operator {
        // TODO: Implement each operation
        "+" => todo!("Implement addition"),
        "-" => todo!("Implement subtraction"), 
        "*" => todo!("Implement multiplication"),
        "/" => todo!("Implement division"),
        _ => panic!("Unknown operator: {}", operator),
    }
}

// BONUS: Implement these helper functions (optional)
// Remove todo!() and add real implementations

fn add(a: f64, b: f64) -> f64 {
    todo!("Implement addition")
}

fn subtract(a: f64, b: f64) -> f64 {
    todo!("Implement subtraction")
}

fn multiply(a: f64, b: f64) -> f64 {
    todo!("Implement multiplication")
}

fn divide(a: f64, b: f64) -> f64 {
    // TODO: What should happen if b is 0?
    todo!("Implement division")
}

// Tests to verify your implementation
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        let result = calculate(5.0, "+", 3.0);
        assert_eq!(result, 8.0);
    }
    
    #[test]
    fn test_subtraction() {
        let result = calculate(10.0, "-", 4.0);
        assert_eq!(result, 6.0);
    }
    
    #[test]
    fn test_multiplication() {
        let result = calculate(6.0, "*", 7.0);
        assert_eq!(result, 42.0);
    }
    
    #[test]
    fn test_division() {
        let result = calculate(15.0, "/", 3.0);
        assert_eq!(result, 5.0);
    }
    
    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), 42.0);
        assert_eq!(parse_number("3.14"), 3.14);
    }
    
    // Uncomment when you implement the helper functions
    // #[test]
    // fn test_helper_functions() {
    //     assert_eq!(add(2.0, 3.0), 5.0);
    //     assert_eq!(subtract(10.0, 4.0), 6.0);
    //     assert_eq!(multiply(3.0, 4.0), 12.0);
    //     assert_eq!(divide(8.0, 2.0), 4.0);
    // }
}

// LEARNING OBJECTIVES:
// 1. Function definition syntax: fn name(params) -> return_type
// 2. Pattern matching with match statements
// 3. String parsing with .parse() method
// 4. Basic error handling with .expect()
// 5. Command line argument parsing
// 6. Writing and running tests
//
// C# COMPARISON:
// C#: public double Add(double a, double b) { return a + b; }
// Rust: fn add(a: f64, b: f64) -> f64 { a + b }
//
// C#: double.Parse(str)
// Rust: str.parse::<f64>().expect("message")
//
// C#: switch (op) { case "+": return a + b; }
// Rust: match op { "+" => a + b, }
//
// COMPILATION ERRORS TO EXPECT AND FIX:
// 1. "cannot find function `parse_number`" - implement the function
// 2. "cannot find function `calculate`" - implement the function  
// 3. "`todo!` not yet implemented" - replace with actual code
// 4. Pattern match errors - complete the match arms
//
// SUCCESS CRITERIA:
// ✅ `cargo build` compiles without errors
// ✅ `cargo test` passes all tests
// ✅ `cargo run 5 + 3` outputs "5 + 3 = 8"
// ✅ All four operations work correctly
