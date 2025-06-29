# Exercise 1 Hints - Level 3 (Nearly Complete Solutions)

## ⚠️ Last Resort Guidance

**Use these hints only if Level 1 and Level 2 didn't help!** These contain nearly complete solutions.

## 🛠️ Complete Fix for Each Section

### Exercise 1.1: Fix the compile error
```rust
// BEFORE (broken):
println("Hello, World!");

// AFTER (fixed):
println!("Hello, World!");
```

### Exercise 1.2: Fix variable usage
```rust
// Add this line before the println:
let name = "Your Name Here";  // Replace with your actual name
println!("Hello, {}! Welcome to Rust!", name);
```

### Exercise 1.3: Fix multiple values
```rust
// Add these lines before the println:
let favorite_number = 42;
let favorite_color = "blue";
println!("My favorite number is {} and my favorite color is {}", favorite_number, favorite_color);
```

### Exercise 1.4: Fix debug printing
```rust
let coordinates = (10, 20, 30);
println!("Coordinates: {:?}", coordinates);  // Changed {} to {:?}
```

### Exercise 1.5: Fix pretty printing
```rust
println!("Nested data: {:#?}", nested_data);  // Changed {} to {:#?}
```

### Exercise 1.6: Function implementation
```rust
fn print_initials() {
    println!("=== My Initials in ASCII Art ===");
    // Replace these with your actual initials
    println!("  JJ    SSSS  ");
    println!("  JJ   S      ");
    println!("  JJ    SSS   ");
    println!("  JJ       S  ");
    println!("  JJ   SSSS   ");
}
```

### Exercise 1.7: Fix function parameters and return
```rust
fn greet_user(user_name: &str, age: i32) -> String {
    format!("Hello {}! You are {} years old.", user_name, age)
}
```

**Key changes:**
- Added parameter types: `user_name: &str, age: i32`
- Added return type: `-> String`
- Used `format!` macro to return a String
- Removed explicit `return` keyword (Rust uses expressions)

### Exercise 1.8: Uncomment and fix test
```rust
fn test_functions() {
    let greeting = greet_user("Alice", 25);
    println!("Function returned: {}", greeting);
}

// Call it in main():
test_functions();
```

## 🔍 Complete Working Example

Here's how the key parts should look when working:

```rust
fn main() {
    println!("=== Exercise 1: Hello World Variations (Fix the Code!) ===\n");
    
    // Exercise 1.1: Fixed
    println!("Hello, World!");
    
    // Exercise 1.2: Fixed  
    let name = "Rustacean";
    println!("Hello, {}! Welcome to Rust!", name);
    
    // Exercise 1.3: Fixed
    let favorite_number = 42;
    let favorite_color = "blue";
    println!("My favorite number is {} and my favorite color is {}", favorite_number, favorite_color);
    
    // Exercise 1.4: Fixed
    let coordinates = (10, 20, 30);
    println!("Coordinates: {:?}", coordinates);
    
    // Exercise 1.5: Fixed
    let nested_data = vec![
        ("Alice", 25, vec!["Rust", "C#", "Python"]),
        ("Bob", 30, vec!["JavaScript", "Go"]),
        ("Charlie", 28, vec!["Java", "Kotlin", "Scala"]),
    ];
    println!("Nested data: {:#?}", nested_data);

    // Exercise 1.6: Fixed
    print_initials();
}

fn print_initials() {
    println!("=== My Initials in ASCII Art ===");
    println!("  JJ    SSSS  ");
    println!("  JJ   S      ");
    println!("  JJ    SSS   ");
    println!("  JJ       S  ");
    println!("  JJ   SSSS   ");
}

fn greet_user(user_name: &str, age: i32) -> String {
    format!("Hello {}! You are {} years old.", user_name, age)
}
```

## 🎯 Understanding Check

Before moving on, make sure you understand:

1. **Why `println!` needs the `!`** - It's a macro, not a function
2. **Why we use `let`** - Rust's way of declaring variables  
3. **What `{:?}` does** - Debug formatting for data structures
4. **Parameter syntax** - `name: type` format in Rust functions
5. **Return types** - `-> Type` syntax and expression-based returns

## 🚀 Next Steps

- Compile and run: `rustc ex01-hello-world.rs && ./ex01-hello-world`
- Move to [Exercise 2](ex02-level1.md)
- If you used these hints, review the concepts in the lesson material

## 🎓 Reflection Questions

1. How is Rust's macro system different from C# methods?
2. Why does Rust require explicit type annotations in function parameters?
3. What's the advantage of expression-based returns over explicit `return` statements?

Remember: Understanding the "why" is more important than just making it compile! 🦀
