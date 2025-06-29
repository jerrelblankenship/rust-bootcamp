# Exercise 2 Hints - Level 3 (Nearly Complete Solutions)

## ⚠️ Last Resort Guidance

**Use these hints only if Level 1 and Level 2 didn't help!** These contain nearly complete solutions.

## 🛠️ Complete Fixes for Common Issues

### Variable Declaration Section
```rust
// BEFORE (broken):
x = 42;
y = 3.14;
is_rust_fun = true;

// AFTER (fixed):
let x = 42;
let y = 3.14;
let is_rust_fun = true;
```

### Type Annotation Section
```rust
// BEFORE (broken):
let age: = 25;
let height: = 5.9;

// AFTER (fixed):
let age: i32 = 25;
let height: f64 = 5.9;
let initial: char = 'R';
```

### Mutability Section
```rust
// BEFORE (broken):
let counter = 0;
counter = counter + 1;  // Error!

// AFTER (fixed):
let mut counter = 0;
counter = counter + 1;  // Works!
counter += 5;           // Also works!
```

### Type Conversion Section
```rust
// BEFORE (broken):
let x: i32 = 10;
let y: i64 = 20;
let sum = x + y;  // Error!

// AFTER (fixed):
let x: i32 = 10;
let y: i64 = 20;
let sum = x as i64 + y;  // Convert x to i64 first
```

### Tuple Access Section
```rust
// BEFORE (broken):
let coordinates = (3, 5);
let x = coordinates[0];  // Error!

// AFTER (fixed):
let coordinates = (3, 5);
let x = coordinates.0;   // Use dot notation
let y = coordinates.1;
```

### Array and Slice Section
```rust
// BEFORE (broken):
let data = vec![1, 2, 3, 4, 5];
let slice = data[1:4];  // Error!

// AFTER (fixed):
let data = vec![1, 2, 3, 4, 5];
let slice = &data[1..4];  // Use & and .. syntax
```

### String Conversion Section
```rust
// BEFORE (broken):
let number = 42;
let number_str = number;  // Error!

// AFTER (fixed):
let number = 42;
let number_str = number.to_string();  // Convert to String

// OR for parsing:
let text = "123";
let parsed_num: i32 = text.parse().expect("Failed to parse");
```

## 🔍 Complete Working Example

Here's how key parts should look when working:

```rust
fn main() {
    // Basic variable declarations
    let x = 42;
    let y = 3.14;
    let is_rust_fun = true;
    let greeting = "Hello, Rust!";
    
    // Type annotations when needed
    let age: i32 = 25;
    let height: f64 = 5.9;
    let initial: char = 'R';
    
    // Mutability when needed
    let mut counter = 0;
    counter += 1;
    
    // Type conversions
    let x_as_i64 = x as i64;
    let number_str = x.to_string();
    
    // Tuple access
    let coordinates = (10, 20);
    let coord_x = coordinates.0;
    let coord_y = coordinates.1;
    
    // Array operations
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    
    println!("All working! x = {}, counter = {}", x, counter);
}
```

## 🎯 Understanding Check

Before moving on, make sure you understand:

1. **Why `let` is required** - Rust's explicit variable declaration
2. **Why `mut` is needed** - Immutability by default for safety
3. **When type annotations help** - When compiler can't infer
4. **Why conversions are explicit** - No hidden performance costs
5. **Tuple vs array access** - Different syntax for different purposes

## 🚀 Next Steps

- Compile and run: `rustc ex02-types.rs && ./ex02-types`
- Move to [Exercise 3](ex03-level1.md)
- If you used these hints, review the concepts in the lesson material

## 🎓 Reflection Questions

1. How does Rust's immutability-by-default compare to C#'s mutability-by-default?
2. Why does Rust require explicit type conversions?
3. What are the benefits of Rust's strict type system?

Remember: Understanding the "why" is more important than just making it compile! 🦀
