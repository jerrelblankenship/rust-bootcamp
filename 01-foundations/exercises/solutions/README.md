# Exercise Solutions - Module 01: Foundations

This directory contains complete solutions for all exercises in Module 01. Use these to check your work after attempting the exercises yourself.

## 📚 How to Use These Solutions

1. **Try First**: Attempt each exercise on your own before looking at the solution
2. **Compare**: After your attempt, compare your solution with the provided one
3. **Learn**: Look for different approaches and Rust idioms you might have missed
4. **Practice**: If you found an exercise challenging, try similar problems

## 🔍 Solution Files

- **ex01-hello-world-solution.rs** - Basic printing and string formatting
- **ex02-types-solution.rs** - Type system exploration and conversions  
- **ex03-functions-solution.rs** - Functions, control flow, and pattern matching
- **ex04-structs-solution.rs** - Custom types with structs and enums

## 🎯 Key Learning Points

### Exercise 1: Hello World
- **println!** macro usage and formatting
- String literals vs String types
- Debug formatting with {:?} and {:#?}
- Function definition and calling

### Exercise 2: Types
- Type inference vs explicit annotations
- Numeric type conversions with `as`
- Compound types (tuples, arrays)
- String slice (&str) vs owned String
- Shadowing vs mutation

### Exercise 3: Functions
- Expression-based returns (no semicolon)
- Pattern matching with `match` and guards
- Control flow: if, loop, while, for
- Error handling with Option and Result
- Enum variants with associated data

### Exercise 4: Structs and Enums
- Struct definition and implementation blocks
- Methods vs associated functions (&self vs Self)
- Tuple structs and unit structs
- Enum variants with different data types
- Generic structs and trait implementations
- Builder pattern in Rust

## 🚀 Extension Challenges

After completing the basic exercises, try these additional challenges:

### Challenge 1: Advanced Calculator
Extend the calculator project to support:
- Parentheses for order of operations
- Multiple operations in one expression
- Memory functions (store/recall values)

### Challenge 2: Custom Data Structures
Implement these data structures from scratch:
- A growable vector (like Vec<T>)
- A stack with push/pop operations
- A simple hash map with string keys

### Challenge 3: Text Processing
Create a program that:
- Counts words, lines, and characters in text
- Finds the most common words
- Implements a simple spell checker

## 💡 Common Patterns You'll See

### Error Handling Pattern
```rust
fn do_something() -> Result<T, ErrorType> {
    let value = might_fail()?;  // ? operator for early return
    let processed = process(value)?;
    Ok(processed)
}
```

### Builder Pattern
```rust
let config = ConfigBuilder::new()
    .option1(value1)
    .option2(value2)
    .build();
```

### Method Chaining
```rust
let result = data
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.transform())
    .collect();
```

### Pattern Matching
```rust
match value {
    Some(x) if x > 0 => "positive",
    Some(x) if x < 0 => "negative", 
    Some(0) => "zero",
    None => "no value",
}
```

## 🔧 Running the Solutions

Each solution file can be run independently:

```bash
# Run a specific exercise solution
rustc exercises/solutions/ex01-hello-world-solution.rs -o ex01
./ex01

# Or use cargo for the project
cd project-calculator
cargo run -- 5 + 3
cargo test
```

## 📖 Further Reading

- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) - Common patterns

---

Remember: There's often more than one correct way to solve a problem in Rust. The solutions provided emphasize clarity and demonstrate common Rust patterns, but your solution might be equally valid!
