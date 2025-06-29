# Lesson 03: Lifetimes

## 🎯 Learning Objectives

By the end of this lesson, you will:
- Understand what lifetimes are and why they exist
- Read and write lifetime annotations
- Apply lifetime elision rules
- Write structs and methods with lifetime parameters
- Debug lifetime-related compiler errors

## 📚 Introduction

Lifetimes are Rust's way of tracking how long references are valid. They're perhaps the most unique feature of Rust and often the most challenging for developers coming from garbage-collected languages like C#.

Here's the key insight: **Lifetimes are not about extending how long values live, but about describing the relationships between reference lifetimes to the compiler.**

## 🤔 Why Lifetimes?

In C#, the garbage collector ensures references remain valid. In Rust, the compiler needs to prove at compile time that all references are valid. Lifetimes are how we help the compiler do this.

### The Problem Lifetimes Solve

```rust
// This won't compile
fn main() {
    let r;                // Declare reference
    
    {
        let x = 5;        // x is created
        r = &x;           // r borrows x
    }                     // x is dropped here
    
    println!("r: {}", r); // ERROR: x doesn't live long enough
}
```

The compiler prevents us from using a reference to freed memory!

## 📝 Lifetime Annotations

Lifetime annotations describe the relationships between lifetimes of references. They don't change how long anything lives - they just describe the constraints.

### Basic Syntax

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime named 'a
&'a mut i32 // a mutable reference with lifetime 'a
```

### Function Signatures

Sometimes the compiler needs help understanding lifetime relationships:

```rust
// This won't compile without lifetime annotations
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

With lifetime annotations:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This says: "The returned reference will be valid as long as both input references are valid."

## 🔍 Understanding Lifetime Annotations

Let's break down what lifetime annotations mean:

```rust
fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
```

- `'a` and `'b` are lifetime parameters (like generic type parameters)
- `x` has lifetime `'a`
- `y` has lifetime `'b` 
- The return value has lifetime `'a`
- This means the return value can't outlive `x`

## 📐 Lifetime Elision Rules

Rust has lifetime elision rules that allow you to omit lifetime annotations in common cases:

### Rule 1: Each Input Reference Gets Its Own Lifetime

```rust
fn foo(x: &str, y: &str)
// Compiler sees: fn foo<'a, 'b>(x: &'a str, y: &'b str)
```

### Rule 2: One Input Lifetime Assigns to All Outputs

```rust
fn foo(x: &str) -> &str
// Compiler sees: fn foo<'a>(x: &'a str) -> &'a str
```

### Rule 3: Methods - &self Lifetime Assigns to Outputs

```rust
impl SomeStruct {
    fn method(&self, x: &str) -> &str
    // Compiler sees: fn method<'a, 'b>(&'a self, x: &'b str) -> &'a str
}
```

## 🏗️ Structs with Lifetime Parameters

When structs hold references, they need lifetime parameters:

```rust
// This struct holds a reference, so it needs a lifetime parameter
struct Book<'a> {
    title: &'a str,
    author: &'a str,
    year: u32,
}

impl<'a> Book<'a> {
    fn new(title: &'a str, author: &'a str, year: u32) -> Self {
        Book { title, author, year }
    }
    
    fn title(&self) -> &str {
        self.title
    }
}

fn main() {
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik and Carol Nichols");
    
    let book = Book::new(&title, &author, 2018);
    println!("{} by {}", book.title(), book.author);
}
```

## 🌟 Static Lifetime

The `'static` lifetime is special - it means the reference is valid for the entire program:

```rust
// String literals have 'static lifetime
let s: &'static str = "I live forever!";

// This function only accepts 'static references
fn requires_static(s: &'static str) {
    println!("Static string: {}", s);
}

fn main() {
    requires_static("Hello");  // OK - string literals are 'static
    
    let dynamic = String::from("Dynamic");
    // requires_static(&dynamic);  // ERROR - dynamic is not 'static
}
```

## 🔄 Lifetime Bounds

You can specify relationships between lifetimes:

```rust
// 'a must live at least as long as 'b
fn example<'a: 'b, 'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > 0 {
        x  // OK because 'a: 'b (a outlives b)
    } else {
        y
    }
}
```

## 🐛 Common Lifetime Errors

### Error 1: Returning Reference to Local Value

```rust
fn bad_function() -> &str {
    let s = String::from("hello");
    &s  // ERROR: returns reference to local variable
}
```

**Fix**: Return owned data:
```rust
fn good_function() -> String {
    String::from("hello")
}
```

### Error 2: Lifetime Mismatch

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }  // string2 dropped here
    println!("The longest string is {}", result);  // ERROR if result borrows string2
}
```

**Fix**: Ensure all references live long enough:
```rust
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}
```

## 💡 Practical Example: String Cache

Let's build a cache that holds references:

```rust
use std::collections::HashMap;

struct Cache<'a> {
    data: HashMap<String, &'a str>,
}

impl<'a> Cache<'a> {
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: String, value: &'a str) {
        self.data.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&'a str> {
        self.data.get(key).copied()
    }
}

fn main() {
    // The data we're caching references to
    let data1 = String::from("Hello, World!");
    let data2 = String::from("Rust is awesome!");
    
    let mut cache = Cache::new();
    cache.insert("greeting".to_string(), &data1);
    cache.insert("opinion".to_string(), &data2);
    
    if let Some(greeting) = cache.get("greeting") {
        println!("Cached: {}", greeting);
    }
    
    // cache outlives this scope, but that's OK because
    // data1 and data2 also outlive this scope
}
```

## 🎯 Lifetime Thinking Process

When you see a lifetime error:

1. **Identify the references involved**
2. **Trace where the data comes from**
3. **Determine how long the data lives**
4. **Ensure references don't outlive their data**
5. **Add lifetime annotations to express the relationships**

## 📝 Key Takeaways

1. **Lifetimes Describe Relationships**: They don't extend lifetimes, they describe them
2. **Compiler Inference**: Many lifetimes are inferred automatically
3. **Explicit When Needed**: Complex relationships need annotations
4. **'static is Forever**: Valid for the entire program
5. **Safety Guarantee**: Prevents dangling references at compile time

## 🔗 Comparison with C#

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| GC keeps refs valid | Lifetime checking | Compile-time vs runtime |
| No explicit lifetimes | Lifetime annotations | Explicit relationships |
| Weak references | Lifetime bounds | Compile-time guarantees |
| Object lifetime | Value lifetime | Scope-based |

## ✏️ Practice Exercises

1. **Lifetime Annotations**: Write functions that require explicit lifetime annotations. Try different combinations.

2. **Struct Lifetimes**: Create structs that hold references with different lifetime requirements.

3. **Fix the Errors**: Debug lifetime errors in the exercise files.

4. **Cache Implementation**: Extend the cache example to support multiple lifetime parameters.

---

Next: [Smart Pointers](04-smart-pointers.md) - When single ownership isn't enough →
