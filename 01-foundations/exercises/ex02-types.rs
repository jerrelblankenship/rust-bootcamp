// Exercise 2: Variables and Types - Fix the Broken Code!
//
// Your task: Make this code compile and work correctly by fixing
// all the compilation errors and understanding Rust's type system.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one
// 3. Run `rustc ex02-types.rs && ./ex02-types` to test
// 4. Understand Rust's type system differences from C#

fn main() {
    println!("=== Exercise 2: Variables and Types (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each section as you fix the errors
    test_basic_types();
    // test_mutability();
    // test_shadowing();
    // test_type_inference();
    // test_conversions();
    // test_compound_types();
    
    println!("\n🎉 All type exercises completed!");
}

fn test_basic_types() {
    println!("Testing basic types...");
    
    // Exercise 2.1: Fix variable declarations
    // FIXME: Variables need to be declared with 'let'
    x = 42;  // COMPILE ERROR: Missing variable declaration!
    y = 3.14;  // COMPILE ERROR: Missing variable declaration!
    is_rust_fun = true;  // COMPILE ERROR: Missing variable declaration!
    greeting = "Hello, Rust!";  // COMPILE ERROR: Missing variable declaration!
    
    println!("Integer: {}", x);
    println!("Float: {}", y);
    println!("Boolean: {}", is_rust_fun);
    println!("String: {}", greeting);
    
    // Exercise 2.2: Fix explicit type annotations
    // FIXME: These need correct type annotations
    let age: = 25;  // COMPILE ERROR: Missing type!
    let height: = 5.9;  // COMPILE ERROR: Missing type!
    let initial: = 'R';  // COMPILE ERROR: Missing type!
    
    // TODO: What types should these be?
    // HINT: Integers default to i32, floats to f64, characters are char
    
    println!("Age: {}, Height: {}, Initial: {}", age, height, initial);
    
    // Exercise 2.3: Fix integer size specification
    // FIXME: Use specific integer types
    let small_num: i8 = 300;  // COMPILE ERROR: Value too large for i8!
    let big_num: u16 = -100;  // COMPILE ERROR: Negative value in unsigned type!
    
    // TODO: Fix the values or types to make these work
    println!("Small: {}, Big: {}", small_num, big_num);
}

fn test_mutability() {
    println!("Testing mutability...");
    
    // Exercise 2.4: Fix mutability errors
    let counter = 0;
    counter = counter + 1;  // COMPILE ERROR: Cannot assign to immutable variable!
    counter += 5;  // COMPILE ERROR: Cannot modify immutable variable!
    
    println!("Counter: {}", counter);
    
    // Exercise 2.5: Const vs immutable variables
    // FIXME: Constants need values at compile time
    const MAX_POINTS;  // COMPILE ERROR: Missing value and type!
    let runtime_value = get_current_time();
    const COMPUTED_VALUE = runtime_value;  // COMPILE ERROR: Not compile-time constant!
    
    // TODO: Fix the const declarations
    // HINT: const needs type annotation and compile-time value
    
    println!("Max points: {}", MAX_POINTS);
}

fn test_shadowing() {
    println!("Testing shadowing...");
    
    // Exercise 2.6: Understanding variable shadowing
    let spaces = "   ";
    let spaces = spaces.len();  // This is legal in Rust - shadowing!
    
    // FIXME: But this doesn't work
    let mut guess = "42";
    guess = guess.len();  // COMPILE ERROR: Type mismatch!
    
    // TODO: What's the difference between shadowing and mutation?
    // ANSWER: Shadowing creates a new variable, mutation changes existing one
    
    println!("Spaces count: {}", spaces);
    println!("Guess: {}", guess);
}

fn test_type_inference() {
    println!("Testing type inference...");
    
    // Exercise 2.7: Fix type inference issues
    let numbers = vec![1, 2, 3];
    let first = numbers[0];
    
    // FIXME: Type annotation needed here
    let parsed = "42".parse();  // COMPILE ERROR: Cannot infer type!
    
    // TODO: Add type annotation to help compiler
    // HINT: What type should parsed be? i32? u32? f64?
    
    println!("First number: {}, Parsed: {}", first, parsed);
    
    // Exercise 2.8: Generic type specification
    let empty_vec = Vec::new();  // FIXME: Compiler can't infer type!
    empty_vec.push(1);  // COMPILE ERROR: Type not known!
    
    // TODO: Either annotate the variable type or the Vec::new() call
    println!("Vector length: {}", empty_vec.len());
}

fn test_conversions() {
    println!("Testing type conversions...");
    
    // Exercise 2.9: Fix type conversion errors
    let x: i32 = 10;
    let y: i64 = 20;
    
    // FIXME: Cannot add different integer types directly
    let sum = x + y;  // COMPILE ERROR: Type mismatch!
    
    // TODO: Use 'as' keyword or into() method for conversion
    // HINT: Either convert x to i64 or y to i32
    
    println!("Sum: {}", sum);
    
    // Exercise 2.10: String conversions
    let number = 42;
    
    // FIXME: Cannot directly convert integer to string
    let number_str = number;  // COMPILE ERROR: Expected &str, found i32!
    println!("Number as string: {}", number_str);
    
    // TODO: Use .to_string() or format! macro
    
    // FIXME: Cannot directly convert string to integer
    let text = "123";
    let parsed_num: i32 = text;  // COMPILE ERROR: Expected i32, found &str!
    
    // TODO: Use .parse() method (returns Result)
    println!("Parsed number: {}", parsed_num);
}

fn test_compound_types() {
    println!("Testing compound types...");
    
    // Exercise 2.11: Fix tuple usage
    let coordinates = (3, 5);
    
    // FIXME: Wrong tuple access syntax
    let x = coordinates[0];  // COMPILE ERROR: Cannot index tuple!
    let y = coordinates[1];  // COMPILE ERROR: Cannot index tuple!
    
    // TODO: Use .0, .1 syntax for tuple access
    println!("Coordinates: ({}, {})", x, y);
    
    // Exercise 2.12: Fix array errors
    // FIXME: Array size must be known at compile time
    let size = get_array_size();
    let array = [0; size];  // COMPILE ERROR: Non-constant array size!
    
    // TODO: Use either a constant size or Vec for dynamic size
    
    // FIXME: Array bounds checking
    let fixed_array = [1, 2, 3, 4, 5];
    let index = 10;
    let value = fixed_array[index];  // RUNTIME ERROR: Index out of bounds!
    
    // TODO: Use .get() method for safe array access
    println!("Array value: {}", value);
    
    // Exercise 2.13: Fix slice errors
    let data = vec![1, 2, 3, 4, 5];
    
    // FIXME: Wrong slice syntax
    let slice = data[1:4];  // COMPILE ERROR: Wrong slice syntax!
    
    // TODO: Use &data[1..4] syntax for slices
    // HINT: Slices are references to parts of arrays/vectors
    
    println!("Slice: {:?}", slice);
}

// Helper functions (these will have errors too!)

// FIXME: Function signature is incomplete
fn get_current_time() {  // COMPILE ERROR: Missing return type!
    // TODO: Add return type -> i64
    42  // Pretend this is current time
}

// FIXME: This function has parameter and return type issues
fn get_array_size() {  // COMPILE ERROR: Missing return type!
    let user_input = "5";
    // TODO: Parse the input and return it
    // TODO: Add proper return type
    // HINT: This should return usize for array sizing
    user_input  // COMPILE ERROR: Wrong return type!
}

// Exercise 2.14: Complex type combinations
fn complex_type_exercise() {
    // FIXME: Fix this complex type declaration
    let mut data: Vec<Option<Result<i32, String>>> = Vec::new();
    
    // TODO: Add some values to this complex structure
    data.push(Some(Ok(42)));
    data.push(Some(Err("Error message".to_string())));
    data.push(None);
    
    // FIXME: Process the complex data structure
    for item in data {
        match item {
            Some(Ok(value)) => println!("Success: {}", value),
            Some(Err(error)) => println!("Error: {}", error),
            None => println!("No value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_basics() {
        // TODO: Uncomment when you fix the type errors
        // let x: i32 = 42;
        // let y: f64 = 3.14;
        // assert_eq!(x, 42);
        // assert_eq!(y, 3.14);
    }
    
    #[test]
    fn test_mutability_fixed() {
        // TODO: Uncomment when you understand mutability
        // let mut counter = 0;
        // counter += 1;
        // assert_eq!(counter, 1);
    }
    
    #[test]
    fn test_conversions_fixed() {
        // TODO: Uncomment when you fix type conversions
        // let x: i32 = 10;
        // let y: i64 = 20;
        // let sum = x as i64 + y;
        // assert_eq!(sum, 30);
    }
}

// COMPILATION CHALLENGES:
// 1. Learn variable declaration with 'let'
// 2. Understand explicit type annotations
// 3. Fix mutability with 'mut' keyword  
// 4. Handle integer size limits (i8, i16, i32, i64, u8, u16, u32, u64)
// 5. Use const for compile-time constants
// 6. Understand shadowing vs mutation
// 7. Help type inference with annotations
// 8. Convert between types with 'as' and methods
// 9. Access tuples with .0, .1 syntax
// 10. Create slices with &array[start..end] syntax
//
// LEARNING OBJECTIVES FOR C# DEVELOPERS:
// - Variables are immutable by default (unlike C#)
// - let vs const vs mut (different from C#'s var, const)
// - Explicit type annotations: let x: i32 = 5
// - No implicit conversions (must be explicit)
// - Fixed-size arrays vs dynamic Vec<T>
// - Tuple access with .0, .1 (not indexing)
// - Slices as references to array portions
// - Pattern matching for complex types
//
// KEY DIFFERENCES FROM C#:
// C#: var x = 5; x = 10; (mutable by default)
// Rust: let mut x = 5; x = 10; (must specify mut)
//
// C#: int y = x; (implicit conversion sometimes)
// Rust: let y = x as i64; (explicit conversion required)
//
// C#: var tuple = (1, 2); var x = tuple.Item1;
// Rust: let tuple = (1, 2); let x = tuple.0;
//
// C#: var array = new int[5]; var slice = array[1..4];
// Rust: let array = [0; 5]; let slice = &array[1..4];
