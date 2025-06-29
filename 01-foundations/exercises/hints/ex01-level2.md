# Exercise 1 Hints - Level 2 (More Specific Guidance)

## 🎯 Specific Solutions

You've tried Level 1 hints but still need more guidance. Here are more specific solutions:

## 🔧 Fix 1: Macro Syntax

**Problem**: `println("Hello, World!");`
**Solution**: `println!("Hello, World!");`

**Explanation**: In Rust, `println!` is a macro (note the `!`). Macros are compile-time code generation tools that are checked more strictly than functions.

## 🔧 Fix 2: Variable Declaration

**Problem**: Using undefined variable `name`
**Solution**: Add before the println:
```rust
let name = "Your Name";
```

**Explanation**: 
- `let` keyword declares variables in Rust
- Variables are immutable by default (use `let mut` for mutable)
- String literals are `&str` type in Rust

## 🔧 Fix 3: String Formatting

**Problem**: `println!("Hello, {}! Welcome to Rust!", name);` - missing variable
**Solution**: Either declare `name` first, or use a literal:
```rust
let name = "Rustacean";
println!("Hello, {}! Welcome to Rust!", name);
```

**Explanation**: 
- `{}` is a placeholder for variables in Rust format strings
- Similar to C#'s `{0}` but you don't need numbers
- Variables are passed as additional arguments to `println!`

## 🔧 Fix 4: Debug Printing

**Problem**: `println!("Coordinates: {}", coordinates);` - wrong format specifier
**Solution**: `println!("Coordinates: {:?}", coordinates);`

**Explanation**:
- `{:?}` is debug formatting (like C#'s `.ToString()` for debugging)
- `{:#?}` is pretty debug formatting (nicely formatted)
- Tuples and most types implement debug formatting automatically

## 🔧 Fix 5: Function Implementation

**Problem**: `print_initials();` function doesn't exist
**Solution**: Replace the `todo!()` with actual implementation:
```rust
fn print_initials() {
    println!("=== My Initials in ASCII Art ===");
    println!(" A   B ");
    println!("A A B B");
    println!("AAA BBB");
    println!("A A B B");
    println!("A A B B");
}
```

**Explanation**:
- Functions in Rust use `fn` keyword
- No need for access modifiers like C#'s `public`
- `todo!()` is a placeholder that panics when called

## 📋 Compilation Checklist

After each fix, run:
```bash
rustc ex01-hello-world.rs
```

You should see fewer errors each time. Work through them systematically.

## ➡️ Next Level

Still having trouble? Try [Level 3 Hints](ex01-level3.md) for nearly complete solutions.

## 🎓 Key Learning Points

- Rust macros end with `!` 
- Variables declared with `let`
- Debug formatting uses `{:?}`
- Functions defined with `fn`
- Compiler errors guide you to solutions

Keep going - you're learning Rust's unique approaches! 🦀
