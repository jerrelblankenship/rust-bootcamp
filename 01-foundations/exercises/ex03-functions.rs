// Exercise 3: Functions and Control Flow - Fix the Broken Code!
//
// Your task: Make all the functions compile and work correctly by fixing
// all the compilation errors and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one
// 3. Run `rustc ex03-functions.rs && ./ex03-functions` to test
// 4. Uncomment test sections as you implement each function
// 5. Use pattern matching and control flow to solve the challenges

use std::cmp::Ordering;

fn main() {
    println!("=== Exercise 3: Functions and Control Flow (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each section one by one as you implement the functions
    
    // Section 1: Basic functions
    println!("Testing basic functions...");
    test_basic_functions();
    
    // TODO: Uncomment these sections as you implement the functions
    // test_conditionals();
    // test_loops();
    // test_pattern_matching();
    // test_advanced_patterns();
    
    println!("\n🎉 All exercises completed successfully!");
}

fn test_basic_functions() {
    // Exercise 3.1: Test basic addition
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    // Exercise 3.2: Test volume calculation
    let volume = calculate_volume(2.0, 3.0, 4.0);
    println!("Volume of 2x3x4 box: {}", volume);
}

fn test_conditionals() {
    // Exercise 3.3: Test safe division
    match safe_divide(10.0, 2.0) {
        Some(result) => println!("10.0 / 2.0 = {}", result),
        None => println!("Division by zero!"),
    }
    
    match safe_divide(10.0, 0.0) {
        Some(result) => println!("10.0 / 0.0 = {}", result),
        None => println!("Division by zero detected!"),
    }
    
    // Exercise 3.4: Test grade conversion
    let grades = [95, 85, 75, 65, 55, 101];
    for grade in grades {
        println!("Grade {}: {}", grade, number_to_letter_grade(grade));
    }
    
    // Exercise 3.5: Test leap year
    let years = [2000, 2020, 2021, 1900, 2024];
    for year in years {
        println!("{} is {}a leap year", year, if is_leap_year(year) { "" } else { "not " });
    }
}

fn test_loops() {
    // Exercise 3.6: Test factorial
    for n in 0..=10 {
        println!("{}! = {}", n, factorial(n));
    }
    
    // Exercise 3.7: Test next power of two
    for n in [1, 7, 15, 16, 100] {
        println!("Next power of 2 after {}: {}", n, next_power_of_two(n));
    }
    
    // Exercise 3.8: Test sum even numbers
    for n in [1, 10, 20] {
        println!("Sum of even numbers 1 to {}: {}", n, sum_even_numbers(n));
    }
}

fn test_pattern_matching() {
    // Exercise 3.9: Test calculator operations
    let operations = [
        Operation::Add(5.0, 3.0),
        Operation::Subtract(10.0, 4.0),
        Operation::Multiply(3.0, 7.0),
        Operation::Divide(15.0, 3.0),
        Operation::Divide(10.0, 0.0),
    ];
    
    for op in operations {
        match calculate(op) {
            Ok(result) => println!("{:?} = {}", op, result),
            Err(e) => println!("{:?} -> Error: {}", op, e),
        }
    }
    
    // Exercise 3.10: Test message processing
    let messages = [
        Message::Text("Hello, World!".to_string()),
        Message::Number(42),
        Message::Number(13),
        Message::Position { x: 5, y: 3 },
        Message::Position { x: -2, y: -4 },
        Message::Position { x: 0, y: 0 },
        Message::Color(128, 128, 128),
        Message::Color(255, 0, 0),
    ];
    
    for msg in messages {
        process_message(msg);
    }
}

fn test_advanced_patterns() {
    // Exercise 3.11: Test if let
    print_some_number(Some(42));
    print_some_number(Some(-5));
    print_some_number(None);
    
    // Exercise 3.12: Test number categorization
    let numbers = [-5, 0, 1, 7, 15, 50, 150];
    for n in numbers {
        println!("{} is {}", n, categorize_number(n));
    }
    
    // Exercise 3.13: Test perfect square
    for n in [1, 8, 15, 16, 25, 30] {
        println!("Next perfect square after {}: {}", n, next_perfect_square(n));
    }
}

// Exercise 3.1: Basic function - BROKEN CODE TO FIX!
fn add(a: i32, b: i32) -> i32 {
    // TODO: Implement addition function
    // HINT: Return the sum of a and b
    // C# equivalent: public int Add(int a, int b) => a + b;
    todo!("Implement addition")
}

// Exercise 3.2: Function with multiple parameters - BROKEN CODE TO FIX!
fn calculate_volume(length: f64, width: f64, height: f64) -> f64 {
    // TODO: Calculate volume by multiplying dimensions
    // HINT: Volume = length × width × height
    // QUESTION: What's the return type for floating point numbers?
    todo!("Implement volume calculation")
}

// Exercise 3.3: Early return with Option - BROKEN CODE TO FIX!
fn safe_divide(dividend: f64, divisor: f64) -> Option<f64> {
    // TODO: Implement safe division that returns None for division by zero
    // HINT: Check if divisor is 0.0, return None if so, otherwise Some(result)
    // HINT: Use early return with 'return' keyword or if/else expression
    // C# equivalent: public double? SafeDivide(double a, double b) 
    //                => b == 0 ? null : a / b;
    todo!("Implement safe division")
}

// Exercise 3.4: Pattern matching with match - BROKEN CODE TO FIX!
fn number_to_letter_grade(grade: u8) -> &'static str {
    // TODO: Convert numeric grade to letter grade using match
    // HINT: Use match with range patterns
    // 90-100 => "A", 80-89 => "B", 70-79 => "C", 60-69 => "D", 0-59 => "F", other => "Invalid"
    // HINT: Range patterns look like: 90..=100 => "A"
    // C# equivalent: grade switch { >= 90 => "A", >= 80 => "B", ... }
    todo!("Implement grade conversion")
}

// Exercise 3.5: If expressions - BROKEN CODE TO FIX!
fn is_leap_year(year: u32) -> bool {
    // TODO: Implement leap year logic using if expressions
    // HINT: A year is a leap year if:
    // - It's divisible by 4 AND not divisible by 100, OR
    // - It's divisible by 400
    // HINT: Use % operator for modulo and && || for logic
    // C# equivalent: (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    todo!("Implement leap year check")
}

// Exercise 3.6: Loops with for - BROKEN CODE TO FIX!
fn factorial(n: u32) -> u64 {
    // TODO: Calculate factorial using a loop
    // HINT: Handle the special case n == 0 (factorial is 1)
    // HINT: Use a for loop with range 1..=n
    // HINT: Use a mutable variable to accumulate the result
    // C# equivalent: 
    // if (n == 0) return 1;
    // ulong result = 1;
    // for (uint i = 1; i <= n; i++) result *= i;
    // return result;
    todo!("Implement factorial")
}

// Exercise 3.7: While loop - BROKEN CODE TO FIX!
fn next_power_of_two(n: u32) -> u32 {
    // TODO: Find the next power of 2 greater than n using while loop
    // HINT: Start with power = 1, keep doubling while power <= n
    // HINT: Use mut for variables that change
    // C# equivalent:
    // uint power = 1;
    // while (power <= n) power *= 2;
    // return power;
    todo!("Implement next power of two")
}

// Exercise 3.8: For loop with range and conditionals - BROKEN CODE TO FIX!
fn sum_even_numbers(n: u32) -> u32 {
    // TODO: Sum all even numbers from 1 to n (inclusive)
    // HINT: Use for loop with range 1..=n
    // HINT: Use if condition to check if number is even (i % 2 == 0)
    // HINT: Use mutable variable to accumulate sum
    // C# equivalent:
    // uint sum = 0;
    // for (uint i = 1; i <= n; i++)
    //     if (i % 2 == 0) sum += i;
    // return sum;
    todo!("Implement sum of even numbers")
}

// Exercise 3.9: Pattern matching with enums - BROKEN CODE TO FIX!
#[derive(Debug, PartialEq)]
enum Operation {
    // TODO: Define enum variants for calculator operations
    // HINT: Each variant should hold two f64 values
    // Add(f64, f64), Subtract(f64, f64), etc.
    // FIXME: Add the missing variants
}

fn calculate(op: Operation) -> Result<f64, String> {
    // TODO: Implement calculator using match on Operation enum
    // HINT: Match each Operation variant and perform the calculation
    // HINT: Return Err for division by zero
    // C# equivalent: 
    // return op switch {
    //     Add(a, b) => a + b,
    //     Divide(a, 0) => throw new DivideByZeroException(),
    //     ...
    // };
    todo!("Implement calculator operations")
}

// Exercise 3.10: Nested patterns - BROKEN CODE TO FIX!
#[derive(Debug)]
enum Message {
    // TODO: Define message types
    // HINT: Text(String), Number(i32), Position { x: i32, y: i32 }, Color(u8, u8, u8)
    // FIXME: Add the missing variants
}

fn process_message(msg: Message) {
    // TODO: Process different message types using match
    // HINT: For Text - just print it
    // HINT: For Number - check if even/odd
    // HINT: For Position - determine quadrant (I, II, III, IV, or axes)
    // HINT: For Color - check if grayscale (r == g == b)
    todo!("Implement message processing")
}

// Exercise 3.11: If let patterns - BROKEN CODE TO FIX!
fn print_some_number(opt: Option<i32>) {
    // TODO: Use if let to handle Some values, print different message for None
    // HINT: if let Some(n) = opt { ... } else { ... }
    // HINT: For Some values, check if positive or non-positive
    // C# equivalent: if (opt.HasValue) { ... } else { ... }
    todo!("Implement if let pattern")
}

// Exercise 3.12: Match with guards - BROKEN CODE TO FIX!
fn categorize_number(n: i32) -> &'static str {
    // TODO: Categorize numbers using match with guard clauses
    // HINT: Use 'if' guards in match arms: n if n < 0 => "negative"
    // Categories: negative, zero, small positive (1-10), medium positive (11-100), large positive (>100)
    // C# equivalent: 
    // return n switch {
    //     < 0 => "negative",
    //     0 => "zero",
    //     > 0 and <= 10 => "small positive",
    //     ...
    // };
    todo!("Implement number categorization")
}

// Exercise 3.13: Loop with break value - BROKEN CODE TO FIX!
fn next_perfect_square(n: u32) -> u32 {
    // TODO: Find next perfect square using loop with break value
    // HINT: Use loop { ... } and break with a value
    // HINT: Calculate i * i and break when it's > n
    // C# equivalent:
    // for (uint i = 1; ; i++) {
    //     uint square = i * i;
    //     if (square > n) return square;
    // }
    todo!("Implement next perfect square")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        // TODO: Uncomment when you implement add()
        // assert_eq!(add(2, 3), 5);
        // assert_eq!(add(-1, 1), 0);
        // assert_eq!(add(0, 0), 0);
    }
    
    #[test]
    fn test_calculate_volume() {
        // TODO: Uncomment when you implement calculate_volume()
        // assert_eq!(calculate_volume(2.0, 3.0, 4.0), 24.0);
        // assert_eq!(calculate_volume(1.0, 1.0, 1.0), 1.0);
    }
    
    #[test]
    fn test_safe_divide() {
        // TODO: Uncomment when you implement safe_divide()
        // assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
        // assert_eq!(safe_divide(10.0, 0.0), None);
        // assert_eq!(safe_divide(-10.0, 2.0), Some(-5.0));
    }
    
    #[test]
    fn test_letter_grade() {
        // TODO: Uncomment when you implement number_to_letter_grade()
        // assert_eq!(number_to_letter_grade(95), "A");
        // assert_eq!(number_to_letter_grade(85), "B");
        // assert_eq!(number_to_letter_grade(75), "C");
        // assert_eq!(number_to_letter_grade(65), "D");
        // assert_eq!(number_to_letter_grade(55), "F");
        // assert_eq!(number_to_letter_grade(101), "Invalid");
    }
    
    #[test]
    fn test_leap_year() {
        // TODO: Uncomment when you implement is_leap_year()
        // assert!(is_leap_year(2000));
        // assert!(is_leap_year(2024));
        // assert!(!is_leap_year(2023));
        // assert!(!is_leap_year(1900));
        // assert!(is_leap_year(2004));
    }
    
    #[test]
    fn test_factorial() {
        // TODO: Uncomment when you implement factorial()
        // assert_eq!(factorial(0), 1);
        // assert_eq!(factorial(1), 1);
        // assert_eq!(factorial(5), 120);
        // assert_eq!(factorial(10), 3628800);
    }
    
    #[test]
    fn test_next_power_of_two() {
        // TODO: Uncomment when you implement next_power_of_two()
        // assert_eq!(next_power_of_two(1), 2);
        // assert_eq!(next_power_of_two(7), 8);
        // assert_eq!(next_power_of_two(8), 16);
        // assert_eq!(next_power_of_two(15), 16);
    }
    
    #[test]
    fn test_sum_even_numbers() {
        // TODO: Uncomment when you implement sum_even_numbers()
        // assert_eq!(sum_even_numbers(1), 0);
        // assert_eq!(sum_even_numbers(2), 2);
        // assert_eq!(sum_even_numbers(10), 30); // 2+4+6+8+10
    }
    
    #[test]
    fn test_calculate_operation() {
        // TODO: Uncomment when you implement Operation enum and calculate()
        // assert_eq!(calculate(Operation::Add(5.0, 3.0)), Ok(8.0));
        // assert_eq!(calculate(Operation::Subtract(10.0, 4.0)), Ok(6.0));
        // assert_eq!(calculate(Operation::Multiply(3.0, 4.0)), Ok(12.0));
        // assert_eq!(calculate(Operation::Divide(10.0, 2.0)), Ok(5.0));
        // assert!(calculate(Operation::Divide(10.0, 0.0)).is_err());
    }
    
    #[test]
    fn test_categorize_number() {
        // TODO: Uncomment when you implement categorize_number()
        // assert_eq!(categorize_number(-5), "negative");
        // assert_eq!(categorize_number(0), "zero");
        // assert_eq!(categorize_number(5), "small positive");
        // assert_eq!(categorize_number(50), "medium positive");
        // assert_eq!(categorize_number(150), "large positive");
    }
    
    #[test]
    fn test_next_perfect_square() {
        // TODO: Uncomment when you implement next_perfect_square()
        // assert_eq!(next_perfect_square(1), 4);
        // assert_eq!(next_perfect_square(8), 9);
        // assert_eq!(next_perfect_square(9), 16);
        // assert_eq!(next_perfect_square(15), 16);
        // assert_eq!(next_perfect_square(16), 25);
    }
}

// COMPILATION CHALLENGES:
// 1. Start with basic functions (add, calculate_volume)
// 2. Implement conditional logic (safe_divide, is_leap_year)
// 3. Learn pattern matching (number_to_letter_grade)
// 4. Master loops (factorial, next_power_of_two, sum_even_numbers)
// 5. Define and match enums (Operation, Message)
// 6. Use advanced patterns (guards, if let, loop with break value)
// 7. Handle error cases (division by zero, invalid input)
// 8. Understand Rust expressions vs statements
//
// LEARNING OBJECTIVES FOR C# DEVELOPERS:
// - Functions in Rust: fn keyword, parameter types, return types
// - Pattern matching: match expressions (like C# switch expressions but more powerful)
// - Loops: for, while, loop (different from C# foreach syntax)
// - Option<T>: Rust's null-safe alternative (like C# nullable types)
// - Result<T, E>: Error handling without exceptions
// - Enums: More powerful than C# enums, can hold data
// - Expression-based language: Most things return values
// - Immutability by default: mut keyword needed for changes
//
// KEY DIFFERENCES FROM C#:
// - No null references - use Option<T>
// - No exceptions - use Result<T, E>  
// - Pattern matching more powerful than switch
// - Immutable by default
// - Expression-based (if, match, loops can return values)
// - No method overloading (use different names or generics)
