// Exercise 5: Advanced Patterns - Fix the Broken Code!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (10 pattern challenges to fix)
//
// Your task: Make this advanced pattern-matching code compile and work correctly
// by fixing all the compilation errors and implementing missing functionality.
//
// INSTRUCTIONS:
// 1. Fix ONE error at a time - these are advanced concepts building on Module 01
// 2. Compile after each fix: `rustc ex05-advanced-patterns.rs`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (the enum definition error)
// - Each checkpoint builds on the previous one
// - This exercise combines everything from Module 01 in advanced ways
//
// COMPLETED CONCEPTS:
// [] Enum definitions with data
// [] Pattern matching with guards
// [] Option and Result handling
// [] Iterator methods with closures
// [] Complex struct implementations
// [] Error propagation
// [] Method chaining
// [] Advanced match expressions
// [] Custom implementations
// [] Real-world patterns

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 5: Advanced Patterns (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each exercise one by one as you fix the errors
    exercise_5_1();
    // exercise_5_2();
    // exercise_5_3();
    // exercise_5_4();
    // exercise_5_5();
    // exercise_5_6();
    // exercise_5_7();
    // exercise_5_8();
    // exercise_5_9();
    // exercise_5_10();
    
    println!("\n🎉 All advanced pattern exercises completed!");
}

// Exercise 5.1: Fix the enum definition - BROKEN CODE TO FIX!
fn exercise_5_1() {
    println!("Exercise 5.1: Fix enum definition");
    
    // FIXME: This enum doesn't compile - what's missing?
    enum Message {
        // TODO: Add message variants that can hold data
        // HINT: Text(String), Move { x: i32, y: i32 }, ChangeColor(u8, u8, u8), Quit
    }  // COMPILE ERROR: enum has no variants!
    
    // FIXME: This won't compile until the enum is defined
    let msg = Message::Text("Hello".to_string());  // COMPILE ERROR!
    process_message(msg);  // COMPILE ERROR!
    
    println!("✅ Exercise 5.1 complete\n");
    
    // 📊 CHECKPOINT 1: When this compiles, uncomment exercise_5_2() in main()
}

// Exercise 5.2: Fix pattern matching - BROKEN CODE TO FIX!
fn exercise_5_2() {
    println!("Exercise 5.2: Fix pattern matching");
    
    let messages = vec![
        Message::Text("Hello World".to_string()),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];
    
    for msg in messages {
        // FIXME: This function doesn't exist yet
        process_message(msg);  // COMPILE ERROR: function not found!
    }
    
    println!("✅ Exercise 5.2 complete\n");
}

// TODO: Implement the missing process_message function
// HINT: Use match to handle each Message variant
fn process_message(msg: Message) {
    // TODO: Match on msg and handle each variant:
    // Text: print "Message: <text>"
    // Move: print "Moving to ({}, {})"
    // ChangeColor: print "Changing color to RGB({}, {}, {})"
    // Quit: print "Goodbye!"
    todo!("Implement message processing")
}

// Exercise 5.3: Fix Option handling with complex patterns - BROKEN CODE TO FIX!
fn exercise_5_3() {
    println!("Exercise 5.3: Fix Option handling");
    
    let numbers = vec![Some(1), None, Some(2), Some(3), None, Some(4)];
    
    // FIXME: This doesn't compile - implement the function
    let sum = sum_some_numbers(&numbers);  // COMPILE ERROR!
    println!("Sum of Some numbers: {}", sum);
    
    // FIXME: This function doesn't exist
    let count = count_some_values(&numbers);  // COMPILE ERROR!
    println!("Count of Some values: {}", count);
    
    println!("✅ Exercise 5.3 complete\n");
}

// TODO: Implement sum_some_numbers function
fn sum_some_numbers(numbers: &[Option<i32>]) -> i32 {
    // TODO: Sum only the Some(n) values, ignore None
    // HINT: Use iterator methods like .iter(), .filter_map(), .sum()
    // HINT: filter_map extracts Some values and filters out None
    todo!("Sum only the Some values")
}

// TODO: Implement count_some_values function  
fn count_some_values(numbers: &[Option<i32>]) -> usize {
    // TODO: Count how many Some values there are
    // HINT: Use .iter(), .filter(), .count()
    todo!("Count the Some values")
}

// Exercise 5.4: Fix Result handling with error propagation - BROKEN CODE TO FIX!
fn exercise_5_4() {
    println!("Exercise 5.4: Fix Result handling");
    
    let strings = vec!["42", "13", "not_a_number", "99"];
    
    // FIXME: These functions don't exist yet
    let results = parse_all_numbers(&strings);  // COMPILE ERROR!
    println!("Parse results: {:?}", results);
    
    let sum_result = sum_parsed_numbers(&strings);  // COMPILE ERROR!
    match sum_result {
        Ok(sum) => println!("Sum of parsed numbers: {}", sum),
        Err(e) => println!("Error parsing: {}", e),
    }
    
    println!("✅ Exercise 5.4 complete\n");
}

// TODO: Implement parse_all_numbers function
fn parse_all_numbers(strings: &[&str]) -> Vec<Result<i32, std::num::ParseIntError>> {
    // TODO: Try to parse each string to i32, collect all Results (both Ok and Err)
    // HINT: Use .iter(), .map(), .collect()
    // HINT: Use s.parse::<i32>() for each string
    todo!("Parse all strings, keeping both successes and failures")
}

// TODO: Implement sum_parsed_numbers function
fn sum_parsed_numbers(strings: &[&str]) -> Result<i32, String> {
    // TODO: Parse all strings and sum only the successful ones
    // Return error if ANY parsing fails
    // HINT: Use .iter(), .map(), .collect::<Result<Vec<_>, _>>(), then .sum()
    // HINT: Use ? operator for early return on first error
    todo!("Sum all numbers, fail if any parse fails")
}

// Exercise 5.5: Fix struct with advanced methods - BROKEN CODE TO FIX!
fn exercise_5_5() {
    println!("Exercise 5.5: Fix struct methods");
    
    // FIXME: This struct doesn't exist yet
    let mut analyzer = TextAnalyzer::new("Hello World! How are you?");  // COMPILE ERROR!
    
    // FIXME: These methods don't exist yet
    println!("Word count: {}", analyzer.word_count());  // COMPILE ERROR!
    println!("Character count: {}", analyzer.char_count());  // COMPILE ERROR!
    println!("Most common word: {:?}", analyzer.most_common_word());  // COMPILE ERROR!
    
    println!("✅ Exercise 5.5 complete\n");
}

// TODO: Define TextAnalyzer struct
struct TextAnalyzer {
    // TODO: What fields do we need to store?
    // HINT: text: String, words: Vec<String>, word_counts: HashMap<String, usize>
}

impl TextAnalyzer {
    // TODO: Implement constructor
    fn new(text: &str) -> Self {
        // TODO: Create TextAnalyzer, process the text into words
        // HINT: Split text on whitespace, count word frequencies
        todo!("Create and initialize TextAnalyzer")
    }
    
    // TODO: Implement word_count method
    fn word_count(&self) -> usize {
        // TODO: Return number of words
        todo!("Return word count")
    }
    
    // TODO: Implement char_count method
    fn char_count(&self) -> usize {
        // TODO: Return number of characters
        todo!("Return character count")
    }
    
    // TODO: Implement most_common_word method
    fn most_common_word(&self) -> Option<&str> {
        // TODO: Return the most frequently used word
        // HINT: Find the entry with maximum count in word_counts HashMap
        todo!("Find most common word")
    }
}

// Exercise 5.6: Fix closure usage - BROKEN CODE TO FIX!
fn exercise_5_6() {
    println!("Exercise 5.6: Fix closure usage");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // FIXME: This function doesn't exist
    let processed = process_with_closure(&numbers, |x| x * x);  // COMPILE ERROR!
    println!("Squared numbers: {:?}", processed);
    
    // FIXME: This function doesn't exist  
    let filtered = filter_with_closure(&numbers, |x| x % 2 == 0);  // COMPILE ERROR!
    println!("Even numbers: {:?}", filtered);
    
    println!("✅ Exercise 5.6 complete\n");
}

// TODO: Implement process_with_closure function
fn process_with_closure<F>(numbers: &[i32], operation: F) -> Vec<i32> 
where 
    F: Fn(i32) -> i32,
{
    // TODO: Apply the operation to each number and collect results
    // HINT: Use .iter(), .map(), .collect()
    todo!("Apply closure to each number")
}

// TODO: Implement filter_with_closure function
fn filter_with_closure<F>(numbers: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    // TODO: Filter numbers using the predicate and collect results
    // HINT: Use .iter(), .filter(), .copied(), .collect()
    todo!("Filter numbers using closure")
}

// Exercise 5.7: Fix complex match with guards - BROKEN CODE TO FIX!
fn exercise_5_7() {
    println!("Exercise 5.7: Fix complex match with guards");
    
    let values = vec![
        (Some(5), Some(10)),
        (Some(3), None),
        (None, Some(7)),
        (None, None),
        (Some(0), Some(0)),
    ];
    
    for pair in values {
        // FIXME: This function doesn't exist
        let result = analyze_pair(pair);  // COMPILE ERROR!
        println!("{:?} -> {}", pair, result);
    }
    
    println!("✅ Exercise 5.7 complete\n");
}

// TODO: Implement analyze_pair function with complex matching
fn analyze_pair(pair: (Option<i32>, Option<i32>)) -> &'static str {
    // TODO: Match on the pair with guards:
    // (Some(a), Some(b)) if a + b > 10 => "High sum"
    // (Some(a), Some(b)) if a == b => "Equal values"  
    // (Some(a), Some(b)) => "Both present"
    // (Some(_), None) => "First only"
    // (None, Some(_)) => "Second only"
    // (None, None) => "Both missing"
    todo!("Implement complex pattern matching with guards")
}

// Exercise 5.8: Fix iterator chaining - BROKEN CODE TO FIX!
fn exercise_5_8() {
    println!("Exercise 5.8: Fix iterator chaining");
    
    let words = vec!["hello", "world", "rust", "programming", "is", "fun"];
    
    // FIXME: This function doesn't exist
    let result = complex_word_processing(&words);  // COMPILE ERROR!
    println!("Processed words: {:?}", result);
    
    println!("✅ Exercise 5.8 complete\n");
}

// TODO: Implement complex_word_processing function
fn complex_word_processing(words: &[&str]) -> Vec<String> {
    // TODO: Chain multiple iterator operations:
    // 1. Filter words longer than 3 characters
    // 2. Map to uppercase
    // 3. Map to add "!" at the end
    // 4. Collect into Vec<String>
    // HINT: .iter(), .filter(), .map(), .map(), .collect()
    todo!("Chain multiple iterator operations")
}

// Exercise 5.9: Fix custom trait implementation - BROKEN CODE TO FIX!
fn exercise_5_9() {
    println!("Exercise 5.9: Fix custom trait implementation");
    
    // FIXME: This trait doesn't exist yet
    let point = Point { x: 3, y: 4 };
    println!("Point distance from origin: {}", point.distance_from_origin());  // COMPILE ERROR!
    
    let rect = Rectangle { width: 5.0, height: 3.0 };
    println!("Rectangle area: {}", rect.area());  // COMPILE ERROR!
    
    println!("✅ Exercise 5.9 complete\n");
}

// TODO: Define the Drawable trait
trait Drawable {
    // TODO: Add methods that shapes should implement
    // fn area(&self) -> f64;
    // fn perimeter(&self) -> f64;
}

// TODO: Define Point struct
struct Point {
    // TODO: Add x and y fields
}

impl Point {
    // TODO: Implement distance_from_origin method
    fn distance_from_origin(&self) -> f64 {
        // TODO: Calculate distance using Pythagorean theorem
        // HINT: (x² + y²).sqrt()
        todo!("Calculate distance from origin")
    }
}

// TODO: Define Rectangle struct  
struct Rectangle {
    // TODO: Add width and height fields
}

// TODO: Implement Drawable for Rectangle
impl Drawable for Rectangle {
    // TODO: Implement area method
    fn area(&self) -> f64 {
        // TODO: width * height
        todo!("Calculate rectangle area")
    }
    
    // TODO: Implement perimeter method
    fn perimeter(&self) -> f64 {
        // TODO: 2 * (width + height)
        todo!("Calculate rectangle perimeter")
    }
}

// Exercise 5.10: Fix error handling with custom types - BROKEN CODE TO FIX!
fn exercise_5_10() {
    println!("Exercise 5.10: Fix error handling");
    
    // FIXME: These functions don't exist yet
    let result1 = safe_divide(10.0, 2.0);  // COMPILE ERROR!
    println!("10 / 2 = {:?}", result1);
    
    let result2 = safe_divide(10.0, 0.0);  // COMPILE ERROR!
    println!("10 / 0 = {:?}", result2);
    
    let results = vec!["42", "abc", "13", "xyz"];
    let parsed = parse_and_sum(&results);  // COMPILE ERROR!
    println!("Parse and sum result: {:?}", parsed);
    
    println!("✅ Exercise 5.10 complete\n");
}

// TODO: Define custom error enum
#[derive(Debug)]
enum MathError {
    // TODO: Add error variants
    // DivisionByZero, ParseError(String)
}

// TODO: Implement safe_divide function
fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    // TODO: Return Err(MathError::DivisionByZero) if b is 0.0
    // Otherwise return Ok(a / b)
    todo!("Implement safe division")
}

// TODO: Implement parse_and_sum function
fn parse_and_sum(strings: &[&str]) -> Result<i32, MathError> {
    // TODO: Try to parse each string to i32 and sum them
    // Return Err(MathError::ParseError) if any parsing fails
    // HINT: Use ? operator for early returns
    todo!("Parse all strings and sum them")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_processing() {
        // TODO: Uncomment when you implement Message enum and process_message
        // let msg = Message::Text("test".to_string());
        // process_message(msg); // Should not panic
    }
    
    #[test]
    fn test_option_handling() {
        // TODO: Uncomment when you implement sum_some_numbers
        // let numbers = vec![Some(1), None, Some(2)];
        // assert_eq!(sum_some_numbers(&numbers), 3);
    }
    
    #[test]
    fn test_result_handling() {
        // TODO: Uncomment when you implement sum_parsed_numbers
        // let strings = vec!["1", "2", "3"];
        // assert_eq!(sum_parsed_numbers(&strings), Ok(6));
        
        // let bad_strings = vec!["1", "abc", "3"];
        // assert!(sum_parsed_numbers(&bad_strings).is_err());
    }
    
    #[test]
    fn test_text_analyzer() {
        // TODO: Uncomment when you implement TextAnalyzer
        // let analyzer = TextAnalyzer::new("hello world hello");
        // assert_eq!(analyzer.word_count(), 3);
        // assert_eq!(analyzer.most_common_word(), Some("hello"));
    }
    
    #[test]
    fn test_closures() {
        // TODO: Uncomment when you implement closure functions
        // let numbers = vec![1, 2, 3];
        // let doubled = process_with_closure(&numbers, |x| x * 2);
        // assert_eq!(doubled, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_complex_matching() {
        // TODO: Uncomment when you implement analyze_pair
        // assert_eq!(analyze_pair((Some(5), Some(10))), "High sum");
        // assert_eq!(analyze_pair((Some(3), Some(3))), "Equal values");
        // assert_eq!(analyze_pair((Some(1), None)), "First only");
    }
    
    #[test]
    fn test_iterator_chaining() {
        // TODO: Uncomment when you implement complex_word_processing
        // let words = vec!["hi", "hello", "world"];
        // let result = complex_word_processing(&words);
        // assert_eq!(result, vec!["HELLO!", "WORLD!"]);
    }
    
    #[test]
    fn test_shapes() {
        // TODO: Uncomment when you implement Rectangle and Drawable
        // let rect = Rectangle { width: 4.0, height: 3.0 };
        // assert_eq!(rect.area(), 12.0);
        // assert_eq!(rect.perimeter(), 14.0);
    }
    
    #[test]
    fn test_error_handling() {
        // TODO: Uncomment when you implement safe_divide and MathError
        // assert!(safe_divide(10.0, 2.0).is_ok());
        // assert!(safe_divide(10.0, 0.0).is_err());
    }
}

// COMPILATION CHALLENGES:
// 1. Define enums with data-carrying variants
// 2. Implement pattern matching for complex enums
// 3. Handle Option and Result types effectively
// 4. Use iterator methods with closures
// 5. Create structs with advanced methods
// 6. Implement traits for custom types
// 7. Chain multiple iterator operations
// 8. Use pattern matching with guards
// 9. Create custom error types
// 10. Propagate errors with ? operator
//
// LEARNING OBJECTIVES:
// - Master advanced pattern matching techniques
// - Understand functional programming in Rust
// - Work with complex data structures
// - Handle errors gracefully
// - Use iterator methods effectively
// - Implement traits and custom types
// - Apply all Module 01 concepts together
//
// C# COMPARISON:
// - Enums in Rust are much more powerful than C# enums
// - Pattern matching replaces switch statements and is exhaustive
// - Option<T> replaces nullable types
// - Result<T, E> replaces exception handling
// - Closures work similarly to C# lambdas
// - Iterator methods similar to LINQ
//
// 🎯 WHEN YOU FINISH:
// You've mastered all the foundational concepts of Rust!
// ✅ Advanced pattern matching
// ✅ Functional programming patterns  
// ✅ Error handling without exceptions
// ✅ Complex data structure manipulation
// ✅ Trait implementation and custom types
// ✅ Real-world Rust programming patterns
//
// 🚀 Ready for Module 02: Ownership and Borrowing!