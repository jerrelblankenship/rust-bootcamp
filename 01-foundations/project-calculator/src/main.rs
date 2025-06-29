// CLI Calculator - Fix the Broken Code!
//
// Your mission: Make this calculator compile and work correctly by fixing
// all the compilation errors and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one
// 3. Run `cargo build` frequently to check your progress
// 4. Run `cargo test` to verify your implementations
// 5. Check the solutions/ directory if you get completely stuck

use std::env;
use std::fmt;

// TODO: Define the Operation enum
// HINT: What operations should a calculator support?
// FIXME: This enum is missing - add the variants!
enum Operation {
    // TODO: Add operation variants here
    // What operations do calculators typically support?
}

impl Operation {
    // FIXME: This method doesn't compile - implement it!
    fn symbol(&self) -> &str {
        // TODO: Return the symbol string for each operation
        // HINT: Use a match statement like in the exercises
        todo!("Return the symbol (+, -, *, /) for each operation")
    }
    
    // FIXME: This function is missing implementation
    fn from_str(s: &str) -> Result<Operation, CalculatorError> {
        // TODO: Parse a string into an Operation
        // HINT: Match on different string patterns
        // Should support both symbols (+, -, *, /) and words (add, sub, etc.)
        todo!("Parse string into Operation enum")
    }
}

// TODO: Define the Expression struct
// HINT: What data does a mathematical expression need?
// Think: "5 + 3" - what are the components?
struct Expression {
    // TODO: Add fields to represent a mathematical expression
    // HINT: You need the left number, operation, and right number
}

impl Expression {
    // FIXME: Constructor is missing parameters
    fn new(/* TODO: add parameters */) -> Self {
        // TODO: Create a new Expression
        todo!("Create new Expression with given values")
    }
    
    // FIXME: This method won't compile without the struct fields
    fn display(&self) -> String {
        // TODO: Return a formatted string like "5 + 3"
        // HINT: Use format! macro and the operation's symbol
        todo!("Format expression as string")
    }
    
    // FIXME: Implement the actual calculation logic
    fn calculate(&self) -> Result<f64, CalculatorError> {
        // TODO: Perform the calculation based on the operation
        // IMPORTANT: Handle division by zero!
        // HINT: Use a match statement on self.operation
        match self.operation {
            // TODO: Implement each operation
            // Remember to handle division by zero case!
        }
        todo!("Implement calculation logic")
    }
}

// TODO: Define error types for the calculator
// HINT: What kinds of errors can happen? Invalid input? Division by zero?
#[derive(Debug, Clone, PartialEq)]
enum CalculatorError {
    // TODO: Add error variants
    // HINT: Think about what can go wrong:
    // - Invalid input format
    // - Invalid operation
    // - Parse errors  
    // - Division by zero
}

// FIXME: This implementation is missing
impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Implement Display trait for nice error messages
        // HINT: Match on self and write appropriate messages
        todo!("Implement error display messages")
    }
}

// FIXME: This trait implementation is missing
impl std::error::Error for CalculatorError {}

// Type alias for cleaner Result types (this one is correct!)
type Result<T> = std::result::Result<T, CalculatorError>;

// TODO: Define Calculator struct for state management
struct Calculator {
    // TODO: Add field to store calculation history
    // HINT: Use Vec to store previous calculations
}

impl Calculator {
    // FIXME: Constructor is incomplete
    fn new() -> Self {
        // TODO: Create a new Calculator with empty history
        todo!("Create new Calculator")
    }
    
    // FIXME: This method doesn't compile due to missing fields
    fn calculate(&mut self, expression: Expression) -> Result<f64> {
        // TODO: Calculate the result and add to history
        // HINT: 
        // 1. Call expression.calculate()
        // 2. If successful, add to history
        // 3. Return the result
        todo!("Calculate expression and store in history")
    }
    
    // FIXME: This method won't work without history field
    fn print_history(&self) {
        // TODO: Print all calculations in history
        // HINT: Check if history is empty first
        todo!("Print calculation history")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Handle help requests (this part works!)
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }
    
    // TODO: Add version handling
    // HINT: Check for "--version" or "-v" flags
    
    // TODO: Add interactive mode handling  
    // HINT: Check for "--interactive" or "-i" flags
    
    // Parse and execute single calculation
    match parse_args(&args) {
        Ok(expression) => {
            // FIXME: This won't compile until Calculator is implemented
            let mut calculator = Calculator::new();
            match calculator.calculate(expression.clone()) {
                Ok(result) => {
                    // TODO: Print the result nicely
                    // HINT: Use expression.display() and the result
                    println!("TODO: Print calculation result");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nUse 'calculator --help' for usage information");
            std::process::exit(1);
        }
    }
}

// FIXME: This function has several compilation errors
fn parse_args(args: &[String]) -> Result<Expression> {
    // TODO: Check argument count (should be 4: program name + 3 args)
    // FIXME: What error should we return for wrong number of args?
    
    // TODO: Parse the arguments
    // args[1] = left number
    // args[2] = operation  
    // args[3] = right number
    
    // HINT: You'll need parse_number() and Operation::from_str()
    todo!("Parse command line arguments into Expression")
}

// FIXME: This function is missing implementation
fn parse_number(s: &str) -> Result<f64> {
    // TODO: Parse string to f64
    // HINT: Use s.parse() and handle the error
    // Convert ParseFloatError to CalculatorError::ParseError
    todo!("Parse string to number")
}

// TODO: Implement interactive mode (stretch goal)
fn interactive_mode() {
    println!("🧮 Interactive Calculator Mode");
    println!("Enter calculations like: 5 + 3");
    println!("Commands: 'help', 'quit'");
    println!();
    
    // TODO: Implement REPL loop
    // HINT: Use std::io::stdin().read_line()
    // Handle commands and calculations
    todo!("Implement interactive calculator mode")
}

fn print_help() {
    println!("🧮 Calculator - A Rust command-line calculator");
    println!();
    println!("USAGE:");
    println!("    calculator <number> <operation> <number>");
    println!("    calculator [OPTIONS]");
    println!();
    println!("OPERATIONS:");
    println!("    +, add          Addition");
    println!("    -, sub          Subtraction");
    println!("    *, x, mul       Multiplication");
    println!("    /, div          Division");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help         Show this help message");
    println!("    -v, --version      Show version information");
    println!("    -i, --interactive  Start interactive mode");
    println!();
    println!("EXAMPLES:");
    println!("    calculator 5 + 3");
    println!("    calculator 10.5 - 2.5");
    println!("    calculator 4 x 5");
    println!("    calculator 20 / 4");
}

// BONUS: Implement these utility functions for extra features
// fn is_prime(n: u64) -> bool {
//     todo!("Check if number is prime")
// }

// fn is_perfect_square(n: u64) -> bool {
//     todo!("Check if number is perfect square")  
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    // FIXME: These tests won't compile until you implement the code
    #[test]
    fn test_operation_from_str() {
        // TODO: Test that Operation::from_str works correctly
        // assert_eq!(Operation::from_str("+").unwrap(), Operation::Add);
        // Add more test cases...
    }
    
    #[test]
    fn test_expression_calculation() {
        // TODO: Test that calculations work correctly
        // let expr = Expression::new(5.0, Operation::Add, 3.0);
        // assert_eq!(expr.calculate().unwrap(), 8.0);
        // Add more test cases...
    }
    
    #[test]
    fn test_division_by_zero() {
        // TODO: Test that division by zero returns an error
        // let expr = Expression::new(10.0, Operation::Divide, 0.0);
        // assert!(expr.calculate().is_err());
    }
    
    #[test]
    fn test_parse_number() {
        // TODO: Test number parsing
        // assert_eq!(parse_number("42").unwrap(), 42.0);
        // assert!(parse_number("not_a_number").is_err());
    }
    
    // TODO: Add more tests as you implement functionality
}

// COMPILATION CHALLENGES:
// 1. Start by defining the Operation enum variants
// 2. Implement Operation::symbol() method  
// 3. Define Expression struct fields
// 4. Implement Expression::new() constructor
// 5. Define CalculatorError variants
// 6. Implement the calculation logic
// 7. Complete the Calculator struct and methods
// 8. Implement parse_args and parse_number functions
// 9. Fix the main function to use your implementations
// 10. Make all tests pass!
//
// LEARNING OBJECTIVES:
// - Enum definition and pattern matching
// - Struct definition and implementation
// - Error handling with Result types
// - Method vs associated function syntax
// - String parsing and error conversion
// - Command-line argument processing
// - Test-driven development
//
// Remember: It's okay to get stuck! Check the solutions/ directory for help,
// and don't forget to leverage your C# knowledge - many concepts are similar!
