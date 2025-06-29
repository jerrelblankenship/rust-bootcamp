# Lesson 01: Ownership Basics

## 🎯 Learning Objectives

By the end of this lesson, you will:
- Understand Rust's three rules of ownership
- Distinguish between move and copy semantics
- Recognize how ownership differs from C#'s reference model
- Write functions that transfer ownership
- Debug common ownership errors

## 📚 Introduction

Coming from C#, you're used to the garbage collector managing memory behind the scenes. Objects live on the heap, references are freely copied, and the GC cleans up when objects are no longer reachable. This model has served you well, but it comes with costs: GC pauses, memory overhead, and unpredictable performance.

Rust takes a fundamentally different approach: **ownership**.

## 🔑 The Three Rules of Ownership

Rust's entire memory management system is built on three simple rules:

1. **Each value in Rust has a single owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped**

Let's see what this means in practice.

## 🔄 Ownership vs References: C# and Rust

### C# Reference Semantics

```csharp
public class Person {
    public string Name { get; set; }
    public int Age { get; set; }
}

// In C#
var alice = new Person { Name = "Alice", Age = 30 };
var bob = alice;  // Both variables reference the same object
bob.Age = 31;
Console.WriteLine(alice.Age);  // Prints 31 - alice was modified!
```

In C#, `alice` and `bob` are references pointing to the same object on the heap.

### Rust Ownership Semantics

```rust
struct Person {
    name: String,
    age: u32,
}

// In Rust
let alice = Person {
    name: String::from("Alice"),
    age: 30,
};

let bob = alice;  // Ownership MOVES from alice to bob
// println!("{}", alice.age);  // ERROR: alice no longer owns the value!
println!("{}", bob.age);  // This works - bob is now the owner
```

This is the key difference: **assignment transfers ownership** in Rust.

## 📦 Stack vs Heap: Where Data Lives

### Simple Types (Stack)

```rust
// These types implement Copy and live on the stack
let x = 42;          // i32
let y = x;           // x is COPIED to y
println!("{}", x);   // Still works!

let a = 3.14;        // f64
let b = a;           // Copied
let c = true;        // bool
let d = c;           // Copied
let e = 'R';         // char
let f = e;           // Copied
```

Types that implement the `Copy` trait are duplicated on assignment. These include:
- All integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, etc.)
- Floating point types (`f32`, `f64`)
- Boolean (`bool`)
- Character (`char`)
- Tuples containing only `Copy` types

### Complex Types (Heap)

```rust
// String allocates on the heap
let s1 = String::from("Hello");
let s2 = s1;  // s1 is MOVED to s2

// This would error:
// println!("{}", s1);  // ERROR: value borrowed after move

// Vec also allocates on the heap
let v1 = vec![1, 2, 3];
let v2 = v1;  // v1 is MOVED to v2
```

## 🎭 The Clone Trait

When you need a deep copy (like C#'s `ICloneable`):

```rust
let s1 = String::from("Hello");
let s2 = s1.clone();  // Explicit deep copy
println!("{}", s1);   // Works! s1 still owns its data
println!("{}", s2);   // s2 owns a separate copy
```

## 🔧 Functions and Ownership

### Taking Ownership

```rust
fn take_ownership(s: String) {
    println!("I now own: {}", s);
} // s goes out of scope and is dropped

fn main() {
    let my_string = String::from("Hello");
    take_ownership(my_string);
    // println!("{}", my_string);  // ERROR: my_string was moved
}
```

### Giving Ownership

```rust
fn give_ownership() -> String {
    let s = String::from("Created inside");
    s  // Ownership is transferred to caller
}

fn main() {
    let my_string = give_ownership();
    println!("{}", my_string);  // We own it now
}
```

### Taking and Giving Back

```rust
fn take_and_give_back(s: String) -> String {
    println!("Processing: {}", s);
    s  // Return ownership to caller
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = take_and_give_back(s1);
    println!("Got back: {}", s2);
}
```

## 🐛 Common Ownership Errors

### Error 1: Use After Move

```rust
let s1 = String::from("Hello");
let s2 = s1;
println!("{}", s1);  // ERROR: borrow of moved value: `s1`
```

**Fix**: Clone if you need both:
```rust
let s1 = String::from("Hello");
let s2 = s1.clone();
println!("{} {}", s1, s2);  // Both work!
```

### Error 2: Moving in a Loop

```rust
let data = vec![1, 2, 3];
for _ in 0..3 {
    do_something(data);  // ERROR: use of moved value: `data`
}
```

**Fix**: Clone or borrow (we'll cover borrowing next):
```rust
let data = vec![1, 2, 3];
for _ in 0..3 {
    do_something(data.clone());
}
```

## 💡 Understanding Drop

When a value goes out of scope, Rust calls its destructor (the `Drop` trait):

```rust
{
    let s = String::from("Hello");
    // s is valid here
}   // s goes out of scope, memory is freed

// This is like C#'s IDisposable, but automatic:
// No need for 'using' statements!
```

## 🎯 Practical Example: Building a Stack

Let's build a simple stack to see ownership in action:

```rust
struct Stack {
    items: Vec<String>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    // This method takes ownership of the item
    fn push(&mut self, item: String) {
        self.items.push(item);
    }
    
    // This method gives ownership to the caller
    fn pop(&mut self) -> Option<String> {
        self.items.pop()
    }
}

fn main() {
    let mut stack = Stack::new();
    
    let item1 = String::from("First");
    stack.push(item1);
    // println!("{}", item1);  // ERROR: item1 was moved
    
    if let Some(popped) = stack.pop() {
        println!("Popped: {}", popped);  // We own popped
    }
}
```

## 🤔 Ownership Philosophy

Why does Rust do this? Three main benefits:

1. **Memory Safety**: No use-after-free bugs
2. **Thread Safety**: No data races (we'll see this later)
3. **Performance**: No GC overhead, predictable performance

## 📝 Key Takeaways

1. **One Owner Rule**: Each value has exactly one owner
2. **Move Semantics**: Assignment moves ownership (for non-Copy types)
3. **Automatic Cleanup**: Values are dropped when owner goes out of scope
4. **Explicit Cloning**: Deep copies must be explicit with `.clone()`
5. **Compile-Time Checks**: Ownership rules enforced at compile time

## 🔗 Comparison with C#

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| Reference assignment | Move | Ownership transferred |
| `new` keyword | Various (`new()`, `from()`) | Ownership starts here |
| Garbage collection | Automatic drop | Deterministic |
| `ICloneable` | `Clone` trait | Must be explicit |
| `using` statement | Scope-based drop | Automatic |

## ✏️ Practice Exercises

1. **Ownership Transfer**: Write a function that takes a `Vec<i32>` and returns its length. What happens to the Vec?

2. **Clone vs Move**: Create two functions: one that moves a String and one that clones it. Call each multiple times.

3. **Scope and Drop**: Create nested scopes and observe when values are dropped. Use `println!` to track destruction.

4. **Fix the Errors**: Debug common ownership mistakes in the exercise files.

---

Next: [Borrowing and References](02-borrowing-rules.md) - Learn how to use values without taking ownership →
