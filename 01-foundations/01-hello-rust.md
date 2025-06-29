# Lesson 01: Hello Rust

Welcome to Rust! Let's start with the classic "Hello, World!" program and explore how Rust compares to C#.

## 🎯 Learning Objectives

- Write and run your first Rust program
- Understand the structure of a Rust project
- Learn about cargo, Rust's build tool
- Compare Rust syntax with C# equivalents

## 📝 Your First Rust Program

### C# Version
```csharp
using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
        }
    }
}
```

### Rust Version
```rust
fn main() {
    println!("Hello, World!");
}
```

Notice the differences:
- No namespace or class required
- `fn` instead of method modifiers
- `println!` is a macro (note the `!`)
- No explicit `using` statements needed for basic functionality

## 🏗️ Creating a Rust Project

### Using Cargo (Rust's Build Tool)

```bash
# Create a new binary project
cargo new hello_rust
cd hello_rust

# Build the project
cargo build

# Run the project
cargo run

# Build optimized release version
cargo build --release
```

This creates:
```
hello_rust/
├── Cargo.toml      # Similar to .csproj
├── src/
│   └── main.rs     # Entry point
└── target/         # Build output (like bin/obj)
```

### Understanding Cargo.toml

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# Dependencies go here (like NuGet packages)
```

## 🔍 Anatomy of a Rust Program

Let's expand our program:

```rust
// This is a comment, like C#'s //

/* 
   Multi-line comments work 
   the same as C# 
*/

/// Documentation comments use three slashes
/// and support Markdown formatting
fn main() {
    // Variables and printing
    let greeting = "Hello";
    let name = "Rustacean";
    
    // String formatting
    println!("{}, {}!", greeting, name);
    
    // Multiple ways to print
    print!("This doesn't add a newline");
    println!(" - but this does!");
    
    // Debug printing
    let numbers = vec![1, 2, 3];
    println!("Debug: {:?}", numbers);
    println!("Pretty: {:#?}", numbers);
}
```

## 🔄 C# to Rust Comparison

### Console Output

| C# | Rust | Notes |
|----|------|-------|
| `Console.WriteLine("Hello");` | `println!("Hello");` | Macro, not method |
| `Console.Write("Hello");` | `print!("Hello");` | No newline |
| `Console.WriteLine($"Hello {name}");` | `println!("Hello {}", name);` | Different interpolation |
| `Console.WriteLine("{0} {1}", a, b);` | `println!("{} {}", a, b);` | Positional args |

### String Formatting

```rust
fn main() {
    let name = "Alice";
    let age = 30;
    
    // Basic formatting
    println!("Name: {}, Age: {}", name, age);
    
    // Named arguments
    println!("{name} is {age} years old", name=name, age=age);
    
    // Positional arguments
    println!("{0} is {1}. {0} likes Rust!", name, age);
    
    // Formatting specifiers
    let pi = 3.14159;
    println!("Pi: {:.2}", pi);  // 2 decimal places
    
    // Debug vs Display
    let point = (3, 4);
    println!("Display: ({}, {})", point.0, point.1);
    println!("Debug: {:?}", point);
}
```

## 💻 Practice Exercises

### Exercise 1: Personalized Greeting

Create a program that:
1. Stores your name in a variable
2. Prints a personalized greeting
3. Prints your name in uppercase

```rust
fn main() {
    // Your code here
    let name = "Your Name";
    
    // Hint: Use name.to_uppercase()
}
```

### Exercise 2: Multi-line Output

Create ASCII art using println!:

```rust
fn main() {
    // Create a simple ASCII art
    println!("   /\\_/\\  ");
    println!("  ( o.o ) ");
    println!("   > ^ <  ");
    println!("  Rust Cat");
}
```

### Exercise 3: Format Exploration

Experiment with different format specifiers:

```rust
fn main() {
    let integer = 42;
    let float = 3.14159;
    let boolean = true;
    let character = '🦀';
    
    // Print each with different formats
    println!("Integer: {}", integer);
    println!("Integer (hex): {:x}", integer);
    println!("Integer (binary): {:b}", integer);
    
    // Continue for other types...
}
```

## 🚀 Advanced Topics

### Macros vs Functions

In C#, `Console.WriteLine` is a method. In Rust, `println!` is a macro:

```rust
// Macro - checked at compile time
println!("Hello {}", "World");  // ✓ Correct
// println!("Hello {}");        // ❌ Compile error - missing argument

// If it were a function, format errors would be runtime errors
```

### The main Function

```rust
// Simple main
fn main() {
    // Program logic
}

// Main with Result (for error handling)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Program logic that might fail
    Ok(())
}

// Main with exit code
fn main() -> std::process::ExitCode {
    // Return custom exit code
    std::process::ExitCode::SUCCESS
}
```

## 🎯 Mini-Project: Enhanced Greeting

Create a program that:
1. Prints a welcome banner
2. Shows the current Rust version
3. Displays system information

```rust
fn main() {
    // Banner
    println!("╔════════════════════════╗");
    println!("║   Welcome to Rust! 🦀  ║");
    println!("╚════════════════════════╝");
    
    // Rust version (compile-time)
    println!("\nRust version: {}", env!("CARGO_PKG_RUST_VERSION"));
    
    // System info
    println!("Operating System: {}", std::env::consts::OS);
    println!("Architecture: {}", std::env::consts::ARCH);
    
    // Your additions here...
}
```

## 🔗 Key Takeaways

1. **Simplicity**: Rust programs can be very concise
2. **Macros**: `println!` is compile-time checked
3. **No Boilerplate**: No class or namespace required
4. **Cargo**: Powerful build and dependency management
5. **Safety**: Even "Hello World" is memory-safe!

## 📚 Additional Resources

- [Rust Book - Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- [Rust by Example - Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Create a new Rust project with cargo
- [ ] Write and run a basic Rust program
- [ ] Use different print macros and formatting
- [ ] Understand the basic structure of a Rust program
- [ ] Compare Rust syntax with C# equivalents

---

Next: [Variables and Types](02-variables-and-types.md) →
