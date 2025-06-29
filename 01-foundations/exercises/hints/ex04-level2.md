# Exercise 4 Hints - Level 2 (More Specific Guidance)

## 🎯 Specific Solutions

You've tried Level 1 hints but need more guidance. Here are more specific solutions for common struct and enum issues:

## 🔧 Fix 1: Struct Definition

**Problem**: Cannot find struct type
**Solution**: Define struct with proper syntax:
```rust
// BEFORE (broken):
// Missing struct definition

// AFTER (fixed):
struct Person {
    name: String,
    age: u32,
    email: Option<String>,  // Optional field
}
```

**Explanation**:
- `struct` keyword defines the type
- Fields have `name: type` syntax
- Use `Option<T>` for optional fields

## 🔧 Fix 2: Struct Instantiation

**Problem**: Cannot create struct instances
**Solution**: Use struct literal syntax:
```rust
// BEFORE (broken):
let person = Person("Alice", 30);  // Wrong syntax

// AFTER (fixed):
let person = Person {
    name: String::from("Alice"),
    age: 30,
    email: Some(String::from("alice@example.com")),
};

// Or using constructor:
let person = Person::new("Alice", 30, Some("alice@example.com"));
```

**Explanation**:
- Struct literals use `{ field: value }` syntax
- All fields must be specified (unless using `..` syntax)
- Constructor functions are a common pattern

## 🔧 Fix 3: Method Implementation

**Problem**: No method found for struct
**Solution**: Define methods in `impl` block:
```rust
impl Person {
    // Constructor (associated function)
    fn new(name: &str, age: u32, email: Option<&str>) -> Self {
        Person {
            name: name.to_string(),
            age,
            email: email.map(|e| e.to_string()),
        }
    }
    
    // Instance method
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
    
    // Mutable method
    fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}
```

**Explanation**:
- `impl StructName` creates an implementation block
- `&self` for reading, `&mut self` for modifying
- Associated functions don't take `self`

## 🔧 Fix 4: Enum Definition and Matching

**Problem**: Enum variants not defined or matched
**Solution**: Define enum with data variants:
```rust
// BEFORE (broken):
enum Message;  // Empty enum

// AFTER (fixed):
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Pattern matching:
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }
}
```

**Explanation**:
- Enum variants can hold different types of data
- Use `::` to access variants
- Pattern matching extracts data from variants

## 🔧 Fix 5: Trait Implementation

**Problem**: Missing trait implementations
**Solution**: Implement common traits:
```rust
// Add derives for common traits:
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

// Or manual implementation:
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}
```

**Explanation**:
- `#[derive(...)]` automatically implements common traits
- Manual trait implementation for custom behavior
- `Debug` for `{:?}`, `Display` for `{}`

## 📋 Compilation Checklist

After each fix, run:
```bash
rustc ex04-structs.rs
```

Focus on one struct or enum at a time.

## ➡️ Next Level

Still having trouble? Try [Level 3 Hints](ex04-level3.md) for nearly complete solutions.

## 🎓 Key Learning Points

- Structs group related data with named fields
- Methods go in `impl` blocks, not struct definitions
- Enums can hold data, making them very powerful
- Traits provide shared behavior across types

Keep going - you're learning Rust's powerful type system! 🦀
