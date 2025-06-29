// Exercise 1: Hello World Variations - Fix the Broken Code!
// 
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (8 checkpoints to fix)
// 
// Your task: Make this code compile and work correctly by fixing
// all the compilation errors and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Fix ONE error at a time - don't try to fix everything at once!
// 2. Compile after each fix: `rustc ex01-hello-world.rs`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (the macro syntax error)
// - Each checkpoint builds on the previous one
// - Celebrate each successful compilation - you're learning!
//
// COMPLETED CONCEPTS:
// [] Macro syntax (println!)
// [] Variable declaration (let)
// [] String formatting ({})
// [] Debug printing ({:?})
// [] Pretty printing ({:#?})
// [] Function implementation
// [] Parameter types
// [] Return types

fn main() {
    println!("=== Exercise 1: Hello World Variations (Fix the Code!) ===\n");
    
    // Exercise 1.1: Fix the compile error
    // FIXME: This doesn't compile - what's missing?
    println("Hello, World!");  // COMPILE ERROR: Fix this!
    
    // ✅ CHECKPOINT 1: Run `rustc ex01-hello-world.rs` - should show fewer errors
    // If the println error is fixed, move to the next section
    
    // Exercise 1.2: Fix variable usage
    // FIXME: This code has an error - the variable isn't declared properly
    println!("Hello, {}! Welcome to Rust!", name);  // COMPILE ERROR!
    // TODO: Declare the 'name' variable with your name
    // HINT: Use let keyword in Rust
    
    // ✅ CHECKPOINT 2: Compile again - variable error should be resolved
    // The undefined variable error should disappear
    
    // Exercise 1.3: Fix multiple values printing
    // FIXME: These variables aren't declared
    println!("My favorite number is {} and my favorite color is {}", favorite_number, favorite_color);  // COMPILE ERROR!
    // TODO: Declare favorite_number and favorite_color variables
    // HINT: What types should these be?
    
    // ✅ CHECKPOINT 3: Both variables should now be declared
    // Multiple variable errors should be resolved
    
    // Exercise 1.4: Fix debug printing
    // FIXME: This won't compile without the right syntax
    let coordinates = (10, 20, 30);
    println!("Coordinates: {}", coordinates);  // COMPILE ERROR: Wrong format specifier!
    // TODO: Use the correct format specifier for debug printing
    // HINT: Debug printing uses a different format than regular printing
    
    // ✅ CHECKPOINT 4: Debug formatting should work
    // Tuple should print properly with debug format
    
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
    
    // ✅ CHECKPOINT 5: Pretty printing should work
    // Complex data should display in formatted way

    // Exercise 1.6: Function call
    // FIXME: This function doesn't exist yet - implement it below
    print_initials();  // COMPILE ERROR: Function not found!
    
    // ✅ CHECKPOINT 6: Function should be implemented
    // print_initials() call should work when function exists below
}

// Exercise 1.6: Create ASCII Art Function
// TODO: Implement this function to print your initials in ASCII art
// HINT: Functions in Rust use the 'fn' keyword
fn print_initials() {
    // TODO: Implement this function
    // Should print "=== My Initials in ASCII Art ===" and your initials
    // HINT: Use println! macro to print multiple lines
    todo!("Implement ASCII art function")  // Remove this line and add your implementation
    
    // ✅ CHECKPOINT 7: Replace todo!() with actual println! statements
    // Function should print ASCII art when implemented
}

// Exercise 1.7: Advanced - Fix the broken function
// FIXME: This function has multiple issues
fn greet_user(user_name, age) {  // COMPILE ERROR: Missing parameter types!
    // TODO: Fix the parameter types
    // TODO: Fix the string formatting
    println!("Hello {}! You are {} years old.", user_name, age);
    // TODO: Add a return value - this function should return a greeting string
    // HINT: Functions can return values without the 'return' keyword
    
    // ✅ CHECKPOINT 8: Function signature should be complete
    // Parameters need types, function needs return type
    // Replace println! with format! to return a String
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
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Macro syntax): [ ] Complete
// Checkpoint 2 (Variable declaration): [ ] Complete  
// Checkpoint 3 (Multiple variables): [ ] Complete
// Checkpoint 4 (Debug printing): [ ] Complete
// Checkpoint 5 (Pretty printing): [ ] Complete
// Checkpoint 6 (Function implementation): [ ] Complete
// Checkpoint 7 (Function parameters): [ ] Complete
// Checkpoint 8 (Return types): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Rust macro syntax vs C# method calls
// ✅ Variable declaration patterns
// ✅ String formatting differences
// ✅ Debug vs display formatting
// ✅ Function definition syntax
// ✅ Parameter and return type annotations
//
// 🚀 Ready for the next challenge?
// Move on to ex02-types.rs to explore Rust's type system!
// Or check your work with: `rustc ex01-hello-world.rs && ./ex01-hello-world`
