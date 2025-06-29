# Detailed Rust Concepts - Module 01

**Note**: These are detailed reference materials. **Start with the exercises first!**

## 📚 Available References

- [Hello Rust Fundamentals](hello-rust-detailed.md) - Complete guide to first programs
- [Variables and Types Deep Dive](variables-types-detailed.md) - Type system exploration  
- [Functions and Control Flow](functions-flow-detailed.md) - Advanced function concepts
- [Structs and Enums Guide](structs-enums-detailed.md) - Custom type design
- [C# to Rust Translation Guide](csharp-comparisons.md) - Side-by-side comparisons

## 🎯 How to Use These

1. **Try the exercises first** - Learn by fixing broken code
2. **Reference when stuck** - Look up specific concepts
3. **Deepen understanding** - Read after completing exercises
4. **Compare with C#** - Use translation guide for familiar concepts

## 📋 Quick Reference

### Essential Syntax
```rust
// Variables
let x = 5;          // Immutable
let mut y = 10;     // Mutable

// Functions  
fn add(a: i32, b: i32) -> i32 {
    a + b           // No semicolon = return value
}

// Print macros
println!("Hello {}", name);     // With newline
print!("No newline");           // Without newline
println!("Debug: {:?}", data);  // Debug formatting
```

### C# → Rust Quick Translations
```rust
// C#: Console.WriteLine($"Hello {name}");
println!("Hello {}", name);

// C#: int x = 5;
let x: i32 = 5;  // or just: let x = 5;

// C#: public int Add(int a, int b) { return a + b; }
fn add(a: i32, b: i32) -> i32 { a + b }
```

---

**Remember**: Start with hands-on exercises, use these for deeper understanding! 🦀
