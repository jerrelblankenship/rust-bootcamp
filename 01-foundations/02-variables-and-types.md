# Lesson 02: Variables and Types

Rust's type system is one of its greatest strengths. Coming from C#, you'll find familiar concepts with some important differences that enhance safety and performance.

## 🎯 Learning Objectives

- Understand variable declarations and mutability
- Master Rust's primitive and compound types
- Learn type inference and annotations
- Work with type conversions safely
- Understand shadowing vs mutation

## 📝 Variables and Mutability

### The Immutable Default

Unlike C#, Rust variables are immutable by default:

```rust
fn main() {
    // C#: var x = 5; x = 6; // OK
    // Rust:
    let x = 5;
    // x = 6;  // ERROR: cannot assign twice to immutable variable
    
    // Mutable variables require explicit mut
    let mut y = 5;
    y = 6;  // OK
    println!("y = {}", y);
}
```

### Why Immutable by Default?

```rust
// Benefits of immutability:
fn process_data(data: Vec<i32>) -> i32 {
    let sum = data.iter().sum();  // sum doesn't need to change
    let count = data.len();        // count is constant
    sum / count as i32             // Safe: no accidental modifications
}
```

### C# readonly vs Rust Immutability

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| `const` | `const` | Compile-time constant |
| `readonly` | default `let` | Runtime immutable |
| `var` | `let mut` | Mutable binding |
| `ref readonly` | `&T` | Immutable reference |

## 🔢 Primitive Types

### Integers

```rust
fn main() {
    // Signed integers
    let i8_val: i8 = -128;          // 8-bit signed
    let i16_val: i16 = -32_768;     // 16-bit signed
    let i32_val: i32 = -2_147_483_648; // 32-bit signed (default)
    let i64_val: i64 = 9_223_372_036_854_775_807;
    let i128_val: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    
    // Unsigned integers
    let u8_val: u8 = 255;
    let u16_val: u16 = 65_535;
    let u32_val: u32 = 4_294_967_295;
    let u64_val: u64 = 18_446_744_073_709_551_615u64;
    
    // Architecture-dependent
    let isize_val: isize = 9_223_372_036_854_775_807; // 64-bit on x64
    let usize_val: usize = 18_446_744_073_709_551_615; // Array indices
    
    // Numeric literals
    let decimal = 98_222;        // Underscores for readability
    let hex = 0xff;             // Hexadecimal
    let octal = 0o77;           // Octal
    let binary = 0b1111_0000;   // Binary
    let byte = b'A';            // u8 only
}
```

### Floating Point

```rust
fn main() {
    let f32_val: f32 = 3.14159;    // 32-bit float
    let f64_val = 2.71828;         // 64-bit float (default)
    
    // Special values
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    // Scientific notation
    let scientific = 1.234e-10;
    
    // Comparison with C#
    // C#: float (32-bit), double (64-bit), decimal (128-bit)
    // Rust: f32 (32-bit), f64 (64-bit), no built-in decimal
}
```

### Boolean and Character

```rust
fn main() {
    // Boolean
    let is_active: bool = true;
    let is_complete = false;  // Type inferred
    
    // Character (Unicode scalar value)
    let ch: char = 'z';
    let emoji = '😀';
    let japanese = '京';
    
    // C# char is 16-bit (UTF-16), Rust char is 32-bit (Unicode)
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}
```

## 🎨 Compound Types

### Tuples

```rust
fn main() {
    // Tuple - fixed size, heterogeneous
    let tup: (i32, f64, char) = (500, 6.4, 'a');
    
    // Destructuring
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    
    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    
    // Unit type (empty tuple)
    let unit: () = ();  // Like C#'s void
}
```

### Arrays

```rust
fn main() {
    // Array - fixed size, homogeneous
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Initialize with same value
    let zeros = [0; 10];  // [0, 0, 0, ..., 0]
    
    // Access elements
    let first = arr[0];
    let last = arr[arr.len() - 1];
    
    // Compile-time bounds checking
    // let oops = arr[10];  // ERROR: index out of bounds
    
    // Compare with C#
    // C#: int[] dynamic array, Rust: Vec<i32> for dynamic
}
```

## 🔄 Type Inference and Annotations

```rust
fn main() {
    // Type inference
    let x = 5;              // i32 inferred
    let y = 5.0;            // f64 inferred
    
    // Type annotations when needed
    let guess: u32 = "42".parse().expect("Not a number!");
    
    // Inference from usage
    let mut vec = Vec::new();
    vec.push(5);            // Now known to be Vec<i32>
    
    // Suffix notation
    let x = 5u8;            // Explicitly u8
    let y = 1_000i64;       // Explicitly i64
}
```

## 🔄 Type Conversions

### Safe Conversions with `as`

```rust
fn main() {
    // Numeric casts
    let x = 5u8;
    let y = x as u16;       // Safe widening
    let z = 1000u16;
    let w = z as u8;        // Truncation (1000 % 256 = 232)
    
    // C# comparison
    // C#: implicit widening, explicit narrowing
    // Rust: all numeric casts are explicit
    
    // Character conversions
    let ch = 'A';
    let ascii = ch as u8;   // 65
    println!("'{}' = {}", ch, ascii);
}
```

### String Types

```rust
fn main() {
    // String slice (borrowed)
    let s1: &str = "Hello";         // Immutable, UTF-8
    
    // String (owned)
    let mut s2 = String::from("Hello");
    s2.push_str(", World!");
    
    // C# comparison
    // C#: string (immutable), StringBuilder (mutable)
    // Rust: &str (borrowed), String (owned)
    
    // Conversions
    let slice: &str = &s2;          // String to &str
    let owned: String = slice.to_string(); // &str to String
}
```

## 🌟 Shadowing

A unique Rust feature:

```rust
fn main() {
    let x = 5;
    let x = x + 1;          // New variable shadows the old
    let x = x * 2;          // Another shadow
    println!("x = {}", x);  // 12
    
    // Can change types
    let spaces = "   ";
    let spaces = spaces.len();  // Now it's a number!
    
    // Different from mutation
    let mut y = 5;
    y = 6;                  // Same variable, new value
    // let mut y = "hello"; // ERROR: can't change type
}
```

## 🛡️ Type Safety Features

### Option Type (No Nulls!)

```rust
fn main() {
    // No null references!
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    
    // Must handle both cases
    match some_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }
    
    // C# nullable vs Rust Option
    // C#: int? x = null;
    // Rust: let x: Option<i32> = None;
}
```

## 💻 Practice Exercises

### Exercise 1: Variable Challenge

```rust
fn main() {
    // Fix this code to compile
    let x = 5;
    println!("x = {}", x);
    x = 6;  // Make this work
    println!("x = {}", x);
    
    // Bonus: Use shadowing instead of mutation
}
```

### Exercise 2: Type Conversion

```rust
fn main() {
    // Convert between types safely
    let a: u8 = 200;
    let b: u8 = 100;
    
    // Calculate a + b without overflow
    // Hint: Convert to a larger type first
    
    // Convert the result to a string
    // Display with proper formatting
}
```

### Exercise 3: Compound Types

```rust
fn main() {
    // Create a function that returns student info
    // Return: (name, age, grades)
    // where grades is an array of 5 scores
    
    // Destructure the result
    // Calculate average grade
    // Print formatted report
}
```

## 🚀 Mini-Project: Type Explorer

Create a program that demonstrates Rust's type system:

```rust
use std::mem;

fn main() {
    println!("=== Rust Type Explorer ===\n");
    
    // Display size of various types
    println!("Primitive Types:");
    println!("bool:     {} byte(s)", mem::size_of::<bool>());
    println!("char:     {} byte(s)", mem::size_of::<char>());
    println!("i32:      {} byte(s)", mem::size_of::<i32>());
    println!("f64:      {} byte(s)", mem::size_of::<f64>());
    
    // Explore compound types
    type Point3D = (f64, f64, f64);
    println!("\nCompound Types:");
    println!("(i32, f64): {} byte(s)", mem::size_of::<(i32, f64)>());
    println!("Point3D:    {} byte(s)", mem::size_of::<Point3D>());
    
    // Option type size
    println!("\nOption Types:");
    println!("Option<u8>:  {} byte(s)", mem::size_of::<Option<u8>>());
    println!("Option<u32>: {} byte(s)", mem::size_of::<Option<u32>>());
    
    // Your additions...
}
```

## 🔑 Key Takeaways

1. **Immutable by Default**: Prevents bugs, enables optimizations
2. **No Null References**: Option<T> makes absence explicit
3. **Type Inference**: Smart but explicit when needed
4. **Memory Efficient**: Types have predictable sizes
5. **Shadowing**: Reuse names safely with new bindings

## 📚 Additional Resources

- [Rust Book - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust Reference - Types](https://doc.rust-lang.org/reference/types.html)

## ✅ Checklist

Before moving on, ensure you understand:
- [ ] Difference between immutable and mutable bindings
- [ ] All primitive types and their sizes
- [ ] Tuples and arrays
- [ ] Type inference and when to use annotations
- [ ] Shadowing vs mutation
- [ ] Basic type conversions

---

Next: [Functions and Control Flow](03-functions-and-flow.md) →
