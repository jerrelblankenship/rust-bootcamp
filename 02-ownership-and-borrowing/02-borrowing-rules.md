# Lesson 02: Borrowing and References

## 🎯 Learning Objectives

By the end of this lesson, you will:
- Create and use immutable and mutable references
- Understand Rust's borrowing rules
- Apply the principle of exclusive mutable access
- Debug borrow checker errors
- Write functions that borrow instead of taking ownership

## 📚 Introduction

In the last lesson, we saw how ownership transfer can be limiting. What if you want to use a value without taking ownership? This is where **borrowing** comes in.

Borrowing is Rust's solution to accessing data without transferring ownership. It's similar to passing by reference in C#, but with compile-time guarantees about memory safety.

## 🔗 References: Borrowing Without Owning

### Immutable References (&T)

```rust
fn calculate_length(s: &String) -> usize {
    s.len()  // We can read s, but not modify it
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // & creates a reference
    println!("The length of '{}' is {}.", s1, len);  // s1 still valid!
}
```

The `&` symbol creates a reference. Unlike ownership transfer, the original owner retains ownership.

### C# Comparison

```csharp
// C# - passing by reference (similar but different!)
public static int CalculateLength(ref string s) {
    // s can be modified here unless marked 'in'
    return s.Length;
}

string s1 = "hello";
int len = CalculateLength(ref s1);
```

Key difference: Rust references are immutable by default!

## 🔐 The Rules of Borrowing

Rust enforces two rules that prevent data races at compile time:

### Rule 1: Many Immutable OR One Mutable

You can have either:
- Any number of immutable references (`&T`)
- Exactly one mutable reference (`&mut T`)
- But not both at the same time!

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Multiple immutable references - OK
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // Mutable reference - OK (after immutable refs are done)
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}", r3);
}
```

### Rule 2: References Must Be Valid

References must not outlive the data they point to:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {  // ERROR!
    let s = String::from("hello");
    &s  // s is dropped here, but we're returning a reference to it
}
```

## 🔄 Mutable References

To modify borrowed data, use mutable references:

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);
    
    println!("{}", s);  // Prints "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Exclusive Access

Mutable references have exclusive access:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;  // ERROR: cannot borrow `s` as mutable more than once

println!("{}, {}", r1, r2);
```

This prevents data races at compile time!

## 🎭 Reference Scope and Lifetimes

References have a scope from creation to last use:

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // Immutable borrow starts
    let r2 = &s;      // Another immutable borrow
    println!("{} and {}", r1, r2);  // Last use of r1 and r2
    
    let r3 = &mut s;  // Mutable borrow is OK - previous borrows ended
    println!("{}", r3);
}
```

## 🐛 Common Borrowing Errors

### Error 1: Mutable and Immutable Together

```rust
let mut s = String::from("hello");

let r1 = &s;       // Immutable borrow
let r2 = &mut s;   // ERROR: cannot borrow as mutable
println!("{}", r1); // r1 is still in use
```

**Fix**: Ensure immutable references are done before mutable:
```rust
let mut s = String::from("hello");

let r1 = &s;
println!("{}", r1);  // Last use of r1

let r2 = &mut s;     // Now it's OK
r2.push_str(", world");
```

### Error 2: Iterator Invalidation

```rust
let mut v = vec![1, 2, 3];

for i in &v {
    v.push(4);  // ERROR: cannot borrow as mutable
    println!("{}", i);
}
```

**Fix**: Collect what you need first:
```rust
let mut v = vec![1, 2, 3];
let items_to_add: Vec<_> = v.iter().map(|&x| x + 10).collect();

for item in items_to_add {
    v.push(item);
}
```

### Error 3: Returning References to Local Data

```rust
fn get_string() -> &str {
    let s = String::from("hello");
    &s  // ERROR: `s` does not live long enough
}
```

**Fix**: Return owned data instead:
```rust
fn get_string() -> String {
    String::from("hello")
}
```

## 🔍 Method Receivers

Methods can borrow `self` in different ways:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Immutable borrow of self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Mutable borrow of self
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Takes ownership of self
    fn destroy(self) {
        println!("Rectangle destroyed!");
        // self is dropped here
    }
}
```

## 💡 Slices: Borrowing Parts

Slices let you reference a contiguous sequence of elements:

```rust
fn main() {
    let s = String::from("hello world");
    
    let hello = &s[0..5];    // Or &s[..5]
    let world = &s[6..11];   // Or &s[6..]
    let whole = &s[..];      // Entire string
    
    println!("{} {}", hello, world);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // [2, 3]
}
```

## 🎯 Practical Example: String Parser

Let's build a simple parser that borrows data:

```rust
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }
    
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn consume_while<F>(&mut self, predicate: F) -> &'a str
    where
        F: Fn(char) -> bool,
    {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if predicate(ch) {
                self.advance();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }
}

fn main() {
    let input = "123 + 456";
    let mut parser = Parser::new(input);
    
    let number = parser.consume_while(|ch| ch.is_numeric());
    println!("Found number: {}", number);
}
```

## 🤔 Why These Rules?

Rust's borrowing rules prevent:

1. **Data races**: Can't have simultaneous mutable access
2. **Dangling references**: Can't use freed memory
3. **Iterator invalidation**: Can't modify while iterating

All checked at compile time!

## 📝 Key Takeaways

1. **References Don't Own**: Borrowing lets you use without owning
2. **Immutable by Default**: `&T` can't modify the data
3. **Exclusive Mutable Access**: Only one `&mut T` at a time
4. **Lifetime Tracking**: Compiler ensures references remain valid
5. **Zero Runtime Cost**: All checks happen at compile time

## 🔗 Comparison with C#

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| `ref` parameter | `&T` | Immutable by default |
| `ref` with mutation | `&mut T` | Exclusive access |
| `in` parameter | `&T` | Same semantics |
| `out` parameter | Return value | Rust prefers returns |
| Reference types | All borrows | Explicit borrowing |

## ✏️ Practice Exercises

1. **Reference Practice**: Write functions that take immutable and mutable references. Try to violate the borrowing rules and understand the errors.

2. **Slice Operations**: Implement functions that work with string and array slices without taking ownership.

3. **Borrow Checker Challenges**: Fix the borrowing errors in the exercise files.

4. **Parser Extension**: Extend the parser example to handle operators and whitespace.

---

Next: [Lifetimes](03-lifetimes.md) - Understanding how long references live →
