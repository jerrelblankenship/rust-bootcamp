# Exercise 1 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working implementation. Here's the full solution with detailed explanations.

## 📝 Complete ex01-ownership.rs Implementation

```rust
// Exercise 1: Ownership Basics - Complete Solutions
//
// This file demonstrates all ownership concepts from Module 02

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 1: Ownership Basics (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    exercise_1_1();
    exercise_1_2();
    exercise_1_3();
    exercise_1_4();
    exercise_1_5();
    exercise_1_6();
    exercise_1_7();
    exercise_1_8();
    
    println!("\n🎉 All exercises completed successfully!");
}

// Exercise 1.1: Fixed move error with cloning
fn exercise_1_1() {
    println!("Exercise 1.1: Fix the move error");
    
    let s1 = String::from("hello");
    let s2 = s1.clone();  // SOLUTION: Clone creates independent copy
    
    // Both print statements work now
    println!("s1 = {}, s2 = {}", s1, s2);
    
    println!("✅ Exercise 1.1 complete\n");
}

// Exercise 1.2: Understanding Copy vs Move with fixes
fn exercise_1_2() {
    println!("Exercise 1.2: Understanding Copy vs Move semantics");
    
    // This works - integers implement Copy trait
    let x = 5;
    let y = x;  // x is copied, not moved
    println!("x = {}, y = {}", x, y);
    
    // SOLUTION: Clone the String to fix compilation error
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Explicit clone for heap-allocated data
    println!("s1 = {}, s2 = {}", s1, s2);
    
    // This works - fixed-size arrays of Copy types implement Copy
    let a = [1, 2, 3];
    let b = a;  // Entire array is copied
    println!("a = {:?}, b = {:?}", a, b);
    
    // SOLUTION: Clone the Vec to fix compilation error
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();  // Vec doesn't implement Copy, needs explicit clone
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
    
    // Answers to questions:
    println!("1. Copy trait types: i32, bool, char, fixed arrays of Copy types");
    println!("2. Explicit cloning needed: String, Vec, HashMap, custom structs");
    println!("3. Copy vs Move: Copy is automatic/cheap, Move transfers ownership");
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Functions and ownership with borrowing solution
fn exercise_1_3() {
    println!("Exercise 1.3: Functions and ownership");
    
    let s = String::from("hello");
    
    // SOLUTION: Use borrowing function instead of ownership-taking
    borrows_string(&s);  // Pass reference, not ownership
    println!("Can I still use s? {}", s);  // Works because s still owns the data
    
    // Alternative solution: get ownership back
    let s2 = String::from("world");
    let s2 = takes_and_returns(s2);
    println!("Got ownership back: {}", s2);
    
    println!("✅ Exercise 1.3 complete\n");
}

// SOLUTION: Function that borrows instead of taking ownership
fn borrows_string(some_string: &String) {
    println!("Borrowed: {}", some_string);
    // No ownership transfer - original remains valid
}

// SOLUTION: Function that takes and returns ownership
fn takes_and_returns(some_string: String) -> String {
    println!("Got and returning: {}", some_string);
    some_string  // Return ownership to caller
}

// Keep original function for educational comparison
fn takes_ownership(some_string: String) {
    println!("Function received: {}", some_string);
}

// Exercise 1.4: Ownership return patterns with borrowing solution
fn exercise_1_4() {
    println!("Exercise 1.4: Returning ownership patterns");
    
    let s1 = String::from("hello");
    
    // SOLUTION: Use borrowing version that doesn't take ownership
    let s2 = append_world_borrow(&s1);
    
    println!("Original: {}", s1);  // Works because s1 wasn't moved
    println!("Result: {}", s2);
    
    println!("✅ Exercise 1.4 complete\n");
}

// Original function that takes ownership (for comparison)
fn append_world(mut s: String) -> String {
    s.push_str(" world");
    s
}

// SOLUTION: Borrowing version that creates new string
fn append_world_borrow(s: &str) -> String {
    format!("{} world", s)  // Create new string without taking ownership
}

// Exercise 1.5: Clone vs Move usage patterns
fn exercise_1_5() {
    println!("Exercise 1.5: Clone vs Move");
    
    let original = String::from("hello");
    
    // These work multiple times because they borrow
    use_string(&original);
    use_string(&original);
    
    // These work once because they move ownership
    use_string_once(original.clone());  // Clone for first call
    // SOLUTION: Clone again for multiple calls
    use_string_once(original.clone());  // Clone for second call
    
    // Alternative: use function that clones internally
    use_string_with_clone(&original);
    use_string_with_clone(&original);
    
    println!("✅ Exercise 1.5 complete\n");
}

fn use_string(s: &String) {
    println!("Using borrowed string: {}", s);
}

fn use_string_once(s: String) {
    println!("Using owned string: {}", s);
}

// SOLUTION: Function that clones internally for multiple use
fn use_string_with_clone(s: &String) {
    let owned_copy = s.clone();
    println!("Using cloned string: {}", owned_copy);
    // owned_copy is dropped here, original remains untouched
}

// Exercise 1.6: Ownership in collections with different strategies
fn exercise_1_6() {
    println!("Exercise 1.6: Ownership in collections");
    
    // Strategy 1: Clone before pushing (keeps originals usable)
    let mut vec = Vec::new();
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    vec.push(s1.clone());  // Clone before move
    vec.push(s2.clone());  // Clone before move
    println!("s1: {}", s1);  // Original still usable
    println!("s2: {}", s2);  // Original still usable
    println!("vec: {:?}", vec);
    
    // Strategy 2: Accept the move (more efficient, originals become invalid)
    let mut vec2 = Vec::new();
    let s3 = String::from("moved");
    let s4 = String::from("data");
    
    vec2.push(s3);  // s3 is moved into vec2
    vec2.push(s4);  // s4 is moved into vec2
    // s3 and s4 are no longer usable - but that's OK for this strategy
    println!("vec2: {:?}", vec2);
    
    // Strategy 3: Vec of references (when data lives elsewhere)
    let data1 = String::from("reference");
    let data2 = String::from("example");
    let ref_vec: Vec<&String> = vec![&data1, &data2];
    println!("ref_vec: {:?}", ref_vec);
    println!("data1 still usable: {}", data1);
    
    println!("✅ Exercise 1.6 complete\n");
}

// Exercise 1.7: Complete Stack implementation
fn exercise_1_7() {
    println!("Exercise 1.7: Implement a Stack");
    
    let mut stack = Stack::new();
    stack.push(String::from("first"));
    stack.push(String::from("second"));
    stack.push(String::from("third"));
    
    println!("Stack size: {}", stack.len());
    
    // Pop items and show ownership transfer
    while let Some(item) = stack.pop() {
        println!("Popped: {} (now I own this string)", item);
        // item is dropped here when it goes out of scope
    }
    
    println!("Stack is empty: {}", stack.is_empty());
    
    // Demonstrate peek (borrow without taking ownership)
    let mut stack2 = Stack::new();
    stack2.push(String::from("peek_test"));
    
    if let Some(top) = stack2.peek() {
        println!("Top item (peeked): {}", top);
    }
    println!("Stack still has {} items", stack2.len());
    
    println!("✅ Exercise 1.7 complete\n");
}

// SOLUTION: Complete Stack implementation
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);  // Takes ownership of item
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()  // Transfers ownership to caller
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    // Bonus: Peek returns reference without taking ownership
    fn peek(&self) -> Option<&T> {
        self.items.last()  // Borrow the last item
    }
}

// Exercise 1.8: Drop and RAII demonstration
fn exercise_1_8() {
    println!("Exercise 1.8: Understanding Drop and RAII");
    
    println!("Starting outer scope");
    {
        let d1 = Droppable::new("d1");
        println!("Starting inner scope");
        {
            let d2 = Droppable::new("d2");
            let d3 = Droppable::new("d3");
            
            // d3 and d2 will be dropped when this scope ends (LIFO order)
        }
        println!("Inner scope ended");
        // d1 will be dropped when outer scope ends
    }
    println!("Outer scope ended");
    
    // Testing move semantics with Drop
    println!("\nTesting move semantics:");
    let d4 = Droppable::new("d4");
    let d5 = d4; // d4 is moved to d5
    
    // Only d5 will be dropped - d4 was moved and no longer owns the data
    // Expected: only one drop message for d5
    
    println!("✅ Exercise 1.8 complete\n");
}

// SOLUTION: Complete Droppable implementation
struct Droppable {
    name: String,
}

impl Droppable {
    fn new(name: &str) -> Self {
        println!("🔧 Creating resource: {}", name);
        Droppable {
            name: name.to_string(),
        }
    }
}

// SOLUTION: Drop implementation shows when cleanup happens
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("🗑️ Dropping resource: {}", self.name);
    }
}

// BONUS IMPLEMENTATIONS

// Challenge 1: Function that can use a String multiple times
fn use_string_multiple_times(s: &str) {  // Takes &str to work with String and literals
    for i in 0..3 {
        println!("Using {} (iteration {})", s, i);
    }
}

// Challenge 2: Function that optionally takes ownership
fn maybe_take_ownership(s: String, take: bool) -> Option<String> {
    if take {
        println!("Taking ownership of: {}", s);
        None  // Consumed the string, return None
    } else {
        println!("Not taking ownership of: {}", s);
        Some(s)  // Return the string back
    }
}

// Challenge 3: Custom String type that tracks copies
#[derive(Debug)]
struct TrackedString {
    value: String,
    id: u32,
}

impl TrackedString {
    fn new(value: &str) -> Self {
        static mut NEXT_ID: u32 = 1;
        let id = unsafe {
            let current = NEXT_ID;
            NEXT_ID += 1;
            current
        };
        
        println!("🆔 Creating TrackedString {} with value: {}", id, value);
        TrackedString {
            value: value.to_string(),
            id,
        }
    }
}

impl Clone for TrackedString {
    fn clone(&self) -> Self {
        println!("📋 Cloning TrackedString {} ({})", self.id, self.value);
        TrackedString::new(&self.value)  // Creates new ID
    }
}

impl Drop for TrackedString {
    fn drop(&mut self) {
        println!("🗑️ Dropping TrackedString {} ({})", self.id, self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }
    
    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1);  // Peek doesn't remove
        
        assert_eq!(stack.pop(), Some(42));
        assert_eq!(stack.peek(), None);
    }
    
    #[test]
    fn test_ownership_patterns() {
        // Test cloning
        let s1 = String::from("test");
        let s2 = s1.clone();
        assert_eq!(s1, "test");
        assert_eq!(s2, "test");
        
        // Test borrowing
        let s3 = String::from("borrow");
        let len = calculate_length(&s3);
        assert_eq!(len, 6);
        assert_eq!(s3, "borrow");  // s3 still usable
    }
    
    #[test]
    fn test_maybe_take_ownership() {
        let s1 = String::from("test");
        let result = maybe_take_ownership(s1, false);
        assert_eq!(result, Some(String::from("test")));
        
        let s2 = String::from("consumed");
        let result = maybe_take_ownership(s2, true);
        assert_eq!(result, None);
    }
}

// Helper function for tests
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## 🎓 Complete Code Walkthrough

### 1. Ownership Transfer Patterns
```rust
// Move (ownership transfer)
let s1 = String::from("hello");
let s2 = s1;  // s1 no longer valid

// Clone (independent copy)
let s1 = String::from("hello");
let s2 = s1.clone();  // Both valid, separate data

// Borrow (shared access)
let s1 = String::from("hello");
let s2 = &s1;  // s1 still owns, s2 just references
```

### 2. Function Ownership Patterns
```rust
// Consuming function (takes ownership)
fn consume(s: String) { ... }
let data = String::from("test");
consume(data);  // data is no longer usable

// Borrowing function (shared access)
fn borrow(s: &String) { ... }
let data = String::from("test");
borrow(&data);  // data still usable

// Mutable borrowing (exclusive access)
fn modify(s: &mut String) { ... }
let mut data = String::from("test");
modify(&mut data);  // data still usable, possibly modified
```

### 3. Collection Ownership Strategies
```rust
// Strategy 1: Clone before insertion
vec.push(item.clone());  // Original remains usable

// Strategy 2: Move into collection
vec.push(item);  // item is consumed, more efficient

// Strategy 3: Store references
let ref_vec: Vec<&String> = vec![&item1, &item2];  // Items must outlive vec
```

### 4. RAII and Drop Semantics
- Values are dropped in **reverse order** of creation (LIFO)
- Moved values don't drop in their original location
- Drop happens deterministically at scope exit
- Custom Drop implementations allow resource cleanup

## 🏆 Advanced Ownership Patterns

### Copy-on-Write Pattern
```rust
use std::borrow::Cow;

fn process_data(input: Cow<str>) -> String {
    if input.contains("modify") {
        // Only clone if we need to modify
        let mut owned = input.into_owned();
        owned.push_str(" - modified");
        owned
    } else {
        // Use borrowed data if no modification needed
        input.to_string()
    }
}
```

### Builder Pattern with Ownership
```rust
struct ConfigBuilder {
    name: Option<String>,
    version: Option<String>,
}

impl ConfigBuilder {
    fn name(mut self, name: String) -> Self {  // Takes self by value
        self.name = Some(name);
        self  // Returns owned self for chaining
    }
    
    fn build(self) -> Result<Config, &'static str> {  // Consumes builder
        Ok(Config {
            name: self.name.ok_or("Name required")?,
            version: self.version.unwrap_or("1.0.0".to_string()),
        })
    }
}
```

## 🎯 Key Learning Achievements

By completing this exercise, you've mastered:

### ✅ **Ownership Fundamentals**
- Three rules of ownership and why they prevent bugs
- Move vs Copy semantics for different types
- When values are dropped and cleaned up

### ✅ **Function Design Patterns**
- When to take ownership vs borrow parameters
- How to return ownership or create new data
- Designing APIs that are both safe and ergonomic

### ✅ **Memory Management**
- Stack vs heap allocation strategies
- RAII pattern for deterministic cleanup
- How Rust prevents use-after-free and double-free

### ✅ **Real-World Patterns**
- Collection ownership strategies
- Builder patterns with move semantics
- Resource management with custom Drop

### ✅ **C# to Rust Translation**
You now understand how to translate C# patterns:
- Reference types → String/Vec with ownership
- Value types → Copy types (i32, bool, etc.)
- Object sharing → Clone or borrowing strategies
- Resource disposal → RAII with Drop trait

## 🚀 Next Steps

**Congratulations!** You've mastered Rust's ownership system. You're ready for:

1. **Module 02 Exercise 2**: Borrowing and References
2. **Module 02 Exercise 3**: Lifetimes and Reference Validity  
3. **Module 02 Exercise 4**: Smart Pointers (Rc, Arc, Box)

The foundation you've built here is essential for all advanced Rust programming! 🦀