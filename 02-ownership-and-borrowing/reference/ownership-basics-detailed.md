# Lesson 01: Ownership Basics

Welcome to Rust's most revolutionary feature! Ownership is what makes Rust unique among programming languages, providing memory safety without garbage collection. Let's explore how it works and why it matters.

## 🎯 Learning Objectives

- Understand Rust's three fundamental ownership rules
- Distinguish between move and copy semantics
- Recognize how ownership prevents memory bugs that plague C#
- Write functions that properly transfer ownership
- Debug common ownership errors using compiler messages

## 📚 Introduction

Coming from C#, you're accustomed to the garbage collector managing memory automatically. Objects live on the heap, references can be copied freely, and the GC eventually reclaims unused memory. This approach is convenient but comes with costs: unpredictable GC pauses, memory overhead, and potential memory leaks.

Rust takes a fundamentally different approach with **ownership** - a compile-time memory management system that guarantees safety without any runtime overhead.

## 🔑 The Three Rules of Ownership

Rust's entire memory management system is built on three simple rules:

1. **Each value in Rust has a single owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped**

These rules prevent use-after-free bugs, double-free errors, and memory leaks at compile time.

## 🔄 Move vs Copy Semantics

### Copy Types (Stack)

```rust
// Simple types implement Copy trait
let x = 5;          // i32 stored on stack
let y = x;          // x is COPIED to y
println!("{}", x);  // Still works! x wasn't moved

// Other Copy types
let a = 3.14;       // f64
let b = true;       // bool  
let c = 'R';        // char
let point = (3, 4); // Tuple of Copy types
```

**Stack Memory Visualization:**
```
Before copy:              After copy:
┌─────────┐              ┌─────────┐
│ x = 5   │              │ x = 5   │
└─────────┘              └─────────┘
                         ┌─────────┐
                         │ y = 5   │
                         └─────────┘
Both variables have their own copy!
```

### Move Types (Heap)

```rust
// Complex types are moved, not copied
let s1 = String::from("Hello");
let s2 = s1;        // s1 is MOVED to s2

// This would cause a compilation error:
// println!("{}", s1);  // ERROR: value borrowed after move

// Vector example
let v1 = vec![1, 2, 3];
let v2 = v1;        // v1 is moved to v2
// println!("{:?}", v1); // ERROR: v1 no longer valid
```

**Heap Memory Visualization - Move Semantics:**
```
Before move:                      After move:
Stack        Heap                Stack        Heap
┌──────┐    ┌─────────┐         ┌──────┐    ┌─────────┐
│ s1 ──┼───►│ "Hello" │         │ s1 X │    │ "Hello" │
└──────┘    └─────────┘         └──────┘    └─────────┘
                                ┌──────┐          ▲
                                │ s2 ──┼──────────┘
                                └──────┘
s1 is invalidated, only s2 owns the heap data!
```

## 🏭 Functions and Ownership Transfer

### Taking Ownership

```rust
fn take_string(s: String) {
    println!("I now own: {}", s);
} // s goes out of scope and is dropped here

fn main() {
    let my_string = String::from("Hello");
    take_string(my_string);
    // println!("{}", my_string); // ERROR: my_string was moved
}
```

### Returning Ownership

```rust
fn give_string() -> String {
    let s = String::from("Created inside function");
    s  // Ownership transferred to caller
}

fn take_and_give_back(s: String) -> String {
    println!("Processing: {}", s);
    s  // Return ownership to caller
}

fn main() {
    let s1 = give_string();
    let s2 = take_and_give_back(s1);
    println!("Final string: {}", s2);
}
```

## 🛡️ Memory Safety Through Ownership

### Preventing Use-After-Free

```rust
fn main() {
    let s = String::from("Hello");
    {
        let s2 = s;  // s moved to s2
        println!("{}", s2);
    } // s2 dropped here
    
    // println!("{}", s); // ERROR: s was moved, can't use it
}
```

### Preventing Double-Free

```rust
// This is impossible in Rust due to ownership rules
let s1 = String::from("Hello");
let s2 = s1;  // s1 moved to s2

// Only s2 will be dropped - no double-free possible
```

## 🎨 The Clone Trait

When you need a deep copy (similar to C#'s `ICloneable`):

```rust
let s1 = String::from("Hello");
let s2 = s1.clone();  // Explicit deep copy

println!("{}", s1);   // Works! s1 still owns its data
println!("{}", s2);   // s2 owns a separate copy

// Clone is expensive - use when necessary
let expensive_data = vec![1; 1_000_000];
let copy = expensive_data.clone(); // Creates new 1M element vector
```

## 🔄 Comparison with C#

| C# Memory Model | Rust Ownership | Key Difference |
|-----------------|----------------|----------------|
| Garbage Collector | Compile-time ownership | When safety is ensured |
| Multiple references to objects | Single owner at a time | Prevents data races |
| Automatic memory management | Explicit ownership transfer | Who controls lifetime |
| Runtime memory errors possible | Compile-time memory safety | When errors are caught |
| `ICloneable` for deep copies | `Clone` trait | Both explicit |
| `using` statement for disposal | Automatic Drop | Resource cleanup |

## 💻 Practice Exercises

### Exercise 1: Understanding Move vs Copy

```rust
fn main() {
    // Fix this code to make it compile
    let x = vec![1, 2, 3];
    let y = x;
    println!("x: {:?}", x);  // How can we make this work?
    println!("y: {:?}", y);
    
    // Try the same with an integer
    let a = 42;
    let b = a;
    println!("a: {}", a);  // Does this work? Why?
    println!("b: {}", b);
}
```

### Exercise 2: Function Ownership

```rust
fn process_string(s: String) -> String {
    let processed = format!("Processed: {}", s);
    processed
}

fn main() {
    let my_string = String::from("Hello");
    let result = process_string(my_string);
    
    // Can we use my_string here? Why or why not?
    // println!("Original: {}", my_string);
    println!("Result: {}", result);
}
```

### Exercise 3: Scope and Drop

```rust
fn main() {
    println!("Starting program");
    
    {
        let s = String::from("Inner scope");
        println!("Created: {}", s);
    } // What happens to s here?
    
    println!("Back in outer scope");
    // Can we access s here?
}
```

## 🚀 Mini-Project: Simple Stack

Build a stack data structure to practice ownership:

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    // This method takes ownership of the item
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    // This method gives ownership to the caller
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn main() {
    let mut stack = Stack::new();
    
    // Test your stack
    stack.push(String::from("First"));
    stack.push(String::from("Second"));
    
    while let Some(item) = stack.pop() {
        println!("Popped: {}", item);
    }
}
```

## 🔑 Key Takeaways

1. **Single Ownership Rule**: Each value has exactly one owner at any time
2. **Move Semantics**: Assignment transfers ownership for non-Copy types
3. **Automatic Cleanup**: Values are automatically dropped when their owner goes out of scope
4. **Compile-Time Safety**: Ownership violations are caught at compile time, not runtime
5. **No Garbage Collection**: Deterministic destruction without GC overhead
6. **Explicit Cloning**: Deep copies must be requested explicitly with `.clone()`

## 📚 Additional Resources

- [Rust Book - Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - Ownership and Moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Memory Management in Programming Languages](https://deepu.tech/memory-management-in-programming/)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Explain the three rules of ownership in your own words
- [ ] Identify which types implement Copy vs Move semantics
- [ ] Write functions that transfer ownership correctly
- [ ] Debug basic ownership errors using compiler messages
- [ ] Understand when values are dropped and cleaned up
- [ ] Compare Rust's ownership with C#'s garbage collection

---

Next: [Borrowing Rules](02-borrowing-rules.md) - Learn how to use values without taking ownership →
