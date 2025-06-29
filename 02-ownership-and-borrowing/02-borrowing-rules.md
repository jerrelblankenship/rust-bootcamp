# Lesson 02: Borrowing Rules

Learn to use values without taking ownership through Rust's borrowing system. This lesson covers references, the borrow checker, and how to work with data safely and efficiently.

## 🎯 Learning Objectives

- Create and use immutable and mutable references
- Master Rust's borrowing rules and understand why they exist
- Apply the principle of exclusive mutable access
- Debug borrow checker errors using compiler guidance
- Write functions that borrow instead of taking ownership
- Understand reference lifetimes and scope

## 📚 Introduction

In the previous lesson, we saw how ownership transfer can be limiting. What if you want to use a value multiple times without transferring ownership each time? This is where **borrowing** comes in.

Borrowing allows you to access data without taking ownership. It's similar to passing by reference in C#, but with compile-time guarantees that prevent data races and memory safety issues.

## 🔗 References: Borrowing Without Owning

### Immutable References (&T)

```rust
fn calculate_length(s: &String) -> usize {
    s.len()  // We can read s, but not modify it
} // s goes out of scope, but nothing is dropped (we don't own the data)

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &s1 creates a reference
    
    println!("The length of '{}' is {}.", s1, len);
    // s1 is still valid! We only borrowed it
}
```

The `&` symbol creates a reference (borrow) to the value without taking ownership.

### Mutable References (&mut T)

```rust
fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);  // Prints "hello, world"
}
```

## 🔐 The Two Rules of Borrowing

Rust enforces strict borrowing rules to prevent data races at compile time:

### Rule 1: Exclusive Mutable Access

You can have **either**:
- Any number of immutable references (`&T`)
- **OR** exactly one mutable reference (`&mut T`)
- But **not both** at the same time!

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Multiple immutable references - OK
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    // Now we can have a mutable reference
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}", r3);
}
```

### Rule 2: References Must Always Be Valid

References must not outlive the data they point to (no dangling references):

```rust
fn main() {
    let reference_to_nothing = dangle(); // This won't compile!
}

fn dangle() -> &String {  // ERROR!
    let s = String::from("hello");
    &s  // We return a reference to s, but s is dropped here
} // s goes out of scope and is dropped
```

**Fix**: Return owned data instead:
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Ownership is moved to caller
}
```

## 🎭 Reference Scope and Non-Lexical Lifetimes

References are valid from their creation until their last use:

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // Immutable borrow starts
    let r2 = &s;      // Another immutable borrow
    println!("{} and {}", r1, r2);  // Last use of r1 and r2
    
    // Now we can create a mutable reference because r1 and r2 aren't used anymore
    let r3 = &mut s;  // Mutable borrow is OK here
    println!("{}", r3);
}
```

## 🐛 Common Borrowing Errors and Solutions

### Error 1: Simultaneous Mutable and Immutable Borrows

```rust
// This won't compile
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;       // Immutable borrow
    let r2 = &mut s;   // ERROR: cannot borrow as mutable
    
    println!("{}, {}", r1, r2);
}
```

**Fix**: Ensure borrows don't overlap:
```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    println!("{}", r1);  // Last use of r1
    
    let r2 = &mut s;     // Now it's OK
    r2.push_str(", world");
    println!("{}", r2);
}
```

### Error 2: Multiple Mutable References

```rust
// This won't compile
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    let r2 = &mut s;   // ERROR: cannot borrow as mutable more than once
    
    println!("{}, {}", r1, r2);
}
```

**Fix**: Use one at a time:
```rust
fn main() {
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
        r1.push_str(", world");
    } // r1 goes out of scope
    
    let r2 = &mut s;   // Now it's OK
    r2.push_str("!");
}
```

### Error 3: Iterator Invalidation

```rust
// This won't compile
fn main() {
    let mut v = vec![1, 2, 3];
    
    for i in &v {
        v.push(4);     // ERROR: cannot borrow as mutable
        println!("{}", i);
    }
}
```

**Fix**: Collect what you need first:
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let items_to_add: Vec<_> = v.iter().map(|&x| x + 10).collect();
    
    for item in items_to_add {
        v.push(item);
    }
    
    println!("Final vector: {:?}", v);
}
```

## 🔍 Slices: Borrowing Parts of Collections

Slices let you reference a contiguous sequence of elements:

```rust
fn main() {
    // String slices
    let s = String::from("hello world");
    
    let hello = &s[0..5];    // or &s[..5]
    let world = &s[6..11];   // or &s[6..]
    let whole = &s[..];      // entire string
    
    println!("{} {}", hello, world);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];    // [2, 3]
    
    // Function that works with slices
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let word = first_word("hello world");
    println!("First word: {}", word);
}
```

## 🔄 Comparison with C#

| C# Reference Model | Rust Borrowing | Key Difference |
|-------------------|----------------|----------------|
| `ref` parameter | `&T` reference | Immutable by default |
| `ref` with mutation | `&mut T` reference | Exclusive mutable access |
| `in` parameter | `&T` reference | Same immutability guarantee |
| `out` parameter | Return value | Rust prefers return values |
| Multiple refs to object | Multiple `&T` OR one `&mut T` | Prevents data races |
| Reference types always | Explicit borrowing | Clear ownership intent |

## 💻 Practice Exercises

### Exercise 1: Reference Practice

```rust
fn main() {
    // Fix this code to compile
    let mut data = vec![1, 2, 3];
    
    let first_ref = &data[0];
    data.push(4);  // This will cause an error - why?
    println!("First element: {}", first_ref);
    
    // How can you fix this?
}
```

### Exercise 2: Borrowing in Functions

```rust
// Complete these function signatures and implementations
fn calculate_length(s: /* what type? */) -> usize {
    // Return the length without taking ownership
}

fn make_plural(s: /* what type? */) {
    // Add an "s" to the end of the string
}

fn get_first_word(s: /* what type? */) -> /* what type? */ {
    // Return the first word without taking ownership
}

fn main() {
    let mut text = String::from("hello");
    
    let len = calculate_length(/* what goes here? */);
    make_plural(/* what goes here? */);
    let first = get_first_word(/* what goes here? */);
    
    println!("Length: {}, Text: {}, First: {}", len, text, first);
}
```

### Exercise 3: Slice Operations

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Write functions that work with slices:
    // 1. Find the sum of a slice
    // 2. Find the maximum value in a slice
    // 3. Return a slice of even numbers (indices)
    
    // Test with different slices
    println!("Sum of first 5: {}", sum_slice(/* what slice? */));
    println!("Max of last 3: {}", max_slice(/* what slice? */));
}
```

## 🚀 Mini-Project: String Analyzer

Build a string analysis tool that uses borrowing efficiently:

```rust
struct StringAnalyzer;

impl StringAnalyzer {
    fn word_count(text: &str) -> usize {
        text.split_whitespace().count()
    }
    
    fn char_count(text: &str) -> usize {
        text.chars().count()
    }
    
    fn longest_word(text: &str) -> &str {
        text.split_whitespace()
            .max_by_key(|word| word.len())
            .unwrap_or("")
    }
    
    fn starts_with_vowel(text: &str) -> bool {
        text.chars()
            .next()
            .map(|c| "aeiouAEIOU".contains(c))
            .unwrap_or(false)
    }
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog";
    
    println!("Text: {}", text);
    println!("Words: {}", StringAnalyzer::word_count(text));
    println!("Characters: {}", StringAnalyzer::char_count(text));
    println!("Longest word: {}", StringAnalyzer::longest_word(text));
    println!("Starts with vowel: {}", StringAnalyzer::starts_with_vowel(text));
    
    // The original text is still available!
    println!("Original text still available: {}", text);
}
```

## 🔑 Key Takeaways

1. **Borrowing enables sharing**: Use data without taking ownership
2. **Immutable by default**: References are read-only unless explicitly mutable
3. **Exclusive mutable access**: Only one mutable reference at a time prevents data races
4. **Reference lifetime tracking**: Compiler ensures references remain valid
5. **Zero runtime cost**: All checking happens at compile time
6. **Slices for partial access**: Borrow parts of collections efficiently

## 📚 Additional Resources

- [Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust by Example - Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)
- [Understanding the Borrow Checker](https://blog.logrocket.com/introducing-the-rust-borrow-checker/)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Create immutable and mutable references correctly
- [ ] Understand why the borrowing rules prevent data races
- [ ] Debug borrow checker errors using compiler messages
- [ ] Write functions that borrow parameters instead of taking ownership
- [ ] Use slices to work with parts of collections
- [ ] Explain when references are valid and when they expire

---

Next: [Lifetimes](03-lifetimes.md) - Understanding how long references live →
