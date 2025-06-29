# Lesson 03: Lifetimes

Master Rust's lifetime system to ensure references remain valid and prevent dangling pointers. This lesson covers lifetime annotations, elision rules, and how to work with complex reference relationships.

## 🎯 Learning Objectives

- Understand what lifetimes are and why they're necessary
- Read and write lifetime annotations correctly
- Apply lifetime elision rules to reduce boilerplate
- Write structs and methods with lifetime parameters
- Debug lifetime-related compiler errors effectively
- Create functions that return references safely

## 📚 Introduction

Lifetimes are Rust's way of tracking how long references are valid. They're perhaps the most unique feature of Rust and can be challenging for developers coming from garbage-collected languages like C#.

**Key insight**: Lifetimes don't control how long values live - they describe the relationships between reference lifetimes to help the compiler verify safety.

## 🤔 The Problem Lifetimes Solve

In C#, the garbage collector ensures references remain valid. In Rust, the compiler needs proof at compile time that all references are valid:

```rust
// This won't compile - lifetime error
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

## 📝 Lifetime Annotation Syntax

Lifetime annotations describe relationships between reference lifetimes:

```rust
&i32        // a reference
&'a i32     // a reference with explicit lifetime 'a
&'a mut i32 // a mutable reference with lifetime 'a
```

### Function Lifetime Annotations

Sometimes the compiler needs help understanding relationships:

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

**With lifetime annotations:**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}
```

This says: "The returned reference will be valid as long as both input references are valid."

## 🔍 Understanding Lifetime Parameters

Let's break down what lifetime annotations mean:

```rust
fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x  // We can only return x because return type has lifetime 'a
}
```

- `'a` and `'b` are lifetime parameters (like generic type parameters)
- `x` has lifetime `'a`, `y` has lifetime `'b`
- Return value has lifetime `'a`
- This means the return value can't outlive `x`

### Multiple Lifetime Parameters

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn announce_and_return_part<'a, 'b>(
    announcement: &'a str,
    x: &'b str,
    y: &'b str,
) -> &'b str {
    println!("Attention please: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 📐 Lifetime Elision Rules

Rust has rules that allow you to omit lifetime annotations in common cases:

### Rule 1: Each Input Reference Gets Its Own Lifetime

```rust
fn foo(x: &str, y: &str)
// Compiler sees: fn foo<'a, 'b>(x: &'a str, y: &'b str)
```

### Rule 2: Single Input Lifetime Assigns to All Outputs

```rust
fn foo(x: &str) -> &str
// Compiler sees: fn foo<'a>(x: &'a str) -> &'a str
```

### Rule 3: Method Self Lifetime Assigns to Outputs

```rust
impl SomeStruct {
    fn method(&self, x: &str) -> &str
    // Compiler sees: fn method<'a, 'b>(&'a self, x: &'b str) -> &'a str
}
```

## 🏗️ Structs with Lifetime Parameters

When structs hold references, they need lifetime parameters:

```rust
// Struct that holds references
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn new(text: &'a str, start: usize, end: usize) -> Self {
        ImportantExcerpt {
            part: &text[start..end],
        }
    }
    
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part  // Returns reference with same lifetime as self
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = ImportantExcerpt::new(&novel, 0, first_sentence.len());
    println!("Excerpt: {}", excerpt.part);
}
```

## 🌟 The Static Lifetime

The `'static` lifetime means the reference is valid for the entire program:

```rust
// String literals have 'static lifetime
let s: &'static str = "I live for the entire program!";

// Function that only accepts 'static references
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

// Generic with lifetime bounds
struct Parser<'text, T> 
where 
    T: 'text,  // T must live at least as long as 'text
{
    text: &'text str,
    current_token: Option<T>,
}
```

## 🐛 Common Lifetime Errors and Solutions

### Error 1: Returning Reference to Local Value

```rust
// This won't compile
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
// This won't compile
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }  // string2 dropped here, but result might reference it
    println!("The longest string is {}", result);  // ERROR
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

## 🔄 Comparison with C#

| C# Memory Model | Rust Lifetimes | Key Difference |
|-----------------|-----------------|----------------|
| GC keeps references valid | Lifetime annotations | Compile-time vs runtime |
| No explicit relationship tracking | Explicit lifetime relationships | Developer control |
| Weak references for cycles | Lifetime bounds and 'static | Safety guarantees |
| Object lifetime automatic | Value lifetime scope-based | Deterministic cleanup |
| Runtime null reference errors | Compile-time lifetime errors | When errors occur |

## 💻 Practice Exercises

### Exercise 1: Basic Lifetime Annotations

```rust
// Add lifetime annotations to make these functions compile
fn longer_string(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_part(s: &str, delimiter: char) -> &str {
    for (i, ch) in s.char_indices() {
        if ch == delimiter {
            return &s[..i];
        }
    }
    s
}
```

### Exercise 2: Struct with Lifetimes

```rust
// Fix this struct definition and implementation
struct TextProcessor {
    content: &str,  // What lifetime parameter is needed?
    line_endings: Vec<usize>,
}

impl TextProcessor {
    fn new(text: &str) -> TextProcessor {
        let mut endings = vec![];
        for (i, ch) in text.char_indices() {
            if ch == '\n' {
                endings.push(i);
            }
        }
        
        TextProcessor {
            content: text,
            line_endings: endings,
        }
    }
    
    fn get_line(&self, line_num: usize) -> &str {
        // Implementation details...
        self.content
    }
}
```

### Exercise 3: Complex Lifetime Relationships

```rust
// Create a function that takes two string slices and returns
// the one that starts with a vowel, or the first one if neither
// or both start with vowels

fn choose_by_vowel(first: &str, second: &str) -> &str {
    let first_starts_vowel = first.chars()
        .next()
        .map(|c| "aeiouAEIOU".contains(c))
        .unwrap_or(false);
    
    let second_starts_vowel = second.chars()
        .next()
        .map(|c| "aeiouAEIOU".contains(c))
        .unwrap_or(false);
    
    match (first_starts_vowel, second_starts_vowel) {
        (true, false) => first,
        (false, true) => second,
        _ => first,  // Default to first
    }
}
```

## 🚀 Mini-Project: Configuration Parser

Build a configuration parser that borrows string data:

```rust
use std::collections::HashMap;

struct ConfigParser<'a> {
    content: &'a str,
    current_line: usize,
}

impl<'a> ConfigParser<'a> {
    fn new(content: &'a str) -> Self {
        ConfigParser {
            content,
            current_line: 0,
        }
    }
    
    fn parse(&mut self) -> HashMap<&'a str, &'a str> {
        let mut config = HashMap::new();
        
        for line in self.content.lines() {
            self.current_line += 1;
            
            // Skip empty lines and comments
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse key=value pairs
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim();
                let value = line[eq_pos + 1..].trim();
                config.insert(key, value);
            }
        }
        
        config
    }
    
    fn get_current_line(&self) -> usize {
        self.current_line
    }
}

fn main() {
    let config_text = r#"
        # Database configuration
        host = localhost
        port = 5432
        database = myapp
        
        # Cache settings
        cache_size = 1000
        cache_ttl = 3600
    "#;
    
    let mut parser = ConfigParser::new(config_text);
    let config = parser.parse();
    
    for (key, value) in &config {
        println!("{} = {}", key, value);
    }
    
    println!("Processed {} lines", parser.get_current_line());
}
```

## 🔑 Key Takeaways

1. **Lifetimes describe relationships**: They don't extend lifetimes, they describe constraints
2. **Compiler inference works most of the time**: Many lifetimes are inferred automatically
3. **Explicit annotations for complex cases**: Use when the compiler needs help
4. **'static means program duration**: Valid for the entire program lifetime
5. **Safety at compile time**: Prevents dangling references before runtime
6. **Zero runtime cost**: All lifetime checking happens at compile time

## 📚 Additional Resources

- [Rust Book - Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust by Example - Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Explain what lifetimes are and why they're necessary
- [ ] Write basic lifetime annotations for functions
- [ ] Create structs that hold references with lifetime parameters
- [ ] Understand when lifetime annotations can be omitted
- [ ] Debug lifetime errors using compiler messages
- [ ] Distinguish between 'static and other lifetime parameters

---

Next: [Smart Pointers](04-smart-pointers.md) - When single ownership isn't enough →
