// CLI Calculator - COMPLETE SOLUTION
//
// This is the complete working implementation that students will build
// through fixing broken code and implementing missing functionality.

use std::env;
use std::fmt;

// Define the Operation enum with all supported operations
#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn symbol(&self) -> &str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        }
    }
    
    fn from_str(s: &str) -> Result<Operation, CalculatorError> {
        match s {
            "+" | "add" => Ok(Operation::Add),
            "-" | "sub" | "subtract" => Ok(Operation::Subtract),
            "*" | "x" | "mul" | "multiply" => Ok(Operation::Multiply),
            "/" | "div" | "divide" => Ok(Operation::Divide),
            _ => Err(CalculatorError::InvalidOperation(s.to_string())),
        }
    }
}

// Define the Expression struct to hold calculation data
#[derive(Debug, Clone)]
struct Expression {
    left: f64,
    operation: Operation,
    right: f64,
}

impl Expression {
    fn new(left: f64, operation: Operation, right: f64) -> Self {
        Expression { left, operation, right }
    }
    
    fn display(&self) -> String {
        format!("{} {} {}", self.left, self.operation.symbol(), self.right)
    }
    
    fn calculate(&self) -> Result<f64, CalculatorError> {
        match self.operation {
            Operation::Add => Ok(self.left + self.right),
            Operation::Subtract => Ok(self.left - self.right),
            Operation::Multiply => Ok(self.left * self.right),
            Operation::Divide => {
                if self.right == 0.0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    Ok(self.left / self.right)
                }
            }
        }
    }
}

// Define custom error types for better error handling
#[derive(Debug, Clone, PartialEq)]
enum CalculatorError {
    InvalidInput(String),
    InvalidOperation(String),
    ParseError(String),
    DivisionByZero,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CalculatorError::InvalidOperation(op) => write!(f, "Invalid operation: '{}'", op),
            CalculatorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "Division by zero is not allowed"),
        }
    }
}

impl std::error::Error for CalculatorError {}

// Type alias for cleaner Result types
type Result<T> = std::result::Result<T, CalculatorError>;

// Calculator struct to hold state and methods
struct Calculator {
    history: Vec<(Expression, f64)>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }
    
    fn calculate(&mut self, expression: Expression) -> Result<f64> {
        let result = expression.calculate()?;
        self.history.push((expression, result));
        Ok(result)
    }
    
    fn print_history(&self) {
        if self.history.is_empty() {
            println!("No calculations in history.");
            return;
        }
        
        println!("Calculation History:");
        for (i, (expr, result)) in self.history.iter().enumerate() {
            println!("  {}. {} = {}", i + 1, expr.display(), result);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Handle help requests
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }
    
    // Handle version requests
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-v") {
        println!("Calculator v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    // Handle interactive mode
    if args.len() == 2 && (args[1] == "--interactive" || args[1] == "-i") {
        interactive_mode();
        return;
    }
    
    // Parse and execute single calculation
    match parse_args(&args) {
        Ok(expression) => {
            let mut calculator = Calculator::new();
            match calculator.calculate(expression.clone()) {
                Ok(result) => {
                    println!("{} = {}", expression.display(), result);
                    
                    // Show additional info for interesting results
                    if result.fract() == 0.0 && result.abs() <= 1000.0 {
                        let int_result = result as i64;
                        if is_prime(int_result.abs() as u64) {
                            println!("  ✨ {} is a prime number!", int_result);
                        }
                        if is_perfect_square(int_result.abs() as u64) {
                            println!("  📐 {} is a perfect square!", int_result);
                        }
                    }
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

fn parse_args(args: &[String]) -> Result<Expression> {
    if args.len() != 4 {
        return Err(CalculatorError::InvalidInput(
            "Expected format: calculator <number> <operation> <number>".to_string()
        ));
    }
    
    let left = parse_number(&args[1])?;
    let operation = Operation::from_str(&args[2])?;
    let right = parse_number(&args[3])?;
    
    Ok(Expression::new(left, operation, right))
}

fn parse_number(s: &str) -> Result<f64> {
    s.parse::<f64>()
        .map_err(|_| CalculatorError::ParseError(
            format!("'{}' is not a valid number", s)
        ))
}

fn interactive_mode() {
    println!("🧮 Interactive Calculator Mode");
    println!("Enter calculations like: 5 + 3");
    println!("Commands: 'history', 'clear', 'help', 'quit'");
    println!();
    
    let mut calculator = Calculator::new();
    
    loop {
        print!("calc> ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                // Handle commands
                match input.to_lowercase().as_str() {
                    "quit" | "exit" | "q" => {
                        println!("Goodbye! 👋");
                        break;
                    }
                    "help" | "h" => {
                        print_interactive_help();
                        continue;
                    }
                    "history" => {
                        calculator.print_history();
                        continue;
                    }
                    "clear" => {
                        calculator.history.clear();
                        println!("History cleared.");
                        continue;
                    }
                    "" => continue,
                    _ => {}
                }
                
                // Parse and execute calculation
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 3 {
                    println!("Error: Expected format: <number> <operation> <number>");
                    continue;
                }
                
                match parse_expression_parts(&parts) {
                    Ok(expression) => {
                        match calculator.calculate(expression.clone()) {
                            Ok(result) => {
                                println!("{} = {}", expression.display(), result);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn parse_expression_parts(parts: &[&str]) -> Result<Expression> {
    if parts.len() != 3 {
        return Err(CalculatorError::InvalidInput(
            "Expected format: <number> <operation> <number>".to_string()
        ));
    }
    
    let left = parse_number(parts[0])?;
    let operation = Operation::from_str(parts[1])?;
    let right = parse_number(parts[2])?;
    
    Ok(Expression::new(left, operation, right))
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
    println!("    calculator --interactive");
    println!();
    println!("FEATURES:");
    println!("    • Floating-point arithmetic");
    println!("    • Division by zero protection");
    println!("    • Interactive mode with history");
    println!("    • Prime number detection");
    println!("    • Perfect square detection");
}

fn print_interactive_help() {
    println!("Interactive Mode Commands:");
    println!("  <num> <op> <num>  - Perform calculation (e.g., 5 + 3)");
    println!("  history           - Show calculation history");
    println!("  clear             - Clear history");
    println!("  help              - Show this help");
    println!("  quit, exit, q     - Exit calculator");
}

// Utility functions for interesting number properties
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_perfect_square(n: u64) -> bool {
    let root = (n as f64).sqrt() as u64;
    root * root == n
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_operation_from_str() {
        assert_eq!(Operation::from_str("+").unwrap(), Operation::Add);
        assert_eq!(Operation::from_str("add").unwrap(), Operation::Add);
        assert_eq!(Operation::from_str("-").unwrap(), Operation::Subtract);
        assert_eq!(Operation::from_str("*").unwrap(), Operation::Multiply);
        assert_eq!(Operation::from_str("x").unwrap(), Operation::Multiply);
        assert_eq!(Operation::from_str("/").unwrap(), Operation::Divide);
        
        assert!(Operation::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_expression_calculation() {
        let expr = Expression::new(5.0, Operation::Add, 3.0);
        assert_eq!(expr.calculate().unwrap(), 8.0);
        
        let expr = Expression::new(10.0, Operation::Subtract, 4.0);
        assert_eq!(expr.calculate().unwrap(), 6.0);
        
        let expr = Expression::new(3.0, Operation::Multiply, 4.0);
        assert_eq!(expr.calculate().unwrap(), 12.0);
        
        let expr = Expression::new(15.0, Operation::Divide, 3.0);
        assert_eq!(expr.calculate().unwrap(), 5.0);
        
        // Test division by zero
        let expr = Expression::new(10.0, Operation::Divide, 0.0);
        assert!(expr.calculate().is_err());
    }
    
    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42").unwrap(), 42.0);
        assert_eq!(parse_number("3.14").unwrap(), 3.14);
        assert_eq!(parse_number("-5").unwrap(), -5.0);
        
        assert!(parse_number("not_a_number").is_err());
    }
    
    #[test]
    fn test_expression_display() {
        let expr = Expression::new(5.0, Operation::Add, 3.0);
        assert_eq!(expr.display(), "5 + 3");
        
        let expr = Expression::new(10.5, Operation::Multiply, 2.0);
        assert_eq!(expr.display(), "10.5 * 2");
    }
    
    #[test]
    fn test_calculator_history() {
        let mut calc = Calculator::new();
        
        let expr1 = Expression::new(5.0, Operation::Add, 3.0);
        let result1 = calc.calculate(expr1).unwrap();
        assert_eq!(result1, 8.0);
        
        let expr2 = Expression::new(10.0, Operation::Multiply, 2.0);
        let result2 = calc.calculate(expr2).unwrap();
        assert_eq!(result2, 20.0);
        
        assert_eq!(calc.history.len(), 2);
    }
    
    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(!is_prime(21));
    }
    
    #[test]
    fn test_is_perfect_square() {
        assert!(is_perfect_square(0));
        assert!(is_perfect_square(1));
        assert!(!is_perfect_square(2));
        assert!(!is_perfect_square(3));
        assert!(is_perfect_square(4));
        assert!(!is_perfect_square(5));
        assert!(is_perfect_square(9));
        assert!(is_perfect_square(16));
        assert!(is_perfect_square(25));
        assert!(!is_perfect_square(26));
    }
    
    #[test]
    fn test_error_display() {
        let error = CalculatorError::DivisionByZero;
        assert_eq!(format!("{}", error), "Division by zero is not allowed");
        
        let error = CalculatorError::InvalidOperation("xyz".to_string());
        assert_eq!(format!("{}", error), "Invalid operation: 'xyz'");
    }
}
