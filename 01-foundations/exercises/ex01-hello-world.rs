// Exercise 1: Hello World Variations - Fix the Broken Code!
// 
// Your task: Make this code compile and work correctly by fixing
// all the compilation errors and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one
// 3. Run `rustc ex01-hello-world.rs && ./ex01-hello-world` to test
// 4. Understand basic Rust syntax through hands-on debugging

fn main() {
    println!("=== Exercise 1: Hello World Variations (Fix the Code!) ===\n");
    
    // Exercise 1.1: Fix the compile error
    // FIXME: This doesn't compile - what's missing?
    println("Hello, World!");  // COMPILE ERROR: Fix this!
    
    // Exercise 1.2: Fix variable usage
    // FIXME: This code has an error - the variable isn't declared properly
    println!("Hello, {}! Welcome to Rust!", name);  // COMPILE ERROR!
    // TODO: Declare the 'name' variable with your name
    // HINT: Use let keyword in Rust
    
    // Exercise 1.3: Fix multiple values printing
    // FIXME: These variables aren't declared
    println!("My favorite number is {} and my favorite color is {}", favorite_number, favorite_color);  // COMPILE ERROR!
    // TODO: Declare favorite_number and favorite_color variables
    // HINT: What types should these be?
    
    // Exercise 1.4: Fix debug printing
    // FIXME: This won't compile without the right syntax
    let coordinates = (10, 20, 30);
    println!("Coordinates: {}", coordinates);  // COMPILE ERROR: Wrong format specifier!
    // TODO: Use the correct format specifier for debug printing
    // HINT: Debug printing uses a different format than regular printing
    
    // Exercise 1.5: Fix pretty printing
    let nested_data = vec![
        ("Alice", 25, vec!["Rust", "C#", "Python"]),
        ("Bob", 30, vec!["JavaScript", "Go"]),
        ("Charlie", 28, vec!["Java", "Kotlin", "Scala"]),
    ];
    // FIXME: Use the correct format specifier for pretty debug printing
    println!("Nested data: {}", nested_data);  // COMPILE ERROR: Wrong format!
    // TODO: Make this print in a nice, readable format
    // HINT: Pretty printing has a special format specifier

    // Exercise 1.6: Function call
    // FIXME: This function doesn't exist yet - implement it below
    print_initials();  // COMPILE ERROR: Function not found!
}

// Exercise 1.6: Create ASCII Art Function
// TODO: Implement this function to print your initials in ASCII art
// HINT: Functions in Rust use the 'fn' keyword
fn print_initials() {
    // TODO: Implement this function
    // Should print "=== My Initials in ASCII Art ===" and your initials
    // HINT: Use println! macro to print multiple lines
    todo!("Implement ASCII art function")  // Remove this line and add your implementation
}

// Exercise 1.7: Advanced - Fix the broken function
// FIXME: This function has multiple issues
fn greet_user(user_name, age) {  // COMPILE ERROR: Missing parameter types!
    // TODO: Fix the parameter types
    // TODO: Fix the string formatting
    println!("Hello {}! You are {} years old.", user_name, age);
    // TODO: Add a return value - this function should return a greeting string
    // HINT: Functions can return values without the 'return' keyword
}

// Exercise 1.8: Test your functions
// TODO: Uncomment this function and make it work
// fn test_functions() {
//     let greeting = greet_user("Alice", 25);  // Should return a String
//     println!("Function returned: {}", greeting);
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_print_initials() {
        // This test will work once you implement the function
        print_initials();
    }
    
    #[test]
    fn test_greet_user() {
        // TODO: Uncomment when you fix the greet_user function
        // let result = greet_user("Test", 30);
        // assert!(result.contains("Hello Test"));
        // assert!(result.contains("30"));
    }
}

// COMPILATION CHALLENGES:
// 1. Fix the println! macro syntax (missing !)
// 2. Declare variables before using them  
// 3. Use correct format specifiers ({:?} for debug, {:#?} for pretty)
// 4. Implement the print_initials function
// 5. Fix function parameter types
// 6. Make greet_user return a String
// 7. Understand Rust's expression-based return syntax
//
// LEARNING OBJECTIVES:
// - Basic Rust syntax differences from C#
// - Variable declaration with 'let'
// - Function definition with 'fn'
// - Debug printing vs regular printing
// - String formatting in Rust
// - Expression-based returns vs explicit return statements
//
// C# COMPARISON:
// C#: Console.WriteLine($"Hello {name}!");
// Rust: println!("Hello {}!", name);
//
// C#: public string Greet(string name, int age) { return $"Hello {name}"; }
// Rust: fn greet(name: &str, age: i32) -> String { format!("Hello {}", name) }
